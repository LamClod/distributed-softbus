//! 错误类型定义

use thiserror::Error;

/// SoftBus错误类型
#[derive(Error, Debug)]
pub enum Error {
    #[error("网络错误: {0}")]
    Network(String),

    #[error("序列化错误: {0}")]
    Serialization(String),

    #[error("服务未找到: {0}")]
    ServiceNotFound(String),

    #[error("方法未找到: {0}")]
    MethodNotFound(String),

    #[error("连接错误: {0}")]
    Connection(String),

    #[error("超时")]
    Timeout,

    #[error("认证失败: {0}")]
    Authentication(String),

    #[error("加密错误: {0}")]
    Encryption(String),

    #[error("内部错误: {0}")]
    Internal(String),

    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("其他错误: {0}")]
    Other(String),
}

/// SoftBus Result类型
pub type Result<T> = std::result::Result<T, Error>;

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}
