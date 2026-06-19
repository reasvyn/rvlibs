//! Logarithmic functions for [`Numeric`](crate::Numeric) types.
//!
//! # Example
//!
//! ```rust
//! use rvmath::ops::logarithm;
//!
//! assert!((logarithm::ln(2.718281828_f64) - 1.0).abs() < 1e-9);
//! assert_eq!(logarithm::log10(100.0_f64), 2.0);
//! assert_eq!(logarithm::log2(8.0_f64), 3.0);
//! assert!((logarithm::ln_1p(1.0_f64) - 0.693147).abs() < 1e-5);
//! ```

use crate::num::Numeric;

/// Natural logarithm `ln(a)`.
pub fn ln<T: Numeric>(a: T) -> T { a.ln() }

/// Base-10 logarithm.
pub fn log10<T: Numeric>(a: T) -> T { a.log10() }

/// Logarithm with custom base.
pub fn log<T: Numeric>(a: T, base: T) -> T { a.log(&base) }

/// Base-2 logarithm.
pub fn log2<T: Numeric>(a: T) -> T { a.log(&T::from_f64(2.0)) }

/// `ln(1 + a)` more accurate for small `a`.
pub fn ln_1p<T: Numeric>(a: T) -> T { a.ln_1p() }
