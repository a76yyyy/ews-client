# Rust API Documentation

## Overview

`ews-client-core` crate 提供纯 Rust 异步 EWS 客户端实现。

## 核心类型

### EwsClient

与 Exchange Web Services 交互的主客户端。

```rust
pub struct EwsClient {
    endpoint: Url,
    credentials: Credentials,
    client: reqwest::Client,
}
```

#### 方法

```rust
impl EwsClient {
    /// 创建新的 EWS 客户端
    pub fn new(endpoint: Url, credentials: Credentials) -> Result<Self, EwsError>

    /// 获取端点 URL
    pub fn endpoint(&self) -> &Url

    /// 检查是否为 Office365 服务器
    pub fn is_office365(&self) -> bool
}
```

### Credentials

EWS 认证凭据。

```rust
pub enum Credentials {
    Basic { username: String, password: String },
    OAuth2 { token: String },
}
```

#### 便捷方法

```rust
impl Credentials {
    pub fn basic(username: impl Into<String>, password: impl Into<String>) -> Self
    pub fn oauth2(token: impl Into<String>) -> Self
}
```

### EwsError

操作返回的错误类型。

```rust
pub enum EwsError {
    Http(reqwest::Error),
    Protocol(ews::Error),
    Authentication,
    Response(ews::response::ResponseError),
    Operation { message: String },
    MissingId,
    InvalidUrl(url::ParseError),
    Serialization(serde_json::Error),
}
```

### 数据类型

#### FolderInfo

```rust
pub struct FolderInfo {
    pub id: String,
    pub parent_id: Option<String>,
    pub display_name: String,
}
```

#### FolderSyncResult

```rust
pub struct FolderSyncResult {
    pub folders: Vec<FolderInfo>,
    pub sync_state: String,
}
```

#### MessageInfo

```rust
pub struct MessageInfo {
    pub id: String,
    pub subject: Option<String>,
    pub from: Option<String>,
    pub is_read: bool,
}
```

#### MessageSyncResult

```rust
pub struct MessageSyncResult {
    pub messages: Vec<MessageInfo>,
    pub sync_state: String,
}
```

## 使用方法

### 创建客户端

```rust
use ews_client_core::{EwsClient, Credentials};
use url::Url;

let endpoint = Url::parse("https://outlook.office365.com/EWS/Exchange.asmx")?;
let credentials = Credentials::basic("user@example.com", "password");
let client = EwsClient::new(endpoint, credentials)?;
```

### Office365 检测

```rust
if client.is_office365() {
    println!("连接到 Office365 服务器");
}
```

## 错误处理

所有操作返回 `Result<T, EwsError>`,使用标准 Rust 错误处理:

```rust
match client.some_operation().await {
    Ok(result) => println!("操作成功: {:?}", result),
    Err(e) => eprintln!("错误: {}", e),
}
```

或使用 `?` 操作符:

```rust
let result = client.some_operation().await?;
```

## 异步运行时

客户端使用 `tokio` 作为异步运行时:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 你的代码
    Ok(())
}
```

## 内部实现细节

### 请求批处理

客户端自动处理批量请求以避免限流:

- `GetItem`: 每批最多 10 个项目
- `GetFolder`: 每批最多 10 个文件夹

### 限流处理

当服务器返回限流响应时,客户端会自动:

1. 解析 `BackOffMilliseconds` 值
2. 等待指定时间
3. 自动重试请求

### 文件夹过滤

`batch_get_folders` 方法自动过滤非邮件文件夹,仅返回:

- `IPF.Note` 类型文件夹
- `IPF.Note.*` 类型文件夹
