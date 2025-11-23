# EWS Client Architecture

## Overview

三层架构设计，从底层到上层依次为：

```
┌─────────────────────────────────────┐
│   Python Layer (pyo3 bindings)     │
│   - 类型转换                         │
│   - 异步桥接 (tokio → asyncio)      │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│   Pure Rust Async Client           │
│   - reqwest + tokio                 │
│   - 业务逻辑层                       │
│   - 参考 ews_xpcom 结构              │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│   ews-rs (Protocol Layer)          │
│   - SOAP 序列化/反序列化             │
│   - EWS 类型定义                     │
└─────────────────────────────────────┘
```

## Project Structure

```
ews-client/
├── ews-client-core/              # 纯 Rust 异步客户端
│   └── src/
│       ├── client/
│       │   ├── mod.rs            # EwsClient 主结构
│       │   ├── credentials.rs    # 认证管理
│       │   ├── error.rs          # 错误类型
│       │   ├── operations/       # EWS 操作
│       │   └── types.rs          # 内部类型
│       ├── headers.rs            # 消息头处理
│       └── utils.rs
│
├── ews-client-python/            # Python bindings
│   └── src/
│       ├── lib.rs                # PyO3 模块入口
│       ├── client.rs             # PyEwsClient
│       ├── types.rs              # 类型转换
│       └── error.rs              # 错误映射
│
├── python/ews_client/            # Python 包
│   ├── __init__.py
│   ├── __init__.pyi              # 类型存根
│   └── types.py
│
├── tests/
│   ├── rust/                     # Rust 测试 + Mock server
│   └── python/                   # Python 测试
│
├── docs/
│   ├── api/                      # API 文档
│   ├── examples/                 # 使用示例
│   └── roadmap/                  # 开发路线图
│
└── examples/                     # 示例代码
    ├── rust/
    └── python/
```

## Core Modules

### 1. Client Core (`ews-client-core`)

**EwsClient** - 主客户端结构

- HTTP 请求管理 (reqwest)
- 认证处理
- 操作请求封装
- 错误处理
- 重试逻辑

**Credentials** - 认证管理

- Basic Authentication
- OAuth2 (with token refresh)

**Operations** - EWS 操作集
参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/operations/`，包括：

- 文件夹操作：sync, create, delete, update, move, copy
- 消息操作：sync, get, create, delete, mark_read, move, copy
- 其他：send_message, mark_as_junk, check_connectivity

### 2. Python Bindings (`ews-client-python`)

**PyEwsClient** - Python 客户端封装

- 异步方法暴露
- 类型转换 (Rust ↔ Python)
- 错误映射

**Type Conversion**

- Rust types → Python types
- Python types → Rust types
- 处理 `Vec`, `Option`, `Result`

### 3. Testing Strategy

**Rust Tests**

- 单元测试
- Mock server (wiremock)
- 集成测试

**Python Tests**

- pytest + pytest-asyncio
- Mock server
- 类型检查 (mypy)

## Design Decisions

### ✅ 从 ews_xpcom 复制

- 核心 EWS 操作逻辑
- 错误处理模式
- 操作模块结构
- 消息头处理逻辑

### ❌ 不复制

- XPCOM 特定的 listener 模式
- UI 交互代码 (mailnews_ui_glue)
- nsIMsgIncomingServer 等 XPCOM 类型
- moz_task 异步模型

### 技术栈

**Rust Layer**

- `tokio` - 异步运行时
- `reqwest` - HTTP 客户端
- `ews` - EWS 协议库
- `thiserror` - 错误处理
- `serde` - 序列化

**Python Layer**

- `pyo3` - Rust-Python 绑定
- `pyo3-async-runtimes` - 异步桥接
- 完整类型提示 (`.pyi`)

**Testing**

- `wiremock` - Mock HTTP server
- `pytest` - Python 测试框架
- `pytest-asyncio` - 异步测试支持

## API Design

### Rust API

```rust
pub struct EwsClient {
    endpoint: Url,
    credentials: Credentials,
    client: reqwest::Client,
}

impl EwsClient {
    pub async fn new(endpoint: Url, credentials: Credentials) -> Result<Self, EwsError>;

    // 文件夹操作
    pub async fn sync_folder_hierarchy(&self, sync_state: Option<String>) -> Result<FolderSyncResult, EwsError>;
    pub async fn create_folder(&self, parent_id: &str, name: &str) -> Result<String, EwsError>;
    pub async fn delete_folder(&self, folder_id: &str) -> Result<(), EwsError>;

    // 消息操作
    pub async fn sync_messages(&self, folder_id: &str, sync_state: Option<String>) -> Result<MessageSyncResult, EwsError>;
    pub async fn get_message(&self, message_id: &str) -> Result<Vec<u8>, EwsError>;
    pub async fn create_message(&self, folder_id: &str, content: Vec<u8>) -> Result<String, EwsError>;

    // 其他操作
    pub async fn mark_as_read(&self, message_ids: Vec<String>, is_read: bool) -> Result<(), EwsError>;
    pub async fn move_messages(&self, message_ids: Vec<String>, target_folder: &str) -> Result<Vec<String>, EwsError>;
}
```

### Python API

```python
class EwsClient:
    def __init__(self, endpoint: str, username: str, password: str): ...

    async def sync_folder_hierarchy(self, sync_state: Optional[str] = None) -> FolderSyncResult: ...
    async def create_folder(self, parent_id: str, name: str) -> str: ...
    async def delete_folder(self, folder_id: str) -> None: ...

    async def sync_messages(self, folder_id: str, sync_state: Optional[str] = None) -> MessageSyncResult: ...
    async def get_message(self, message_id: str) -> bytes: ...
    async def create_message(self, folder_id: str, content: bytes) -> str: ...

    async def mark_as_read(self, message_ids: list[str], is_read: bool) -> None: ...
    async def move_messages(self, message_ids: list[str], target_folder: str) -> list[str]: ...
```

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum EwsError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("EWS protocol error: {0}")]
    Protocol(#[from] ews::Error),

    #[error("Authentication failed")]
    Authentication,

    #[error("Response error: {0:?}")]
    Response(ews::response::ResponseError),

    #[error("Operation error: {message}")]
    Operation { message: String },

    #[error("Missing ID in response")]
    MissingId,
}
```

## Quality Standards

- Rust 代码通过 `cargo clippy`
- Rust 代码通过 `cargo fmt`
- 单元测试覆盖率 > 80%
- 所有公开 API 有文档注释
- Python 类型检查通过 `mypy`
- Python 代码通过 `ruff` 检查
