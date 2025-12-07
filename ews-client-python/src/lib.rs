//! Python bindings for the EWS (Exchange Web Services) client.
//!
//! This crate provides Python bindings for the `ews-client-core` library,
//! allowing Python applications to interact with Microsoft Exchange servers.

/// Python client wrapper for EWS operations.
pub mod client;
/// Error mapping from Rust to Python exceptions.
pub mod error;
/// Type conversion between Rust and Python.
pub mod types;

use pyo3::prelude::*;

#[pymodule]
fn _ews_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register exception classes
    error::register_exceptions(m)?;

    // Register client class
    m.add_class::<client::PyEwsClient>()?;

    Ok(())
}
