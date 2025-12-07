//! Mock-based integration tests entry point
//!
//! This file serves as the entry point for all mock-based integration tests.
//! It includes tests from the integration/mock directory.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::ignored_unit_patterns,
    clippy::indexing_slicing,
    clippy::print_stdout
)]

// Include common test infrastructure
#[path = "common/mod.rs"]
mod common;

// Include mock integration tests
#[path = "integration/mock/infrastructure.rs"]
mod infrastructure;

#[path = "integration/mock/folder_operations.rs"]
mod folder_operations;

#[path = "integration/mock/item_operations.rs"]
mod item_operations;
