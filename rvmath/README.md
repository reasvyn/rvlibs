# rvmath

[![Crates.io](https://img.shields.io/crates/v/rvmath.svg)](https://crates.io/crates/rvmath)
[![Docs.rs](https://img.shields.io/docsrs/rvmath)](https://docs.rs/rvmath)
[![Rust CI](https://github.com/reasvyn/rvmath/actions/workflows/rust.yml/badge.svg)](https://github.com/reasvyn/rvmath/actions)
[![License](https://img.shields.io/crates/l/rvmath.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-blue.svg)](https://www.rust-lang.org)

`rvmath` is a comprehensive, lightweight, and type-safe mathematics library for Rust.

## Key Features

- **Type-Safe Unit System**: Effortlessly declare unit dimensions (families) and specific units using intuitive macros.
- **Unit-Aware Matrix & Tensor**: All primary structures (`Vector`, `Matrix`, `Tensor`) can handle both raw numeric types and unit-aware types with mathematical consistency.
- **Percentage System**: Dedicated `Percentage` struct for intuitive scaling and arithmetic (e.g., $100m + 10\% = 110m$).
- **Expression Evaluator**: Parse and evaluate mathematical expressions from strings with proper operator precedence. Supports 30+ math functions, parentheses, and error handling.
- **Geometry Module**: High-precision constants (PI, GOLDEN_RATIO, SQRT_2, etc.) and optimized formulas for 3D shapes (sphere, cylinder, cone, torus) and 2D shapes (ellipse, triangle, polygon) with unit-aware support.
- **Calculus Module**: Comprehensive calculus functions including derivatives (power, exponential, logarithmic, trigonometric), integrals (antiderivatives and numerical methods), series expansions (Maclaurin, Binomial), and numerical methods (Newton-Raphson, Bisection, numerical differentiation).
- **Algebra Module**: Symbolic algebraic operations on string-based expressions with multi-variable support (a, b, c, ..., x, y, z). Includes `simplify()` for reducing expressions, `derivative()` for symbolic differentiation with automatic variable differentiation (power, product, quotient, chain rules), `rationalize()` for rationalizing denominators, `resolve()` for evaluating expressions with variable substitution returning `Num<f64>`, and `map_resolve()` for batch evaluation across multiple values (e.g., apply formula to dataset).
- **Dimensional Consistency**: Automatically manages unit exponents during multiplication/division and prevents invalid addition of different physical dimensions.
- **N-Dimensional Tensors**: Flexible `Tensor` structure with dynamic shape and stride management. Support for `zero()`, `ones()`, `reshape()`, and element-wise operations.
- **Vector Operations**: Generic N-dimensional vectors (`VecN`) with dot product, unit-aware dot product, and element-wise arithmetic operations.
- **Matrix System**: Fixed-size `MatN` structure for efficient geometric and linear algebra calculations, including row/column access, transpose, and element-wise operations.
- **Generic Numeric Support**: Works with any numeric type that implements the `Numeric` trait (e.g., `f32`, `f64`, `i32`, or custom types).
- **Comprehensive Math Operations**: 
  - Arithmetic: `Add`, `Sub`, `Mul`, `Div`, `Rem` (element-wise and scalar) for all systems.
  - Roots: `sqrt()`, `cbrt()`
  - Trigonometry: `sin()`, `cos()`, `tan()`, `asin()`, `acos()`, `atan()`, `atan2()`, `sinh()`, `cosh()`, `tanh()`
  - Logarithms: `ln()`, `log10()`, `log()` (custom base), `ln_1p()`
  - Exponentials: `exp()`, `exp_m1()`
  - Rounding: `round()`, `floor()`, `ceil()`, `fract()`, `abs()`
  - Utilities: `hypot()`, `min()`, `max()`, `recip()` (inverse)
  - Angle Conversion: `to_degrees()`, `to_radians()`
  - Linear Algebra: Matrix transpose, vector dot product, scalar and element-wise operations.
- **Mathematical Constants**: Access to PI, E, TAU, and PHI constants.
- **Seamless Conversion**: Convert between units in the same family (e.g., Kilometers to Meters) with ease.
- **Serialization Support**: Serialize and deserialize vectors and units using `serde`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rvmath = "0.1.0"
```

## Prelude

For convenience, `rvmath` provides a prelude that re-exports the most commonly-used types and
traits. Bring them all into scope with:

```rust
use rvmath::prelude::*;
```

The prelude includes: `Num`, `Numeric`, `Percentage`, `Unit`, `Meta`, `Dimension`,
`VecN`, `MatN`, `Tensor`, and the `declare_family!` / `declare_units!` macros.

## Quick Start Example

Get a feel for rvmath's code style with this brief example. For detailed documentation, run `cargo doc --open` or visit [docs.rs/rvmath](https://docs.rs/rvmath).

```rust
use rvmath::algebra;
use rvmath::utils::evaluate;
use rvmath::num::Num;

// Evaluate mathematical expressions with operator precedence
let result = evaluate("2 + 3 * sqrt(16)");
assert_eq!(result.value, 14.0); // 2 + (3*4) = 14

// Symbolic algebra: simplify expressions
let simplified = algebra::simplify("2*x + 3*x + 4")?;
// Result: "5*x+4"

// Symbolic differentiation
let derivative = algebra::derivative("x^3 + 2*x^2", "x")?;
// Result: "(3*x^2+4*x)"

// Evaluate expressions with variables
let result = algebra::resolve("2*x + 5*y", &[("x", Num::new(4.0)), ("y", Num::new(6.0))])?;
assert_eq!(result.value, 38.0);

// Batch evaluation across multiple values
let inputs = vec![Num::new(1.0), Num::new(2.0), Num::new(3.0)];
let results = algebra::map_resolve("x^2 + 1", "x", &inputs)?;
// Returns [2.0, 5.0, 10.0]
```

This demonstrates the library's clean, expressive API for mathematical computing. See the module-level docs for each feature's complete API reference.

## Contributing

We welcome contributions from the community! Whether it's adding a new unit family, fixing a bug, or proposing a new mathematical module, please check our [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

## License

This project is dual-licensed under the [MIT](LICENSE) or [Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0) license.

---
Developed with ❤️ by **Reas Vyn** (reasvyn@gmail.com)
