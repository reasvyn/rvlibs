//! CLI subcommand implementations for `cargo rvtest`.
//!
//! # Submodules
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`args`] | CLI argument struct ([`Cli`]) |
//! | [`profile`] | Profile resolution, colour detection |
//! | [`impact`] | Git-based impact analysis |
//! | [`toolchain`] | Rust toolchain checks (nightly, cranelift) |
//! | [`tune`] | Hardware auto-tuning |
//! | [`cache`] | Build cache & warm state |
//! | [`runner`] | `cargo test` execution |
//! | [`builder`] | Test binary building & isolated execution |
//! | [`render`] | Report rendering |
//! | [`coverage`] | Coverage collection |
//! | [`watch`] | File-watch mode (`--watch`) |
//! | [`flaky`] | Flaky test detection (`--detect-flaky`) |

pub mod args;
pub mod builder;
pub mod cache;
pub mod coverage;
pub mod flaky;
pub mod impact;
pub mod profile;
pub mod render;
pub mod runner;
pub mod toolchain;
pub mod tune;
pub mod watch;

// Re-export for backward compat — main.rs calls these at `cli::*`.
pub use self::builder::*;
pub use self::cache::*;
pub use self::coverage::*;
pub use self::impact::*;
pub use self::profile::*;
pub use self::render::*;
pub use self::runner::*;
pub use self::toolchain::*;
pub use self::tune::*;
