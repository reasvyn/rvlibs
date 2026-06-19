//! Numerical methods for solving calculus problems.
//!
//! Includes:
//! - Root finding (Newton-Raphson, Bisection)
//! - Optimization
//! - Numerical differentiation

use crate::num::Numeric;

/// Newton-Raphson method for finding roots.
///
/// Finds x such that f(x) = 0 by iteratively using: x_(n+1) = x_n - f(x_n)/f'(x_n)
///
/// # Arguments
/// * `f` - The function f(x)
/// * `df` - The derivative f'(x)
/// * `x0` - Initial guess
/// * `tol` - Convergence tolerance
/// * `max_iter` - Maximum number of iterations
///
/// # Returns
/// Approximate root, or NaN if it fails to converge
///
pub fn newton_raphson<T, F, DF>(f: F, df: DF, mut x: T, tol: T, max_iter: u32) -> T
where
    T: Numeric,
    F: Fn(T) -> T,
    DF: Fn(T) -> T,
{
    let tol_f64 = tol.to_f64();
    if tol_f64.is_nan() || tol_f64 <= 0.0 {
        return T::from_f64(f64::NAN);
    }

    for _ in 0..max_iter {
        let fx = f(x);
        let dfx = df(x);

        if fx.abs().to_f64() < tol_f64 {
            return x;
        }

        if dfx.to_f64().abs() < tol_f64 {
            return T::from_f64(f64::NAN);
        }

        x -= fx / dfx;
    }

    x
}

/// Bisection method for finding roots.
///
/// Finds x in [a, b] such that f(x) = 0 by repeatedly halving the interval.
/// Requires f(a) and f(b) to have opposite signs.
///
/// # Arguments
/// * `f` - The function f(x)
/// * `a` - Lower bound
/// * `b` - Upper bound
/// * `tol` - Convergence tolerance
/// * `max_iter` - Maximum number of iterations
///
/// # Returns
/// Approximate root in [a, b], or NaN if bounds don't bracket a root
///
pub fn bisection<T, F>(f: F, mut a: T, mut b: T, tol: T, max_iter: u32) -> T
where
    T: Numeric,
    F: Fn(T) -> T,
{
    let tol_f64 = tol.to_f64();
    if tol_f64.is_nan() || tol_f64 <= 0.0 {
        return T::from_f64(f64::NAN);
    }

    let fa = f(a);
    let fb = f(b);

    // Handle NaN in function evaluation
    if fa.to_f64().is_nan() || fb.to_f64().is_nan() {
        return T::from_f64(f64::NAN);
    }

    // Check if bounds have opposite signs
    if (fa.to_f64() > 0.0 && fb.to_f64() > 0.0) || (fa.to_f64() < 0.0 && fb.to_f64() < 0.0) {
        return T::from_f64(f64::NAN);
    }

    for _ in 0..max_iter {
        let mid = (a + b) / T::from_f64(2.0);
        let fmid = f(mid);

        // Check for convergence
        if (b - a).abs().to_f64() < tol_f64 {
            return mid;
        }

        if fmid.abs().to_f64() < tol_f64 {
            return mid;
        }

        // Update bounds
        if (fa.to_f64() > 0.0 && fmid.to_f64() > 0.0) || (fa.to_f64() < 0.0 && fmid.to_f64() < 0.0)
        {
            a = mid;
        } else {
            b = mid;
        }
    }

    (a + b) / T::from_f64(2.0)
}

/// Numerical differentiation using forward difference.
///
/// Approximates f'(x) ≈ (f(x+h) - f(x))/h
///
/// The choice of h is critical: too small causes round-off error, too large causes truncation error.
/// Default h = sqrt(ε) ≈ 1.5e-8 is often suitable.
///
/// # Arguments
/// * `f` - The function f(x)
/// * `x` - The point at which to estimate the derivative
/// * `h` - The step size (default √ε ≈ 1.5e-8)
///
/// # Returns
/// Approximate value of f'(x)
///
pub fn forward_difference<T, F>(f: F, x: T, h: T) -> T
where
    T: Numeric,
    F: Fn(T) -> T,
{
    (f(x + h) - f(x)) / h
}

/// Numerical differentiation using central difference.
///
/// Approximates f'(x) ≈ (f(x+h) - f(x-h))/(2h)
///
/// More accurate than forward difference but requires 2 function evaluations.
///
/// # Arguments
/// * `f` - The function f(x)
/// * `x` - The point at which to estimate the derivative
/// * `h` - The step size (default √ε ≈ 1.5e-8)
///
/// # Returns
/// Approximate value of f'(x)
///
pub fn central_difference<T, F>(f: F, x: T, h: T) -> T
where
    T: Numeric,
    F: Fn(T) -> T,
{
    (f(x + h) - f(x - h)) / (T::from_f64(2.0) * h)
}

/// Numerical second derivative using central difference.
///
/// Approximates f''(x) ≈ (f(x+h) - 2f(x) + f(x-h))/h²
///
/// # Arguments
/// * `f` - The function f(x)
/// * `x` - The point at which to estimate the second derivative
/// * `h` - The step size (default √ε ≈ 1.5e-8)
///
/// # Returns
/// Approximate value of f''(x)
///
pub fn second_derivative<T, F>(f: F, x: T, h: T) -> T
where
    T: Numeric,
    F: Fn(T) -> T,
{
    let h_sq = h * h;
    (f(x + h) - T::from_f64(2.0) * f(x) + f(x - h)) / h_sq
}

