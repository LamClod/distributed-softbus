//! Wi-Fi Direct适配器实现

use async_trait::async_trait;
use crate::adapter::{NetworkAdapter, AdapterResult, AdapterError, Connection, Listener};

/// Wi-Fi Direct适配器
pub struct WiFiDirectAdapter {
    initialized: bool,
    name: String,
}

impl WiFiDirectAdapter {
    /// 创建新的Wi-Fi Direct适配器
    pub fn new() -> Self {
        Self {
            initialized: false,
            name: "WiFi-Direct".to_string(),
        }
    }
}

impl Default for WiFiDirectAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NetworkAdapter for WiFiDirectAdapter {
    async fn initialize(&mut self) -> AdapterResult<()> {
        // TODO: 初始化Wi-Fi Direct
        tracing::info!("Initializing Wi-Fi Direct adapter");
        super::ffi::wifi_direct_init()
            .map_err(|e| AdapterError::Other(e))?;
        self.initialized = true;
        Ok(())
    }

    async fn shutdown(&mut self) -> AdapterResult<()> {
        // TODO: 关闭Wi-Fi Direct
        tracing::info!("Shutting down Wi-Fi Direct adapter");
        super::ffi::wifi_direct_cleanup()
            .map_err(|e| AdapterError::Other(e))?;
        self.initialized = false;
        Ok(())
    }

    async fn connect(&self, address: &str) -> AdapterResult<Box<dyn Connection>> {
        if !self.initialized {
            return Err(AdapterError::NotInitialized);
        }
        
        // TODO: 实现Wi-Fi Direct连接
        tracing::info!("Connecting to Wi-Fi Direct device: {}", address);
        Err(AdapterError::Other("Not implemented".to_string()))
    }

    async fn listen(&self, address: &str) -> AdapterResult<Box<dyn Listener>> {
        if !self.initialized {
            return Err(AdapterError::NotInitialized);
        }
        
        // TODO: 实现Wi-Fi Direct监听
        tracing::info!("Starting Wi-Fi Direct listener on: {}", address);
        Err(AdapterError::Other("Not implemented".to_string()))
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
    async fn test_wifi_direct_adapter_creation() {
        let adapter = WiFiDirectAdapter::new();
        assert!(!adapter.is_initialized());
        assert_eq!(adapter.name(), "WiFi-Direct");
    }
}
