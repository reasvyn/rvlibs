# Roadmap

## Current State

rvlibs is in active development. Both crates have stable, functional cores:

- **rvmath** — Type-safe numeric system with `Num<T>`/`Numeric`, unit dimensions, linear algebra (`VecN`, `MatN`, `Tensor`), symbolic algebra, calculus, geometry, and string expression evaluation.
- **rvtest** — BDD specs (`describe`/`it`), property-based testing (`check`/`Strategy`), parametrized tests, assertion macros with diffs, mocking (`Spy`/`Stub`/`patch!`), snapshots, architecture tests, code coverage (self-contained `.profraw` parser), CLI runner (filter, tag, retry, timeout, parallel, fail-fast, watch, daemon, flaky detection, benchmark regression, HTML reports).

## Pure Mathematics

### Polynomial Module
`Polynomial<T>` with dense coefficient storage, generic over `Numeric`. Root finding (closed-form for degrees 1–4, Newton/Durand–Kerner for higher), Lagrange and Newton interpolation, integration with `algebra::Expr`.

### Special Functions Module
Gamma family (`gamma`, `ln_gamma`, `digamma`, `beta`, incomplete gamma), error function (`erf`, `erfc`, `erf_inv`), Riemann zeta (`zeta`), Bessel functions, `sinc`, `airy_ai`.

### Enhanced Linear Algebra
LU, QR, SVD, Cholesky decompositions. Eigenvalue decomposition. Linear system solvers. Matrix norms. Unit-aware extensions.

### Graph Module
`Graph<N, E>` with adjacency list storage. BFS, DFS, topological sort. Dijkstra, Bellman-Ford, Floyd-Warshall shortest paths. Kruskal and Prim MST. Edmonds-Karp and Dinic max flow. `MatN` conversion.

### Out of Scope
Probability and statistics, advanced numerical methods (ODE, optimisation), signal processing (FFT), SIMD optimisation, `no_std`.

## Testing Infrastructure

### Security
- Process-per-test isolation (thread → process model)
- Secrets masking in test output
- Test execution sandboxing (filesystem, network, env)
- Resource limits per test

### Performance
- Source-level impact analysis (`--impact`)
- Test result caching
- Test binary build cache
- Parallel execution within a single binary
- Smart `--fast` defaults (auto-detect linker, RAM, CPU)
- Warm daemon auto-start

### Features
- Trait mocking (`#[automock]`)
- Async test support (tokio)
- Time/clock mocking
- Composable matchers (`assert_that!`)
- Inline snapshots
- Config file (`rvtest.toml`)
- Property testing depth (proptest-compatible)
- Typed test fixtures (`#[fixture]`)

### Non-Goals
- Replacing Cargo's test harness entirely
- Runtime reflection or code generation
- Competing with dedicated tools (mockall, proptest, insta) where integration provides no clear value

## Shared Contracts

- Expand `rvlibs` crate with more shared traits as cross-crate patterns emerge
- Extract common types from rvmath and rvtest into `rvlibs` when they need to be shared
- Maintain zero external dependencies for the shared crate

## Ecosystem

- Establish release cadence and versioning strategy across workspace crates
- Publish all crates to crates.io
- Set up CI with coverage gates and cross-crate integration tests
- Add examples and cookbook-style documentation for common use cases combining multiple crates
