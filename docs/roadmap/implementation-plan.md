# Implementation Plan

## 当前进度概览

**当前阶段:** Phase 2 - Step 2.3 ✅ 已完成

**已完成:**

- ✅ Phase 1.1: 基础设施搭建
- ✅ Phase 1.2: 核心操作实现 (连接检查、文件夹同步、消息获取)
- ✅ Phase 2.1: 文件夹操作 (创建、删除、更新、复制、移动)
- ✅ Phase 2.2: 消息操作 (同步、创建、删除、读取状态、垃圾邮件、复制、移动)
- ✅ Phase 2.3: 发送消息功能

**进行中:**

- 🔄 Phase 2.4: 消息头支持

**待开展:**

- ⏸️ Phase 1.3: 系统性测试 (优先完成所有操作后再进行)
- ⏸️ Phase 3: Python 绑定
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

### Step 1.3: Testing

- [ ] Unit tests for each operation
- [ ] Mock server integration tests
- [ ] Error handling tests

**Files to create:**

- `tests/rust/test_client.rs`
- `tests/rust/test_operations.rs`
- `tests/rust/fixtures/*.xml` (mock responses)

**Status:** ⏸️ 暂未开展 - 优先完成所有操作实现后再进行系统性测试

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

**Files created:**

- `ews-client-core/src/client/operations/sync_messages.rs`
- `ews-client-core/src/client/operations/create_message.rs`
- `ews-client-core/src/client/operations/delete_messages.rs`
- `ews-client-core/src/client/operations/change_read_status.rs` (包含 `change_read_status_all`)
- `ews-client-core/src/client/operations/mark_as_junk.rs`
- `ews-client-core/src/client/operations/copy_move_operations/item.rs`

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

### Step 2.4: Headers Support

- [ ] Message headers parsing - 参考 `reference/thunderbird-desktop/rust/ews_xpcom/src/headers.rs`
- [ ] Mailbox type support

**Files to create:**

- `ews-client-core/src/headers.rs`

## Phase 3: Python Bindings (Week 5)

### Step 3.1: Basic Bindings

- [ ] PyO3 module setup
- [ ] `PyEwsClient` base structure
- [ ] Async bridge (tokio → asyncio)
- [ ] Error mapping to Python exceptions

**Files to create:**

- `ews-client-python/Cargo.toml`
- `ews-client-python/src/lib.rs`
- `ews-client-python/src/client.rs`
- `ews-client-python/src/error.rs`

### Step 3.2: Type Conversion

- [ ] Rust → Python type conversion
- [ ] Python → Rust type conversion
- [ ] Handle `Vec`, `Option`, `Result`
- [ ] Complex types (FolderSyncResult, MessageSyncResult, etc.)

**Files to create:**

- `ews-client-python/src/types.rs`
- `python/ews_client/types.py`

### Step 3.3: Type Hints

- [ ] `.pyi` stub files
- [ ] `types.py` type definitions
- [ ] `py.typed` marker

**Files to create:**

- `python/ews_client/__init__.pyi`
- `python/ews_client/types.pyi`
- `python/ews_client/py.typed`

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
