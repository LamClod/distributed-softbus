//! SoftBus Network Layer
//! 
//! 提供网络传输的抽象接口和具体实现

pub mod adapter;
pub mod ble;
pub mod wifi;
pub mod mdns;

pub use adapter::NetworkAdapter;

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
