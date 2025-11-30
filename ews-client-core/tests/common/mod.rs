//! Common test utilities and fixtures for ews-client-core tests
//!
//! This module provides:
//! - Mock EWS server implementation
//! - SOAP response fixtures
//! - Test utilities and helpers

pub mod fixtures;
pub mod mock_server;
pub mod test_utils;

pub use mock_server::*;
