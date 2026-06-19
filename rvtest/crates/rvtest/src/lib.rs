//! **rvtest** — A Next Level Testing Library for Rust.
//!
//! *Just a library — not a framework, not a product.*
//!
//! `rvtest` extends Rust's built-in testing capabilities with a rich suite
//! of features:
//!
//! It does **not** replace `#[test]` or `cargo test`. It does **not** require
//! you to adopt a new framework or restructure your project. You add it as a
//! dev-dependency and use it exactly where it helps — nothing more.
//!
//! - **BDD-style specs** — organise tests with `describe` / `it` blocks,
//!   nested hierarchies, lifecycle hooks (`before_all`, `after_all`,
//!   `before_each`, `after_each`), tags, timeouts, and retries.
//! - **Property-based testing** — verify invariants over randomly generated
//!   inputs, with automatic counterexample shrinking.
//! - **Parametrized tests** — run the same test logic against multiple
//!   inputs without boilerplate.
//! - **Assertion macros** — `assert_eq!` with structural diffs, `assert_ok!`,
//!   `assert_err!`, `assert_matches!`, `assert_delta!`.
//! - **Mocking utilities** — `Spy` (call-recording), `Stub` (fixed return),
//!   `patch!` (scoped function replacement). No proc-macro required.
//! - **Snapshot testing** — file-based snapshot assertions with
//!   `--update-all` auto-accept and `--review` interactive mode.
//! - **Architecture tests** — enforce module dependency rules
//!   (`may_depend_on`, `may_not_depend_on`, `must_not_have_cycles`,
//!   `public_api_doc_required`).
//! - **Output capture** — per-test stdout/stderr capture, shown only
//!   on failure.
//! - **Rich reporting** — Pretty (human-readable with colour), TAP, JUnit
//!   XML, JSON, Compact, GitHub Actions annotations, and Agent-native
//!   (LLM-optimised JSON with source snippets).
//! - **Code coverage** — measure line/function/region coverage via
//!   pure-Rust `.profraw` parser (`cargo rvtest --coverage`). No external
//!   LLVM tools required. Falls back to `cargo-llvm-cov` or `llvm-tools`
//!   when available.
//! - **Configurable runner** — parallel execution, name/tag/skip filtering,
//!   fail-fast, configurable timeouts, retries, and execution-order
//!   shuffling.
//! - **Flaky quarantine** — `--quarantine` skips known-flaky tests;
//!   `--flaky-report` lists them.
//! - **Git-aware test selection** — `--changed` auto-filters tests based
//!   on `git diff`.
//! - **Last-run cache** — `--retest` / `--failed` re-runs only previously
//!   failed tests; `--diff` shows new failures, recovered tests, and
//!   duration changes.
//! - **Environment & filesystem utilities** — `rvtest::env` (RAII env var
//!   guards) and `rvtest::fs` (auto-cleaning temp directories).
//! - **Optional proc-macro API** — `#[describe]` / `#[it]` attribute macros
//!   via the `macros` feature.
//! - **Zero proc-macro deps by default** — Everything works with plain
//!   Rust functions and closures inside standard `#[test]` functions.
//!
//! # Library API
//!
//! Use rvtest inside standard `#[test]` functions:
//!
//! ```ignore
//! use rvtest::spec::describe;
//!
//! #[test]
//! fn calculator_tests() {
//!     describe("Calculator")
//!         .it("adds two positive numbers", || {
//!             assert_eq!(2 + 2, 4);
//!         })
//!         .it("subtracts", || {
//!             assert_eq!(5 - 3, 2);
//!         })
//!         .tag("arithmetic")
//!         .timeout(std::time::Duration::from_secs(2))
//!         .retries(1)
//!         .run()
//!         .assert_all_pass();
//! }
//! ```
//!
//! # CLI (`cargo rvtest`)
//!
//! The `cargo-rvtest` binary (install via `cargo install cargo-rvtest`)
//! runs your project's tests via `cargo test` and renders results in
//! any supported format. See `cargo rvtest --help` for all options.
//!
//! # Feature flags
//!
//! - `macros` — Enables `#[describe]` / `#[it]` proc-macro attributes
//!   (re-exported from `rvtest-macros`).

/// Architecture-enforcement tests — declare module dependency rules.
pub mod arch;
pub mod assert;
pub mod capture;
/// Artifact integrity checksums — SHA-256 manifest for snapshots, coverage, etc.
pub mod checksum;
pub mod config;
pub mod core;
/// Persistent compile daemon — builds once, runs test binaries directly.
pub mod daemon;
/// Environment variable utilities with RAII guards.
pub mod env;
/// Filesystem utilities — temporary directories with automatic cleanup.
pub mod fs;
pub mod mock;
/// Code coverage collection with multiple backend strategies.
pub mod coverage;
/// Pure-Rust `.profraw` parser for self-contained coverage.
pub mod coverage_raw;
/// Parametrized tests — run the same logic with multiple inputs.
pub mod param;
/// Property-based testing with random generation and shrinking.
pub mod property;
pub mod report;
pub mod runner;
/// Test execution sandboxing — restrict filesystem, network, and env access.
pub mod sandbox;
/// Secrets masking — redact API keys, tokens, and passwords in test output.
pub mod secrets;
pub mod snapshot;
pub mod spec;
/// Tag and name filtering utilities for test selection.
pub mod tag;

/// Re-export of the optional proc-macro crate.
///
/// Enabled via the `macros` feature:
///
/// ```toml
/// [dependencies]
/// rvtest = { version = "0.2", features = ["macros"] }
/// ```
///
/// Then use:
///
/// ```ignore
/// use rvtest::*;
/// ```
#[cfg(feature = "macros")]
pub use rvtest_macros::{after_all, before_all, describe, it, retries, tag, timeout};

/// The `prelude` module re-exports the most commonly used types and
/// functions for convenience.
pub mod prelude {
    pub use crate::arch::{arch_check, ArchCheck};
    pub use crate::core::{CoverageFormat, CoverageReport, ReportFormat, RunnerConfig, TestRun, TestStatus, TestSuite};
    pub use crate::coverage::{CoverageCollector, CoverageConfig};
    pub use crate::env::{remove_var, set_var, set_vars};
    pub use crate::fs::{temp_dir, temp_dir_with_prefix, TempDir};
    pub use crate::param::{parametrize, parametrize_named};
    pub use crate::property::{any, check, check_with, PropertyConfig, Strategy};
    pub use crate::runner::TestRunner;
    pub use crate::snapshot::{assert_snapshot, assert_snapshot_in};
    #[cfg(feature = "macros")]
    pub use rvtest_macros::{after_all, before_all, describe, it, retries, tag, timeout};
    #[cfg(not(feature = "macros"))]
    pub use crate::spec::{describe, Spec};
}
