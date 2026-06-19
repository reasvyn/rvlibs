//! Derivative formulas and rules for common functions.
//!
//! This module provides:
//! - Common derivatives (power, exponential, logarithmic, trigonometric)
//! - Derivative rules (product, quotient, chain)
//! - Higher-order derivatives

use crate::num::Numeric;

/// Power rule: d/dx(x^n) = n·x^(n-1)
///
/// # Arguments
/// * `x` - The variable
/// * `n` - The exponent
///
/// # Returns
/// The derivative value
///
pub fn power_rule<T: Numeric>(x: T, n: T) -> T {
    n * x.pow(&(n - T::from_f64(1.0)))
}

/// Exponential rule: d/dx(e^x) = e^x
///
/// # Arguments
/// * `x` - The variable
///
/// # Returns
/// The derivative value (e^x)
///
pub fn exp_derivative<T: Numeric>(x: T) -> T {
    x.exp()
}

/// Exponential with arbitrary base: d/dx(a^x) = a^x · ln(a)
///
/// # Arguments
/// * `x` - The variable
/// * `base` - The base (a)
///
/// # Returns
/// The derivative value
///
pub fn exp_base_derivative<T: Numeric>(x: T, base: T) -> T {
    base.pow(&x) * base.ln()
}

/// Logarithm rule: d/dx(ln x) = 1/x
///
/// # Arguments
/// * `x` - The variable (must be positive)
///
/// # Returns
/// The derivative value
///
pub fn ln_derivative<T: Numeric>(x: T) -> T {
    T::from_f64(1.0) / x
}

/// Logarithm with arbitrary base: d/dx(log_a x) = 1/(x·ln(a))
///
/// # Arguments
/// * `x` - The variable (must be positive)
/// * `base` - The logarithm base
///
/// # Returns
/// The derivative value
///
pub fn log_base_derivative<T: Numeric>(x: T, base: T) -> T {
    T::from_f64(1.0) / (x * base.ln())
}

/// Sine derivative: d/dx(sin x) = cos x
///
/// # Arguments
/// * `x` - The variable (in radians)
///
/// # Returns
/// cos x
///
pub fn sin_derivative<T: Numeric>(x: T) -> T {
    x.cos()
}

/// Cosine derivative: d/dx(cos x) = -sin x
///
/// # Arguments
/// * `x` - The variable (in radians)
///
/// # Returns
/// -sin x
///
pub fn cos_derivative<T: Numeric>(x: T) -> T {
    T::from_f64(-1.0) * x.sin()
}

/// Tangent derivative: d/dx(tan x) = sec²x = 1/cos²x
///
/// # Arguments
/// * `x` - The variable (in radians)
///
/// # Returns
/// sec²x = 1/cos²x
///
pub fn tan_derivative<T: Numeric>(x: T) -> T {
    let cos_x = x.cos();
    T::from_f64(1.0) / (cos_x * cos_x)
}

/// Arcsine derivative: d/dx(arcsin x) = 1/√(1-x²)
///
/// # Arguments
/// * `x` - The variable (must be in [-1, 1])
///
/// # Returns
/// The derivative value
///
pub fn arcsin_derivative<T: Numeric>(x: T) -> T {
    let one = T::from_f64(1.0);
    one / (one - x * x).sqrt()
}

/// Arccosine derivative: d/dx(arccos x) = -1/√(1-x²)
///
/// # Arguments
/// * `x` - The variable (must be in [-1, 1])
///
/// # Returns
/// The derivative value
///
pub fn arccos_derivative<T: Numeric>(x: T) -> T {
    let one = T::from_f64(1.0);
    T::from_f64(-1.0) * one / (one - x * x).sqrt()
}

/// Arctangent derivative: d/dx(arctan x) = 1/(1+x²)
///
/// # Arguments
/// * `x` - The variable
///
/// # Returns
/// The derivative value
///
pub fn arctan_derivative<T: Numeric>(x: T) -> T {
    T::from_f64(1.0) / (T::from_f64(1.0) + x * x)
}

/// Sinh derivative: d/dx(sinh x) = cosh x
///
/// # Arguments
/// * `x` - The variable
///
/// # Returns
/// cosh x
///
pub fn sinh_derivative<T: Numeric>(x: T) -> T {
    x.cosh()
}

/// Cosh derivative: d/dx(cosh x) = sinh x
///
/// # Arguments
/// * `x` - The variable
///
/// # Returns
/// sinh x
///
pub fn cosh_derivative<T: Numeric>(x: T) -> T {
    x.sinh()
}

/// Tanh derivative: d/dx(tanh x) = sech²x = 1/cosh²x
///
/// # Arguments
/// * `x` - The variable
///
/// # Returns
/// sech²x = 1/cosh²x
///
pub fn tanh_derivative<T: Numeric>(x: T) -> T {
    let cosh_x = x.cosh();
    T::from_f64(1.0) / (cosh_x * cosh_x)
}

