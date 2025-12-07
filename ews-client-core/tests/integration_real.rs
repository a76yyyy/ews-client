//! Real server integration tests entry point
//!
//! This file serves as the entry point for all real EWS server integration tests.
//! It includes tests from the integration/real directory.
//!
//! These tests require a live EWS server and are marked with `#[ignore]` by default.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::ignored_unit_patterns,
    clippy::indexing_slicing,
    clippy::print_stdout
)]

// Include real server integration tests
#[path = "integration/real/folder_operations.rs"]
mod folder_operations;

#[path = "integration/real/item_operations.rs"]
mod item_operations;
