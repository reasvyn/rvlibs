//! Trigonometric functions for [`Numeric`](crate::Numeric) types.
//!
//! All angle arguments are in radians.
//!
//! # Example
//!
//! ```rust
//! use rvmath::ops::trig;
//!
//! assert_eq!(trig::sin(0.0_f64), 0.0);
//! assert_eq!(trig::cos(0.0_f64), 1.0);
//! assert!((trig::atan2(1.0_f64, 1.0_f64) - 0.785398).abs() < 1e-5);
//! ```

use crate::num::Numeric;

/// Sine.
pub fn sin<T: Numeric>(a: T) -> T { a.sin() }

/// Cosine.
pub fn cos<T: Numeric>(a: T) -> T { a.cos() }

/// Tangent.
pub fn tan<T: Numeric>(a: T) -> T { a.tan() }

/// Arc sine.
pub fn asin<T: Numeric>(a: T) -> T { a.asin() }

/// Arc cosine.
pub fn acos<T: Numeric>(a: T) -> T { a.acos() }

/// Arc tangent.
pub fn atan<T: Numeric>(a: T) -> T { a.atan() }

/// Arc tangent with two arguments (`atan2(y, x)`).
pub fn atan2<T: Numeric>(y: T, x: T) -> T { y.atan2(&x) }
