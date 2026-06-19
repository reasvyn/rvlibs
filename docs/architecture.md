# Architecture

rvlibs is a Rust monorepo organised as a Cargo workspace. Each crate is an independent library with its own module hierarchy, sharing common conventions and the workspace build system.

## Workspace Layout

```
Cargo.toml (root workspace)
├── crates/rvmath/        # Mathematics library
├── crates/rvtest/        # Testing library
├── crates/rvtest-macros/ # Proc-macros for rvtest
└── crates/cargo-rvtest/  # CLI binary for rvtest
```

The root `Cargo.toml` defines the workspace and shared package metadata. Each crate has its own `Cargo.toml`, source tree, and tests. Internal path dependencies (`path = ".."`) keep everything local.

## rvmath Module Hierarchy

```
prelude ← re-exports from all layers
  ├── algebra    — symbolic expressions (Expr enum), simplification, differentiation
  ├── calculus   — analytical & numerical derivatives, integrals, series
  ├── geometry   — constants, 2D/3D shape formulas, unit-aware
  ├── num        — Num<T> wrapper, Numeric trait, Percentage, Set
  ├── la         — VecN<T,N>, MatN<T,R,C>, Tensor<T> (linear algebra)
  ├── unit       — type-safe dimensional analysis, families and units via macros
  ├── utils      — string expression parser and evaluator
  └── consts     — mathematical and physical constants
```

Dependencies flow upward: `consts` stands alone, `num`/`unit`/`la` form the foundation, `geometry`/`utils` build on them, `algebra`/`calculus` are the highest layer.

## rvtest Module Hierarchy

```
lib.rs
├── core      — TestSuite, TestCase, TestStatus (no deps)
├── tag       — tag/name filtering (depends on core)
├── spec      — BDD builder: describe/it/run (depends on core, tag)
├── property  — property-based testing: Strategy, check (depends on rand)
├── param     — parametrized tests: parametrize (depends on core)
├── assert    — assertion macros with diff output (depends on core, similar)
├── mock      — Spy, Stub, patch! (depends on core)
├── arch      — architecture dependency checks
├── snapshot  — file-based snapshot assertions
├── capture   — per-test stdout/stderr capture (depends on libc)
├── report    — TestReporter trait + Pretty/TAP/JUnit/JSON/Compact/GitHub
├── runner    — TestRunner, execution orchestration (depends on core, report, spec)
├── daemon    — persistent compile daemon (depends on core, report, runner, notify)
├── coverage  — multi-strategy coverage collector (depends on core, coverage_raw)
└── coverage_raw — pure-Rust .profraw parser (depends on serde, core)
```

No circular dependencies. Each module depends only on `core` or sibling modules.

## CLI Architecture (cargo-rvtest)

```
cargo rvtest
├── cargo test (subprocess) ──► parse output ──► render via TestReporter
├── --coverage ──► cargo test --no-run ──► collect .profraw ──► parse ──► report
├── --watch ──► notify-based file watcher, re-run on change
└── --daemon ──► persistent compile daemon, sub-second iteration
```

The CLI is a separate crate (`cargo-rvtest`) that wraps `cargo test` and adds formatting, coverage, watch, and daemon modes.

## Key Design Decisions

- **Generic numerics** — All rvmath operations work on any `Numeric` type (f32, f64, i32, Num<f64>, custom types).
- **Type-level dimensions** — rvmath units carry dimensions at the type level (`Unit<N, Meter>` vs `Unit<N, Kilometer>`).
- **Zero proc-macros by default** — rvtest core API uses only plain Rust functions; proc-macros are optional.
- **Dogfooding** — rvtest tests itself with its own BDD API.
- **Safe Rust only** — No `unsafe` in either crate.
