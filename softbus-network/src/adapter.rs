//! 网络适配器trait定义

use async_trait::async_trait;
use bytes::Bytes;

/// 网络适配器错误
#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("连接失败: {0}")]
    ConnectionFailed(String),

    #[error("发送失败: {0}")]
    SendFailed(String),

    #[error("接收失败: {0}")]
    ReceiveFailed(String),

    #[error("适配器未初始化")]
    NotInitialized,

    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("其他错误: {0}")]
    Other(String),
}

pub type AdapterResult<T> = Result<T, AdapterError>;

/// 网络适配器trait
/// 
/// 定义网络传输层的统一接口
#[async_trait]
pub trait NetworkAdapter: Send + Sync {
    /// 初始化适配器
    async fn initialize(&mut self) -> AdapterResult<()>;

    /// 关闭适配器
    async fn shutdown(&mut self) -> AdapterResult<()>;

    /// 连接到对端
    async fn connect(&self, address: &str) -> AdapterResult<Box<dyn Connection>>;

    /// 开始监听连接
    async fn listen(&self, address: &str) -> AdapterResult<Box<dyn Listener>>;

    /// 适配器是否已初始化
    fn is_initialized(&self) -> bool;

    /// 获取适配器名称
    fn name(&self) -> &str;
}

/// 连接trait
#[async_trait]
pub trait Connection: Send + Sync {
    /// 发送数据
    async fn send(&self, data: Bytes) -> AdapterResult<()>;

    /// 接收数据
    async fn receive(&self) -> AdapterResult<Bytes>;

    /// 关闭连接
    async fn close(&self) -> AdapterResult<()>;

    /// 获取对端地址
    fn peer_address(&self) -> Option<String>;
}

/// 监听器trait
#[async_trait]
pub trait Listener: Send + Sync {
    /// 接受新连接
    async fn accept(&self) -> AdapterResult<Box<dyn Connection>>;

    /// 停止监听
    async fn stop(&self) -> AdapterResult<()>;

    /// 获取监听地址
    fn local_address(&self) -> String;
}
