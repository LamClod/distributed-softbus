//! 相机客户端示例

use softbus_core::*;
use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("camera_client=debug,softbus=debug")
        .init();

    info!("Starting camera service client...");

    // TODO: 实现客户端逻辑
    // 1. 发现CameraService
    // 2. 连接到服务
    // 3. 调用远程方法

    info!("Calling remote camera service...");
    
    // 示例：列出相机
    // let cameras = camera_proxy.list_cameras().await?;
    // info!("Available cameras: {:?}", cameras);

    Ok(())
}
