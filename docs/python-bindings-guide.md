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

## Rust 实现细节

### 1. 错误映射 (`ews-client-python/src/error.rs`)

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

### 2. 基础类型转换 (`ews-client-python/src/types.rs`)

```rust
//! Type conversion between Rust and Python

use pyo3::prelude::*;

/// Convert Vec<String> to Python list[str]
impl IntoPy<PyObject> for Vec<String> {
    fn into_py(self, py: Python) -> PyObject {
        self.into_py(py)
    }
}

/// Convert Python list[str] to Vec<String>
impl<'a> FromPyObject<'a> for Vec<String> {
    fn extract(ob: &'a Bound<PyAny>) -> PyResult<Self> {
        ob.extract::<Vec<String>>()
    }
}

/// Convert Option<T> to Python Optional[T]
impl<T: IntoPy<PyObject>> IntoPy<PyObject> for Option<T> {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Some(val) => val.into_py(py),
            None => py.None(),
        }
    }
}

/// Convert Python Optional[T] to Option<T>
impl<'a, T: FromPyObject<'a>> FromPyObject<'a> for Option<T> {
    fn extract(ob: &'a Bound<PyAny>) -> PyResult<Self> {
        if ob.is_none() {
            Ok(None)
        } else {
            T::extract(ob).map(Some)
        }
    }
}

/// Convert bytes to Python bytes
impl IntoPy<PyObject> for Vec<u8> {
    fn into_py(self, py: Python) -> PyObject {
        PyBytes::new_bound(py, &self).into()
    }
}

/// Convert Python bytes to Vec<u8>
impl<'a> FromPyObject<'a> for Vec<u8> {
    fn extract(ob: &'a Bound<PyAny>) -> PyResult<Self> {
        Ok(ob.extract::<&[u8]>()?.to_vec())
    }
}
```

### 3. 异步方法包装 (`ews-client-python/src/client.rs`)

```rust
//! Python client wrapper with async support

use pyo3::prelude::*;
use ews_client_core::EwsClient;

#[pyclass]
pub struct PyEwsClient {
    #[allow(dead_code)]
    inner: EwsClient,
}

#[pymethods]
impl PyEwsClient {
    #[new]
    fn new(endpoint: String, username: String, password: String) -> PyResult<Self> {
        let endpoint = endpoint
            .parse()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;

        let credentials = ews_client_core::Credentials::basic(username, password);
        let client = EwsClient::new(endpoint, credentials)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}")))?;

        Ok(Self { inner: client })
    }

    /// Test connection and authentication to the EWS server
    fn check_connectivity<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client.check_connectivity().await.map_err(Into::into)
        })
    }
}
```

## 关键实现要点

### 1. EwsClient 需要实现 Clone

由于异步方法需要将 `EwsClient` 移动到异步块中，`EwsClient` 必须实现 `Clone`。

在 `ews-client-core/src/client/mod.rs` 中添加：

```rust
#[derive(Clone)]
pub struct EwsClient {
    endpoint: Url,
    credentials: Credentials,
    client: Client,
    pub(crate) server_version: AtomicCell<ExchangeServerVersion>,
}
```

### 2. 异步方法的 GIL 管理

使用 `pyo3_async_runtimes::tokio::future_into_py` 自动处理 GIL 释放：

```rust
fn async_method<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    let client = self.inner.clone();
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        // 异步代码在这里运行，GIL 已释放
        client.some_async_operation().await.map_err(Into::into)
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
    let client = self.inner.clone();
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        // 将 Vec<String> 转换为 Vec<&str>
        let ids: Vec<&str> = item_ids.iter().map(|s| s.as_str()).collect();
        client.change_read_status(&ids, is_read).await.map_err(Into::into)
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

### 第一步：实现错误映射

1. 创建 `python/ews_client/errors.py` 定义 Python 异常类
2. 实现 `ews-client-python/src/error.rs` 中的 `From<EwsError> for PyErr`
3. 在 `ews-client-python/src/lib.rs` 中注册错误类

### 第二步：实现基础类型转换

1. 在 `ews-client-python/src/types.rs` 中实现基本类型的 `FromPyObject` 和 `IntoPy`
2. 测试类型转换是否正确

### 第三步：实现异步方法

1. 确保 `EwsClient` 实现了 `Clone`
2. 在 `ews-client-python/src/client.rs` 中实现 `#[pymethods]`
3. 使用 `pyo3_async_runtimes::tokio::future_into_py` 包装异步调用

### 第四步：测试

1. 编写 Python 测试验证错误映射
2. 编写 Python 测试验证类型转换
3. 编写 Python 测试验证异步方法

## 常见问题

### Q: 为什么需要 Clone？

A: 异步方法需要将 `EwsClient` 移动到异步块中。由于 Rust 的所有权规则，我们需要克隆它以保持原始引用。

### Q: 如何处理 GIL？

A: `pyo3_async_runtimes::tokio::future_into_py` 自动处理 GIL 释放。在异步代码执行期间，GIL 被释放，允许其他 Python 线程运行。

### Q: 如何处理错误？

A: 使用 `map_err(Into::into)` 将 `EwsError` 转换为 `PyErr`。`From<EwsError> for PyErr` 实现会自动处理转换。

### Q: 如何处理复杂类型？

A: 为每个复杂类型实现 `IntoPy<PyObject>`。可以创建 Python 字典或使用 dataclass。

## 参考资源

- [PyO3 文档](https://pyo3.rs/)
- [PyO3 异步运行时](https://github.com/PyO3/pyo3-async-runtimes)
- [Python 异常处理](https://docs.python.org/3/library/exceptions.html)
