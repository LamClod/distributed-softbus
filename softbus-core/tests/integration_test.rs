//! 集成测试

use softbus_core::*;

#[test]
fn test_library_basics() {
    // 测试基本类型创建
    let device_id = DeviceId::new();
    assert!(!device_id.as_str().is_empty());

    let service_id = ServiceId::new();
    assert!(!service_id.as_str().is_empty());
}

#[test]
fn test_service_registry() {
    use std::collections::HashMap;
    use service::ServiceRegistry;

    let registry = ServiceRegistry::new();
    
    let service = ServiceInfo {
        service_id: ServiceId::new(),
        service_name: "test_service".to_string(),
        device_id: DeviceId::new(),
        methods: vec!["method1".to_string(), "method2".to_string()],
        metadata: HashMap::new(),
    };

    registry.register(service.clone()).unwrap();
    
    let found = registry.find_by_name("test_service");
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].service_name, "test_service");
}
