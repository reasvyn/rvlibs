//! Derivatives, integrals, series, and numerical methods.
//!
//! This module provides calculus formulas and numerical methods. All functions
//! are generic over [`Numeric`] and ready for direct evaluation.
//!
//! # Examples
//!
//! ```
//! # use rvmath::calculus::derivatives::power_rule;
//! # use rvmath::calculus::integrals::simpsons_rule;
//! # use rvmath::calculus::numerical::newton_raphson;
//! // Power rule: d/dx(x³) at x=2 → 3·2² = 12
//! let slope = power_rule(2.0_f64, 3.0);
//! assert!((slope - 12.0).abs() < 1e-12);
//!
//! // Simpson's rule: ∫₀¹ x² dx ≈ 0.33333...
//! let integral = simpsons_rule(|x: f64| x * x, 0.0, 1.0, 10);
//! assert!((integral - 1.0/3.0).abs() < 1e-6);
//!
//! // Newton-Raphson: root of x² - 4 = 0 (initial guess 3.0)
//! let root = newton_raphson(
//!     |x: f64| x * x - 4.0,
//!     |x: f64| 2.0 * x,
//!     3.0, 1e-12, 100,
//! );
//! assert!((root - 2.0).abs() < 1e-10);
//! ```
//!
//! # Derivatives
//!
//! Common derivative formulas for basic functions:
//! - Power rule: d/dx(x^n) = n·x^(n-1)
//! - Exponential: d/dx(e^x) = e^x, d/dx(a^x) = a^x·ln(a)
//! - Logarithmic: d/dx(ln x) = 1/x, d/dx(log_a x) = 1/(x·ln(a))
//! - Trigonometric: d/dx(sin x) = cos x, d/dx(cos x) = -sin x, d/dx(tan x) = sec²x
//! - Inverse trig: d/dx(arcsin x) = 1/√(1-x²), etc.
//! - Hyperbolic: d/dx(sinh x) = cosh x, d/dx(cosh x) = sinh x, etc.
//!
//! # Integrals
//!
//! Common antiderivative formulas and numerical integration methods:
//! - Power rule: ∫x^n dx = x^(n+1)/(n+1) + C
//! - Exponential: ∫e^x dx = e^x + C, ∫a^x dx = a^x/ln(a) + C
//! - Logarithmic: ∫1/x dx = ln|x| + C
//! - Trigonometric: ∫sin x dx = -cos x + C, ∫cos x dx = sin x + C
//! - Special forms: ∫1/(1+x²) dx = arctan x + C, ∫1/√(1-x²) dx = arcsin x + C
//! - Numerical methods: Simpson's rule, Trapezoidal rule
//!
//! # Series & Limits
//!
//! Taylor and Maclaurin series expansions:
//! - Maclaurin series for exponential: e^x = 1 + x + x²/2! + x³/3! + ...
//! - Maclaurin series for sine: sin x = x - x³/3! + x⁵/5! - ...
//! - Maclaurin series for cosine: cos x = 1 - x²/2! + x⁴/4! - ...
//! - Maclaurin series for ln(1+x): ln(1+x) = x - x²/2 + x³/3 - ...
//! - Maclaurin series for arctan: arctan x = x - x³/3 + x⁵/5 - ...
//! - Binomial series: (1+x)^α = 1 + α·x + α(α-1)x²/2! + ...
//!
//! # Numerical Methods
//!
//! - **Root finding**: Newton-Raphson, Bisection
//! - **Numerical differentiation**: Forward difference, Central difference, Second derivative
//!
//! # Constants
//!
//! Special calculus constants:
//! - Euler-Mascheroni constant (γ) ≈ 0.5772...
//! - Catalan's constant (G) ≈ 0.9160...
//! - Apéry's constant (ζ(3)) ≈ 1.2021...
//!
//! # Design
//!
//! - **Numerical focus**: not symbolic; designed for computation.
//! - **Flexibility**: works with any `Numeric` type (`f64`, `f32`, custom).
//! - **Unit-aware**: supports unit types where mathematically valid.
//! - **Accuracy**: high-precision constants and approximation methods.

pub mod constants;
pub mod derivatives;
pub mod integrals;
pub mod numerical;
pub mod series;

// Re-export commonly used items
pub use constants::*;
pub use derivatives::*;
pub use integrals::*;
pub use numerical::*;
pub use series::*;
