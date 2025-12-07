//! Type conversion between Rust and Python.
//!
//! This module provides type conversion utilities for EWS client types.
//! PyO3 automatically handles conversions for basic types:
//! - `Vec<T>` ↔ `list[T]`
//! - `Option<T>` ↔ `Optional[T]`
//! - `String` ↔ `str`
//! - `Vec<u8>` ↔ `bytes`
//! - `HashMap<K, V>` ↔ `dict[K, V]`
//! - `(T, U)` ↔ `tuple[T, U]`
//!
//! See:
//! - `reference/pyo3/guide/src/conversions/tables.md` - Type mapping table
//! - `reference/pyo3/guide/src/conversions/traits.md` - Conversion traits
//!
//! Complex type conversions (`FolderInfo`, `FolderHierarchySyncResult`, etc.)
//! will be implemented in Phase 2 when we add the actual methods that return these types.
