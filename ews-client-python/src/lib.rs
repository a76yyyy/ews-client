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
use std::sync::OnceLock;

/// Get the version of the ews-client-python library
///
/// Returns the version string, converting Rust semver format to Python-compatible format
pub fn get_ews_client_version() -> &'static str {
    static VERSION: OnceLock<String> = OnceLock::new();

    VERSION.get_or_init(|| {
        let version = env!("CARGO_PKG_VERSION");
        // cargo uses "1.0-alpha1" etc. while python uses "1.0.0a1", this is not full compatibility,
        // but it's good enough for now
        // see https://docs.rs/semver/1.0.9/semver/struct.Version.html#method.parse for rust spec
        // see https://peps.python.org/pep-0440/ for python spec
        // it seems the dot after "alpha/beta" e.g. "-alpha.1" is not necessary, hence why this works
        version.replace("-alpha", "a").replace("-beta", "b")
    })
}

#[pymodule]
fn _ews_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add version
    m.add("__version__", get_ews_client_version())?;

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
