//! 服务路由器

use std::sync::Arc;
use crate::{Error, Result, ServiceId, ServiceInfo, DeviceId};
use super::ServiceRegistry;

/// 路由策略
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingStrategy {
    /// 轮询
    RoundRobin,
    /// 随机
    Random,
    /// 最近最少使用
    LeastRecentlyUsed,
    /// 本地优先
    LocalFirst,
}

/// 服务路由器
/// 
/// 负责将服务请求路由到合适的服务实例
pub struct ServiceRouter {
    registry: Arc<ServiceRegistry>,
    strategy: RoutingStrategy,
    local_device_id: DeviceId,
}

impl ServiceRouter {
    /// 创建新的服务路由器
    pub fn new(registry: Arc<ServiceRegistry>, local_device_id: DeviceId) -> Self {
        Self {
            registry,
            strategy: RoutingStrategy::LocalFirst,
            local_device_id,
        }
    }

    /// 设置路由策略
    pub fn set_strategy(&mut self, strategy: RoutingStrategy) {
        self.strategy = strategy;
    }

    /// 路由服务请求
    pub fn route(&self, service_name: &str) -> Result<ServiceInfo> {
        let services = self.registry.find_by_name(service_name);
        
        if services.is_empty() {
            return Err(Error::ServiceNotFound(service_name.to_string()));
        }

        match self.strategy {
            RoutingStrategy::LocalFirst => self.route_local_first(&services),
            RoutingStrategy::RoundRobin => self.route_round_robin(&services),
            RoutingStrategy::Random => self.route_random(&services),
            RoutingStrategy::LeastRecentlyUsed => self.route_lru(&services),
        }
    }

    fn route_local_first(&self, services: &[ServiceInfo]) -> Result<ServiceInfo> {
        // 优先选择本地服务
        for service in services {
            if service.device_id == self.local_device_id {
                return Ok(service.clone());
            }
        }
        
        // 如果没有本地服务，返回第一个
        Ok(services[0].clone())
    }

    fn route_round_robin(&self, services: &[ServiceInfo]) -> Result<ServiceInfo> {
        // TODO: 实现真正的轮询逻辑
        Ok(services[0].clone())
    }

    fn route_random(&self, services: &[ServiceInfo]) -> Result<ServiceInfo> {
        // TODO: 实现随机选择
        Ok(services[0].clone())
    }

    fn route_lru(&self, services: &[ServiceInfo]) -> Result<ServiceInfo> {
        // TODO: 实现LRU策略
        Ok(services[0].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_service_router() {
        let registry = Arc::new(ServiceRegistry::new());
        let device_id = DeviceId::new();
        let router = ServiceRouter::new(registry.clone(), device_id.clone());

        let service = ServiceInfo {
            service_id: ServiceId::new(),
            service_name: "test_service".to_string(),
            device_id: device_id.clone(),
            methods: vec!["method1".to_string()],
            metadata: HashMap::new(),
        };

        registry.register(service).unwrap();
        
        let result = router.route("test_service");
        assert!(result.is_ok());
    }
}
