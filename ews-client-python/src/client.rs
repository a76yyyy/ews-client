//! HTTP Client for EWS, Implemented from Rust to Python

use crate::error::ews_error_to_py_err;
use ews_client_core::{Credentials, EwsClient};
use pyo3::prelude::*;
use std::sync::Arc;

/// Python wrapper for the EWS client.
///
/// Provides async methods for interacting with Microsoft Exchange servers.
///
/// Uses `Arc` to share the client across multiple async tasks, ensuring that
/// server version updates are visible to all tasks.
#[pyclass]
pub struct PyEwsClient {
    #[allow(dead_code)]
    inner: Arc<EwsClient>,
}

#[pymethods]
impl PyEwsClient {
    #[new]
    #[allow(clippy::needless_pass_by_value)]
    fn new(endpoint: String, username: String, password: String) -> PyResult<Self> {
        let endpoint = endpoint
            .parse()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;

        let credentials = Credentials::basic(username, password);
        let client = EwsClient::new(endpoint, credentials).map_err(|e| ews_error_to_py_err(&e))?;

        Ok(Self {
            inner: Arc::new(client),
        })
    }

    /// Test connection and authentication to the EWS server.
    ///
    /// Returns a coroutine that resolves to `None` on success.
    ///
    /// # Errors
    ///
    /// Raises an exception if the connection test fails.
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
