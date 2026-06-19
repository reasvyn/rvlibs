//! Taylor and Maclaurin series formulas and utilities.
//!
//! Maclaurin series are Taylor series centered at x=0.
//! Used for approximating functions near 0 or near a specific point.

use crate::num::Numeric;

/// Factorial function: n! = n × (n-1) × ... × 1
///
/// # Arguments
/// * `n` - Non-negative integer
///
/// # Returns
/// n! as f64
///
pub fn factorial(n: u32) -> f64 {
    match n {
        0 | 1 => 1.0,
        _ if n > 170 => f64::INFINITY,
        _ => (2..=n).fold(1.0, |acc, i| acc * i as f64),
    }
}

/// Maclaurin series for e^x: 1 + x + x²/2! + x³/3! + ...
///
/// Converges for all x. More terms provide better accuracy.
///
/// # Arguments
/// * `x` - The input value
/// * `terms` - Number of terms in the series
///
/// # Returns
/// Approximate value of e^x
///
pub fn maclaurin_exp<T: Numeric>(x: T, terms: u32) -> T {
    let mut result = T::from_f64(0.0);
    for n in 0..terms {
        result += x.pow(&T::from_f64(n as f64)) / T::from_f64(factorial(n));
    }
    result
}

/// Maclaurin series for sin(x): x - x³/3! + x⁵/5! - ...
///
/// Converges for all x. More terms provide better accuracy.
///
/// # Arguments
/// * `x` - The input value (in radians)
/// * `terms` - Number of terms in the series
///
/// # Returns
/// Approximate value of sin(x)
///
pub fn maclaurin_sin<T: Numeric>(x: T, terms: u32) -> T {
    let mut result = T::from_f64(0.0);
    for n in 0..terms {
        let power = T::from_f64((2 * n + 1) as f64);
        let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
        result += T::from_f64(sign) * x.pow(&power) / T::from_f64(factorial(2 * n + 1));
    }
    result
}

/// Maclaurin series for cos(x): 1 - x²/2! + x⁴/4! - ...
///
/// Converges for all x. More terms provide better accuracy.
///
/// # Arguments
/// * `x` - The input value (in radians)
/// * `terms` - Number of terms in the series
///
/// # Returns
/// Approximate value of cos(x)
///
pub fn maclaurin_cos<T: Numeric>(x: T, terms: u32) -> T {
    let mut result = T::from_f64(0.0);
    for n in 0..terms {
        let power = T::from_f64((2 * n) as f64);
        let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
        result += T::from_f64(sign) * x.pow(&power) / T::from_f64(factorial(2 * n));
    }
    result
}

/// Maclaurin series for ln(1+x): x - x²/2 + x³/3 - x⁴/4 + ...
///
/// Converges for |x| ≤ 1. For x > 1 or x < -1, convergence is slower or fails.
///
/// # Arguments
/// * `x` - The input value (must be in (-1, 1] for good convergence)
/// * `terms` - Number of terms in the series
///
/// # Returns
/// Approximate value of ln(1+x)
///
pub fn maclaurin_ln1p<T: Numeric>(x: T, terms: u32) -> T {
    let mut result = T::from_f64(0.0);
    for n in 1..=terms {
        let power = T::from_f64(n as f64);
        let sign = if n % 2 == 1 { 1.0 } else { -1.0 };
        result += T::from_f64(sign) * x.pow(&power) / T::from_f64(n as f64);
    }
    result
}

/// Maclaurin series for arctan(x): x - x³/3 + x⁵/5 - x⁷/7 + ...
///
/// Converges for |x| ≤ 1. For |x| close to 1, many terms needed.
///
/// # Arguments
/// * `x` - The input value (|x| ≤ 1 recommended)
/// * `terms` - Number of terms in the series
///
/// # Returns
/// Approximate value of arctan(x)
///
pub fn maclaurin_arctan<T: Numeric>(x: T, terms: u32) -> T {
    let mut result = T::from_f64(0.0);
    for n in 0..terms {
        let power = T::from_f64((2 * n + 1) as f64);
        let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
        result += T::from_f64(sign) * x.pow(&power) / T::from_f64((2 * n + 1) as f64);
    }
    result
}

/// Binomial series: (1+x)^α = 1 + α·x + α(α-1)x²/2! + ...
///
/// Converges for |x| < 1. Can be used for computing roots and powers.
///
/// # Arguments
/// * `x` - The input value (|x| < 1)
/// * `alpha` - The exponent
/// * `terms` - Number of terms in the series
///
/// # Returns
/// Approximate value of (1+x)^α
///
pub fn binomial_series<T: Numeric>(x: T, alpha: T, terms: u32) -> T {
    let mut result = T::from_f64(1.0);
    let mut coeff = T::from_f64(1.0);

    for n in 1..terms {
        coeff = coeff * (alpha - T::from_f64((n - 1) as f64)) / T::from_f64(n as f64);
        result += coeff * x.pow(&T::from_f64(n as f64));
    }

    result
}

