//! Shared contracts and foundations for the rvlibs ecosystem.
//!
//! This crate provides types, traits, and utilities shared across all rvlibs
//! crates.  It exists to prevent circular dependencies and to define the
//! public contracts that crate implementations fulfil.
//!
//! # Contracts defined here
//!
//! - **`Error`** / **`Result`** — shared error types for cross-crate operations
//! - **`Version`** — package version metadata
//! - **`Rvlibs`** — re-exports and convenience constants
//!
//! Crates that implement a shared contract depend on `rvlibs`; crates that
//! *use* an implementation also depend on `rvlibs` (for the type) plus the
//! implementing crate.

// ---------------------------------------------------------------------------
// Version
// ---------------------------------------------------------------------------

/// Semantic version information for this build of rvlibs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    /// Pre-release tag, e.g. `"alpha.1"`, `""` for stable.
    pub pre: &'static str,
}

impl Version {
    pub const fn new(major: u16, minor: u16, patch: u16, pre: &'static str) -> Self {
        Self { major, minor, patch, pre }
    }
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if !self.pre.is_empty() {
            write!(f, "-{}", self.pre)?;
        }
        Ok(())
    }
}

/// The current rvlibs version, sourced from `Cargo.toml` at build time.
pub fn rvlibs_version() -> Version {
    Version {
        major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
        minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
        patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        pre: "",
    }
}

// ---------------------------------------------------------------------------
// Error / Result
// ---------------------------------------------------------------------------

/// A generic error type for the rvlibs ecosystem.
///
/// Individual crates may define their own error enums; this type is used
/// at the boundaries between crates.
#[derive(Debug)]
pub enum Error {
    NotFound(String),
    InvalidInput(String),
    Internal(String),
    Io(std::io::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::NotFound(msg) => write!(f, "not found: {msg}"),
            Error::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            Error::Internal(msg) => write!(f, "internal error: {msg}"),
            Error::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

/// Convenience alias for `Result<T, rvlibs::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

// ---------------------------------------------------------------------------
// Re-exports
// ---------------------------------------------------------------------------

/// Re-export the workspace package metadata at runtime.
///
/// Useful for crates that want to expose their version without
/// duplicating the version string.
pub mod meta {
    /// Crate name.
    pub const NAME: &str = env!("CARGO_PKG_NAME");
    /// Full version string (e.g. `"0.1.0"`).
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    /// Short description from `Cargo.toml`.
    pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    /// Package license expression.
    pub const LICENSE: &str = env!("CARGO_PKG_LICENSE");
    /// Package repository URL.
    pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
}
