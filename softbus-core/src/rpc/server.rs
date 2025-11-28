//! RPC服务端

use std::sync::Arc;
use std::collections::HashMap;
use bytes::Bytes;
use async_trait::async_trait;
use parking_lot::RwLock;
use crate::{Error, Result, Channel};

/// 方法处理器trait
#[async_trait]
pub trait MethodHandler: Send + Sync {
    async fn handle(&self, request: Bytes) -> Result<Bytes>;
}

/// RPC服务端
/// 
/// 用于处理远程过程调用请求
pub struct RpcServer {
    handlers: Arc<RwLock<HashMap<String, Arc<dyn MethodHandler>>>>,
}

impl RpcServer {
    /// 创建新的RPC服务端
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 注册方法处理器
    pub fn register_method(&self, method_name: &str, handler: Arc<dyn MethodHandler>) {
        self.handlers.write().insert(method_name.to_string(), handler);
    }

    /// 注销方法处理器
    pub fn unregister_method(&self, method_name: &str) {
        self.handlers.write().remove(method_name);
    }

    /// 处理RPC请求
    pub async fn handle_request(&self, request: Bytes) -> Result<Bytes> {
        // 解析请求
        let (method_name, payload) = self.parse_request(&request)?;

        // 查找处理器
        let handler = {
            let handlers = self.handlers.read();
            handlers
                .get(&method_name)
                .map(Arc::clone)
                .ok_or_else(|| Error::MethodNotFound(method_name.clone()))?
        };

        // 调用处理器
        handler.handle(payload).await
    }

    fn parse_request(&self, request: &Bytes) -> Result<(String, Bytes)> {
        // TODO: 使用protobuf解析请求
        // 这里暂时使用简单的格式
        let data = request.as_ref();
        
        // 查找第一个分隔符（服务名结束）
        let first_null = data.iter().position(|&b| b == 0)
            .ok_or_else(|| Error::Serialization("Invalid request format".to_string()))?;
        
        // 查找第二个分隔符（方法名结束）
        let second_null = data[first_null + 1..].iter().position(|&b| b == 0)
            .ok_or_else(|| Error::Serialization("Invalid request format".to_string()))?;
        
        let method_name = String::from_utf8_lossy(&data[first_null + 1..first_null + 1 + second_null]).to_string();
        let payload = Bytes::copy_from_slice(&data[first_null + second_null + 2..]);
        
        Ok((method_name, payload))
    }

    /// 启动服务端，监听指定通道
    pub async fn serve(&self, channel: Arc<dyn Channel>) -> Result<()> {
        loop {
            // 接收请求
            let request = channel.recv().await?;

            // 处理请求
            let response = self.handle_request(request).await?;

            // 发送响应
            channel.send(response).await?;
        }
    }
}

impl Default for RpcServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_server_creation() {
        let server = RpcServer::new();
        assert_eq!(server.handlers.read().len(), 0);
    }
}
