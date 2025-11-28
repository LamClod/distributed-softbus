#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
GitHub 仓库上传脚本
使用Personal Access Token安全上传项目到GitHub

使用方法:
    python upload_to_github.py

创建Personal Access Token:
    1. 访问 https://github.com/settings/tokens
    2. 点击 "Generate new token (classic)"
    3. 选择权限: repo (完整控制)
    4. 生成并复制token
"""

import os
import subprocess
import sys
import getpass
from pathlib import Path

class GitHubUploader:
    def __init__(self):
        self.project_root = Path(__file__).parent.parent
        self.username = None
        self.token = None
        self.repo_name = None
        
    def run_command(self, cmd, check=True, capture_output=True):
        """执行命令并返回结果"""
        try:
            result = subprocess.run(
                cmd,
                cwd=self.project_root,
                shell=True,
                check=check,
                capture_output=capture_output,
                text=True,
                encoding='utf-8',
                errors='ignore'  # 忽略编码错误
            )
            if capture_output and result.stdout:
                return result.stdout.strip()
            return None
        except subprocess.CalledProcessError as e:
            if capture_output and hasattr(e, 'stderr') and e.stderr:
                print(f"❌ 命令执行失败: {e.stderr}")
            raise
    
    def check_git_installed(self):
        """检查git是否已安装"""
        try:
            self.run_command("git --version")
            print("✅ Git 已安装")
            return True
        except:
            print("❌ Git 未安装，请先安装 Git")
            print("   下载地址: https://git-scm.com/downloads")
            return False
    
    def get_user_input(self):
        """获取用户输入"""
        print("\n" + "="*60)
        print("GitHub 仓库配置".center(60))
        print("="*60 + "\n")
        
        # 获取用户名（邮箱）
        self.username = input("请输入GitHub用户名或邮箱: ").strip()
        if not self.username:
            print("❌ 用户名不能为空")
            return False
        
        # 获取仓库名
        print(f"\n推荐仓库名: distributed-softbus")
        self.repo_name = input("请输入仓库名 (直接回车使用推荐名): ").strip()
        if not self.repo_name:
            self.repo_name = "distributed-softbus"
        
        # 获取Token
        print("\n⚠️  重要: 请输入GitHub Personal Access Token (不是密码!)")
        print("   创建Token: https://github.com/settings/tokens")
        print("   需要权限: repo (完整控制)")
        self.token = getpass.getpass("请输入Token (输入时不显示): ").strip()
        
        if not self.token:
            print("❌ Token 不能为空")
            return False
        
        # 确认信息
        print("\n" + "-"*60)
        print("请确认以下信息:")
        print(f"  用户名/邮箱: {self.username}")
        print(f"  仓库名: {self.repo_name}")
        print(f"  Token: {'*' * 20} (已隐藏)")
        print("-"*60)
        
        confirm = input("\n确认信息正确? (y/n): ").lower()
        return confirm == 'y'
    
    def init_git_repo(self):
        """初始化Git仓库"""
        print("\n📦 初始化Git仓库...")
        
        git_dir = self.project_root / ".git"
        if git_dir.exists():
            print("   Git仓库已存在")
        else:
            self.run_command("git init", capture_output=False)
            self.run_command("git branch -M main", capture_output=False)
            print("   ✅ Git仓库初始化完成")
    
    def add_files(self):
        """添加文件到Git"""
        print("\n📝 添加文件到Git...")
        try:
            self.run_command("git add .", capture_output=False)
            print("   ✅ 文件已添加")
        except subprocess.CalledProcessError as e:
            print(f"   ⚠️  部分文件添加可能失败，继续...")
            pass
    
    def create_commit(self):
        """创建提交"""
        print("\n💾 创建提交...")
        
        # 配置用户信息
        self.run_command(f'git config user.email "{self.username}"')
        username_part = self.username.split("@")[0] if "@" in self.username else self.username
        self.run_command(f'git config user.name "{username_part}"')
        
        # 使用简单的提交信息避免多行问题
        commit_message = "Initial commit: 完整的分布式软总线项目框架"
        
        try:
            self.run_command(f'git commit -m "{commit_message}"', capture_output=False)
            print("   ✅ 提交已创建")
        except subprocess.CalledProcessError:
            # 检查是否已经有提交
            try:
                self.run_command("git rev-parse HEAD")
                print("   ℹ️  已存在提交，跳过创建")
            except:
                raise
    
    def check_github_repo_exists(self):
        """检查GitHub仓库是否存在"""
        print("\n🔍 检查GitHub仓库...")
        
        # 提取用户名
        if '@' in self.username:
            print("   ⚠️  使用邮箱登录，需要手动在GitHub创建仓库")
            github_username = input("   请输入GitHub用户名: ").strip()
        else:
            github_username = self.username
        
        self.github_username = github_username
        
        print(f"\n   请确保已在GitHub创建仓库:")
        print(f"   🔗 https://github.com/{github_username}/{self.repo_name}")
        print(f"\n   如果还未创建，请访问:")
        print(f"   🔗 https://github.com/new")
        print(f"   - Repository name: {self.repo_name}")
        print(f"   - 不要勾选 'Initialize this repository with a README'")
        
        confirm = input("\n   仓库已创建? (y/n): ").lower()
        return confirm == 'y'
    
    def add_remote(self):
        """添加远程仓库"""
        print("\n🌐 配置远程仓库...")
        
        # 构造带token的URL
        repo_url = f"https://{self.token}@github.com/{self.github_username}/{self.repo_name}.git"
        
        # 检查是否已有remote
        try:
            self.run_command("git remote get-url origin")
            print("   更新远程仓库URL...")
            self.run_command(f'git remote set-url origin "{repo_url}"', capture_output=False)
        except:
            print("   添加远程仓库...")
            self.run_command(f'git remote add origin "{repo_url}"', capture_output=False)
        
        print("   ✅ 远程仓库已配置")
    
    def push_to_github(self):
        """推送到GitHub"""
        print("\n🚀 推送到GitHub...")
        print("   这可能需要几分钟时间...")
        
        try:
            # 推送到main分支
            self.run_command("git push -u origin main", capture_output=False)
            print("\n   ✅ 推送成功!")
            return True
        except subprocess.CalledProcessError:
            print("\n   ❌ 推送失败，可能的原因:")
            print("      1. Token权限不足")
            print("      2. 仓库不存在")
            print("      3. 网络问题")
            return False
    
    def cleanup_credentials(self):
        """清理URL中的token"""
        print("\n🔒 清理凭证...")
        try:
            clean_url = f"https://github.com/{self.github_username}/{self.repo_name}.git"
            self.run_command(f'git remote set-url origin "{clean_url}"', capture_output=False)
            print("   ✅ 凭证已清理")
        except:
            pass
    
    def show_success_info(self):
        """显示成功信息"""
        print("\n" + "="*60)
        print("🎉 上传成功!".center(60))
        print("="*60)
        print(f"\n📦 仓库地址: https://github.com/{self.github_username}/{self.repo_name}")
        print(f"\n💡 下次更新代码，使用以下命令:")
        print(f"   cd {self.project_root}")
        print(f"   git add .")
        print(f"   git commit -m \"你的提交信息\"")
        print(f"   git push")
        print("\n" + "="*60 + "\n")
    
    def run(self):
        """主流程"""
        print("\n" + "🌟 "*30)
        print("分布式软总线 - GitHub上传工具".center(60))
        print("🌟 "*30 + "\n")
        
        # 检查Git
        if not self.check_git_installed():
            return 1
        
        # 获取用户输入
        if not self.get_user_input():
            print("\n❌ 配置取消")
            return 1
        
        try:
            # 初始化仓库
            self.init_git_repo()
            
            # 添加文件
            self.add_files()
            
            # 创建提交
            self.create_commit()
            
            # 检查GitHub仓库
            if not self.check_github_repo_exists():
                print("\n❌ 请先在GitHub创建仓库")
                return 1
            
            # 添加远程仓库
            self.add_remote()
            
            # 推送
            if self.push_to_github():
                self.cleanup_credentials()
                self.show_success_info()
                return 0
            else:
                return 1
                
        except KeyboardInterrupt:
            print("\n\n❌ 操作已取消")
            return 1
        except Exception as e:
            print(f"\n❌ 发生错误: {e}")
            import traceback
            traceback.print_exc()
            return 1

def main():
    """主函数"""
    uploader = GitHubUploader()
    sys.exit(uploader.run())

if __name__ == "__main__":
    main()
