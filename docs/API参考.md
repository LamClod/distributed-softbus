# API参考文档

## 核心API

### Softbus - 主入口

```rust
pub struct Softbus {
    device_id: DeviceId,
    conn_manager: Arc<ConnectionManager>,
    service_router: Arc<ServiceRouter>,
    registry: Arc<ServiceRegistry>,
}

impl Softbus {
    /// 创建新的Softbus实例
    pub async fn new() -> Result<Self>
    
    /// 使用自定义配置创建实例
    pub async fn with_config(config: SoftbusConfig) -> Result<Self>
    
    /// 启动软总线（开始设备发现和服务监听）
    pub async fn start(&self) -> Result<()>
    
    /// 停止软总线
    pub async fn stop(&self) -> Result<()>
    
    /// 获取本设备ID
    pub fn device_id(&self) -> &DeviceId
    
    /// 注册服务
    pub async fn register_service(
        &self,
        service_id: ServiceId,
        handler: Box<dyn ServiceHandler>,
    ) -> Result<()>
    
    /// 注销服务
    pub async fn unregister_service(&self, service_id: &ServiceId) -> Result<()>
    
    /// 发现服务
    pub async fn discover_service(&self, service_id: &ServiceId) -> Result<Vec<ServiceMetadata>>
    
    /// 获取服务路由器（用于创建Proxy）
    pub fn router(&self) -> Arc<ServiceRouter>
    
    /// 获取连接管理器
    pub fn connection_manager(&self) -> Arc<ConnectionManager>
}
```

#### 使用示例

```rust
use softbus_core::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // 创建并启动Softbus
    let softbus = Softbus::new().await?;
    softbus.start().await?;
    
    // 获取设备ID
    println!("Device ID: {}", softbus.device_id());
    
    // 发现服务
    let service_id = ServiceId::new("com.example.MyService");
    let services = softbus.discover_service(&service_id).await?;
    
    println!("Found {} services", services.len());
    
    Ok(())
}
```

---

## 配置API

### SoftbusConfig

```rust
pub struct SoftbusConfig {
    /// 设备名称
    pub device_name: String,
    
    /// 设备类型
    pub device_type: DeviceType,
    
    /// 启用的网络适配器
    pub enabled_adapters: Vec<TransportType>,
    
    /// BLE配置
    pub ble_config: BleConfig,
    
    /// Wi-Fi配置
    pub wifi_config: WiFiConfig,
    
    /// 安全配置
    pub security_config: SecurityConfig,
    
    /// 性能配置
    pub performance_config: PerformanceConfig,
}

impl Default for SoftbusConfig {
    fn default() -> Self {
        Self {
            device_name: hostname::get().unwrap().to_string_lossy().to_string(),
            device_type: DeviceType::Laptop,
            enabled_adapters: vec![
                TransportType::BLE,
                TransportType::WiFiDirect,
                TransportType::MDns,
            ],
            ble_config: BleConfig::default(),
            wifi_config: WiFiConfig::default(),
            security_config: SecurityConfig::default(),
            performance_config: PerformanceConfig::default(),
        }
    }
}

pub struct BleConfig {
    /// 广播间隔（毫秒）
    pub advertising_interval_ms: u64,
    
    /// 扫描窗口（毫秒）
    pub scan_window_ms: u64,
    
    /// 扫描间隔（毫秒）
    pub scan_interval_ms: u64,
    
    /// 连接超时（秒）
    pub connection_timeout_secs: u64,
}

pub struct SecurityConfig {
    /// 启用认证
    pub enable_authentication: bool,
    
    /// 启用加密
    pub enable_encryption: bool,
    
    /// 信任的设备列表
    pub trusted_devices: Vec<DeviceId>,
    
    /// 证书路径
    pub cert_path: Option<PathBuf>,
    
    /// 私钥路径
    pub key_path: Option<PathBuf>,
}

pub struct PerformanceConfig {
    /// 连接池大小
    pub connection_pool_size: usize,
    
    /// RPC超时（秒）
    pub rpc_timeout_secs: u64,
    
    /// 重试次数
    pub max_retries: u32,
    
    /// 缓冲区大小
    pub buffer_size: usize,
}
```

#### 使用示例

```rust
let config = SoftbusConfig {
    device_name: "MyPhone".to_string(),
    device_type: DeviceType::Phone,
    enabled_adapters: vec![TransportType::BLE, TransportType::WiFiDirect],
    security_config: SecurityConfig {
        enable_authentication: true,
        enable_encryption: true,
        ..Default::default()
    },
    ..Default::default()
};

let softbus = Softbus::with_config(config).await?;
```

---

## 服务定义API

### IDL语法

```idl
// 命名空间
namespace com.example.myapp;

// 服务定义
service MyService {
    version "1.0.0";
    
    // 简单RPC
    rpc GetData(GetDataRequest) returns (GetDataResponse);
    
    // 服务端流
    rpc StreamData(StreamRequest) returns (stream DataChunk);
    
    // 客户端流
    rpc UploadData(stream DataChunk) returns (UploadResponse);
    
    // 双向流
    rpc BiDirectional(stream Request) returns (stream Response);
}

// 消息定义
message GetDataRequest {
    string query = 1;
    int32 limit = 2;
    optional string cursor = 3;
}

message GetDataResponse {
    repeated DataItem items = 1;
    string next_cursor = 2;
}

message DataItem {
    string id = 1;
    string name = 2;
    int64 timestamp = 3;
    map<string, string> metadata = 4;
}

// 枚举
enum Status {
    PENDING = 0;
    RUNNING = 1;
    COMPLETED = 2;
    FAILED = 3;
}
```

### ServiceHandler trait

```rust
#[async_trait]
pub trait ServiceHandler: Send + Sync {
    /// 处理RPC请求
    async fn handle_request(&self, request: RpcRequest) -> Result<RpcResponse>;
    
    /// 获取服务元数据
    fn metadata(&self) -> ServiceMetadata;
}
```

#### 手动实现ServiceHandler

```rust
struct MyServiceImpl {
    data_store: Arc<DataStore>,
}

#[async_trait]
impl ServiceHandler for MyServiceImpl {
    async fn handle_request(&self, request: RpcRequest) -> Result<RpcResponse> {
        match request.method_name.as_str() {
            "GetData" => {
                let req: GetDataRequest = prost::Message::decode(&request.payload[..])?;
                
                // 执行业务逻辑
                let items = self.data_store.query(&req.query, req.limit).await?;
                
                let response = GetDataResponse {
                    items,
                    next_cursor: "...".to_string(),
                };
                
                Ok(RpcResponse {
                    payload: response.encode_to_vec(),
                    error: None,
                })
            }
            
            _ => Err(SoftbusError::ServiceNotFound(request.method_name)),
        }
    }
    
    fn metadata(&self) -> ServiceMetadata {
        ServiceMetadata {
            id: ServiceId::new("com.example.MyService"),
            name: "MyService".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["query".to_string()],
            provider: softbus.device_id().clone(),
        }
    }
}
```

---

## 代理API（自动生成）

### ServiceProxy

IDL编译器会为每个服务生成Proxy类：

```rust
// 自动生成的代码
pub struct MyServiceProxy {
    router: Arc<ServiceRouter>,
    service_id: ServiceId,
}

impl MyServiceProxy {
    pub fn new(router: Arc<ServiceRouter>) -> Self {
        Self {
            router,
            service_id: ServiceId::new("com.example.MyService"),
        }
    }
    
    /// 调用GetData方法
    pub async fn get_data(&self, request: GetDataRequest) -> Result<GetDataResponse> {
        let rpc_req = RpcRequest {
            request_id: Uuid::new_v4().to_string(),
            service_id: self.service_id.as_str().to_string(),
            method_name: "GetData".to_string(),
            payload: request.encode_to_vec(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        };
        
        let rpc_resp = self.router.route_call(&self.service_id, rpc_req).await?;
        
        match rpc_resp.result {
            Some(rpc_response::Result::Payload(data)) => {
                Ok(GetDataResponse::decode(&data[..])?)
            }
            Some(rpc_response::Result::Error(err)) => {
                Err(SoftbusError::ConnectionFailed(err.message))
            }
            None => Err(SoftbusError::ConnectionFailed("Empty response".into())),
        }
    }
    
    /// 流式调用（服务端流）
    pub async fn stream_data(
        &self,
        request: StreamRequest,
    ) -> Result<impl Stream<Item = Result<DataChunk>>> {
        // 返回异步流
        unimplemented!()
    }
}
```

#### 使用Proxy

```rust
// 创建代理
let proxy = MyServiceProxy::new(softbus.router());

// 调用远程方法
let request = GetDataRequest {
    query: "test".to_string(),
    limit: 10,
    cursor: None,
};

let response = proxy.get_data(request).await?;

for item in response.items {
    println!("Item: {} - {}", item.id, item.name);
}
```

---

## 网络API

### VirtualChannel trait

```rust
#[async_trait]
pub trait VirtualChannel: Send + Sync {
    /// 发送数据
    async fn send(&self, data: &[u8]) -> Result<usize>;
    
    /// 接收数据（阻塞）
    async fn recv(&self, buf: &mut [u8]) -> Result<usize>;
    
    /// 非阻塞接收
    async fn try_recv(&self, buf: &mut [u8]) -> Result<Option<usize>>;
    
    /// 获取链路质量
    fn quality(&self) -> LinkQuality;
    
    /// 获取传输类型
    fn transport_type(&self) -> TransportType;
    
    /// 获取对端设备ID
    fn peer_id(&self) -> &DeviceId;
    
    /// 关闭通道
    async fn close(&self) -> Result<()>;
    
    /// 检查通道是否活跃
    fn is_active(&self) -> bool;
}
```

### ConnectionManager

```rust
pub struct ConnectionManager {
    // ...
}

impl ConnectionManager {
    /// 获取到指定设备的通道
    pub async fn get_channel(&self, device_id: &DeviceId) -> Result<Arc<dyn VirtualChannel>>
    
    /// 获取所有活跃通道
    pub fn get_all_channels(&self) -> Vec<(DeviceId, Arc<dyn VirtualChannel>)>
    
    /// 关闭到指定设备的连接
    pub async fn close_channel(&self, device_id: &DeviceId) -> Result<()>
    
    /// 获取连接统计信息
    pub fn stats(&self) -> ConnectionStats
}

pub struct ConnectionStats {
    pub total_connections: usize,
    pub active_connections: usize,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub average_latency: Duration,
}
```

---

## 安全API

### SecurityManager

```rust
pub struct SecurityManager {
    // ...
}

impl SecurityManager {
    /// 生成新的密钥对
    pub fn generate_keypair() -> Result<(Vec<u8>, Vec<u8>)>
    
    /// 从文件加载密钥
    pub fn load_keypair(cert_path: &Path, key_path: &Path) -> Result<Self>
    
    /// 认证设备
    pub async fn authenticate_device(
        &self,
        device_id: &DeviceId,
        channel: &dyn VirtualChannel,
    ) -> Result<()>
    
    /// 添加信任的设备
    pub fn add_trusted_device(&self, device_id: DeviceId, public_key: Vec<u8>)
    
    /// 移除信任
    pub fn remove_trusted_device(&self, device_id: &DeviceId)
    
    /// 检查设备是否受信任
    pub fn is_trusted(&self, device_id: &DeviceId) -> bool
    
    /// 加密数据
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>>
    
    /// 解密数据
    pub fn decrypt(&self, encrypted: &[u8]) -> Result<Vec<u8>>
}
```

#### 使用示例

```rust
// 生成密钥对
let (private_key, public_key) = SecurityManager::generate_keypair()?;

// 保存到文件
std::fs::write("device.key", &private_key)?;
std::fs::write("device.crt", &public_key)?;

// 加载配置
let config = SoftbusConfig {
    security_config: SecurityConfig {
        enable_authentication: true,
        enable_encryption: true,
        cert_path: Some("device.crt".into()),
        key_path: Some("device.key".into()),
        ..Default::default()
    },
    ..Default::default()
};
```

---

## 事件API

### Event枚举

```rust
pub enum Event {
    /// 设备被发现
    DeviceDiscovered(DeviceInfo),
    
    /// 设备连接建立
    DeviceConnected(DeviceId),
    
    /// 设备断开连接
    DeviceDisconnected(DeviceId),
    
    /// 服务注册
    ServiceRegistered(ServiceMetadata),
    
    /// 服务注销
    ServiceUnregistered(ServiceId),
    
    /// 链路质量变化
    LinkQualityChanged {
        device_id: DeviceId,
        old_quality: LinkQuality,
        new_quality: LinkQuality,
    },
    
    /// 传输切换
    TransportSwitched {
        device_id: DeviceId,
        old_transport: TransportType,
        new_transport: TransportType,
    },
}
```

### EventListener trait

```rust
#[async_trait]
pub trait EventListener: Send + Sync {
    async fn on_event(&self, event: Event);
}
```

#### 订阅事件

```rust
struct MyEventListener;

#[async_trait]
impl EventListener for MyEventListener {
    async fn on_event(&self, event: Event) {
        match event {
            Event::DeviceDiscovered(info) => {
                println!("Discovered device: {}", info.name);
            }
            Event::DeviceConnected(id) => {
                println!("Connected to: {}", id);
            }
            _ => {}
        }
    }
}

// 注册监听器
softbus.add_event_listener(Box::new(MyEventListener)).await?;
```

---

## 错误处理

### SoftbusError

```rust
#[derive(Error, Debug)]
pub enum SoftbusError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Channel error: {0}")]
    ChannelError(String),
    
    #[error("Serialization error")]
    SerializationError(#[from] prost::DecodeError),
    
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    
    #[error("Timeout")]
    Timeout,
}
```

#### 错误处理示例

```rust
match proxy.get_data(request).await {
    Ok(response) => {
        // 处理成功响应
    }
    Err(SoftbusError::ServiceNotFound(name)) => {
        eprintln!("Service {} not available", name);
    }
    Err(SoftbusError::Timeout) => {
        eprintln!("Request timed out");
    }
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

---

## 工具API

### IDL编译器命令行

```bash
# 编译IDL文件
softbus-idl-compiler input.idl --output ./generated --language rust

# 多语言生成
softbus-idl-compiler input.idl --output ./generated --language rust,csharp,cpp

# 指定模板
softbus-idl-compiler input.idl --template custom.hbs

# 详细输出
softbus-idl-compiler input.idl -v
```

### Cargo集成

```toml
[build-dependencies]
softbus-idl-compiler = "0.1"

# build.rs
fn main() {
    softbus_idl_compiler::compile("service.idl", "src/generated").unwrap();
}
```

---

## 性能调优API

### ConnectionPoolConfig

```rust
pub struct ConnectionPoolConfig {
    /// 最大空闲连接数
    pub max_idle_connections: usize,
    
    /// 最大活跃连接数
    pub max_active_connections: usize,
    
    /// 空闲连接超时（秒）
    pub idle_timeout_secs: u64,
    
    /// 连接获取超时（秒）
    pub acquire_timeout_secs: u64,
}
```

### 性能监控

```rust
// 获取性能指标
let stats = softbus.connection_manager().stats();

println!("Total connections: {}", stats.total_connections);
println!("Average latency: {:?}", stats.average_latency);
println!("Throughput: {} bytes/s", 
    stats.bytes_sent / elapsed.as_secs());
```

---

## 测试工具API

### MockChannel（用于单元测试）

```rust
pub struct MockChannel {
    send_buffer: Arc<Mutex<Vec<u8>>>,
    recv_buffer: Arc<Mutex<Vec<u8>>>,
}

impl MockChannel {
    pub fn new() -> Self {
        Self {
            send_buffer: Arc::new(Mutex::new(Vec::new())),
            recv_buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub fn push_recv_data(&self, data: Vec<u8>) {
        self.recv_buffer.lock().unwrap().extend(data);
    }
    
    pub fn get_sent_data(&self) -> Vec<u8> {
        self.send_buffer.lock().unwrap().clone()
    }
}
```

#### 测试示例

```rust
#[tokio::test]
async fn test_rpc_call() {
    let mock_channel = Arc::new(MockChannel::new());
    
    // 模拟接收响应
    let response = RpcResponse {
        payload: vec![1, 2, 3],
        error: None,
    };
    mock_channel.push_recv_data(bincode::serialize(&response).unwrap());
    
    // 执行测试
    // ...
    
    // 验证发送的数据
    let sent = mock_channel.get_sent_data();
    assert!(!sent.is_empty());
}
```

---

**更多信息**: 参见各模块的内嵌文档 (`cargo doc --open`)
