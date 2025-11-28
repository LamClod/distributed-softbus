//! BLE模块测试

use softbus_network::ble::BleAdapter;
use softbus_network::NetworkAdapter;

#[tokio::test]
async fn test_ble_adapter_lifecycle() {
    let mut adapter = BleAdapter::new();
    
    // 初始化
    adapter.initialize().await.unwrap();
    assert!(adapter.is_initialized());
    assert_eq!(adapter.name(), "BLE");
    
    // 关闭
    adapter.shutdown().await.unwrap();
    assert!(!adapter.is_initialized());
}
