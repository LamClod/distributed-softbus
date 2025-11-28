//! mDNS适配器实现

use async_trait::async_trait;
use crate::adapter::{NetworkAdapter, AdapterResult, AdapterError, Connection, Listener};

/// mDNS适配器
/// 
/// 用于设备发现和服务注册
pub struct MdnsAdapter {
    initialized: bool,
    name: String,
    service_type: String,
}

impl MdnsAdapter {
    /// 创建新的mDNS适配器
    pub fn new(service_type: String) -> Self {
        Self {
            initialized: false,
            name: "mDNS".to_string(),
            service_type,
        }
    }

    /// 注册服务
    pub async fn register_service(
        &self,
        service_name: &str,
        port: u16,
    ) -> AdapterResult<()> {
        if !self.initialized {
            return Err(AdapterError::NotInitialized);
        }
        
        // TODO: 使用mdns-sd库注册服务
        tracing::info!("Registering mDNS service: {} on port {}", service_name, port);
        Ok(())
    }

    /// 发现服务
    pub async fn discover_services(&self) -> AdapterResult<Vec<String>> {
        if !self.initialized {
            return Err(AdapterError::NotInitialized);
        }
        
        // TODO: 使用mdns-sd库发现服务
        tracing::info!("Discovering mDNS services of type: {}", self.service_type);
        Ok(Vec::new())
    }
}

#[async_trait]
impl NetworkAdapter for MdnsAdapter {
    async fn initialize(&mut self) -> AdapterResult<()> {
        tracing::info!("Initializing mDNS adapter");
        self.initialized = true;
        Ok(())
    }

    async fn shutdown(&mut self) -> AdapterResult<()> {
        tracing::info!("Shutting down mDNS adapter");
        self.initialized = false;
        Ok(())
    }

    async fn connect(&self, _address: &str) -> AdapterResult<Box<dyn Connection>> {
        Err(AdapterError::Other("mDNS does not support connections".to_string()))
    }

    async fn listen(&self, _address: &str) -> AdapterResult<Box<dyn Listener>> {
        Err(AdapterError::Other("mDNS does not support listening".to_string()))
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mdns_adapter_initialization() {
        let mut adapter = MdnsAdapter::new("_softbus._tcp".to_string());
        assert!(!adapter.is_initialized());
        
        adapter.initialize().await.unwrap();
        assert!(adapter.is_initialized());
    }
}
