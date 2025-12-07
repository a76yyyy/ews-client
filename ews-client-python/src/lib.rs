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
    // Add version
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Register exception classes
    error::register_exceptions(m)?;

    // Register client class with user-friendly name
    m.add_class::<client::PyEwsClient>()?;
    m.add("EwsClient", m.py().get_type::<client::PyEwsClient>())?;

    // Register type classes with user-friendly names
    m.add_class::<types::PyFolderInfo>()?;
    m.add("FolderInfo", m.py().get_type::<types::PyFolderInfo>())?;

    m.add_class::<types::PyFolderHierarchySyncResult>()?;
    m.add(
        "FolderHierarchySyncResult",
        m.py().get_type::<types::PyFolderHierarchySyncResult>(),
    )?;

    m.add_class::<types::PySyncMessageInfo>()?;
    m.add("SyncMessageInfo", m.py().get_type::<types::PySyncMessageInfo>())?;

    m.add_class::<types::PySyncMessagesResult>()?;
    m.add("SyncMessagesResult", m.py().get_type::<types::PySyncMessagesResult>())?;

    m.add_class::<types::PyCreateMessageResult>()?;
    m.add("CreateMessageResult", m.py().get_type::<types::PyCreateMessageResult>())?;

    Ok(())
}
