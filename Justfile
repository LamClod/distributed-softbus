# 使用 just 命令运行任务

# 默认任务：显示帮助
default:
    @just --list

# 构建所有模块
build:
    cargo build --workspace

# 构建Release版本
build-release:
    cargo build --workspace --release

# 运行所有测试
test:
    cargo test --workspace

# 运行单个模块测试
test-core:
    cargo test -p softbus-core

# 代码格式化
fmt:
    cargo fmt --all

# 代码检查
check:
    cargo clippy --workspace -- -D warnings

# 生成文档
docs:
    cargo doc --workspace --no-deps --open

# 运行相机服务示例
run-camera-server:
    cargo run --bin camera-server

# 运行相机客户端
run-camera-client:
    cargo run --bin camera-client

# 性能测试
bench:
    cargo bench --workspace

# 清理构建产物
clean:
    cargo clean
    rm -rf native/linux/build
    rm -rf native/windows/bin

# 安装IDL编译器
install-idl-compiler:
    cargo install --path softbus-idl-compiler

# 编译IDL文件
compile-idl file:
    softbus-idl-compiler {{file}} --output generated/

# 交叉编译到Android
build-android:
    cargo build --target aarch64-linux-android --release

# 运行完整CI流程
ci: fmt check test build-release
