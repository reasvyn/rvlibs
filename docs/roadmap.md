# Roadmap

## rvmath

Focused on **pure mathematics** — algebra, analysis, linear algebra, discrete mathematics.

### Polynomial Module
`Polynomial<T>` type with dense coefficient storage, generic over `Numeric`. Full arithmetic, root finding (closed-form for degrees 1–4, Newton/Durand–Kerner for higher), Lagrange and Newton interpolation. Integration with `algebra::Expr`.

### Special Functions Module
Gamma family (`gamma`, `ln_gamma`, `digamma`, `beta`, incomplete gamma), error function (`erf`, `erfc`, `erf_inv`), Riemann zeta (`zeta`), Bessel functions (`bessel_j0`/`j1`/`jn`/`y0`/`y1`/`yn`), `sinc`, `airy_ai`.

### Enhanced Linear Algebra
LU, QR, SVD, Cholesky decompositions. Eigenvalue decomposition (power iteration, QR algorithm). Linear system solvers. Matrix norms. Unit-aware extensions.

### Graph Module
`Graph<N, E>` with adjacency list storage. BFS, DFS, topological sort. Dijkstra, Bellman-Ford, Floyd-Warshall shortest paths. Kruskal and Prim MST. Edmonds-Karp and Dinic max flow. `MatN` conversion.

### Out of Scope
Probability and statistics, advanced numerical methods (ODE, optimisation), signal processing (FFT), SIMD optimisation, `no_std`.

---

## rvtest

### Current State
All core features are implemented and stable: BDD specs, property-based testing, parametrized tests, assertion macros, mocking, snapshots, architecture tests, code coverage (self-contained profraw parser), CLI runner (filter, tag, retry, timeout, parallel, fail-fast), watch mode, daemon mode, flaky detection, benchmark regression, HTML reports, last-run cache.

### Security
- Process-per-test isolation (thread → process model)
- Secrets masking in test output
- Test execution sandboxing (filesystem, network, env)
- Resource limits per test

### Performance
- Source-level impact analysis (`--impact`)
- Test result caching (skip unchanged tests)
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
- Competing with dedicated tools (mockall, proptest, insta) where no clear value
