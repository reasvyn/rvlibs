//! Shared contracts and foundations for the rvlibs ecosystem.
//!
//! This crate provides types, traits, and utilities shared across all rvlibs
//! crates.  It exists to prevent circular dependencies and to define the
//! public contracts that crate implementations fulfil.
//!
//! # Modules
//!
//! - [`error`] — `Error` enum and `Result` alias
//! - [`version`] — `Version` struct and `current()` function
//! - [`meta`] — compile-time package metadata constants

pub mod error;
pub mod version;
pub mod meta;

pub use error::{Error, Result};
pub use version::Version;
