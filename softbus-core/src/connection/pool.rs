//! 连接池实现

use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::VecDeque;
use crate::{Error, Result, DeviceId, Channel};

/// 连接池配置
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// 最小连接数
    pub min_size: usize,
    /// 最大连接数
    pub max_size: usize,
    /// 连接超时时间（秒）
    pub timeout_seconds: u64,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_size: 1,
            max_size: 10,
            timeout_seconds: 300,
        }
    }
}

/// 连接池
/// 
/// 维护到特定设备的多个连接，提高并发性能
pub struct ConnectionPool {
    device_id: DeviceId,
    config: PoolConfig,
    connections: Arc<RwLock<VecDeque<Arc<dyn Channel>>>>,
}

impl ConnectionPool {
    /// 创建新的连接池
    pub fn new(device_id: DeviceId, config: PoolConfig) -> Self {
        Self {
            device_id,
            config,
            connections: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    /// 获取一个连接
    pub async fn acquire(&self) -> Result<Arc<dyn Channel>> {
        let mut conns = self.connections.write();
        
        // 如果有可用连接，直接返回
        if let Some(conn) = conns.pop_front() {
            if conn.is_connected() {
                return Ok(conn);
            }
        }

        // TODO: 创建新连接
        Err(Error::Connection("No available connection".to_string()))
    }

    /// 释放连接
    pub async fn release(&self, conn: Arc<dyn Channel>) {
        if !conn.is_connected() {
            return;
        }

        let mut conns = self.connections.write();
        if conns.len() < self.config.max_size {
            conns.push_back(conn);
        } else {
            // 如果池已满，关闭连接
            let _ = conn.close().await;
        }
    }

    /// 获取当前池大小
    pub fn size(&self) -> usize {
        self.connections.read().len()
    }

    /// 清空连接池
    pub async fn clear(&self) {
        let mut conns = self.connections.write();
        for conn in conns.drain(..) {
            let _ = conn.close().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_config() {
        let config = PoolConfig::default();
        assert_eq!(config.min_size, 1);
        assert_eq!(config.max_size, 10);
    }
}
