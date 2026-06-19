//! Integration formulas and numerical methods for approximating definite integrals.
//!
//! This module provides:
//! - Common antiderivative formulas
//! - Numerical integration methods (Simpson's rule, Trapezoidal rule)
//! - Special integral forms

use crate::num::Numeric;

/// Power rule antiderivative: ∫x^n dx = x^(n+1)/(n+1) + C  (for n ≠ -1)
///
/// Returns the antiderivative at a specific x value.
///
/// # Arguments
/// * `x` - The variable
/// * `n` - The exponent (must not be -1)
///
/// # Returns
/// The antiderivative value (without constant of integration)
///
pub fn power_integral<T: Numeric>(x: T, n: T) -> T {
    x.pow(&(n + T::from_f64(1.0))) / (n + T::from_f64(1.0))
}

/// Exponential integral: ∫e^x dx = e^x + C
///
/// # Arguments
/// * `x` - The variable
///
/// # Returns
/// The antiderivative value (without constant of integration)
///
pub fn exp_integral<T: Numeric>(x: T) -> T {
    x.exp()
}

/// Exponential with base: ∫a^x dx = a^x/ln(a) + C  (for a > 0, a ≠ 1)
///
/// # Arguments
/// * `x` - The variable
/// * `base` - The base (a)
///
/// # Returns
/// The antiderivative value
///
pub fn exp_base_integral<T: Numeric>(x: T, base: T) -> T {
    base.pow(&x) / base.ln()
}

/// Logarithm integral: ∫1/x dx = ln|x| + C
///
/// # Arguments
/// * `x` - The variable (must be non-zero)
///
/// # Returns
/// The antiderivative value (without constant)
///
pub fn ln_integral<T: Numeric>(x: T) -> T {
    x.ln()
}

/// Sine integral: ∫sin(x) dx = -cos(x) + C
///
/// # Arguments
/// * `x` - The variable (in radians)
///
/// # Returns
/// The antiderivative value
///
pub fn sin_integral<T: Numeric>(x: T) -> T {
    T::from_f64(-1.0) * x.cos()
}

/// Cosine integral: ∫cos(x) dx = sin(x) + C
///
/// # Arguments
/// * `x` - The variable (in radians)
///
/// # Returns
/// The antiderivative value
///
pub fn cos_integral<T: Numeric>(x: T) -> T {
    x.sin()
}

/// Secant² integral: ∫sec²(x) dx = tan(x) + C
///
/// # Arguments
/// * `x` - The variable (in radians)
///
/// # Returns
/// The antiderivative value
///
pub fn sec2_integral<T: Numeric>(x: T) -> T {
    x.tan()
}

/// Arctangent integral: ∫1/(1+x²) dx = arctan(x) + C
///
/// # Arguments
/// * `x` - The variable
///
/// # Returns
/// The antiderivative value (arctan)
///
pub fn arctan_integral<T: Numeric>(x: T) -> T {
    x.atan()
}

/// Arcsine integral: ∫1/√(1-x²) dx = arcsin(x) + C
///
/// # Arguments
/// * `x` - The variable (must be in [-1, 1])
///
/// # Returns
/// The antiderivative value (arcsin)
///
pub fn arcsin_integral<T: Numeric>(x: T) -> T {
    x.asin()
}

/// Simpson's rule for numerical integration.
///
/// Approximates ∫f(x)dx from a to b using n intervals.
/// Formula: ∫f(x)dx ≈ (b-a)/(3n) · [f(x₀) + 4f(x₁) + 2f(x₂) + ... + f(xₙ)]
///
/// Requires n to be even. Returns more accurate results with larger n.
///
/// # Arguments
/// * `f` - Function to integrate (closure)
/// * `a` - Lower bound
/// * `b` - Upper bound
/// * `n` - Number of intervals (will be rounded up to even if odd)
///
/// # Returns
/// Approximate value of the definite integral
///
pub fn simpsons_rule<T, F>(f: F, a: T, b: T, mut n: u32) -> T
where
    T: Numeric,
    F: Fn(T) -> T,
{
    // Ensure n is even
    #[allow(unknown_lints, clippy::manual_is_multiple_of)]
    if n % 2 != 0 {
        n += 1;
    }

    let n_f = T::from_f64(n as f64);
    let h = (b - a) / n_f;

    let mut sum = f(a) + f(b);

    // Odd indices (multiplied by 4)
    for i in 1..n {
        if i % 2 == 1 {
            let x = a + T::from_f64(i as f64) * h;
            sum += T::from_f64(4.0) * f(x);
        }
    }

    // Even indices (multiplied by 2)
    for i in 2..n {
        if i % 2 == 0 {
            let x = a + T::from_f64(i as f64) * h;
            sum += T::from_f64(2.0) * f(x);
        }
    }

    (b - a) / (T::from_f64(3.0) * n_f) * sum
}

/// Trapezoidal rule for numerical integration.
///
/// Approximates ∫f(x)dx from a to b using n intervals.
/// Formula: ∫f(x)dx ≈ (b-a)/(2n) · [f(x₀) + 2f(x₁) + 2f(x₂) + ... + f(xₙ)]
///
/// Generally less accurate than Simpson's rule but simpler.
///
/// # Arguments
/// * `f` - Function to integrate (closure)
/// * `a` - Lower bound
/// * `b` - Upper bound
/// * `n` - Number of intervals
///
/// # Returns
/// Approximate value of the definite integral
///
pub fn trapezoidal_rule<T, F>(f: F, a: T, b: T, n: u32) -> T
where
    T: Numeric,
    F: Fn(T) -> T,
{
    let n_f = T::from_f64(n as f64);
    let h = (b - a) / n_f;

    let mut sum = (f(a) + f(b)) / T::from_f64(2.0);

    for i in 1..n {
        let x = a + T::from_f64(i as f64) * h;
        sum += f(x);
    }

    h * sum
}

