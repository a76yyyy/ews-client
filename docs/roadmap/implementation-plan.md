# Implementation Plan

## 当前进度概览

**当前阶段:** Phase 2 - 全部完成 ✅

**已完成:**

- ✅ Phase 1.1: 基础设施搭建
- ✅ Phase 1.2: 核心操作实现 (连接检查、文件夹同步、消息获取)
- ✅ Phase 1.3: 系统性测试 (核心架构已建立，Mock 测试基本完成)
- ✅ Phase 2.1: 文件夹操作 (创建、删除、更新、复制、移动)
- ✅ Phase 2.2: 消息操作 (同步、创建、删除、读取状态、垃圾邮件、复制、移动)
- ✅ Phase 2.3: 发送消息功能
- ✅ Phase 2.4: 消息头支持

**进行中:**

- 🔄 Phase 3: Python 绑定
  - ✅ P1.1: 错误映射 (error.rs) - 完成
  - ✅ P1.2: 基础类型转换 (types.rs) - 完成
  - ✅ P1.3: check_connectivity 方法 - 完成

**待开展:**

- ⏸️ Phase 4: 测试与文档
- ⏸️ Phase 5: OAuth2 支持 (可选)

---

## Phase 1: Core Rust Client (Week 1-2)

### Step 1.1: Basic Infrastructure

- [x] Create project structure (workspace)
- [x] Setup `EwsClient` base structure
- [x] Implement `Credentials` (Basic Auth only)
- [x] Implement `EwsError` error types
- [x] HTTP request wrapper (reqwest)
- [x] Dependencies configured

**Files created:**

- `ews-client-core/src/lib.rs`
- `ews-client-core/src/client/mod.rs`
- `ews-client-core/src/client/credentials.rs`
- `ews-client-core/src/client/error.rs`
- `ews-client-core/Cargo.toml`

### Step 1.2: Core Operations

- [x] `check_connectivity` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/check_connectivity.rs`
- [x] `sync_folder_hierarchy` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/sync_folder_hierarchy.rs`
- [x] `get_message` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/get_message.rs`
- [ ] Mock server framework setup

**Files created:**

- `ews-client-core/src/client/operations/mod.rs`
- `ews-client-core/src/client/operations/check_connectivity.rs`
- `ews-client-core/src/client/operations/sync_folder_hierarchy.rs`
- `ews-client-core/src/client/operations/get_message.rs`

### Step 1.3: Testing ✅

- [x] Unit tests for each operation (Signature tests)
- [x] Mock server integration tests (Folder & Item operations)
- [x] Error handling tests
- [x] Client-side integration tests (check_connectivity, change_read_status)

**Files created:**

- `ews-client-core/tests/unit/operations.rs`
- `ews-client-core/tests/integration/mock/folder_operations.rs`
- `ews-client-core/tests/integration/mock/item_operations.rs`
- `ews-client-core/tests/common/mock_server.rs`
- `ews-client-core/tests/common/fixtures.rs`

**Status:** ✅ 完成 - 核心测试架构已建立，Mock 测试覆盖了主要操作，包括客户端逻辑验证

## Phase 2: Complete Operation Set (Week 3-4)

### Step 2.1: Folder Operations ✅

- [x] `create_folder` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/create_folder.rs`
- [x] `delete_folder` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/delete_folder.rs`
- [x] `update_folder` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/update_folder.rs`
- [x] `copy_folders` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/copy_move_operations/copy_move_folder.rs`
- [x] `move_folders` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/copy_move_operations/copy_move_folder.rs`

**Files created:**

- `ews-client-core/src/client/operations/create_folder.rs`
- `ews-client-core/src/client/operations/delete_folder.rs`
- `ews-client-core/src/client/operations/update_folder.rs`
- `ews-client-core/src/client/operations/copy_move_operations/mod.rs`
- `ews-client-core/src/client/operations/copy_move_operations/folder.rs`

**Status:** ✅ 完成 - 所有文件夹操作已实现并编译通过。

**测试:**

- 移除了无意义的签名测试
- 创建了完整的集成测试套件 `tests/folder_operations.rs`
- 集成测试默认被 `#[ignore]` 标记,需要真实 EWS 服务器才能运行
- 运行集成测试: `cargo test --package ews-client-core -- --ignored`

### Step 2.2: Message Operations ✅

- [x] `sync_messages` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/sync_messages_for_folder.rs`
- [x] `create_message` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/create_message.rs`
- [x] `delete_messages` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/delete_messages.rs`
- [x] `change_read_status` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/change_read_status.rs`
- [x] `change_read_status_all` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/change_read_status_all.rs`
- [x] `mark_as_junk` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/mark_as_junk.rs`
- [x] `copy_items` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/copy_move_operations/copy_move_item.rs`
- [x] `move_items` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/copy_move_operations/copy_move_item.rs`
- [x] `update_item` - 参考 `ews-client-core/src/client/operations/update_item.rs`

**Files created:**

- `ews-client-core/src/client/operations/sync_messages.rs`
- `ews-client-core/src/client/operations/create_message.rs`
- `ews-client-core/src/client/operations/delete_messages.rs`
- `ews-client-core/src/client/operations/change_read_status.rs` (包含 `change_read_status_all`)
- `ews-client-core/src/client/operations/mark_as_junk.rs`
- `ews-client-core/src/client/operations/copy_move_operations/item.rs`
- `ews-client-core/src/client/operations/update_item.rs`

**Status:** ✅ 完成 - 所有消息操作已实现并编译通过。

**实现特性:**

- `sync_messages`: 支持增量同步,自动处理分页,去重消息状态变更
- `create_message`: 支持 MIME 内容上传,草稿/已读状态设置,MAPI 标志处理
- `delete_messages`: 硬删除模式,自动忽略不存在的消息
- `change_read_status`: 批量更新读取状态,部分失败容错
- `change_read_status_all`: 标记文件夹内所有消息,支持 Exchange 2013+
- `mark_as_junk`: 自动检测服务器版本,Exchange 2013+ 使用 MarkAsJunk,旧版本回退到移动操作
- `copy_items` / `move_items`: 批量操作,自动处理 Exchange 2010 SP1+ 的 ReturnNewItemIds

### Step 2.3: Send Message ✅

- [x] `send_message` - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/client/send_message.rs`

**Files created:**

- `ews-client-core/src/client/operations/send_message.rs`

**Status:** ✅ 完成 - 发送消息功能已实现并编译通过。

**实现特性:**

- `send_message`: 使用 CreateItem 操作发送邮件，MessageDisposition 设置为 SendOnly
- 支持 MIME 内容编码（Base64）
- 支持设置 Internet Message ID
- 支持请求送达回执（DSN）
- 支持 BCC 收件人列表
- 不保存到已发送文件夹（由客户端负责）

### Step 2.4: Headers Support ✅

- [x] Message headers parsing - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/headers.rs`
- [x] Mailbox type support
- [x] MessageHeaders trait for unified header access
- [x] Support for both EWS messages and parsed MIME messages

**Files created:**

- `ews-client-core/src/client/headers.rs`

**Status:** ✅ 完成 - 消息头支持已实现并通过测试。

**实现特性:**

- `MessageHeaders` trait: 统一的消息头访问接口
- `Mailbox` 结构体: 表示邮箱地址（名称 + 邮件地址）
  - 支持 RFC 2047 编码（处理非 ASCII 字符）
  - 格式化为 RFC 822 标准格式（`Name <email@example.com>`）
- `MessagePriority` 枚举: 消息优先级（Highest/High/Normal/Low/Lowest）
- 为 `ews::Message` 实现 MessageHeaders（从 EWS 响应中提取）
- 为 `mail_parser::Message` 实现 MessageHeaders（从 MIME 内容中解析）
- 支持的头字段：
  - Message-ID
  - From/Sender (author)
  - To/Cc/Bcc/Reply-To (recipients)
  - Subject
  - Date (Unix timestamp in microseconds，兼容 Thunderbird PRTime 格式)
  - Priority/Importance (X-Priority 和 EWS Importance)
  - References
  - Attachments flag
  - Read status
  - Message size
  - Preview text
- 辅助函数：
  - `make_header_string_for_mailbox_list`: 格式化邮箱列表为 RFC 822 格式
  - `array_of_recipients_to_mailboxes`: 转换 EWS 收件人列表
  - `address_to_mailboxes`: 转换 mail_parser 地址列表

**依赖:**

- `mail-parser`: 用于解析 MIME 消息
- `mail-builder`: 用于 RFC 2047 编码（处理非 ASCII 邮箱名称）

**测试:**

- ✅ Mailbox Display 格式化测试（包括 RFC 2047 编码）
- ✅ 邮箱列表格式化测试
- ✅ 所有单元测试通过（4/4）

## Phase 3: Python Bindings (Week 5)

### Step 3.1: Error Mapping (P1 - 基础设施) ✅

**Priority:** 🔴 Critical - All other methods depend on error handling

- [x] Implement `BaseEWSError` exception class in Python
- [x] Map Rust `EwsError` variants to Python exception hierarchy
- [x] Implement error conversion in `ews-client-python/src/error.rs`

**Error Hierarchy:**

```
BaseEWSError (base class)
├── EWSAuthenticationError (Authentication failures)
├── EWSHTTPError (Network/HTTP errors)
├── EWSProtocolError (SOAP/XML parsing errors)
├── EWSResponseError (EWS response errors)
├── EWSProcessingError (Data validation/processing errors)
├── EWSMissingIdError (Missing ID in response)
└── EWSSerializationError (JSON/serialization errors)
```

**Files to create/modify:**

- `ews-client-python/src/error.rs` - Implement error mapping (~80 lines)
- `python/ews_client/errors.py` - Define Python exception classes (~60 lines)
- `python/ews_client/__init__.py` - Export error classes

**Implementation Details:**

```rust
// ews-client-python/src/error.rs
use pyo3::prelude::*;
use ews_client_core::EwsError;

pub fn create_error_classes(py: Python, module: &Bound<PyModule>) -> PyResult<()> {
    // Create base exception class
    let base_error = PyErr::new::<pyo3::exceptions::PyException, _>("BaseEWSError");

    // Create specific exception classes
    // Map each EwsError variant to appropriate Python exception
    Ok(())
}

impl From<EwsError> for PyErr {
    fn from(err: EwsError) -> Self {
        match err {
            EwsError::Authentication => /* EWSAuthenticationError */,
            EwsError::Http(_) => /* EWSHTTPError */,
            EwsError::Protocol(_) => /* EWSProtocolError */,
            // ... other variants
        }
    }
}
```

**Implementation:**

使用 PyO3 的 `create_exception!` 宏创建异常类层次结构：

```rust
pyo3::create_exception!(_ews_client, BaseEWSError, PyException, "Base exception for all EWS client errors.");
pyo3::create_exception!(_ews_client, EWSAuthenticationError, BaseEWSError, "Authentication failure (401, invalid credentials, etc.).");
// ... 其他异常类
```

**Status:** ✅ 完成 - 编译通过，Clippy 无警告

---

### Step 3.2: Basic Type Conversion (P1 - 基础设施) ✅

**Priority:** 🔴 Critical - Required by all methods

- [x] Implement basic Rust → Python type conversion
- [x] Implement basic Python → Rust type conversion
- [x] Handle `Vec<String>`, `Option<T>`, `Result<T, E>`
- [x] Implement `FromPyObject` and `IntoPy` traits

**Files modified:**

- `ews-client-python/src/types.rs` - 文档说明
- `python/ews_client/types.pyi` - 类型存根（从 types.py 重命名）

**Implementation:**

PyO3 自动处理基础类型转换，无需手动实现：

- `Vec<T>` ↔ `list[T]`
- `Option<T>` ↔ `Optional[T]`
- `String` ↔ `str`
- `Vec<u8>` ↔ `bytes`
- `HashMap<K, V>` ↔ `dict[K, V]`

参考文档：

- `reference/pyo3/guide/src/conversions/tables.md`
- `reference/pyo3/guide/src/conversions/traits.md`

**Status:** ✅ 完成 - 基础类型依赖 PyO3 自动转换

---

### Step 3.3: Async Bridge & check_connectivity (P1 - 基础设施) ✅

**Priority:** 🔴 Critical - Validates async framework

- [x] Implement async method wrapper using `pyo3-async-runtimes`
- [x] Implement `check_connectivity` as first async method
- [x] Verify tokio → asyncio bridge works correctly
- [x] Use `Arc<EwsClient>` to avoid server_version inconsistency

**Files modified:**

- `ews-client-python/src/client.rs` - Added `check_connectivity` method with `Arc` wrapper

**Implementation:**

使用 `Arc<EwsClient>` 而不是 `Clone`，以确保所有异步任务共享同一个 `server_version`：

```rust
use std::sync::Arc;

#[pyclass]
pub struct PyEwsClient {
    inner: Arc<EwsClient>,
}

#[pymethods]
impl PyEwsClient {
    #[new]
    fn new(endpoint: String, username: String, password: String) -> PyResult<Self> {
        let endpoint = endpoint.parse()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
        let credentials = Credentials::basic(username, password);
        let client = EwsClient::new(endpoint, credentials)
            .map_err(|e| ews_error_to_py_err(&e))?;
        Ok(Self { inner: Arc::new(client) })
    }

    fn check_connectivity<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client.check_connectivity().await
                .map_err(|err| ews_error_to_py_err(&err))
        })
    }
}
```

**关键设计决策:**

1. **使用 `Arc` 而不是 `Clone`**:
   - 避免 `server_version` 数据不一致问题
   - 所有异步任务共享同一个 `EwsClient` 实例
   - `server_version` 更新对所有任务可见
   - 内存高效（只复制指针）

2. **为什么不用 `Clone`**:
   - `EwsClient` 包含 `AtomicCell<ExchangeServerVersion>`
   - 克隆会创建独立的 `AtomicCell`，导致版本更新不同步
   - 系统有全局 `SERVER_VERSION_CACHE`，但每个实例也有本地缓存
   - 使用 `Arc` 确保所有任务看到相同的版本信息

**Status:** ✅ 完成 - 编译通过，Clippy 无警告

---

### Step 3.4: Complex Type Conversion (P2 - 核心功能)

**Priority:** 🟡 High - Required by sync methods

- [ ] Implement `FolderHierarchySyncResult` conversion
- [ ] Implement `SyncMessagesResult` conversion
- [ ] Implement `CreateMessageResult` conversion
- [ ] Handle nested structures (`FolderInfo`, `SyncMessageInfo`)
- [ ] Handle `HashMap` conversion

**Files to modify:**

- `ews-client-python/src/types.rs` - Add complex type conversions (~200 lines)

**Implementation Details:**

```rust
// Convert Rust struct to Python dict/dataclass
impl IntoPy<PyObject> for FolderHierarchySyncResult {
    fn into_py(self, py: Python) -> PyObject {
        // Create Python dict or use dataclass
        // Handle nested FolderInfo objects
    }
}
```

**Status:** ⏳ Pending

---

### Step 3.5: Simple Sync Methods (P2 - 核心功能)

**Priority:** 🟡 High - Basic operations

- [ ] Implement `create_folder(parent_id: str, name: str) -> str`
- [ ] Implement `delete_folder(folder_ids: list[str]) -> None`
- [ ] Implement `update_folder(folder_id: str, folder_name: str) -> None`
- [ ] Implement `delete_messages(item_ids: list[str]) -> None`

**Files to modify:**

- `ews-client-python/src/client.rs` - Add 4 methods (~100 lines)

**Status:** ⏳ Pending

---

### Step 3.6: Sync Operation Methods (P2 - 核心功能)

**Priority:** 🟡 High - Core sync operations

- [ ] Implement `sync_folder_hierarchy(sync_state: str | None) -> FolderHierarchySyncResult`
- [ ] Implement `sync_messages(folder_id: str, sync_state: str | None) -> SyncMessagesResult`
- [ ] Implement `get_message(message_id: str) -> bytes`
- [ ] Implement `create_message(folder_id: str, content: bytes, is_draft: bool, is_read: bool) -> CreateMessageResult`

**Files to modify:**

- `ews-client-python/src/client.rs` - Add 4 methods (~120 lines)

**Status:** ⏳ Pending

---

### Step 3.7: Batch Operation Methods (P3 - 高级功能)

**Priority:** 🟠 Medium - Batch operations with parameter conversion

- [ ] Implement `change_read_status(item_ids: list[str], is_read: bool) -> list[str]`
- [ ] Implement `change_read_status_all(folder_ids: list[str], is_read: bool, suppress_read_receipts: bool) -> None`
- [ ] Implement `mark_as_junk(item_ids: list[str], is_junk: bool, legacy_junk_folder_id: str) -> list[str]`
- [ ] Implement `copy_folders(destination_folder_id: str, folder_ids: list[str]) -> list[str]`
- [ ] Implement `move_folders(destination_folder_id: str, folder_ids: list[str]) -> list[str]`
- [ ] Implement `copy_items(destination_folder_id: str, item_ids: list[str]) -> list[str]`
- [ ] Implement `move_items(destination_folder_id: str, item_ids: list[str]) -> list[str]`

**Files to modify:**

- `ews-client-python/src/client.rs` - Add 7 methods (~150 lines)

**Implementation Note:** These methods require converting `list[str]` to `&[&str]` in the binding layer.

**Status:** ⏳ Pending

---

### Step 3.8: send_message Method (P3 - 高级功能)

**Priority:** 🟠 Medium - Requires special type conversion

- [ ] Implement `Recipient` type conversion from Python tuples
- [ ] Implement `send_message(mime_content: str, message_id: str, should_request_dsn: bool, bcc_recipients: list[tuple[str | None, str | None]]) -> None`

**Files to modify:**

- `ews-client-python/src/types.rs` - Add `Recipient` conversion (~50 lines)
- `ews-client-python/src/client.rs` - Add `send_message` method (~80 lines)

**Implementation Details:**

```rust
// Convert Python tuple to Recipient
impl FromPyObject<'_> for Recipient {
    fn extract(ob: &Bound<PyAny>) -> PyResult<Self> {
        let (name, email): (Option<String>, Option<String>) = ob.extract()?;
        Ok(Recipient {
            mailbox: Mailbox {
                name,
                email_address: email.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("email is required"))?,
            },
            routing_type: None,
        })
    }
}
```

**Status:** ⏳ Pending

---

### Step 3.9: Python Testing (P4 - 测试与文档)

**Priority:** 🔵 Low - Testing phase

- [ ] Write unit tests for all methods
- [ ] Write integration tests with mock server
- [ ] Write type checking tests (mypy)
- [ ] Verify error handling

**Files to create/modify:**

- `tests/python/test_client.py` - Add comprehensive tests (~300 lines)
- `tests/python/conftest.py` - Setup fixtures and mock server

**Status:** ⏳ Pending

## Phase 4: Testing & Documentation (Week 6)

### Step 4.1: Python Tests

- [ ] pytest configuration
- [ ] Mock server for Python tests
- [ ] Integration tests for all operations
- [ ] Type checking tests (mypy)

**Files to create:**

- `tests/python/conftest.py`
- `tests/python/test_client.py`
- `tests/python/test_operations.py`
- `tests/python/test_types.py`

### Step 4.2: Documentation

- [ ] Rust API documentation
- [ ] Python API documentation
- [ ] Usage examples
- [ ] Development guide

**Files to create:**

- `docs/api/rust.md`
- `docs/api/python.md`
- `docs/examples/basic_usage.md`
- `docs/examples/advanced.md`
- `docs/development.md`

### Step 4.3: Example Code

- [ ] Rust examples
- [ ] Python examples

**Files to create:**

- `examples/rust/basic_client.rs`
- `examples/python/basic_usage.py`
- `examples/python/sync_folders.py`
- `examples/python/sync_messages.py`

## Phase 5: OAuth2 Support (Week 7, Optional)

### Step 5.1: OAuth2 Implementation

- [ ] OAuth2 credentials type
- [ ] Token refresh mechanism
- [ ] Token storage (optional)

**Files to modify:**

- `ews-client-core/src/client/credentials.rs`

### Step 5.2: Documentation & Examples

- [ ] OAuth2 documentation
- [ ] OAuth2 examples

**Files to create:**

- `docs/examples/oauth2.md`
- `examples/python/oauth2_example.py`

## Dependencies

### Workspace Dependencies

```toml
[workspace.dependencies]
ews = { git = "https://github.com/thunderbird/ews-rs.git", version = "0.1.0" }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
base64 = "0.22"
url = "2.5"
pyo3 = { version = "0.27", features = ["extension-module"] }
pyo3-async-runtimes = { version = "0.27", features = ["tokio-runtime"] }
```

### Dev Dependencies

```toml
[workspace.dev-dependencies]
wiremock = "0.6"
```

## Milestones

- **Week 2**: Core Rust client with basic operations working
- **Week 4**: All EWS operations implemented and tested
- **Week 5**: Python bindings complete with type hints
- **Week 6**: Full test coverage and documentation
- **Week 7**: OAuth2 support (optional)

## Success Criteria

- [ ] All Rust code passes `cargo clippy` with no warnings
- [ ] All Rust code formatted with `cargo fmt`
- [ ] Unit test coverage > 80%
- [ ] All public APIs have documentation comments
- [ ] Python type checking passes `mypy --strict`
- [ ] Python code passes `ruff check`
- [ ] All examples run successfully
- [ ] Documentation is complete and accurate
- [ ] All examples run successfully
- [ ] Documentation is complete and accurate
