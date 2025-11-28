//! RPC客户端

use std::sync::Arc;
use bytes::Bytes;
use serde::{Serialize, de::DeserializeOwned};
use tokio::time::{timeout, Duration};
use crate::{Error, Result, Channel};

/// RPC客户端
/// 
/// 用于发起远程过程调用
pub struct RpcClient {
    channel: Arc<dyn Channel>,
    timeout: Duration,
}

impl RpcClient {
    /// 创建新的RPC客户端
    pub fn new(channel: Arc<dyn Channel>) -> Self {
        Self {
            channel,
            timeout: Duration::from_secs(30),
        }
    }

    /// 设置超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 调用远程方法
    pub async fn call<Req, Resp>(
        &self,
        service_name: &str,
        method_name: &str,
        request: Req,
    ) -> Result<Resp>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
    {
        // 序列化请求
        let request_bytes = bincode::serialize(&request)
            .map_err(|e| Error::Serialization(e.to_string()))?;

        // 构造RPC请求消息
        let rpc_request = self.build_request(service_name, method_name, request_bytes)?;

        // 发送请求
        let response_bytes = timeout(self.timeout, async {
            self.channel.send(rpc_request).await?;
            self.channel.recv().await
        })
        .await
        .map_err(|_| Error::Timeout)??;

        // 解析响应
        let response: Resp = bincode::deserialize(&response_bytes)
            .map_err(|e| Error::Serialization(e.to_string()))?;

        Ok(response)
    }

    fn build_request(
        &self,
        service_name: &str,
        method_name: &str,
        payload: Vec<u8>,
    ) -> Result<Bytes> {
        // TODO: 使用protobuf构造请求消息
        // 这里暂时使用简单的格式
        let mut request = Vec::new();
        request.extend_from_slice(service_name.as_bytes());
        request.push(0); // 分隔符
        request.extend_from_slice(method_name.as_bytes());
        request.push(0); // 分隔符
        request.extend_from_slice(&payload);
        
        Ok(Bytes::from(request))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_client_creation() {
        // Mock channel would be needed for actual testing
        // This is just a compilation test
    }
}
