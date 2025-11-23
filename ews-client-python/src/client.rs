use ews_client_core::{Credentials, EwsClient};
use pyo3::prelude::*;

#[pyclass]
pub struct PyEwsClient {
    inner: EwsClient,
}

#[pymethods]
impl PyEwsClient {
    #[new]
    fn new(endpoint: String, username: String, password: String) -> PyResult<Self> {
        let endpoint = endpoint
            .parse()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

        let credentials = Credentials::basic(username, password);
        let client = EwsClient::new(endpoint, credentials)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;

        Ok(Self { inner: client })
    }
}
