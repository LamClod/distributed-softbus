//! SoftBus Core Library
//! 
//! 提供分布式软总线的核心功能，包括服务注册、发现、RPC调用等。

pub mod error;
pub mod types;
pub mod channel;
pub mod connection;
pub mod service;
pub mod rpc;
pub mod security;
pub mod arbiter;

// 重新导出常用类型
pub use error::{Error, Result};
pub use types::*;
pub use channel::Channel;

/// 库版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
