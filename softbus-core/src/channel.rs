//! 虚拟通道trait定义

use async_trait::async_trait;
use bytes::Bytes;
use crate::{Error, Result, QosLevel};

/// 虚拟通道trait
/// 
/// 定义了一个抽象的通信通道接口，底层可以是BLE、Wi-Fi Direct等不同的传输协议
#[async_trait]
pub trait Channel: Send + Sync {
    /// 发送数据
    async fn send(&self, data: Bytes) -> Result<()>;

    /// 接收数据
    async fn recv(&self) -> Result<Bytes>;

    /// 关闭通道
    async fn close(&self) -> Result<()>;

    /// 检查通道是否已连接
    fn is_connected(&self) -> bool;

    /// 获取通道的QoS级别
    fn qos_level(&self) -> QosLevel;

    /// 获取对端设备ID
    fn peer_device_id(&self) -> Option<String>;

    /// 设置通道参数
    async fn set_options(&self, options: ChannelOptions) -> Result<()>;
}

/// 通道选项
#[derive(Debug, Clone)]
pub struct ChannelOptions {
    /// 发送缓冲区大小
    pub send_buffer_size: Option<usize>,
    /// 接收缓冲区大小
    pub recv_buffer_size: Option<usize>,
    /// 超时时间（毫秒）
    pub timeout_ms: Option<u64>,
    /// 是否启用压缩
    pub enable_compression: Option<bool>,
}

impl Default for ChannelOptions {
    fn default() -> Self {
        Self {
            send_buffer_size: Some(64 * 1024),
            recv_buffer_size: Some(64 * 1024),
            timeout_ms: Some(5000),
            enable_compression: Some(false),
        }
    }
}
