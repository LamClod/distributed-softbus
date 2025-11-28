//! 身份认证

use std::sync::Arc;
use dashmap::DashMap;
use crate::{Error, Result, DeviceId};

/// 认证凭证
#[derive(Debug, Clone)]
pub struct Credential {
    pub device_id: DeviceId,
    pub token: String,
    pub expires_at: i64,
}

/// 认证管理器
/// 
/// 负责设备身份认证和授权
pub struct AuthManager {
    credentials: Arc<DashMap<DeviceId, Credential>>,
    trusted_devices: Arc<DashMap<DeviceId, bool>>,
}

impl AuthManager {
    /// 创建新的认证管理器
    pub fn new() -> Self {
        Self {
            credentials: Arc::new(DashMap::new()),
            trusted_devices: Arc::new(DashMap::new()),
        }
    }

    /// 验证设备
    pub fn authenticate(&self, device_id: &DeviceId, token: &str) -> Result<()> {
        if let Some(cred) = self.credentials.get(device_id) {
            if cred.token == token {
                // 检查是否过期
                let now = chrono::Utc::now().timestamp();
                if cred.expires_at > now {
                    return Ok(());
                } else {
                    return Err(Error::Authentication("Token expired".to_string()));
                }
            }
        }
        
        Err(Error::Authentication("Invalid credentials".to_string()))
    }

    /// 添加凭证
    pub fn add_credential(&self, credential: Credential) {
        self.credentials.insert(credential.device_id.clone(), credential);
    }

    /// 移除凭证
    pub fn remove_credential(&self, device_id: &DeviceId) {
        self.credentials.remove(device_id);
    }

    /// 添加信任设备
    pub fn trust_device(&self, device_id: DeviceId) {
        self.trusted_devices.insert(device_id, true);
    }

    /// 移除信任设备
    pub fn untrust_device(&self, device_id: &DeviceId) {
        self.trusted_devices.remove(device_id);
    }

    /// 检查设备是否受信任
    pub fn is_trusted(&self, device_id: &DeviceId) -> bool {
        self.trusted_devices.contains_key(device_id)
    }

    /// 生成新的token
    pub fn generate_token(&self) -> String {
        use uuid::Uuid;
        Uuid::new_v4().to_string()
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_manager() {
        let manager = AuthManager::new();
        let device_id = DeviceId::new();
        
        manager.trust_device(device_id.clone());
        assert!(manager.is_trusted(&device_id));
    }

    #[test]
    fn test_generate_token() {
        let manager = AuthManager::new();
        let token = manager.generate_token();
        assert!(!token.is_empty());
    }
}
