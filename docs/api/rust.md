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
    server_version: AtomicCell<ExchangeServerVersion>,
}
```

#### 构造与配置

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

#### 连接测试

```rust
impl EwsClient {
    /// 测试与 EWS 服务器的连接和认证
    pub async fn check_connectivity(&self) -> Result<(), EwsError>
}
```

#### 文件夹操作

```rust
impl EwsClient {
    /// 同步文件夹层次结构
    pub async fn sync_folder_hierarchy(
        &self,
        sync_state: Option<String>,
    ) -> Result<FolderHierarchySyncResult, EwsError>

    /// 创建新文件夹
    pub async fn create_folder(&self, parent_id: &str, name: &str) -> Result<String, EwsError>

    /// 更新文件夹显示名称
    pub async fn update_folder(&self, folder_id: &str, folder_name: &str) -> Result<(), EwsError>

    /// 删除文件夹
    pub async fn delete_folder(&self, folder_ids: &[&str]) -> Result<(), EwsError>

    /// 复制文件夹
    pub async fn copy_folders(
        &self,
        destination_folder_id: &str,
        folder_ids: &[&str],
    ) -> Result<Vec<String>, EwsError>

    /// 移动文件夹
    pub async fn move_folders(
        &self,
        destination_folder_id: &str,
        folder_ids: &[&str],
    ) -> Result<Vec<String>, EwsError>
}
```

#### 消息操作

```rust
impl EwsClient {
    /// 同步文件夹中的消息
    pub async fn sync_messages(
        &self,
        folder_id: &str,
        sync_state: Option<String>,
    ) -> Result<SyncMessagesResult, EwsError>

    /// 获取单个消息的 MIME 内容
    pub async fn get_message(&self, id: impl Into<String>) -> Result<Vec<u8>, EwsError>

    /// 创建消息
    pub async fn create_message(
        &self,
        folder_id: &str,
        content: &[u8],
        is_draft: bool,
        is_read: bool,
    ) -> Result<CreateMessageResult, EwsError>

    /// 发送消息
    pub async fn send_message(
        &self,
        mime_content: &str,
        message_id: &str,
        should_request_dsn: bool,
        bcc_recipients: &[Recipient],
    ) -> Result<(), EwsError>

    /// 删除消息
    pub async fn delete_messages(&self, item_ids: &[&str]) -> Result<(), EwsError>

    /// 更改消息已读状态
    pub async fn change_read_status(&self, item_ids: &[&str], is_read: bool) -> Result<Vec<String>, EwsError>

    /// 标记文件夹中所有消息为已读/未读
    pub async fn change_read_status_all(
        &self,
        folder_ids: &[&str],
        is_read: bool,
        suppress_read_receipts: bool,
    ) -> Result<(), EwsError>

    /// 标记消息为垃圾邮件或非垃圾邮件
    pub async fn mark_as_junk(
        &self,
        item_ids: &[&str],
        is_junk: bool,
        legacy_junk_folder_id: &str,
    ) -> Result<Vec<String>, EwsError>

    /// 复制消息
    pub async fn copy_items(
        &self,
        destination_folder_id: &str,
        item_ids: &[&str],
    ) -> Result<Vec<String>, EwsError>

    /// 移动消息
    pub async fn move_items(
        &self,
        destination_folder_id: &str,
        item_ids: &[&str],
    ) -> Result<Vec<String>, EwsError>
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
    /// HTTP 传输错误 (网络、连接等)
    Http(reqwest::Error),
    /// EWS 协议错误 (SOAP 解析、XML 问题等)
    Protocol(ews::Error),
    /// 认证失败 (401、无效凭据等)
    Authentication,
    /// EWS 响应包含错误代码
    ResponseError(ews::response::ResponseError),
    /// 处理响应数据时的错误 (验证、意外格式等)
    Processing { message: String },
    /// Exchange 响应中缺少必需的 ID
    MissingIdInResponse,
    /// 响应包含意外数量的消息
    UnexpectedResponseMessageCount { expected: usize, actual: usize },
    /// 无效的 URL
    InvalidUrl(url::ParseError),
    /// 序列化错误
    Serialization(serde_json::Error),
}
```

### 数据类型

#### FolderHierarchySyncResult

文件夹层次结构同步结果。

```rust
pub struct FolderHierarchySyncResult {
    /// 新的同步状态令牌,用于下次同步
    pub sync_state: String,
    /// 自上次同步以来创建的文件夹
    pub created_folders: Vec<FolderInfo>,
    /// 自上次同步以来更新的文件夹
    pub updated_folders: Vec<FolderInfo>,
    /// 自上次同步以来删除的文件夹 ID
    pub deleted_folder_ids: Vec<String>,
    /// 知名文件夹 ID 到其区分名称的映射
    /// (例如 "inbox", "deleteditems", "drafts" 等)
    pub well_known_folders: Option<HashMap<String, String>>,
}
```

#### FolderInfo

文件夹信息。

```rust
pub struct FolderInfo {
    /// EWS 文件夹 ID
    pub folder_id: String,
    /// 父文件夹 ID
    pub parent_folder_id: String,
    /// 文件夹显示名称
    pub display_name: String,
    /// 文件夹类别 (例如 "IPF.Note" 表示邮件文件夹)
    pub folder_class: Option<String>,
    /// 文件夹中的项目总数
    pub total_count: Option<u32>,
    /// 未读项目数
    pub unread_count: Option<u32>,
    /// 子文件夹数
    pub child_folder_count: Option<u32>,
}
```

#### SyncMessagesResult

消息同步结果。

```rust
pub struct SyncMessagesResult {
    /// 创建的消息 ID
    pub created: Vec<String>,
    /// 更新的消息 ID
    pub updated: Vec<String>,
    /// 删除的消息 ID
    pub deleted: Vec<String>,
    /// 已读状态改变的消息 (ID, 是否已读)
    pub read_status_changed: Vec<(String, bool)>,
    /// 新的同步状态令牌
    pub sync_state: String,
    /// 是否还有更多变更需要获取
    pub includes_last_item: bool,
}
```

#### SyncMessageInfo

同步消息的详细信息。

```rust
pub struct SyncMessageInfo {
    pub item_id: String,
    pub is_read: Option<bool>,
    pub internet_message_id: Option<String>,
    pub date_time_sent: Option<i64>,
    pub from: Option<String>,
    pub subject: Option<String>,
    pub has_attachments: Option<bool>,
    pub size: Option<usize>,
}
```

#### CreateMessageResult

创建消息的结果。

```rust
pub struct CreateMessageResult {
    /// 新创建消息的 EWS ID
    pub item_id: String,
}
```

#### Mailbox

表示带有可选显示名称的电子邮件地址。

```rust
pub struct Mailbox<'a> {
    /// 此邮箱的显示名称 (例如 "John Doe")
    pub name: Option<&'a str>,
    /// 电子邮件地址 (例如 "john@example.com")
    pub email_address: Option<&'a str>,
}
```

#### MessagePriority

消息优先级级别。

```rust
pub enum MessagePriority {
    Highest,  // 最高优先级
    High,     // 高优先级
    Normal,   // 普通优先级
    Low,      // 低优先级
    Lowest,   // 最低优先级
}
```

#### MessageHeaders Trait

从不同消息表示中提取电子邮件标头的统一接口。

```rust
pub trait MessageHeaders {
    fn internet_message_id(&self) -> Option<impl AsRef<str>>;
    fn is_read(&self) -> Option<bool>;
    fn has_attachments(&self) -> Option<bool>;
    fn sent_timestamp_us(&self) -> Option<i64>;
    fn author<'a>(&'a self) -> Option<Mailbox<'a>>;
    fn reply_to_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>>;
    fn to_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>>;
    fn cc_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>>;
    fn bcc_recipients<'a>(&'a self) -> Option<impl IntoIterator<Item = Mailbox<'a>>>;
    fn message_subject(&self) -> Option<impl AsRef<str>>;
    fn priority(&self) -> Option<MessagePriority>;
    fn references(&self) -> Option<impl AsRef<str>>;
    fn size(&self) -> Option<usize>;
    fn preview(&self) -> Option<impl AsRef<str>>;
}
```

## 使用示例

### 创建客户端

```rust
use ews_client_core::{EwsClient, Credentials};
use url::Url;

let endpoint = Url::parse("https://outlook.office365.com/EWS/Exchange.asmx")?;
let credentials = Credentials::basic("user@example.com", "password");
let client = EwsClient::new(endpoint, credentials)?;
```

### 测试连接

```rust
// 测试连接和认证
client.check_connectivity().await?;
println!("连接成功!");
```

### 同步文件夹

```rust
// 首次同步
let result = client.sync_folder_hierarchy(None).await?;
println!("创建了 {} 个文件夹", result.created_folders.len());

// 后续同步
let result = client.sync_folder_hierarchy(Some(result.sync_state)).await?;
println!("更新了 {} 个文件夹", result.updated_folders.len());
```

### 文件夹操作

```rust
// 创建文件夹
let folder_id = client.create_folder("parent_folder_id", "New Folder").await?;

// 更新文件夹名称
client.update_folder(&folder_id, "Renamed Folder").await?;

// 移动文件夹
let new_ids = client.move_folders("dest_folder_id", &[&folder_id]).await?;

// 删除文件夹
client.delete_folder(&[&folder_id]).await?;
```

### 同步消息

```rust
// 同步文件夹中的消息
let result = client.sync_messages("folder_id", None).await?;
println!("新消息: {}", result.created.len());
println!("更新: {}", result.updated.len());
println!("删除: {}", result.deleted.len());
```

### 消息操作

```rust
// 获取消息内容
let mime_content = client.get_message("message_id").await?;

// 创建消息
let mime_data = b"From: user@example.com\r\nTo: recipient@example.com\r\nSubject: Test\r\n\r\nBody";
let result = client.create_message("folder_id", mime_data, false, true).await?;

// 标记为已读
let updated_ids = client.change_read_status(&["msg_id_1", "msg_id_2"], true).await?;

// 移动消息
let new_ids = client.move_items("dest_folder_id", &["msg_id_1", "msg_id_2"]).await?;

// 删除消息
client.delete_messages(&["msg_id_1", "msg_id_2"]).await?;
```

### 发送消息

```rust
use ews::Recipient;

let mime_content = "From: user@example.com\r\nTo: recipient@example.com\r\nSubject: Test\r\n\r\nBody";
client.send_message(mime_content, "message-id@example.com", false, &[]).await?;
```

## 错误处理

所有操作返回 `Result<T, EwsError>`,使用标准 Rust 错误处理:

```rust
match client.sync_folder_hierarchy(None).await {
    Ok(result) => println!("同步成功: {} 个文件夹", result.created_folders.len()),
    Err(e) => eprintln!("错误: {}", e),
}
```

或使用 `?` 操作符:

```rust
let result = client.sync_folder_hierarchy(None).await?;
```

## 异步运行时

客户端使用 `tokio` 作为异步运行时:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = Url::parse("https://outlook.office365.com/EWS/Exchange.asmx")?;
    let credentials = Credentials::basic("user@example.com", "password");
    let client = EwsClient::new(endpoint, credentials)?;

    client.check_connectivity().await?;
    println!("连接成功!");

    Ok(())
}
```

## 高级特性

### 服务器版本检测

客户端自动检测和缓存 Exchange Server 版本:

```rust
// 版本信息从服务器响应头中自动提取和缓存
// 用于优化不同版本服务器的请求
```

### Office365 检测

```rust
if client.is_office365() {
    println!("连接到 Office365 服务器");
}
```

### 请求批处理

客户端自动处理批量请求以避免限流:

- `GetItem`: 每批最多 10 个项目
- `GetFolder`: 每批最多 10 个文件夹
- `SyncFolderItems`: 每批最多 100 个变更

### 自动限流处理

当服务器返回限流响应时,客户端会自动:

1. 解析 `BackOffMilliseconds` 值
2. 等待指定时间
3. 自动重试请求

### 文件夹过滤

文件夹同步自动过滤非邮件文件夹,仅返回:

- `IPF.Note` 类型文件夹 (标准邮件文件夹)
- `IPF.Note.*` 类型文件夹 (自定义邮件文件夹)

### 消息头提取

使用 `MessageHeaders` trait 统一访问不同来源的消息头:

```rust
use ews_client_core::MessageHeaders;

// 从 EWS Message 提取
let ews_message: &ews::Message = ...;
let subject = ews_message.message_subject();
let author = ews_message.author();

// 从解析的 MIME 消息提取
let parsed_message: mail_parser::Message = ...;
let subject = parsed_message.message_subject();
let author = parsed_message.author();
