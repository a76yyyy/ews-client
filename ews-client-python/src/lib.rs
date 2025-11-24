//! Python bindings for the EWS (Exchange Web Services) client.
//!
//! This crate provides Python bindings for the `ews-client-core` library,
//! allowing Python applications to interact with Microsoft Exchange servers.

mod client;
mod error;
mod types;

use pyo3::prelude::*;

#[pymodule]
fn _ews_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<client::PyEwsClient>()?;
    Ok(())
}
