//! Unit tests entry point
//!
//! This file serves as the entry point for all unit tests.
//! It includes tests from the unit directory.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::ignored_unit_patterns,
    clippy::indexing_slicing,
    clippy::print_stdout
)]

// Include unit tests
#[path = "unit/operations.rs"]
mod operations;
