# Roadmap

> **Last analysis:** 2026-06-23 (architecture-design)
> Status legend: ✅ done · 🟡 partial · ❌ not started · 🔴 blocked

## Current State

rvlibs is in active development with a clear direction: **Rveco** as the final estuary.

### Active crates

| Crate | Role | Status |
|-------|------|--------|
| **rvmath** | Foundation — mathematics | ✅ Active |
| **rvtest** | Cross-cutting — testing | ✅ Active |
| **rvtest-macros** | Cross-cutting — proc-macros | ✅ Active |
| **cargo-rvtest** | Cross-cutting — CLI binary | ✅ Active |
| **rvlibs** | Foundation — shared contracts | ⏳ Internal |
| **rvnx** | Brain — ECS, scene graph, ports | 🔜 Planned |
| **rvfx** | Body — rendering, window, UI | 🔜 Planned |
| **rveco** | Estuary — main application | 🔜 Planned |

### Ecosystem Vision

```
Atomic crates:   rvmath  rvtest  rvstat  rvphysic  rv* ...
                      \     /       |         |
Composite crates:     rvnx (brain)  rvfx (body)
                            \         /
Estuary:                rveco (application)
```

The ecosystem is built modularly and incrementally. New crates are only created when truly needed (not preemptive). Rveco is the final destination that gives direction to all libraries.

---

## Pure Mathematics

### Polynomial Module
> **Status:** ✅ Done (2026-06-19) · **Crate:** rvmath · **Complexity:** Medium

**Analysis:**
`Polynomial<T>` with dense `Vec<T>` coefficient storage. Builds on existing `Expr` AST, `Num<T>`/`Numeric` trait, and numerical root-finding (`newton_raphson`, `bisection`). Closed-form solvers for degrees 1–4, Newton/Durand–Kerner for degree 5+, Lagrange/Newton interpolation.

**Dependencies:** rvlibs only. No new external deps. Safe Rust. MSRV 1.85+.

**API sketch:**
```rust
pub struct Polynomial<T: Numeric>(Vec<T>);
impl<T: Numeric> Polynomial<T> {
    pub fn new(coeffs: Vec<T>) -> Self;
    pub fn degree(&self) -> usize;
    pub fn evaluate(&self, x: T) -> T;
    pub fn derivative(&self) -> Self;
    pub fn integral(&self) -> Self;
    pub fn roots(&self) -> Vec<Num<f64>>;
}
pub fn solve_quadratic<T: Numeric>(a: T, b: T, c: T) -> Vec<Num<f64>>;
pub fn solve_cubic<T: Numeric>(a: T, b: T, c: T, d: T) -> Vec<Num<f64>>;
pub fn solve_quartic<T: Numeric>(a: T, b: T, c: T, d: T, e: T) -> Vec<Num<f64>>;
pub fn lagrange_interpolate<T: Numeric>(pts: &[(T, T)]) -> Polynomial<T>;
pub fn newton_interpolate<T: Numeric>(pts: &[(T, T)]) -> Polynomial<T>;
```

**Module placement:** `rvmath/src/polynomial/` (mod.rs + root.rs + interpolation.rs)

---

### Special Functions Module
> **Status:** ✅ Done (2026-06-19) · **Crate:** rvmath · **Complexity:** Large

**Analysis:**
Gamma family (Lanczos approximation), error function (rational approximation), Riemann zeta (series + Euler-Maclaurin), Bessel functions, `sinc`, `airy_ai`. Each function family requires individual numerical analysis. Constants `EULER_MASCHERONI` and `APERY_CONSTANT` exist.

**Dependencies:** rvlibs only. No new external deps. Safe Rust. MSRV 1.85+.

**API sketch:**
```rust
// rvmath::special
pub fn gamma<T: Numeric>(x: T) -> Num<f64>;
pub fn ln_gamma<T: Numeric>(x: T) -> Num<f64>;
pub fn digamma<T: Numeric>(x: T) -> Num<f64>;
pub fn beta<T: Numeric>(a: T, b: T) -> Num<f64>;
pub fn erf<T: Numeric>(x: T) -> Num<f64>;
pub fn erfc<T: Numeric>(x: T) -> Num<f64>;
pub fn erf_inv<T: Numeric>(x: T) -> Num<f64>;
pub fn zeta<T: Numeric>(s: T) -> Num<f64>;
pub fn bessel_j<T: Numeric>(n: i32, x: T) -> Num<f64>;
```

**Module placement:** `rvmath/src/special/` (gamma.rs, erf.rs, zeta.rs, bessel.rs, mod.rs)

**Notes:**
- Each function family needs ~200-300 lines of implementation
- Numerical stability requires careful coefficient selection (well-studied in literature)
- zeta for s ≤ 1 needs analytic continuation

---

### Enhanced Linear Algebra
> **Status:** ✅ Done (2026-06-19) · **Crate:** rvmath · **Complexity:** Large

**Implemented:**
- LU decomposition with partial pivoting → `lu()` returns `(perm, L, U)`
- Determinant `det()` via LU
- Matrix inverse `inv()` via LU
- Linear system solver `solve()`
- Frobenius norm `norm_frobenius()`, L1 norm `norm_l1()`, infinity norm `norm_inf()`

**Analysis:**
Existing `MatN<T, ROWS, COLS>` has `transpose`, `det2`, `det3`, `inv2`, and matrix multiplication. Missing: LU/QR/SVD/Cholesky decompositions, eigenvalue decomposition, linear system solvers, matrix norms.

**Constraint:** Decompositions need `sqrt`/`abs` operations not guaranteed by `Numeric`. A `Float` subtrait is needed.

**Dependencies:** rvlibs only. No new external deps. Safe Rust. MSRV 1.85+.

**API sketch:**
```rust
pub trait Float: Numeric { fn sqrt(self) -> Self; fn abs(self) -> Self; }

impl<T: Float, const N: usize> MatN<T, N, N> {
    pub fn det(&self) -> T;                              // via LU
    pub fn inv(&self) -> Option<Self>;
    pub fn solve(&self, b: &VecN<T, N>) -> Option<VecN<T, N>>;
    pub fn lu(&self) -> Option<(Self, Self)>;
    pub fn qr(&self) -> Option<(Self, Self)>;
    pub fn cholesky(&self) -> Option<Self>;
    pub fn eigenvalues(&self) -> Vec<Num<f64>>;
}
impl<T: Float, const R: usize, const C: usize> MatN<T, R, C> {
    pub fn svd(&self) -> Option<SVD<T, R, C>>;
    pub fn norm_frobenius(&self) -> T;
}
```

**Module placement:** `rvmath/src/la/` — add `lu.rs`, `qr.rs`, `svd.rs`, `norm.rs`, `eigen.rs` as submodules.

**Notes:**
- SVD is the most algorithmically complex (Golub-Reinsch)
- Compile-time matrix sizes make iterative algorithms allocate dynamically
- A `Float` subtrait is needed (see GitHub Issues)

---

### Graph Module
> **Status:** ✅ Done (2026-06-19) · **Crate:** rvmath · **Complexity:** Large

**Analysis:**
`Graph<N, E>` with adjacency list storage. Standard textbook algorithms. `MatN` conversion is constrained by compile-time sizes — use `Tensor<T>` or `Vec<Vec<T>>` for dynamic graphs.

**Dependencies:** rvlibs, `std::collections::BinaryHeap`. No new external deps. Safe Rust. MSRV 1.85+.

**API sketch:**
```rust
pub struct Graph<N, E = ()> { nodes: Vec<N>, edges: Vec<Vec<(usize, E)>> }
impl<N, E: Clone> Graph<N, E> {
    pub fn new() -> Self;
    pub fn add_node(&mut self, data: N) -> usize;
    pub fn add_edge(&mut self, from: usize, to: usize, weight: E);
    pub fn bfs(&self, start: usize) -> impl Iterator<Item = usize>;
    pub fn dfs(&self, start: usize) -> impl Iterator<Item = usize>;
    pub fn topological_sort(&self) -> Option<Vec<usize>>;
    pub fn dijkstra(&self, start: usize) -> Vec<Option<f64>>;
    pub fn bellman_ford(&self, start: usize) -> Result<Vec<Option<f64>>, NegativeCycle>;
    pub fn floyd_warshall(&self) -> Vec<Vec<Option<f64>>>;
    pub fn kruskal_mst(&self) -> Vec<(usize, usize, E)>;
    pub fn prim_mst(&self) -> Vec<(usize, usize, E)>;
    pub fn edmonds_karp(&self, s: usize, t: usize) -> f64;
    pub fn dinic(&self, s: usize, t: usize) -> f64;
}
```

**Module placement:** `rvmath/src/graph/` (mod.rs, traversal.rs, shortest_path.rs, mst.rs, flow.rs)

**Notes:**
- `MatN` conversion → `Tensor<T>` or `Vec<Vec<T>>`
- Dinic max flow needs level graph + blocking flow

---

### Out of Scope
Probability and statistics, advanced numerical methods (ODE, optimisation), signal processing (FFT), SIMD optimisation, `no_std`.

---

## Testing Infrastructure

### Security — ✅ All Done

| Feature | Status | Details |
|---------|--------|---------|
| Process-per-test isolation | ✅ | `--isolate`, `run_tests_isolated()` |
| Secrets masking | ✅ | 15+ regex patterns, `--mask-secrets` |
| Sandboxing | ✅ | FS/network/env isolation, `--sandbox` |
| Resource limits | ✅ | fds, processes, stack, AS, `setrlimit` |

### Performance

| Feature | Status | Crate | Complexity |
|---------|--------|-------|-----------|
| Impact analysis (`--impact`) | ✅ | cargo-rvtest | Small |
| Test result caching | ✅ | rvtest | Small |
| Build cache | ✅ | cargo-rvtest | Small |
| Parallel execution | ✅ | rvtest | Small |
| Smart `--fast` defaults | 🟡 | cargo-rvtest | **Small** |
| Warm daemon auto-start | ✅ | cargo-rvtest | Medium |

#### Smart `--fast` defaults
> **Status:** 🟡 Partial · **What's needed:**
Auto-apply detected optimal settings (linker, parallelism, cache) in `main()` without explicit `--fast`/`--tune`. Detection code exists in `tune.rs`; just needs wiring.

### Features

| Feature | Status | Crate | Complexity |
|---------|--------|-------|-----------|
| Config file (`rvtest.toml`) | ✅ | rvtest | Small |
| Property testing depth | ✅ | rvtest | Medium |
| Trait mocking (`#[automock]`) | 🔴 | rvtest-macros | Large |
| Async test support | ✅ | rvtest | Medium |
| Time/clock mocking | ✅ | rvtest | Medium |
| Composable matchers (`assert_that!`) | ✅ | rvtest | Medium |
| Inline snapshots | 🔴 | rvtest | Large |
| Typed fixtures (`#[fixture]`) | ❌ | rvtest-macros | Large |

#### Trait mocking (`#[automock]`)
> **Status:** 🔴 Reclassified · **Issue:** Competes with `mockall`

The roadmap's own "Non-Goals" section states "Competing with dedicated tools (mockall, proptest, insta) where integration provides no clear value." A full `#[automock]` proc-macro would be 500-800+ lines to handle generics, lifetimes, async, associated types — all handled by `mockall`. **Recommendation:** Reclassify as "document `mockall` integration" and provide examples. Existing `Spy`/`Stub`/`Mock` cover function-level mocking.

#### Inline snapshots
> **Status:** 🔴 Deferred · **Issue:** Source rewriting fragility

Source file rewriting from a test framework is fragile (line offsets, parallel races, formatting). `insta` solved this with a separate review tool. **Recommendation:** Defer. File-based snapshots handle 95% of cases. Document `insta` integration for advanced needs.

#### Async test support
> **Status:** ✅ Done · **Dependency:** `tokio` (optional)
`.it_async()` wraps async fns in a tokio runtime via `block_on`.

#### Time/clock mocking
> **Status:** ✅ Done
`Clock` trait + `RealClock`/`MockClock` + `test_now()` global.
Users call `rvtest::clock::test_now()` instead of `SystemTime::now()`.

#### Composable matchers (`assert_that!`)
> **Status:** ✅ Done
`Matcher<T>` trait with combinators (`.and()`, `.or()`, `.not()`).
Built-in matchers: `eq`, `ne`, `gt`, `ge`, `lt`, `le`, `contains`, `len`, `some`, `ok`, `err`.

#### Typed fixtures (`#[fixture]`)
> **Status:** ❌ Not started · **Dependency:** `rvtest-macros` (syn/quote)
Proc-macro generating fixture functions with dependency injection. `rstest` does this well; consider documenting integration.

#### Property testing depth
> **Status:** 🟡 Partial · **What's needed:**
More strategies (`string`, `char`, `option`, `result`, `collection`), better shrinking (tree shrink), `#[proptest]`-like attribute macro. Core framework is solid (`Strategy`, `check`, `PropertyConfig`).

---

## Shared Contracts

> **Status:** 🟡 Ongoing

| Item | Status | Notes |
|------|--------|-------|
| Expand shared traits | 🟡 | As cross-crate patterns emerge |
| Extract common types | 🟡 | `Error`, `Version`, `meta` exist |
| Zero external dependencies | ✅ | Maintained |

---

## Ecosystem

| Item | Status | Complexity | Notes |
|------|--------|-----------|-------|
| Release cadence | ❌ | Small | No versioning strategy, no changelog |
| Publish to crates.io | ❌ | Small | Path deps need resolution, rvtest 0.3.2 ≠ workspace |
| CI with coverage gates | ✅ | Small | `cargo check`, `clippy`, `fmt`, `test` in CI |
| Examples + cookbook docs | ❌ | Medium | Doc comments only; no runnable examples |

**Critical path for publish:**
1. Resolve version inconsistency (rvtest 0.3.2 vs workspace 0.1.0)
2. Fix clippy warnings across workspace
3. Switch path deps to version deps for publish
4. Order: `rvlibs` → `rvmath` → `rvtest-macros` → `rvtest` → `cargo-rvtest`

---

## Rveco — Phase 1: Foundation & Infrastructure

> **Status:** 🔜 Planned

Building the foundation of Rveco as the ecosystem estuary.

### Phase 1 Deliverables

| Crate | Module | Description |
|-------|--------|-------------|
| **rvnx** | ecs/ | Entity Component System: World, Entity, Component trait, Query |
| | scene/ | Scene graph: SceneGraph, Transform, Camera |
| | ports/ | Port traits: GpuPort, WindowPort, AssetPort |
| **rvfx** | wgpu/ | GpuPort impl: device, swapchain, pipeline |
| | winit/ | WindowPort impl: window, event loop, input |
| | asset/ | AssetPort impl: file loader, image loader |
| | ui/ | Editor UI toolkit: layout, widgets (panel, button), text, renderer |
| **rveco** | app.rs | App lifecycle, shell, docking, menu bar |
| | workspace.rs | Project/workspace model |
| | plugin_host.rs | Plugin system |

### Principles

- Not preemptive — new crates only created when needed
- rvnx (brain) may use external dependencies
- rvfx (body) implements ports from rvnx
- rveco is the estuary that unifies rvnx + rvfx into one application
