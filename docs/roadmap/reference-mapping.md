# Reference Mapping: ews_xpcom → ews-client-core

本文档记录从 `ews_xpcom` 到 `ews-client-core` 的代码映射关系。

> ## 代码库描述
>
> `ews_xpcom` 参考代码库位于 `reference/thunderbird-desktop/rust/ews_xpcom` 目录中, 是为 Thunderbird 的 XPCOM 架构设计的,它有以下特点:
>
> 1. 依赖 XPCOM 基础设施 - 使用了 nsIMsgIncomingServer、RefPtr 等 XPCOM 特定类型
> 2. 与 Firefox/Thunderbird 紧密耦合 - 依赖 moz_http、moz_task 等 Mozilla 特定的 crate
> 3. 异步模型基于 moz_task - 不是标准的 Tokio runtime
>
> `ews-client-core` 是参考 `ews_xpcom` 代码库中去除 XPCOM 依赖后的一个纯粹的、独立的 Python EWS 客户端, 不依赖任何 Thunderbird/Firefox 的基础设施, 它有以下几个特点:
>
> 1. 直接使用 ews-rs - 这是底层的 EWS 协议实现
> 2. 异步模型基于 Tokio runtime - 不是 moz_task runtime
> 3. 移除 XPCOM 依赖 - 使用了标准 Rust 类型 (String, Arc, etc.)
> 4. 依赖 reqwest 异步 HTTP 客户端 - 使用了标准 Rust 异步类型 (Future, Stream, etc.), 不是 moz_http

## Core Client Structure

| ews_xpcom | ews-client-core | Notes |
|-----------|-----------------|-------|
| `src/lib.rs` (XpcomEwsBridge) | `src/client/mod.rs` (EwsClient) | 移除 XPCOM 依赖 |
| `src/client.rs` (XpComEwsClient) | `src/client/mod.rs` (EwsClient) | 使用 reqwest 替代 moz_http |
| `src/authentication/credentials.rs` | `src/client/credentials.rs` | 移除 nsIMsgIncomingServer trait |
| `src/authentication/oauth_listener.rs` | ❌ 不需要 | OAuth2 直接在 credentials 中处理 |

## Error Handling

| ews_xpcom | ews-client-core | Notes |
|-----------|-----------------|-------|
| `src/client.rs` (XpComEwsError) | `src/client/error.rs` (EwsError) | 移除 nsresult 依赖 |

## Operations

### Folder Operations

| ews_xpcom | ews-client-core | Notes |
|-----------|-----------------|-------|
| `src/client/sync_folder_hierarchy.rs` | `src/client/operations/sync_folder_hierarchy.rs` | 移除 SafeEwsFolderListener |
| `src/client/create_folder.rs` | `src/client/operations/create_folder.rs` | 移除 SafeEwsSimpleOperationListener |
| `src/client/delete_folder.rs` | `src/client/operations/delete_folder.rs` | 移除 listener 模式 |
| `src/client/update_folder.rs` | `src/client/operations/update_folder.rs` | 移除 listener 模式 |
| `src/client/copy_move_operations/copy_move_folder.rs` | `src/client/operations/copy_move_operations/folder.rs` | 简化泛型实现 |

### Message Operations

| ews_xpcom | ews-client-core | Notes |
|-----------|-----------------|-------|
| `src/client/sync_messages_for_folder.rs` | `src/client/operations/sync_messages.rs` | 移除 SafeEwsMessageSyncListener |
| `src/client/get_message.rs` | `src/client/operations/get_message.rs` | 移除 SafeEwsMessageFetchListener |
| `src/client/create_message.rs` | `src/client/operations/create_message.rs` | 移除 SafeEwsMessageCreateListener |
| `src/client/delete_messages.rs` | `src/client/operations/delete_messages.rs` | 移除 listener 模式 |
| `src/client/change_read_status.rs` | `src/client/operations/change_read_status.rs` | 移除 listener 模式 |
| `src/client/change_read_status_all.rs` | `src/client/operations/change_read_status_all.rs` | 移除 listener 模式 |
| `src/client/mark_as_junk.rs` | `src/client/operations/mark_as_junk.rs` | 移除 listener 模式 |
| `src/client/copy_move_operations/copy_move_item.rs` | `src/client/operations/copy_move_operations/item.rs` | 简化泛型实现 |

### Other Operations

| ews_xpcom | ews-client-core | Notes |
|-----------|-----------------|-------|
| `src/client/send_message.rs` | `src/client/operations/send_message.rs` | 移除 SafeMsgOutgoingListener |
| `src/client/check_connectivity.rs` | `src/client/operations/check_connectivity.rs` | 移除 SafeUrlListener |

## Supporting Modules

| ews_xpcom | ews-client-core | Notes |
|-----------|-----------------|-------|
| `src/headers.rs` | `src/headers.rs` | 保持不变，移除 XPCOM 类型 |
| `src/xpcom_io.rs` | ❌ 不需要 | 使用标准 Rust I/O |
| `src/cancellable_request.rs` | ❌ 不需要 | 使用 tokio 的取消机制 |
| `src/client/server_version.rs` | `src/client/server_version.rs` | 移除 nsIPrefBranch 依赖 |

## Removed Modules (XPCOM Specific)

以下模块不需要在 ews-client-core 中实现：

- `src/safe_xpcom/` - 所有 XPCOM 安全包装器
  - `folder_listener.rs`
  - `message_create_listener.rs`
  - `message_fetch_listener.rs`
  - `message_sync_listener.rs`
  - `msg_db_hdr.rs`
  - `msg_outgoing_listener.rs`
  - `simple_operation_listener.rs`
  - `uri.rs`
  - `url_listener.rs`

- `src/outgoing.rs` - XPCOM 出站服务器实现

## Key Differences

### 1. Async Model

- **ews_xpcom**: 使用 `moz_task::spawn_local`
- **ews-client-core**: 使用标准 `tokio::spawn`

### 2. HTTP Client

- **ews_xpcom**: 使用 `moz_http::Client`
- **ews-client-core**: 使用 `reqwest::Client`

### 3. Error Handling

- **ews_xpcom**: 返回 `nsresult`
- **ews-client-core**: 返回 `Result<T, EwsError>`

### 4. Callbacks

- **ews_xpcom**: 使用 listener 模式 (XPCOM callbacks)
- **ews-client-core**: 直接返回 `Future<Output = Result<T, E>>`

### 5. Type System

- **ews_xpcom**: 使用 XPCOM 类型 (nsCString, RefPtr, etc.)
- **ews-client-core**: 使用标准 Rust 类型 (String, Arc, etc.)

## Implementation Strategy

### Phase 1: Direct Port

对于每个操作，按以下步骤移植：

1. 复制核心逻辑
2. 移除 listener 相关代码
3. 将 XPCOM 类型替换为标准 Rust 类型
4. 将 `moz_http` 替换为 `reqwest`
5. 将 `moz_task` 替换为 `tokio`
6. 简化错误处理

### Phase 2: Simplification

移植完成后，进行简化：

1. 移除不必要的抽象层
2. 合并重复代码
3. 优化类型转换
4. 改进错误消息

### Phase 3: Enhancement

添加新功能：

1. 更好的重试机制
2. 连接池管理
3. 请求取消支持
4. 进度报告（可选）

## Code Patterns

### Pattern 1: Operation Implementation

**ews_xpcom**:

```rust
struct DoSyncFolderHierarchy<'a> {
    pub listener: &'a SafeEwsFolderListener,
    pub sync_state_token: Option<String>,
}

impl DoOperation for DoSyncFolderHierarchy<'_> {
    async fn do_operation<ServerT: ServerType>(
        &mut self,
        client: &XpComEwsClient<ServerT>,
    ) -> Result<Self::Okay, XpComEwsError> {
        // ... operation logic
        self.listener.on_folder_created(...)?;
    }
}
```

**ews-client-core**:

```rust
impl EwsClient {
    pub async fn sync_folder_hierarchy(
        &self,
        sync_state: Option<String>,
    ) -> Result<FolderSyncResult, EwsError> {
        // ... operation logic
        Ok(FolderSyncResult {
            folders: created_folders,
            sync_state: new_sync_state,
        })
    }
}
```

### Pattern 2: Error Handling

**ews_xpcom**:

```rust
match result {
    Ok(value) => listener.on_success(value)?,
    Err(err) => listener.on_failure(&err)?,
}
```

**ews-client-core**:

```rust
result // 直接返回 Result
```

### Pattern 3: Type Conversion

**ews_xpcom**:

```rust
let id = nsCString::from(folder_id.id);
unsafe { self.0.OnFolderCreated(&*id, ...) }
```

**ews-client-core**:

```rust
Ok(folder_id.id) // 直接返回 String
