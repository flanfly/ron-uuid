//! UUID type for the [RON crate](https://crates.io/ron-crdt).
//!
//! Handles parsing and serialization of UUIDs.

#![warn(missing_docs)]

extern crate chrono;

mod uuid;
pub use uuid::UUID;
