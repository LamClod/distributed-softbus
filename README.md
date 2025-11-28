# 分布式软总线系统 - 完整设计稿

## 📋 项目概述

本设计稿详细描述了一个基于**虚拟总线模型**的分布式设备互联框架，使用**Rust**作为核心实现语言，底层通过**C#**和**C/C++**封装平台原生API，旨在为多设备协同提供透明、高效、安全的通信基础设施。

### 核心理念

将多个物理设备抽象为一个逻辑上的"超级设备"，应用程序像调用本地函数一样调用远程设备的服务，无需关心：
- 设备的物理位置
- 底层传输协议（BLE、Wi-Fi Direct、以太网）
- 网络拓扑变化
- 连接建立与维护

## 🎯 设计目标

1. **位置透明性** - 应用无需感知服务位于本地还是远程
2. **传输无关性** - 框架自动选择并管理最优网络链路
3. **协议统一性** - 提供标准化的服务发现与RPC接口
4. **高性能** - 零拷贝传输，延迟<50ms（局域网）
5. **安全性** - 端到端加密，设备身份认证

## 📚 文档结构

本设计稿包含以下核心文档：

### 1️⃣ [设计概述.md](./设计概述.md)
- 项目总体介绍
- 技术栈选型理由
- 系统架构分层图
- 核心模块概览
- 开发路线图

### 2️⃣ [架构设计.md](./架构设计.md)
- 网络抽象层详细设计（NAL）
- BLE/Wi-Fi/mDNS适配器实现
- 连接管理器设计
- 服务路由器架构
- RPC引擎与IDL系统
- 安全认证模块
- 性能优化策略

### 3️⃣ [实施指南.md](./实施指南.md)
- 开发环境配置
- 完整代码实现示例
- 服务定义与使用
- 编译与部署流程
- 性能测试方法
- 调试技巧

### 4️⃣ [项目结构.md](./项目结构.md)
- 完整目录树
- 模块依赖关系
- Cargo配置文件
- 构建脚本
- CI/CD配置

### 5️⃣ [API参考.md](./API参考.md)
- Softbus主API
- 配置API
- 服务定义API
- 网络API
- 安全API
- 事件API
- 测试工具API

### 6️⃣ [FAQ.md](./FAQ.md)
- 常见问题解答
- 故障排查指南
- 性能优化技巧
- 安全最佳实践

## 🏗️ 系统架构概览

```
┌─────────────────────────────────────────────────┐
│           应用层 (Application Layer)             │
│        IDL生成的Proxy/Stub透明调用               │
└─────────────────────────────────────────────────┘
                      ▲
                      │
┌─────────────────────────────────────────────────┐
│            核心层 (Core Layer) - Rust            │
│  ┌─────────────┬──────────────┬───────────────┐ │
│  │ 连接管理器   │  服务路由器   │  安全认证模块  │ │
│  └─────────────┴──────────────┴───────────────┘ │
│  ┌─────────────────────────────────────────────┐ │
│  │        传输仲裁器 (Transport Arbiter)       │ │
│  └─────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
                      ▲
                      │
┌─────────────────────────────────────────────────┐
│      网络抽象层 (NAL) - Rust + C#/C++           │
│  ┌──────────┬────────────┬──────────────────┐  │
│  │ BLE适配器 │ WiFi适配器  │  mDNS适配器     │  │
│  └──────────┴────────────┴──────────────────┘  │
└─────────────────────────────────────────────────┘
                      ▲
                      │
┌─────────────────────────────────────────────────┐
│         平台原生层 (Platform Native)             │
│  Windows: C# WinRT / Linux: C/C++ BlueZ         │
└─────────────────────────────────────────────────┘
```

## 🔑 核心特性

### 1. 虚拟总线抽象

```rust
// 应用程序视角：像调用本地服务一样简单
let camera_proxy = CameraServiceProxy::new(softbus.router());
let photo = camera_proxy.take_picture(request).await?;
save_photo(photo);
```

### 2. 自动传输仲裁

框架智能选择最优链路：
- **小数据包** (<1KB) → BLE（低功耗）
- **大文件传输** → Wi-Fi Direct（高带宽）
- **局域网** → mDNS + TCP（低延迟）

### 3. 零配置服务发现

```rust
// 无需手动配对，自动发现邻近设备的服务
let services = softbus.discover_service(&service_id).await?;
```

### 4. IDL驱动开发

```idl
// 定义服务接口
service CameraService {
    rpc TakePicture(Request) returns (Response);
}

// 编译器自动生成Proxy和Stub代码
// 开发者只需实现业务逻辑
```

## 🛠️ 技术栈

| 层次 | 技术 | 用途 |
|------|------|------|
| 核心引擎 | Rust (1.75+) | 主框架实现 |
| 异步运行时 | Tokio | 网络IO、任务调度 |
| 序列化 | Protobuf + Serde | RPC消息编解码 |
| Windows适配 | C# + WinRT | 蓝牙、Wi-Fi Direct API |
| Linux适配 | C/C++ | BlueZ、NetworkManager |
| 加密 | Ring + Rustls | TLS 1.3、设备认证 |
| BLE库 | btleplug | 跨平台蓝牙抽象 |
| 服务发现 | mdns-sd | 零配置网络发现 |

## 📊 性能指标

| 指标 | 目标值 | 说明 |
|------|--------|------|
| 设备发现延迟 | < 500ms | BLE广播周期优化 |
| RPC调用延迟 | < 50ms | 局域网Wi-Fi环境 |
| 链路切换时间 | < 200ms | BLE → Wi-Fi升级 |
| 吞吐量 | > 100 Mbps | Wi-Fi Direct链路 |
| 内存占用 | < 50MB | 支持10个并发连接 |
| CPU占用 | < 5% | 空闲状态 |

## 🚀 快速开始

### 环境准备

```bash
# 安装Rust工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Linux依赖
sudo apt install build-essential libbluetooth-dev

# Windows依赖
winget install Microsoft.DotNet.SDK.8
```

### 创建项目

```bash
# 克隆模板（假设）
git clone https://github.com/yourorg/softbus-template my-app
cd my-app

# 构建
cargo build --release

# 运行示例
cargo run --example camera-service
```

### 最小示例

**服务端**:
```rust
use softbus_core::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let softbus = Softbus::new().await?;
    softbus.start().await?;
    
    // 注册服务
    let service = Arc::new(MyServiceImpl);
    let stub = MyServiceStub::new(service);
    softbus.register_service(
        ServiceId::new("com.example.MyService"),
        Box::new(stub)
    ).await?;
    
    println!("Service started");
    tokio::signal::ctrl_c().await?;
    Ok(())
}
```

**客户端**:
```rust
use softbus_core::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let softbus = Softbus::new().await?;
    softbus.start().await?;
    
    // 创建代理并调用
    let proxy = MyServiceProxy::new(softbus.router());
    let response = proxy.my_method(request).await?;
    
    println!("Response: {:?}", response);
    Ok(())
}
```

## 📖 使用场景

### 1. 跨设备拍照编辑
- 手机拍照
- 平板实时显示并编辑
- 透明调用相机服务

### 2. 智能手表控制
- 手表控制手机音乐播放
- 低功耗BLE保持连接
- 必要时切换到Wi-Fi

### 3. 多设备文件同步
- 3台设备间文件共享
- 自动选择最快链路
- 断点续传支持

### 4. IoT设备协同
- 智能家居设备互联
- 零配置自动发现
- 安全设备认证

## 🔐 安全特性

- ✅ **TLS 1.3** 端到端加密
- ✅ **Ed25519** 设备身份验证
- ✅ **公钥基础设施** (PKI)
- ✅ **设备白名单** 机制
- ✅ **证书固定** (Certificate Pinning)
- ✅ **安全密钥轮换**

## 🧪 测试覆盖

```bash
# 单元测试
cargo test --workspace

# 集成测试
cargo test --test integration_test

# 性能基准测试
cargo bench

# 代码覆盖率
cargo tarpaulin --workspace --out Html
```

## 📈 开发路线图

### Phase 1: 基础框架 ✅
- [x] 网络抽象层接口
- [x] BLE适配器（Rust）
- [x] 基础连接管理

### Phase 2: 服务发现 🚧
- [ ] mDNS集成
- [ ] 分布式路由表
- [ ] 服务注册/发现API

### Phase 3: RPC引擎 📅
- [ ] IDL编译器
- [ ] Proxy/Stub生成
- [ ] 异步调用框架

### Phase 4: 高级特性 📅
- [ ] 传输仲裁器
- [ ] 安全认证模块
- [ ] QoS保证
- [ ] 故障恢复

### Phase 5: 生产就绪 📅
- [ ] 完整测试覆盖
- [ ] 性能优化
- [ ] 文档完善
- [ ] 示例应用

## 🤝 贡献指南

详见各模块的实现细节和代码示例。本设计稿提供了：
- 完整的架构设计
- 详细的接口定义
- 可运行的代码示例
- 性能优化策略
- 安全最佳实践

## 📄 许可证

MIT OR Apache-2.0

---

## 📞 联系方式

- **问题反馈**: 参见 [FAQ.md](./FAQ.md)
- **设计讨论**: 查看 [架构设计.md](./架构设计.md)
- **实现细节**: 阅读 [实施指南.md](./实施指南.md)

---

**版本**: v0.1.0  
**最后更新**: 2024-11-28  
**文档语言**: 简体中文

---

## 🎓 学习路径建议

1. **新手**: 从 `设计概述.md` 开始，了解整体架构
2. **开发者**: 阅读 `实施指南.md`，运行示例代码
3. **架构师**: 深入 `架构设计.md`，理解设计决策
4. **运维**: 查看 `FAQ.md`，了解部署和故障排查

## ⭐ 设计亮点

### 1. 语言选择的智慧
- **Rust核心**: 内存安全、零成本抽象、并发友好
- **C#/C++底层**: 直接利用平台成熟API，避免重复造轮
- **FFI桥接**: 性能损耗<1%，最佳实践

### 2. 虚拟总线模型
灵感来源于PCIe总线，实现真正的位置透明：
```
CPU → PCIe总线 → 显卡
应用 → 软总线 → 远程服务
```

### 3. 传输仲裁智能化
不是简单的协议选择，而是基于：
- 实时链路质量评估
- 数据包大小
- 功耗预算
- QoS要求

### 4. IDL驱动的开发体验
一次定义，自动生成：
- Rust Proxy/Stub
- C# 绑定（可选）
- C++ 绑定（可选）
- API文档

---

**开始探索**: 建议从 [设计概述.md](./设计概述.md) 开始阅读完整设计稿！
