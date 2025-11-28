//! 连接管理器

use std::sync::Arc;
use dashmap::DashMap;
use crate::{Error, Result, DeviceId, Channel};

/// 连接管理器
/// 
/// 负责管理与不同设备的连接
pub struct ConnectionManager {
    connections: Arc<DashMap<DeviceId, Arc<dyn Channel>>>,
}

impl ConnectionManager {
    /// 创建新的连接管理器
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
        }
    }

    /// 添加连接
    pub fn add_connection(&self, device_id: DeviceId, channel: Arc<dyn Channel>) {
        self.connections.insert(device_id, channel);
    }

    /// 获取连接
    pub fn get_connection(&self, device_id: &DeviceId) -> Option<Arc<dyn Channel>> {
        self.connections.get(device_id).map(|entry| Arc::clone(entry.value()))
    }

    /// 移除连接
    pub fn remove_connection(&self, device_id: &DeviceId) -> Option<Arc<dyn Channel>> {
        self.connections.remove(device_id).map(|(_, v)| v)
    }

    /// 获取所有连接的设备ID
    pub fn list_devices(&self) -> Vec<DeviceId> {
        self.connections.iter().map(|entry| entry.key().clone()).collect()
    }

    /// 获取连接数量
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// 清空所有连接
    pub async fn clear(&self) {
        for entry in self.connections.iter() {
            let _ = entry.value().close().await;
        }
        self.connections.clear();
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_manager() {
        let manager = ConnectionManager::new();
        assert_eq!(manager.connection_count(), 0);
    }
}
