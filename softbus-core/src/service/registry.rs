//! 服务注册表

use std::sync::Arc;
use dashmap::DashMap;
use crate::{Error, Result, ServiceId, ServiceInfo, DeviceId};

/// 服务注册表
/// 
/// 管理本地和远程服务的注册信息
pub struct ServiceRegistry {
    /// 服务ID -> 服务信息
    services: Arc<DashMap<ServiceId, ServiceInfo>>,
    /// 服务名称 -> 服务ID列表
    name_index: Arc<DashMap<String, Vec<ServiceId>>>,
    /// 设备ID -> 服务ID列表
    device_index: Arc<DashMap<DeviceId, Vec<ServiceId>>>,
}

impl ServiceRegistry {
    /// 创建新的服务注册表
    pub fn new() -> Self {
        Self {
            services: Arc::new(DashMap::new()),
            name_index: Arc::new(DashMap::new()),
            device_index: Arc::new(DashMap::new()),
        }
    }

    /// 注册服务
    pub fn register(&self, service: ServiceInfo) -> Result<()> {
        let service_id = service.service_id.clone();
        let service_name = service.service_name.clone();
        let device_id = service.device_id.clone();

        // 添加到主索引
        self.services.insert(service_id.clone(), service);

        // 添加到名称索引
        self.name_index
            .entry(service_name)
            .or_insert_with(Vec::new)
            .push(service_id.clone());

        // 添加到设备索引
        self.device_index
            .entry(device_id)
            .or_insert_with(Vec::new)
            .push(service_id);

        Ok(())
    }

    /// 注销服务
    pub fn unregister(&self, service_id: &ServiceId) -> Result<()> {
        if let Some((_, service)) = self.services.remove(service_id) {
            // 从名称索引中移除
            if let Some(mut ids) = self.name_index.get_mut(&service.service_name) {
                ids.retain(|id| id != service_id);
            }

            // 从设备索引中移除
            if let Some(mut ids) = self.device_index.get_mut(&service.device_id) {
                ids.retain(|id| id != service_id);
            }

            Ok(())
        } else {
            Err(Error::ServiceNotFound(service_id.to_string()))
        }
    }

    /// 根据服务ID查找服务
    pub fn find_by_id(&self, service_id: &ServiceId) -> Option<ServiceInfo> {
        self.services.get(service_id).map(|entry| entry.value().clone())
    }

    /// 根据服务名称查找服务
    pub fn find_by_name(&self, service_name: &str) -> Vec<ServiceInfo> {
        if let Some(ids) = self.name_index.get(service_name) {
            ids.iter()
                .filter_map(|id| self.find_by_id(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 根据设备ID查找服务
    pub fn find_by_device(&self, device_id: &DeviceId) -> Vec<ServiceInfo> {
        if let Some(ids) = self.device_index.get(device_id) {
            ids.iter()
                .filter_map(|id| self.find_by_id(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 列出所有服务
    pub fn list_all(&self) -> Vec<ServiceInfo> {
        self.services.iter().map(|entry| entry.value().clone()).collect()
    }

    /// 获取服务数量
    pub fn count(&self) -> usize {
        self.services.len()
    }

    /// 清空所有服务
    pub fn clear(&self) {
        self.services.clear();
        self.name_index.clear();
        self.device_index.clear();
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_service_registry() {
        let registry = ServiceRegistry::new();
        
        let service = ServiceInfo {
            service_id: ServiceId::new(),
            service_name: "test_service".to_string(),
            device_id: DeviceId::new(),
            methods: vec!["method1".to_string()],
            metadata: HashMap::new(),
        };

        registry.register(service.clone()).unwrap();
        assert_eq!(registry.count(), 1);

        let found = registry.find_by_name("test_service");
        assert_eq!(found.len(), 1);
    }
}
