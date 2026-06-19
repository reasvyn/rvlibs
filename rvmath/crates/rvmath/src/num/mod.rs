//! Core numeric abstraction layer.
//!
//! This module provides a unified generic numeric programming framework built
//! around the [`Numeric`] trait. Any type implementing `Numeric` gains access
//! to 30+ mathematical operations, constants, and cross-type comparison via
//! `to_f64`.
//!
//! # Key Types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`Num<T>`] | Generic numeric wrapper with operator overloading |
//! | [`Numeric`] | Core trait defining all numeric operations |
//! | [`Complex<T>`](complex::Complex) | Complex numbers (rectangular & polar forms) |
//! | [`Fraction<T>`](fraction::Fraction) | Reduced rational numbers with arithmetic |
//! | [`Percentage<T>`](percentage::Percentage) | Percentage values with ratio/percent conversion |
//! | [`Natural<T>`](natural::Natural) | Natural numbers (ℕ) with checked arithmetic |
//! | [`Set<T>`] | Generic numeric set with union, intersection, difference |
//! | [`NumberSet`] | Finite set of named number types (ℕ, ℤ, ℚ, ℝ, ℂ) |
//! | [`NumberKind`] | Trait for types that belong to a `NumberSet` |
//!
//! # Operations on `Numeric`
//!
//! **Unary:** `ln`, `log10`, `exp`, `sqrt`, `cbrt`, `sin`, `cos`, `tan`, `asin`,
//! `acos`, `atan`, `sinh`, `cosh`, `tanh`, `to_degrees`, `to_radians`, `ln_1p`,
//! `exp_m1`, `recip` / `inv`, `round`, `floor`, `ceil`, `fract`, `abs`, `sign`
//!
//! **Binary:** `log(base)`, `atan2`, `hypot`, `min`, `max`, `pow`, `powf`, `powi`,
//! `root`, `clamp`, `lerp`, `map_range`
//!
//! **Constants:** `pi`, `e`, `tau`, `phi`
//!
//! **Classification:** `is_nan`, `is_infinite`, `is_finite`
//!
//! Cross-type comparison is supported through `to_f64`, which all `Numeric`
//! types implement.
//!
//! # Sub-modules
//!
//! - [`complex`] — Complex number arithmetic
//! - [`fraction`] — Rational number arithmetic
//! - [`natural`] — Natural number arithmetic
//! - [`percentage`] — Percentage utilities
//!
//! # Example
//!
//! ```rust
//! use rvmath::num::Num;
//!
//! let a = Num::new(10.0_f64);
//! let b = Num::new(3.0_f64);
//! let sum = a + b;
//! let product = a * b;
//! assert_eq!(sum.value, 13.0);
//! assert_eq!(product.value, 30.0);
//! ```

mod numeric;
pub mod complex;
pub mod fraction;
pub mod natural;
pub mod percentage;
mod set;

pub use complex::*;
pub use numeric::*;
pub use fraction::*;
pub use natural::*;
pub use percentage::*;
pub use set::*;

/// Generic number type that wraps a value of type `T` that implements the `Numeric` trait.
///
/// This wrapper provides a convenient way to use generic numeric types with
/// standard arithmetic operator overloading.
#[derive(Debug, Clone, Copy)]
pub struct Num<T: Numeric> {
    /// The wrapped numeric value.
    pub value: T,
}

impl<T: Numeric> Num<T> {
    /// Creates a new `Num` with the given value.
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Returns `Num<T>` with value `0`.
    pub fn zero() -> Self { Self::new(T::from_f64(0.0)) }

    /// Returns `Num<T>` with value `1`.
    pub fn one() -> Self { Self::new(T::from_f64(1.0)) }
}

impl<T: Numeric> Default for Num<T> {
    fn default() -> Self {
        Self::new(T::from_f64(0.0))
    }
}

// ---- Display ---

impl<T: Numeric> std::fmt::Display for Num<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// ── Operator Impls ─────────────────────────────────────────

use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

macro_rules! impl_bin_ops {
    ($(($trait:ident, $method:ident)),+) => {
        $(
            impl<T: Numeric> $trait for Num<T> {
                type Output = Self;
                #[inline]
                fn $method(self, rhs: Self) -> Self {
                    Self::new(self.value.$method(rhs.value))
                }
            }
        )+
    };
}

macro_rules! impl_assign_ops {
    ($(($trait:ident, $method:ident)),+) => {
        $(
            impl<T: Numeric> $trait for Num<T> {
                #[inline]
                fn $method(&mut self, rhs: Self) {
                    self.value.$method(rhs.value);
                }
            }
        )+
    };
}

impl<T: Signed> Neg for Num<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.value)
    }
}

impl_bin_ops!((Add, add), (Sub, sub), (Mul, mul), (Div, div), (Rem, rem));
impl_assign_ops!((AddAssign, add_assign), (SubAssign, sub_assign), (MulAssign, mul_assign), (DivAssign, div_assign), (RemAssign, rem_assign));

macro_rules! impl_comparisons {
    ($($all:ty),*) => {
        $(
            impl_comparisons!(@inner $all, (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64));
        )*
    };
    (@inner $curr:ty, ($($others:ty),*)) => {
        $(
            impl PartialEq<Num<$others>> for Num<$curr> {
                #[inline]
                fn eq(&self, other: &Num<$others>) -> bool {
                    self.value.to_f64() == other.value.to_f64()
                }
            }
            impl PartialOrd<Num<$others>> for Num<$curr> {
                #[inline]
                fn partial_cmp(&self, other: &Num<$others>) -> Option<std::cmp::Ordering> {
                    self.value.to_f64().partial_cmp(&other.value.to_f64())
                }
            }
        )*
    };
}

impl_comparisons!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);
