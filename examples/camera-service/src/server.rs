//! 相机服务端示例

use softbus_core::*;
use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("camera_server=debug,softbus=debug")
        .init();

    info!("Starting camera service server...");

    // TODO: 实现服务端逻辑
    // 1. 初始化SoftBus核心
    // 2. 注册CameraService
    // 3. 启动服务监听

    info!("Camera service server is ready");
    
    // 保持运行
    tokio::signal::ctrl_c().await?;
    info!("Shutting down camera service server");

    Ok(())
}
