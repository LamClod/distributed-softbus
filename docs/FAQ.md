# 常见问题解答 (FAQ)

## 基础概念

### Q1: 什么是分布式软总线？

**A**: 分布式软总线是一个虚拟化的设备互联框架，它将多个物理设备抽象为一个逻辑上的"超级设备"。应用程序可以像调用本地服务一样调用远程设备的功能，无需关心底层网络细节。

**类比**: 就像计算机主板上的PCIe总线，CPU不需要知道显卡的物理位置，只需通过总线发送指令即可。软总线在设备间构建了类似的虚拟通道。

---

### Q2: 为什么选择Rust作为主要实现语言？

**A**: 
- **内存安全**: 编译期消除数据竞争和空指针错误
- **零成本抽象**: 性能接近C/C++，无运行时GC开销
- **并发友好**: `async/await`语法天然适合网络编程
- **跨平台**: 一套代码可编译到Windows、Linux、Android、iOS
- **生态成熟**: tokio、serde等库提供工业级支持

---

### Q3: 与现有方案（如Bluetooth SDK、gRPC）的区别？

**对比表**:

| 特性 | 软总线 | 原生蓝牙SDK | gRPC |
|------|--------|-------------|------|
| 位置透明 | ✅ 自动 | ❌ 需手动管理 | ⚠️ 需配置服务端 |
| 传输切换 | ✅ 自动仲裁 | ❌ 单一协议 | ❌ 固定TCP/HTTP2 |
| 服务发现 | ✅ 零配置 | ❌ 需手动配对 | ⚠️ 需服务注册中心 |
| 开发复杂度 | 低（IDL生成） | 高（底层API） | 中（Protobuf） |
| 跨协议 | ✅ BLE/Wi-Fi/以太网 | ❌ 仅蓝牙 | ❌ 仅IP网络 |

**优势**: 软总线提供更高层次的抽象，自动处理设备发现、连接建立、协议选择等复杂性。

---

## 安装与配置

### Q4: 最低系统要求是什么？

**A**:
- **操作系统**: 
  - Windows 10/11 (1809+)
  - Linux (Kernel 4.15+, BlueZ 5.50+)
  - macOS 11+
  - Android 8.0+
- **Rust**: 1.75+
- **内存**: 最少512MB可用
- **蓝牙**: BLE 4.0+（可选）
- **Wi-Fi**: 支持Wi-Fi Direct（可选）

---

### Q5: 如何在没有蓝牙的设备上运行？

**A**: 软总线支持多种传输协议，蓝牙并非必需：

```rust
let config = SoftbusConfig {
    enabled_adapters: vec![
        // 仅启用以太网和mDNS
        TransportType::Ethernet,
        TransportType::MDns,
    ],
    ..Default::default()
};

let softbus = Softbus::with_config(config).await?;
```

在局域网环境下，mDNS + TCP/IP即可实现完整功能。

---

### Q6: Linux权限问题：`Permission denied: /dev/rfcomm0`

**A**: 需要添加当前用户到`bluetooth`组：

```bash
# 添加到bluetooth组
sudo usermod -a -G bluetooth $USER

# 配置udev规则
sudo tee /etc/udev/rules.d/99-bluetooth.rules << EOF
KERNEL=="rfcomm*", GROUP="bluetooth", MODE="0660"
SUBSYSTEM=="bluetooth", GROUP="bluetooth", MODE="0660"
EOF

# 重新加载规则
sudo udevadm control --reload-rules

# 重新登录使组权限生效
```

或者使用`CAP_NET_RAW` capability:

```bash
sudo setcap 'cap_net_raw,cap_net_admin+eip' target/debug/my_app
```

---

## 开发相关

### Q7: 如何定义自己的服务接口？

**A**: 使用IDL（接口定义语言）：

**步骤**:

1. **创建IDL文件** (`my_service.idl`):
```idl
service MyService {
    version "1.0.0";
    rpc DoSomething(Request) returns (Response);
}

message Request {
    string data = 1;
}

message Response {
    int32 result = 1;
}
```

2. **编译IDL**:
```bash
softbus-idl-compiler my_service.idl --output src/generated
```

3. **实现服务**:
```rust
struct MyServiceImpl;

#[async_trait]
impl MyService for MyServiceImpl {
    async fn do_something(&self, req: Request) -> Result<Response> {
        Ok(Response { result: 42 })
    }
}
```

4. **注册服务**:
```rust
let stub = MyServiceStub::new(Arc::new(MyServiceImpl));
softbus.register_service(ServiceId::new("MyService"), Box::new(stub)).await?;
```

---

### Q8: 如何调试网络连接问题？

**A**: 

**1. 启用详细日志**:
```bash
RUST_LOG=softbus=trace,btleplug=debug cargo run
```

**2. 检查设备发现**:
```rust
let devices = softbus.connection_manager()
    .discover_devices(Duration::from_secs(10))
    .await?;

for device in devices {
    println!("Found: {} (RSSI: {})", device.name, device.rssi);
}
```

**3. 手动测试连接**:
```rust
let channel = softbus.connection_manager()
    .get_channel(&device_id)
    .await?;

println!("Channel quality: {:?}", channel.quality());
```

**4. 抓包分析**:
```bash
# BLE抓包
sudo btmon | tee ble.log

# Wi-Fi抓包
sudo tcpdump -i wlan0 port 8888 -w softbus.pcap
```

---

### Q9: RPC调用超时如何处理？

**A**: 

**方法1 - 全局配置超时**:
```rust
let config = SoftbusConfig {
    performance_config: PerformanceConfig {
        rpc_timeout_secs: 30,  // 30秒超时
        max_retries: 3,        // 重试3次
        ..Default::default()
    },
    ..Default::default()
};
```

**方法2 - 单次调用超时**:
```rust
use tokio::time::timeout;

let result = timeout(
    Duration::from_secs(5),
    proxy.some_method(request)
).await;

match result {
    Ok(Ok(response)) => {
        // 成功
    }
    Ok(Err(e)) => {
        // RPC错误
    }
    Err(_) => {
        // 超时
        eprintln!("Request timed out");
    }
}
```

**方法3 - 实现重试逻辑**:
```rust
async fn call_with_retry<F, T>(
    f: F,
    max_retries: u32,
) -> Result<T>
where
    F: Fn() -> BoxFuture<'static, Result<T>>,
{
    let mut attempts = 0;
    
    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if attempts < max_retries => {
                attempts += 1;
                tokio::time::sleep(Duration::from_secs(1 << attempts)).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

---

### Q10: 如何传输大文件（>100MB）？

**A**: 使用流式传输：

**IDL定义**:
```idl
service FileTransferService {
    rpc UploadFile(stream FileChunk) returns (UploadResult);
    rpc DownloadFile(DownloadRequest) returns (stream FileChunk);
}

message FileChunk {
    bytes data = 1;
    int64 offset = 2;
    int64 total_size = 3;
}
```

**客户端实现**:
```rust
async fn upload_file(proxy: &FileTransferServiceProxy, path: &Path) -> Result<()> {
    let file = File::open(path).await?;
    let total_size = file.metadata().await?.len();
    
    let mut offset = 0;
    let chunk_size = 64 * 1024; // 64KB
    
    let (tx, rx) = mpsc::channel(10);
    
    // 发送块的任务
    tokio::spawn(async move {
        let mut buffer = vec![0u8; chunk_size];
        
        while let Ok(n) = file.read(&mut buffer).await {
            if n == 0 { break; }
            
            let chunk = FileChunk {
                data: buffer[..n].to_vec(),
                offset,
                total_size: total_size as i64,
            };
            
            tx.send(chunk).await.unwrap();
            offset += n as i64;
        }
    });
    
    // 流式上传
    let result = proxy.upload_file(rx).await?;
    Ok(())
}
```

**推荐**: 对于大文件，优先使用Wi-Fi Direct链路（自动仲裁会处理）。

---

## 性能优化

### Q11: 如何提高RPC调用性能？

**A**: 

**1. 启用连接池**:
```rust
let config = SoftbusConfig {
    performance_config: PerformanceConfig {
        connection_pool_size: 10,
        ..Default::default()
    },
    ..Default::default()
};
```

**2. 批量调用**:
```rust
// 不推荐：多次单独调用
for i in 0..100 {
    proxy.process_item(i).await?;
}

// 推荐：批量调用
let items: Vec<_> = (0..100).collect();
proxy.process_batch(items).await?;
```

**3. 并发调用**:
```rust
use futures::future::join_all;

let futures: Vec<_> = items.iter()
    .map(|item| proxy.process_item(item))
    .collect();

let results = join_all(futures).await;
```

**4. 压缩大载荷**:
```rust
use flate2::Compression;

let compressed = compress(&large_data, Compression::fast())?;
proxy.send_data(compressed).await?;
```

---

### Q12: 如何降低功耗？

**A**: 

**1. 调整BLE参数**:
```rust
let config = SoftbusConfig {
    ble_config: BleConfig {
        advertising_interval_ms: 1000,  // 降低广播频率
        scan_interval_ms: 2000,         // 降低扫描频率
        ..Default::default()
    },
    ..Default::default()
};
```

**2. 使用按需连接**:
```rust
// 不推荐：保持长连接
let channel = softbus.connection_manager().get_channel(&device_id).await?;

// 推荐：用完立即释放
{
    let channel = softbus.connection_manager().get_channel(&device_id).await?;
    channel.send(&data).await?;
} // channel在此处被释放
```

**3. 启用空闲超时**:
```rust
let config = SoftbusConfig {
    performance_config: PerformanceConfig {
        idle_timeout_secs: 60,  // 60秒无活动后断开
        ..Default::default()
    },
    ..Default::default()
};
```

---

## 安全相关

### Q13: 如何确保通信安全？

**A**: 

**1. 启用端到端加密**:
```rust
let config = SoftbusConfig {
    security_config: SecurityConfig {
        enable_encryption: true,
        enable_authentication: true,
        ..Default::default()
    },
    ..Default::default()
};
```

**2. 使用设备证书**:
```bash
# 生成自签名证书
openssl req -x509 -newkey rsa:4096 \
    -keyout device.key -out device.crt \
    -days 365 -nodes
```

```rust
let config = SoftbusConfig {
    security_config: SecurityConfig {
        cert_path: Some("device.crt".into()),
        key_path: Some("device.key".into()),
        ..Default::default()
    },
    ..Default::default()
};
```

**3. 实现设备白名单**:
```rust
let config = SoftbusConfig {
    security_config: SecurityConfig {
        trusted_devices: vec![
            DeviceId::new("device-1"),
            DeviceId::new("device-2"),
        ],
        ..Default::default()
    },
    ..Default::default()
};
```

**4. 自定义认证回调**:
```rust
softbus.set_auth_callback(|device_id, challenge| async move {
    // 显示PIN码给用户确认
    println!("Device {} wants to connect. PIN: {}", device_id, challenge);
    
    // 等待用户确认
    user_confirm().await
});
```

---

### Q14: 如何防止中间人攻击？

**A**: 

软总线默认使用公钥基础设施（PKI）：

1. **首次配对时验证指纹**:
```rust
softbus.on_device_discovered(|device| async move {
    let fingerprint = device.public_key_fingerprint();
    
    println!("Device: {}", device.name);
    println!("Fingerprint: {}", fingerprint);
    println!("Verify on the other device and confirm (y/n):");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    if input.trim() == "y" {
        device.trust().await?;
    }
});
```

2. **使用TLS 1.3**（默认启用）

3. **定期轮换密钥**:
```rust
// 每30天轮换一次
tokio::spawn(async {
    loop {
        tokio::time::sleep(Duration::from_secs(30 * 24 * 3600)).await;
        security_manager.rotate_keys().await?;
    }
});
```

---

## 部署相关

### Q15: 如何交叉编译到ARM设备（如树莓派）？

**A**: 

```bash
# 1. 添加目标平台
rustup target add armv7-unknown-linux-gnueabihf

# 2. 安装交叉编译工具链
sudo apt install gcc-arm-linux-gnueabihf

# 3. 配置Cargo
cat >> ~/.cargo/config.toml << EOF
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF

# 4. 编译
cargo build --target armv7-unknown-linux-gnueabihf --release

# 5. 复制到树莓派
scp target/armv7-unknown-linux-gnueabihf/release/my_app pi@raspberrypi:~/
```

---

### Q16: Docker部署最佳实践？

**A**: 

**Dockerfile示例**:
```dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

# 安装依赖
RUN apt-get update && apt-get install -y \
    libbluetooth-dev \
    libdbus-1-dev

# 编译
RUN cargo build --release

FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    bluetooth \
    bluez \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/my_app /usr/local/bin/

# 需要特权模式访问蓝牙
CMD ["my_app"]
```

**运行**:
```bash
docker build -t my-softbus-app .

# 需要--privileged访问蓝牙硬件
docker run --privileged \
    --net=host \
    -v /var/run/dbus:/var/run/dbus \
    my-softbus-app
```

---

### Q17: 如何监控软总线运行状态？

**A**: 

**1. 集成Prometheus**:
```rust
use prometheus::{Encoder, TextEncoder, Registry};

let registry = Registry::new();

// 注册指标
let connections = IntGauge::new("softbus_connections", "Active connections")?;
registry.register(Box::new(connections.clone()))?;

// 更新指标
connections.set(softbus.connection_manager().stats().active_connections as i64);

// 暴露HTTP端点
let metrics_server = warp::path("metrics")
    .map(move || {
        let encoder = TextEncoder::new();
        let metric_families = registry.gather();
        let mut buffer = vec![];
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    });

warp::serve(metrics_server).run(([0, 0, 0, 0], 9090)).await;
```

**2. 结构化日志**:
```rust
use tracing_subscriber::fmt::format::FmtSpan;

tracing_subscriber::fmt()
    .with_env_filter("softbus=info")
    .with_span_events(FmtSpan::CLOSE)
    .json()  // JSON格式，便于日志收集
    .init();
```

**3. 健康检查端点**:
```rust
#[derive(Serialize)]
struct HealthStatus {
    status: String,
    active_connections: usize,
    uptime_seconds: u64,
}

async fn health_check() -> impl Reply {
    let stats = softbus.connection_manager().stats();
    
    warp::reply::json(&HealthStatus {
        status: "ok".to_string(),
        active_connections: stats.active_connections,
        uptime_seconds: UPTIME.load(Ordering::Relaxed),
    })
}
```

---

## 故障排查

### Q18: 设备无法发现怎么办？

**A**: 

**检查清单**:

1. **蓝牙是否开启**:
```bash
# Linux
bluetoothctl show
systemctl status bluetooth

# 开启蓝牙
sudo systemctl start bluetooth
```

2. **检查权限**:
```bash
# 查看当前用户组
groups

# 应该包含bluetooth组
```

3. **查看日志**:
```bash
RUST_LOG=softbus_network::ble=trace cargo run
```

4. **手动扫描测试**:
```rust
let adapter = BleAdapter::new().await?;
adapter.start().await?;

let devices = adapter.discover(Duration::from_secs(10)).await?;
println!("Found {} devices", devices.len());
```

5. **防火墙规则**（Wi-Fi）:
```bash
# 允许软总线端口
sudo ufw allow 8888/tcp
```

---

### Q19: 内存占用过高怎么办？

**A**: 

**诊断**:
```bash
# 使用valgrind检查内存泄漏
valgrind --leak-check=full ./target/debug/my_app

# 或使用heaptrack
heaptrack ./target/debug/my_app
heaptrack_gui heaptrack.my_app.*.gz
```

**优化**:

1. **限制连接池大小**:
```rust
let config = SoftbusConfig {
    performance_config: PerformanceConfig {
        connection_pool_size: 5,  // 减少到5
        ..Default::default()
    },
    ..Default::default()
};
```

2. **及时释放大对象**:
```rust
// 使用作用域控制生命周期
{
    let large_data = vec![0u8; 10_000_000];
    channel.send(&large_data).await?;
} // large_data在此处释放
```

3. **使用Arc共享数据**:
```rust
let shared_data = Arc::new(large_data);

// 多个地方使用，无需复制
proxy1.send(shared_data.clone()).await?;
proxy2.send(shared_data.clone()).await?;
```

---

### Q20: 如何获取技术支持？

**A**: 

- **GitHub Issues**: https://github.com/yourorg/distributed-softbus/issues
- **文档**: https://docs.softbus.io
- **社区讨论**: https://discord.gg/softbus
- **示例代码**: https://github.com/yourorg/distributed-softbus/tree/main/examples
- **API参考**: `cargo doc --open`

**提问模板**:
```
## 环境信息
- OS: [Windows 11 / Ubuntu 22.04 / etc.]
- Rust版本: [rustc --version]
- Softbus版本: [0.1.0]

## 问题描述
[详细描述问题]

## 复现步骤
1. ...
2. ...

## 日志输出
```
RUST_LOG=softbus=trace cargo run
[粘贴相关日志]
```

## 期望行为
[描述期望的正确行为]
```

---

**更新时间**: 2024-11-28  
**版本**: v0.1.0
