//! Free-function arithmetic operations for [`Numeric`](crate::Numeric) types.
//!
//! Every function here delegates to the corresponding [`Numeric`](crate::Numeric) trait method
//! or the standard `std::ops` trait, so it works with any type implementing `Numeric`
//! (f32, f64, i32, Num<T>, Unit<N,T>, Complex<T>, etc.).
//!
//! For more specialised operations see [`logarithm`](super::logarithm),
//! [`trig`](super::trig), and [`hyperbolic`](super::hyperbolic).
//!
//! # Example
//!
//! ```rust
//! use rvmath::ops::arithmetic;
//!
//! assert_eq!(arithmetic::add(2.0_f64, 3.0_f64), 5.0);
//! assert_eq!(arithmetic::mul(4.0_f64, 5.0_f64), 20.0);
//! assert_eq!(arithmetic::abs(-5.0_f64), 5.0);
//! ```

use crate::num::{Numeric, Signed};

// ── Basic Arithmetic ────────────────────────────────────────

/// Addition: `a + b`.
pub fn add<T: Numeric>(a: T, b: T) -> T { a + b }
/// Subtraction: `a - b`.
pub fn sub<T: Numeric>(a: T, b: T) -> T { a - b }
/// Multiplication: `a * b`.
pub fn mul<T: Numeric>(a: T, b: T) -> T { a * b }
/// Division: `a / b`.
pub fn div<T: Numeric>(a: T, b: T) -> T { a / b }
/// Remainder (modulo): `a % b`.
pub fn rem<T: Numeric>(a: T, b: T) -> T { a % b }
/// Negation: `-a` (requires [`Signed`]).
pub fn neg<T: Signed>(a: T) -> T { -a }
/// Absolute value: `|a|`.
pub fn abs<T: Numeric>(a: T) -> T { a.abs() }

// ── Power & Roots ───────────────────────────────────────────

/// Power: `a` raised to `b` (`a^b`).
pub fn pow<T: Numeric>(a: T, b: T) -> T { a.pow(&b) }
/// Power with `f64` exponent.
pub fn powf<T: Numeric>(a: T, b: f64) -> T { a.powf(b) }
/// Power with integer exponent.
pub fn powi<T: Numeric>(a: T, b: i32) -> T { a.powi(b) }
/// Square root.
pub fn sqrt<T: Numeric>(a: T) -> T { a.sqrt() }
/// Cube root.
pub fn cbrt<T: Numeric>(a: T) -> T { a.cbrt() }
/// n-th root (`a^(1/n)`).
pub fn root<T: Numeric>(a: T, n: T) -> T { a.root(&n) }
/// Reciprocal (multiplicative inverse): `1/a`.
pub fn recip<T: Numeric>(a: T) -> T { a.recip() }
/// Hypotenuse: `sqrt(a² + b²)`.
pub fn hypot<T: Numeric>(a: T, b: T) -> T { a.hypot(&b) }
/// Natural exponential `e^a`.
pub fn exp<T: Numeric>(a: T) -> T { a.exp() }
/// `e^a - 1` more accurate for small `a`.
pub fn exp_m1<T: Numeric>(a: T) -> T { a.exp_m1() }

// ── Sign & Comparison ───────────────────────────────────────

/// Sign of a number: returns `1.0`, `-1.0`, or `0.0`.
pub fn sign<T: Numeric>(a: T) -> T { a.sign() }
/// Minimum of two values.
pub fn min<T: Numeric>(a: T, b: T) -> T { T::from_f64(a.to_f64().min(b.to_f64())) }
/// Maximum of two values.
pub fn max<T: Numeric>(a: T, b: T) -> T { T::from_f64(a.to_f64().max(b.to_f64())) }
/// Clamp a value between `lo` and `hi`.
pub fn clamp<T: Numeric>(val: T, lo: T, hi: T) -> T { val.clamp(&lo, &hi) }

// ── Rounding ────────────────────────────────────────────────

/// Round to nearest integer, ties away from zero.
pub fn round<T: Numeric>(a: T) -> T { a.round() }
/// Largest integer ≤ `a`.
pub fn floor<T: Numeric>(a: T) -> T { a.floor() }
/// Smallest integer ≥ `a`.
pub fn ceil<T: Numeric>(a: T) -> T { a.ceil() }
/// Fractional part of `a`.
pub fn fract<T: Numeric>(a: T) -> T { a.fract() }

// ── Utility ─────────────────────────────────────────────────

/// Linear interpolation: `a + (b - a) * t`.
pub fn lerp<T: Numeric>(a: T, b: T, t: f64) -> T { a.lerp(&b, t) }
/// Map a value from one range to another.
pub fn map_range<T: Numeric>(val: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T {
    val.map_range(&in_min, &in_max, &out_min, &out_max)
}
/// Convert radians to degrees.
pub fn to_degrees<T: Numeric>(a: T) -> T { a.to_degrees() }
/// Convert degrees to radians.
pub fn to_radians<T: Numeric>(a: T) -> T { a.to_radians() }

// ── Classification ──────────────────────────────────────────

/// `true` if the value is NaN.
pub fn is_nan<T: Numeric>(a: T) -> bool { a.is_nan() }
/// `true` if the value is infinite.
pub fn is_infinite<T: Numeric>(a: T) -> bool { a.is_infinite() }
/// `true` if the value is finite.
pub fn is_finite<T: Numeric>(a: T) -> bool { a.is_finite() }

// ── Constants ───────────────────────────────────────────────

/// π (Pi).
pub fn pi<T: Numeric>() -> T { T::pi() }
/// Euler's number.
pub fn e<T: Numeric>() -> T { T::e() }
/// τ (2π).
pub fn tau<T: Numeric>() -> T { T::tau() }
/// Golden ratio φ.
pub fn phi<T: Numeric>() -> T { T::phi() }
