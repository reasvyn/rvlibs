# Calculus

Analytical and numerical calculus — derivatives, integrals, series expansions, and numerical methods.

## Prerequisites

- [Numeric Types](numeric-types.md) — `Num<T>`, `Numeric` trait


## Derivatives

```rust
use rvmath::calculus::derivative;

let f = |x: f64| x.powi(3) + 2.0 * x;
let df = derivative(f, 2.0, 1e-6);
// Numerical derivative at x=2
```

## Integrals

```rust
use rvmath::calculus::integral;

let f = |x: f64| x.powi(2);
let area = integral(f, 0.0, 1.0, 1000);
// ∫₀¹ x² dx ≈ 0.333...
```

## Series

```rust
use rvmath::calculus::series;

// Maclaurin series for e^x up to 5 terms
let approx = series::maclaurin(|n| 1.0 / factorial(n), 1.0, 5);
```

## Numerical Methods

```rust
use rvmath::calculus::numerical;

// Newton-Raphson: find root of x² - 4 = 0
let root = numerical::newton_raphson(|x| x.powi(2) - 4.0, 3.0, 1e-10, 100);
assert!((root - 2.0).abs() < 1e-6);
```

## Glossarium

| Term | Definition |
|------|------------|
| Derivative | The rate of change of a function with respect to a variable. |
| Integral | The area under a curve — antiderivative (analytical) or approximation (numerical). |
| Series Expansion | Approximating a function as an infinite sum of terms (Maclaurin, Binomial). |
| Newton-Raphson | An iterative method for finding roots of a function. |
| Bisection | A bracketing method for finding roots. |


## Next Steps

- [Geometry](geometry.md) — constants, 2D and 3D shape formulas
- [Numeric Types](numeric-types.md) — `Num<T>`, `Percentage`
