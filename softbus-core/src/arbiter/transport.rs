//! 传输仲裁器

use std::sync::Arc;
use crate::{Error, Result, QosLevel, Channel, DeviceId};

/// 传输类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportType {
    Ble,
    WiFiDirect,
    Tcp,
}

/// 传输能力
#[derive(Debug, Clone)]
pub struct TransportCapability {
    pub transport_type: TransportType,
    pub max_bandwidth: u64,      // 最大带宽（字节/秒）
    pub latency_ms: u32,          // 延迟（毫秒）
    pub power_consumption: u8,    // 功耗等级（0-100）
    pub available: bool,          // 是否可用
}

/// 传输仲裁器
/// 
/// 根据QoS需求和网络状况，选择最佳的传输通道
pub struct TransportArbiter {
    capabilities: Vec<TransportCapability>,
}

impl TransportArbiter {
    /// 创建新的传输仲裁器
    pub fn new() -> Self {
        Self {
            capabilities: Vec::new(),
        }
    }

    /// 添加传输能力
    pub fn add_capability(&mut self, capability: TransportCapability) {
        self.capabilities.push(capability);
    }

    /// 根据QoS选择最佳传输
    pub fn select_transport(&self, qos: QosLevel) -> Result<TransportType> {
        let available_transports: Vec<&TransportCapability> = self.capabilities
            .iter()
            .filter(|cap| cap.available)
            .collect();

        if available_transports.is_empty() {
            return Err(Error::Network("No available transport".to_string()));
        }

        let selected = match qos {
            QosLevel::LowLatency => {
                // 选择延迟最低的
                available_transports
                    .iter()
                    .min_by_key(|cap| cap.latency_ms)
                    .unwrap()
            }
            QosLevel::HighBandwidth => {
                // 选择带宽最高的
                available_transports
                    .iter()
                    .max_by_key(|cap| cap.max_bandwidth)
                    .unwrap()
            }
            QosLevel::LowPower => {
                // 选择功耗最低的
                available_transports
                    .iter()
                    .min_by_key(|cap| cap.power_consumption)
                    .unwrap()
            }
            QosLevel::Balanced => {
                // 平衡选择
                available_transports
                    .iter()
                    .min_by_key(|cap| cap.latency_ms + (cap.power_consumption as u32) * 10)
                    .unwrap()
            }
        };

        Ok(selected.transport_type)
    }

    /// 更新传输能力状态
    pub fn update_capability(&mut self, transport_type: TransportType, available: bool) {
        if let Some(cap) = self.capabilities.iter_mut().find(|c| c.transport_type == transport_type) {
            cap.available = available;
        }
    }
}

impl Default for TransportArbiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_arbiter() {
        let mut arbiter = TransportArbiter::new();
        
        arbiter.add_capability(TransportCapability {
            transport_type: TransportType::Ble,
            max_bandwidth: 1_000_000,
            latency_ms: 50,
            power_consumption: 20,
            available: true,
        });

        arbiter.add_capability(TransportCapability {
            transport_type: TransportType::WiFiDirect,
            max_bandwidth: 10_000_000,
            latency_ms: 10,
            power_consumption: 60,
            available: true,
        });

        let transport = arbiter.select_transport(QosLevel::LowLatency).unwrap();
        assert_eq!(transport, TransportType::WiFiDirect);

        let transport = arbiter.select_transport(QosLevel::LowPower).unwrap();
        assert_eq!(transport, TransportType::Ble);
    }
}
