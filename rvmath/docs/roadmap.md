# Roadmap

This document outlines the planned development trajectory for rvmath — focused on **pure mathematics** (algebra, analysis, linear algebra, discrete mathematics). Items are organized by rough dependency order, but timelines are flexible.

> **Testing:** All new modules are tested using [`rvtest`](https://crates.io/crates/rvtest) — a BDD-style testing framework with spec blocks, property-based checks, and parametrized cases.

---

## Algebra: Polynomial Module (New)

Algebraic manipulation of univariate polynomials.

### Goals

- `Polynomial<T>` type with dense coefficient storage, generic over `Numeric`.
- Full arithmetic: add, sub, mul, div (synthetic), pow, composition.
- Root finding for degrees 1–4 via closed-form formulas, higher degrees via Newton / Durand–Kerner.
- Evaluation via Horner's method with `Unit` and `Complex` support.
- Polynomial interpolation (Lagrange, Newton divided differences).
- Integration with `algebra::Expr` for symbolic round-trip.

### Proposed API Sketch

```rust
/// A univariate polynomial: a₀ + a₁·x + a₂·x² + ... + aₙ·xⁿ.
/// Coefficients stored in ascending degree order.
pub struct Polynomial<T: Numeric> {
    coeffs: Vec<T>,
}

impl<T: Numeric> Polynomial<T> {
    pub fn new(coeffs: Vec<T>) -> Self;
    pub fn zero() -> Self;
    pub fn constant(c: T) -> Self;
    pub fn monomial(degree: usize) -> Self;
    pub fn from_roots(roots: &[T]) -> Self;
    pub fn degree(&self) -> usize;
    pub fn coeff(&self, i: usize) -> Option<&T>;
    pub fn coeff_mut(&mut self, i: usize) -> Option<&mut T>;
    pub fn len(&self) -> usize;
    pub fn is_zero(&self) -> bool;
    pub fn evaluate(&self, x: T) -> T;
    pub fn evaluate_many(&self, xs: &[T]) -> Vec<T>;
    pub fn derivative(&self) -> Self;
    pub fn integral(&self) -> Self;
}

impl<T: Numeric> Add for Polynomial<T>;
impl<T: Numeric> Sub for Polynomial<T>;
impl<T: Numeric> Mul for Polynomial<T>;
impl<T: Numeric> Neg for Polynomial<T>;

impl<T: Numeric> Polynomial<T> {
    pub fn scale(&self, scalar: T) -> Self;
    pub fn pow(&self, exp: u32) -> Self;
    pub fn compose(&self, other: &Self) -> Self;
    pub fn synthethic_divide(&self, c: T) -> (Self, T);
    pub fn divide(&self, divisor: &Self) -> Option<(Self, Self)>;
    pub fn roots(&self) -> Vec<f64>;
    pub fn roots_complex(&self) -> Vec<Complex<f64>>;
    pub fn roots_newton(&self, initial_guess: f64, tolerance: f64, max_iter: u32) -> Vec<f64>;
    pub fn roots_durand_kerner(&self, tolerance: f64, max_iter: u32) -> Vec<Complex<f64>>;
}

pub fn solve_linear(a: f64, b: f64) -> Option<f64>;
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> (Option<f64>, Option<f64>);
pub fn solve_cubic(a: f64, b: f64, c: f64, d: f64) -> Vec<f64>;
pub fn solve_quartic(a: f64, b: f64, c: f64, d: f64, e: f64) -> Vec<f64>;

pub mod interpolation {
    pub fn lagrange<T: Numeric>(xs: &[T], ys: &[T]) -> Polynomial<T>;
    pub fn newton_divided_diff<T: Numeric>(xs: &[T], ys: &[T]) -> Polynomial<T>;
    pub fn lagrange_eval<T: Numeric>(xs: &[T], ys: &[T], x: T) -> T;
}

impl<T: Numeric> TryFrom<&Expr> for Polynomial<T>;
impl<T: Numeric> From<&Polynomial<T>> for Expr;
```

### Files

| File | Purpose |
|------|---------|
| `polynomial/mod.rs` | Module root, `Polynomial<T>`, arithmetic impls |
| `polynomial/roots.rs` | Root finding (closed-form + iterative) |
| `polynomial/interpolation.rs` | Lagrange, Newton interpolation |
| `polynomial/algebra.rs` | `Expr` ↔ `Polynomial` conversion |

---

## Analysis: Special Functions Module (New)

Implement special functions from classical analysis.

### Goals

- Provide accurate numerical approximations for functions not in `Numeric`.
- All functions take `f64` input and return `f64` output.
- Document domain restrictions, error bounds, and algorithmic references.
- Enable calculus module to use these for derivative/integral/series of special functions.

### Proposed API Sketch

```rust
pub mod special;

// Gamma family
pub fn gamma(x: f64) -> f64;           // Γ(x) Lanczos approximation
pub fn ln_gamma(x: f64) -> f64;
pub fn digamma(x: f64) -> f64;        // ψ(x) = Γ'(x)/Γ(x)
pub fn beta(a: f64, b: f64) -> f64;   // B(a,b)
pub fn ln_beta(a: f64, b: f64) -> f64;
pub fn lower_incomplete_gamma(s: f64, x: f64) -> f64;
pub fn upper_incomplete_gamma(s: f64, x: f64) -> f64;
pub fn gamma_p(s: f64, x: f64) -> f64;  // regularized P(s,x)
pub fn gamma_q(s: f64, x: f64) -> f64;  // regularized Q(s,x)

// Error function
pub fn erf(x: f64) -> f64;
pub fn erfc(x: f64) -> f64;
pub fn erf_inv(x: f64) -> f64;

// Riemann zeta function ζ(s)
pub fn zeta(s: f64) -> f64;

// Bessel functions
pub fn bessel_j0(x: f64) -> f64;
pub fn bessel_j1(x: f64) -> f64;
pub fn bessel_jn(n: i32, x: f64) -> f64;
pub fn bessel_y0(x: f64) -> f64;
pub fn bessel_y1(x: f64) -> f64;
pub fn bessel_yn(n: i32, x: f64) -> f64;

// Other
pub fn sinc(x: f64) -> f64;           // sin(x)/x
pub fn airy_ai(x: f64) -> f64;

// Numeric trait extension
impl<T: Numeric> T {
    pub fn erf(self) -> Self;
    pub fn gamma(self) -> Self;
}
```

### Files

| File | Purpose |
|------|---------|
| `special/mod.rs` | Module root, re-exports |
| `special/gamma.rs` | Gamma family |
| `special/error.rs` | Error function family |
| `special/zeta.rs` | Riemann zeta function |
| `special/bessel.rs` | Bessel J₀, J₁, Jₙ, Y₀, Y₁, Yₙ |
| `special/misc.rs` | sinc, airy_ai |

---

## Linear Algebra: Enhanced LA Module

Deepen the existing `la` module with additional decompositions and solvers.

### Goals

- Add factorizations (LU, QR, SVD, Cholesky) as methods on `MatN`.
- Linear system solvers with multiple strategies.
- Eigenvalue decomposition for symmetric and general matrices.
- All operations remain generic over `Numeric` where possible.

### Proposed API Sketch

```rust
// LU decomposition: PA = LU
pub struct LUDecomp<T: Numeric, const N: usize> {
    pub l: MatN<T, N, N>,
    pub u: MatN<T, N, N>,
    pub p: MatN<T, N, N>,
}

impl<T: Numeric, const N: usize> MatN<T, N, N> {
    pub fn lu(&self) -> Option<LUDecomp<T, N>>;
    pub fn solve_lu(&self, b: &VecN<T, N>) -> Option<VecN<T, N>>;
}

// QR decomposition: A = QR
pub struct QRDecomp<T: Numeric, const M: usize, const N: usize> {
    pub q: MatN<T, M, M>,
    pub r: MatN<T, M, N>,
}

impl<T: Numeric, const R: usize, const C: usize> MatN<T, R, C> {
    pub fn qr_gram_schmidt(&self) -> Option<QRDecomp<T, R, C>>;
    pub fn qr_householder(&self) -> Option<QRDecomp<T, R, C>>;
}

// Cholesky decomposition: A = LL^T (symmetric positive-definite)
pub struct CholeskyDecomp<T: Numeric, const N: usize> {
    pub l: MatN<T, N, N>,
}

impl<T: Numeric, const N: usize> MatN<T, N, N> {
    pub fn cholesky(&self) -> Option<CholeskyDecomp<T, N>>;
}

// Singular Value Decomposition: A = UΣV^T
pub struct SVD<T: Numeric, const M: usize, const N: usize> {
    pub u: MatN<T, M, M>,
    pub s: VecN<T, { min(M, N) }>,
    pub v_t: MatN<T, N, N>,
}

impl<T: Numeric, const R: usize, const C: usize> MatN<T, R, C> {
    pub fn svd(&self) -> Option<SVD<T, R, C>>;
    pub fn condition_number(&self) -> Option<f64>;
    pub fn rank(&self) -> usize;
}

// Eigenvalues
pub struct EigenDecomp<T: Numeric, const N: usize> {
    pub values: VecN<Complex<T>, N>,
    pub vectors: MatN<Complex<T>, N, N>,
}

impl<T: Numeric, const N: usize> MatN<T, N, N> {
    pub fn power_iteration(&self, max_iter: u32, tol: f64) -> Option<(f64, VecN<T, N>)>;
    pub fn eigenvalues(&self) -> Option<EigenDecomp<T, N>>;
}

// Linear system solvers
impl<T: Numeric, const N: usize> MatN<T, N, N> {
    pub fn solve(&self, b: &VecN<T, N>) -> Option<VecN<T, N>>;
    pub fn solve_tridiagonal(
        lower: &VecN<T, { N.saturating_sub(1) }>,
        diag: &VecN<T, N>,
        upper: &VecN<T, { N.saturating_sub(1) }>,
        b: &VecN<T, N>,
    ) -> Option<VecN<T, N>>;
    pub fn det(&self) -> T;
    pub fn inv(&self) -> Option<Self>;
}

// Matrix norms
impl<T: Numeric, const R: usize, const C: usize> MatN<T, R, C> {
    pub fn norm_frobenius(&self) -> f64;
    pub fn norm_one(&self) -> f64;
    pub fn norm_inf(&self) -> f64;
}

// Unit-aware extensions
pub fn solve_units<T, D, const N: usize>(
    a: &MatN<T, N, N>,
    b: &VecN<Unit<T, D>, N>,
) -> Option<VecN<Unit<T, D>, N>>;
```

### Files

| File | Purpose |
|------|---------|
| `la/lu.rs` | LU decomposition + pivoting |
| `la/qr.rs` | QR via Gram–Schmidt and Householder |
| `la/svd.rs` | Singular value decomposition |
| `la/eigen.rs` | Power iteration, QR algorithm for eigenvalues |
| `la/solve.rs` | Gaussian elimination, Thomas algorithm, solve_units |
| `la/norms.rs` | Matrix norms |
| `la/decompose.rs` | Cholesky decomposition |

---

## Discrete Mathematics: Graph Module (New)

### Goals

- `Graph<N, E>` supporting directed and undirected variants, adjacency list storage.
- Classic graph algorithms: traversal (BFS, DFS), shortest path, spanning tree, flow.
- Integration with `MatN` for adjacency matrix representation.

### Proposed API Sketch

```rust
pub mod graph;

pub type NodeIndex = usize;

pub struct Graph<N, E> {
    nodes: Vec<Node<N>>,
    edges: Vec<Vec<(NodeIndex, E)>>,
    directed: bool,
}

impl<N, E> Graph<N, E> {
    pub fn new(directed: bool) -> Self;
    pub fn directed() -> Self;
    pub fn undirected() -> Self;
    pub fn add_node(&mut self, weight: N) -> NodeIndex;
    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: E);
    pub fn remove_node(&mut self, index: NodeIndex);
    pub fn remove_edge(&mut self, from: NodeIndex, to: NodeIndex);
    pub fn node_weight(&self, index: NodeIndex) -> Option<&N>;
    pub fn node_weight_mut(&mut self, index: NodeIndex) -> Option<&mut N>;
    pub fn neighbors(&self, index: NodeIndex) -> impl Iterator<Item = (NodeIndex, &E)>;
    pub fn node_count(&self) -> usize;
    pub fn edge_count(&self) -> usize;
    pub fn is_directed(&self) -> bool;
}

// Traversal
pub fn bfs<N, E>(graph: &Graph<N, E>, start: NodeIndex) -> Vec<NodeIndex>;
pub fn dfs<N, E>(graph: &Graph<N, E>, start: NodeIndex) -> Vec<NodeIndex>;
pub fn topological_sort<N, E>(graph: &Graph<N, E>) -> Option<Vec<NodeIndex>>;

// Shortest path
pub fn dijkstra<N, E: Numeric + PartialOrd>(
    graph: &Graph<N, E>, start: NodeIndex,
) -> Vec<Option<f64>>;
pub fn bellman_ford<N, E: Numeric + PartialOrd>(
    graph: &Graph<N, E>, start: NodeIndex,
) -> Result<Vec<Option<f64>>, String>;
pub fn floyd_warshall<N, E: Numeric + Clone>(
    graph: &Graph<N, E>,
) -> MatN<f64, { dynamic }, { dynamic }>;

// Spanning tree
pub fn kruskal_mst<N, E: Numeric + PartialOrd + Clone>(graph: &Graph<N, E>) -> Graph<N, E>;
pub fn prim_mst<N, E: Numeric + PartialOrd + Clone>(
    graph: &Graph<N, E>, start: NodeIndex,
) -> Graph<N, E>;

// Flow
pub fn edmonds_karp<N, E: Numeric + Clone>(
    graph: &Graph<N, E>, source: NodeIndex, sink: NodeIndex,
) -> f64;
pub fn dinic<N, E: Numeric + Clone>(
    graph: &Graph<N, E>, source: NodeIndex, sink: NodeIndex,
) -> f64;

// Integration with MatN
impl<N, E: Numeric + Clone> From<&Graph<N, E>> for MatN<E, { dynamic }, { dynamic }>;
```

### Files

| File | Purpose |
|------|---------|
| `graph/mod.rs` | Module root, `Graph` struct, basic accessors |
| `graph/traversal.rs` | BFS, DFS, topological sort |
| `graph/shortest_path.rs` | Dijkstra, Bellman-Ford, Floyd-Warshall |
| `graph/spanning_tree.rs` | Kruskal, Prim MST |
| `graph/flow.rs` | Edmonds-Karp, Dinic max flow |
| `graph/la.rs` | MatN ↔ Graph conversion |

---

## Notes

The following topics are excluded from this roadmap as they belong to applied mathematics or engineering, not pure mathematics:

- Probability & statistics (descriptive, inferential, regression)
- Advanced numerical methods (ODE, optimization, spline interpolation)
- Signal processing (FFT, filter, windowing)
- Performance optimization (SIMD, parallelism)
- `no_std` support

This roadmap is a living document and may change based on internal decisions.
