//! Hyperbolic functions for [`Numeric`](crate::Numeric) types.
//!
//! # Example
//!
//! ```rust
//! use rvmath::ops::hyperbolic;
//!
//! assert_eq!(hyperbolic::sinh(0.0_f64), 0.0);
//! assert_eq!(hyperbolic::cosh(0.0_f64), 1.0);
//! assert_eq!(hyperbolic::tanh(0.0_f64), 0.0);
//! ```

use crate::num::Numeric;

/// Hyperbolic sine.
pub fn sinh<T: Numeric>(a: T) -> T { a.sinh() }

/// Hyperbolic cosine.
pub fn cosh<T: Numeric>(a: T) -> T { a.cosh() }

/// Hyperbolic tangent.
pub fn tanh<T: Numeric>(a: T) -> T { a.tanh() }
