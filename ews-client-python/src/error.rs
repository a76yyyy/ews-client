//! Error mapping from Rust to Python
//!
//! Defines custom exception classes for EWS errors using `PyO3`'s `create_exception`! macro.
//! This creates a proper exception hierarchy with `BaseEWSError` as the base class.

use ews_client_core::EwsError;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

// Create the exception hierarchy using PyO3's create_exception! macro
// This creates a proper exception hierarchy with BaseEWSError as the base class
// and specific exception types for different error categories
pyo3::create_exception!(
    _ews_client,
    BaseEWSError,
    PyException,
    "Base exception for all EWS client errors."
);
pyo3::create_exception!(
    _ews_client,
    EWSAuthenticationError,
    BaseEWSError,
    "Authentication failure (401, invalid credentials, etc.)."
);
pyo3::create_exception!(
    _ews_client,
    EWSHTTPError,
    BaseEWSError,
    "HTTP transport error (network, connection, etc.)."
);
pyo3::create_exception!(
    _ews_client,
    EWSProtocolError,
    BaseEWSError,
    "EWS protocol error (SOAP parsing, XML issues, etc.)."
);
pyo3::create_exception!(
    _ews_client,
    EWSResponseError,
    BaseEWSError,
    "EWS response contained an error code."
);
pyo3::create_exception!(
    _ews_client,
    EWSProcessingError,
    BaseEWSError,
    "Error processing response data (validation, unexpected format, etc.)."
);
pyo3::create_exception!(
    _ews_client,
    EWSMissingIdError,
    BaseEWSError,
    "Missing required ID in response from Exchange."
);
pyo3::create_exception!(
    _ews_client,
    EWSSerializationError,
    BaseEWSError,
    "JSON serialization/deserialization error."
);

/// Register all exception classes with the module.
///
/// # Errors
///
/// Returns an error if adding any exception class to the module fails.
pub fn register_exceptions(module: &Bound<PyModule>) -> PyResult<()> {
    module.add("BaseEWSError", module.py().get_type::<BaseEWSError>())?;
    module.add(
        "EWSAuthenticationError",
        module.py().get_type::<EWSAuthenticationError>(),
    )?;
    module.add("EWSHTTPError", module.py().get_type::<EWSHTTPError>())?;
    module.add("EWSProtocolError", module.py().get_type::<EWSProtocolError>())?;
    module.add("EWSResponseError", module.py().get_type::<EWSResponseError>())?;
    module.add("EWSProcessingError", module.py().get_type::<EWSProcessingError>())?;
    module.add("EWSMissingIdError", module.py().get_type::<EWSMissingIdError>())?;
    module.add("EWSSerializationError", module.py().get_type::<EWSSerializationError>())?;
    Ok(())
}

/// Convert Rust `EwsError` to Python `PyErr`.
///
/// Maps each Rust error variant to the corresponding custom Python exception type.
pub fn ews_error_to_py_err(err: &EwsError) -> PyErr {
    let msg = err.to_string();

    match err {
        EwsError::Authentication => EWSAuthenticationError::new_err(msg),
        EwsError::Http(_) => EWSHTTPError::new_err(msg),
        EwsError::Protocol(_) => EWSProtocolError::new_err(msg),
        EwsError::ResponseError(_) => EWSResponseError::new_err(msg),
        EwsError::Processing { .. } => EWSProcessingError::new_err(msg),
        EwsError::MissingIdInResponse => EWSMissingIdError::new_err(msg),
        EwsError::Serialization(_) => EWSSerializationError::new_err(msg),
        EwsError::InvalidUrl(_) => {
            // For URL errors, use EWSProcessingError as it's a data validation issue
            EWSProcessingError::new_err(msg)
        }
        EwsError::UnexpectedResponseMessageCount { .. } => {
            // For response count errors, use EWSProcessingError
            EWSProcessingError::new_err(msg)
        }
    }
}
