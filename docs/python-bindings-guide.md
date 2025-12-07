# Python 绑定实现指南

## 概述

本文档详细说明如何实现 EWS Client 的 Python 绑定，包括错误映射、类型转换和异步方法包装。

## 错误体系设计

### Python 错误类层次结构

所有 EWS 错误都继承自 `BaseEWSError`，这样用户可以用一个 `except` 子句捕获所有 EWS 错误：

```python
try:
    await client.sync_folder_hierarchy()
except BaseEWSError as e:
    print(f"EWS error: {e}")
```

### 错误映射表

| Rust `EwsError` | Python 异常 | 说明 |
|---|---|---|
| `EwsError::Authentication` | `EWSAuthenticationError` | 认证失败 (401, 无效凭据等) |
| `EwsError::Http(...)` | `EWSHTTPError` | HTTP 传输错误 (网络、连接等) |
| `EwsError::Protocol(...)` | `EWSProtocolError` | EWS 协议错误 (SOAP 解析、XML 问题等) |
| `EwsError::ResponseError(...)` | `EWSResponseError` | EWS 响应包含错误代码 |
| `EwsError::Processing { message }` | `EWSProcessingError` | 处理响应数据时出错 (验证、格式等) |
| `EwsError::MissingIdInResponse` | `EWSMissingIdError` | 响应中缺少必需的 ID |
| `EwsError::Serialization(...)` | `EWSSerializationError` | JSON 序列化/反序列化错误 |
| 其他 | `BaseEWSError` | 其他未分类的错误 |

## 实现进度

### P1: 基础设施 ✅

- ✅ **错误映射** - 使用 `create_exception!` 宏创建异常类层次结构
- ✅ **基础类型转换** - 依赖 PyO3 自动转换
- ✅ **check_connectivity** - 完成，使用 `Arc` 共享客户端

### P2: 核心功能 ⏳

- ⏳ 复杂类型转换
- ⏳ 简单同步方法
- ⏳ 同步操作方法

### P3: 高级功能 ⏳

- ⏳ 批量操作方法
- ⏳ send_message 方法

### P4: 测试与文档 ⏳

- ⏳ Python 测试

## Rust 实现细节

### 1. 错误映射 (`ews-client-python/src/error.rs`) ✅

```rust
//! Error mapping from Rust to Python

use pyo3::prelude::*;
use ews_client_core::EwsError;

/// Create Python exception classes and register them with the module
pub fn create_error_classes(py: Python, module: &Bound<PyModule>) -> PyResult<()> {
    // Create base exception class
    let base_error = PyErr::new::<pyo3::exceptions::PyException, _>("BaseEWSError");
    module.add("BaseEWSError", base_error)?;

    // Create specific exception classes
    // Note: In PyO3, we typically use built-in Python exceptions or create custom ones
    // For now, we'll use PyErr::new with string messages

    Ok(())
}

/// Convert Rust EwsError to Python PyErr
impl From<EwsError> for PyErr {
    fn from(err: EwsError) -> Self {
        match err {
            EwsError::Authentication => {
                PyErr::new::<pyo3::exceptions::PyAuthenticationError, _>(
                    "Authentication failed"
                )
            }
            EwsError::Http(http_err) => {
                PyErr::new::<pyo3::exceptions::PyConnectionError, _>(
                    format!("HTTP error: {}", http_err)
                )
            }
            EwsError::Protocol(protocol_err) => {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("EWS protocol error: {}", protocol_err)
                )
            }
            EwsError::ResponseError(response_err) => {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("EWS response error: {:?}", response_err)
                )
            }
            EwsError::Processing { message } => {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("Processing error: {}", message)
                )
            }
            EwsError::MissingIdInResponse => {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Missing ID in response"
                )
            }
            EwsError::Serialization(serde_err) => {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("Serialization error: {}", serde_err)
                )
            }
            EwsError::InvalidUrl(url_err) => {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Invalid URL: {}", url_err)
                )
            }
            EwsError::UnexpectedResponseMessageCount { expected, actual } => {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!(
                        "Unexpected response message count: expected {}, got {}",
                        expected, actual
                    )
                )
            }
        }
    }
}
```

### 2. 基础类型转换 (`ews-client-python/src/types.rs`) ✅

PyO3 自动处理基础类型转换，无需手动实现：

| Rust 类型 | Python 类型 | 说明 |
|-----------|-------------|------|
| `Vec<T>` | `list[T]` | 自动转换 |
| `Option<T>` | `Optional[T]` | 自动转换 |
| `String` | `str` | 自动转换 |
| `Vec<u8>` | `bytes` | 自动转换 |
| `HashMap<K, V>` | `dict[K, V]` | 自动转换 |
| `(T, U)` | `tuple[T, U]` | 自动转换 |

参考文档：

- `reference/pyo3/guide/src/conversions/tables.md`
- `reference/pyo3/guide/src/conversions/traits.md`

复杂类型（如 `FolderHierarchySyncResult`）将在 P2 阶段实现。

### 3. 异步方法包装 (`ews-client-python/src/client.rs`) ✅

```rust
//! Python client wrapper with async support

use crate::error::ews_error_to_py_err;
use ews_client_core::{Credentials, EwsClient};
use pyo3::prelude::*;
use std::sync::Arc;

/// Python wrapper for the EWS client.
///
/// Uses `Arc` to share the client across multiple async tasks, ensuring that
/// server version updates are visible to all tasks.
#[pyclass]
pub struct PyEwsClient {
    inner: Arc<EwsClient>,
}

#[pymethods]
impl PyEwsClient {
    #[new]
    fn new(endpoint: String, username: String, password: String) -> PyResult<Self> {
        let endpoint = endpoint
            .parse()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;

        let credentials = Credentials::basic(username, password);
        let client = EwsClient::new(endpoint, credentials)
            .map_err(|e| ews_error_to_py_err(&e))?;

        Ok(Self {
            inner: Arc::new(client),
        })
    }

    /// Test connection and authentication to the EWS server.
    fn check_connectivity<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .check_connectivity()
                .await
                .map_err(|err| ews_error_to_py_err(&err))
        })
    }
}
```

## 关键实现要点

### 1. 使用 Arc 共享 EwsClient

由于异步方法需要将 `EwsClient` 移动到异步块中，我们使用 `Arc<EwsClient>` 而不是 `Clone`。

**为什么使用 Arc:**

1. **避免 server_version 不一致**: `EwsClient` 包含 `AtomicCell<ExchangeServerVersion>`，克隆会创建独立的 `AtomicCell`，导致版本更新不同步
2. **共享状态**: 所有异步任务共享同一个 `EwsClient` 实例，`server_version` 更新对所有任务可见
3. **内存高效**: 只复制指针（8 字节），而不是整个结构
4. **符合设计**: 系统有全局 `SERVER_VERSION_CACHE`，使用 `Arc` 确保本地缓存也是共享的

**实现:**

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
        let client = EwsClient::new(endpoint, credentials)?;
        Ok(Self { inner: Arc::new(client) })
    }
}
```

### 2. 异步方法的 GIL 管理

使用 `pyo3_async_runtimes::tokio::future_into_py` 自动处理 GIL 释放：

```rust
fn async_method<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    let client = Arc::clone(&self.inner);  // 只复制 Arc 指针
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        // 异步代码在这里运行，GIL 已释放
        client.some_async_operation().await
            .map_err(|err| ews_error_to_py_err(&err))
    })
}
```

### 3. 参数转换

对于需要 `&[&str]` 的方法，在 Python 绑定层进行转换：

```rust
fn change_read_status<'py>(
    &self,
    py: Python<'py>,
    item_ids: Vec<String>,
    is_read: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let client = Arc::clone(&self.inner);
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        // 将 Vec<String> 转换为 Vec<&str>
        let ids: Vec<&str> = item_ids.iter().map(|s| s.as_str()).collect();
        client.change_read_status(&ids, is_read).await
            .map_err(|err| ews_error_to_py_err(&err))
    })
}
```

### 4. 复杂类型转换

对于返回复杂类型的方法，使用 `IntoPy` 特性：

```rust
impl IntoPy<PyObject> for FolderHierarchySyncResult {
    fn into_py(self, py: Python) -> PyObject {
        // 创建 Python 字典或使用 dataclass
        let dict = pyo3::types::PyDict::new_bound(py);
        dict.set_item("sync_state", self.sync_state).ok();
        dict.set_item("created_folders", self.created_folders).ok();
        dict.set_item("updated_folders", self.updated_folders).ok();
        dict.set_item("deleted_folder_ids", self.deleted_folder_ids).ok();
        dict.set_item("well_known_folders", self.well_known_folders).ok();
        dict.into()
    }
}
```

## 开发流程

### P1: 基础设施 ✅

1. ✅ **错误映射** - 使用 `create_exception!` 宏创建异常类
2. ✅ **基础类型转换** - 依赖 PyO3 自动转换
3. 🔄 **check_connectivity** - 实现第一个异步方法

### P2: 核心功能

1. 实现复杂类型转换（`FolderHierarchySyncResult` 等）
2. 实现简单同步方法（`create_folder`, `delete_folder` 等）
3. 实现同步操作方法（`sync_folder_hierarchy`, `sync_messages` 等）

### P3: 高级功能

1. 实现批量操作方法
2. 实现 `send_message` 方法

### P4: 测试与文档

1. 编写 Python 测试
2. 更新文档

## 常见问题

### Q: 为什么使用 Arc 而不是 Clone？

A:

1. **数据一致性**: `EwsClient` 包含 `AtomicCell<ExchangeServerVersion>`，克隆会创建独立的副本，导致版本更新不同步
2. **共享状态**: 使用 `Arc` 确保所有异步任务看到相同的 `server_version`
3. **内存效率**: `Arc` 只复制指针，而 `Clone` 会复制整个结构
4. **符合设计**: 系统有全局版本缓存，`Arc` 确保本地缓存也是共享的

### Q: 如何处理 GIL？

A: `pyo3_async_runtimes::tokio::future_into_py` 自动处理 GIL 释放。在异步代码执行期间，GIL 被释放，允许其他 Python 线程运行。

### Q: 如何处理错误？

A: 使用 `map_err(|err| ews_error_to_py_err(&err))` 将 `EwsError` 转换为 `PyErr`。`ews_error_to_py_err` 函数会将 Rust 错误映射到相应的 Python 异常类。

### Q: 如何处理复杂类型？

A: 为每个复杂类型实现 `IntoPy<PyObject>`。可以创建 Python 字典或使用 dataclass。

## 参考资源

- [PyO3 文档](https://pyo3.rs/)
- [PyO3 异步运行时](https://github.com/PyO3/pyo3-async-runtimes)
- [Python 异常处理](https://docs.python.org/3/library/exceptions.html)
