//! UUID type for the [RON crate](https://crates.io/ron-crdt).
//!
//! Handles parsing and serialization of UUIDs.

#![warn(missing_docs)]

extern crate chrono;
#[macro_use]
extern crate quickcheck;

mod uuid;
pub use uuid::UUID;
