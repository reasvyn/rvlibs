//! Percentage type for ratio-based arithmetic.
//!
//! [`Percentage<T>`] wraps a ratio (`0.5` = 50%) and provides intuitive scaling:
//!
//! ```
//! # use rvmath::num::{Num, Percentage};
//! let twenty_pct = Percentage::from_percent(20.0);
//! let val = Num::new(200.0);
//! assert_eq!((val + twenty_pct).value, 240.0); // 200 + 20%
//! assert_eq!((val * twenty_pct).value, 40.0);  // 200 * 20%
//! ```
//!
//! Construction: [`from_ratio`](Percentage::from_ratio), [`from_percent`](Percentage::from_percent).  
//! Accessors: [`to_percent`](Percentage::to_percent), [`ratio`](Percentage::ratio).  
//! Supports arithmetic with [`Num`], [`Unit`](crate::unit::Unit), and other `Percentage` values.
use crate::num::{Num, Numeric, NumberKind, NumberSet, Signed};
use crate::unit::{Unit, meta::Meta};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

/// A structure representing a percentage value.
///
/// Internally, it stores the value as a ratio (e.g., 0.5 represents 50%).
/// It can be used for scaling and percentage-based arithmetic with `Num` and `Unit`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage<T: Numeric>(pub T);

impl<T: Numeric> Percentage<T> {
    /// Creates a new `Percentage` from a ratio (e.g., 0.1 for 10%).
    pub fn from_ratio(ratio: T) -> Self {
        Self(ratio)
    }

    /// Creates a new `Percentage` from a percent value (e.g., 10.0 for 10%).
    pub fn from_percent(percent: f64) -> Self {
        Self(T::from_f64(percent / 100.0))
    }

    /// Returns the percentage as a percent value (e.g., 10.0 for 10%).
    pub fn to_percent(&self) -> f64 {
        self.0.to_f64() * 100.0
    }

    /// Returns the internal ratio value.
    pub fn ratio(&self) -> T {
        self.0
    }
}

impl<T: Numeric> Default for Percentage<T> {
    fn default() -> Self {
        Self(T::from_f64(0.0))
    }
}

impl<T: Numeric> std::fmt::Display for Percentage<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.to_percent())
    }
}

// --- Num & Percentage Operations ---

/// Multiply a numeric value by a percentage.
///
/// Scales the numeric value by the percentage ratio.
///
impl<T: Numeric> Mul<Percentage<T>> for Num<T> {
    type Output = Num<T>;

    fn mul(self, rhs: Percentage<T>) -> Self::Output {
        Num::new(self.value * rhs.0)
    }
}

/// Add a percentage to a numeric value.
///
/// Increases the value by the specified percentage.
/// Formula: `x + (x * p) = x * (1 + p)`
///
impl<T: Numeric> Add<Percentage<T>> for Num<T> {
    type Output = Num<T>;

    fn add(self, rhs: Percentage<T>) -> Self::Output {
        // x + (x * p) = x * (1 + p)
        Num::new(self.value * (T::from_f64(1.0) + rhs.0))
    }
}

/// Subtract a percentage from a numeric value.
///
/// Decreases the value by the specified percentage.
/// Formula: `x - (x * p) = x * (1 - p)`
///
impl<T: Numeric> Sub<Percentage<T>> for Num<T> {
    type Output = Num<T>;

    fn sub(self, rhs: Percentage<T>) -> Self::Output {
        // x - (x * p) = x * (1 - p)
        Num::new(self.value * (T::from_f64(1.0) - rhs.0))
    }
}

// --- Unit & Percentage Operations ---

/// Multiply a unit-aware value by a percentage.
///
/// Scales the unit value by the percentage ratio while preserving the unit power.
///
impl<N: Numeric, M: Meta> Mul<Percentage<N>> for Unit<N, M> {
    type Output = Unit<N, M>;

    fn mul(self, rhs: Percentage<N>) -> Self::Output {
        Unit::with_power(self.value * rhs.0, self.power)
    }
}

/// Add a percentage to a unit-aware value.
///
/// Increases the unit value by the specified percentage while preserving unit power.
/// Formula: `x + (x * p)`
///
impl<N: Numeric, M: Meta> Add<Percentage<N>> for Unit<N, M> {
    type Output = Unit<N, M>;

    fn add(self, rhs: Percentage<N>) -> Self::Output {
        // x + (x * p)
        Unit::with_power(self.value * (N::from_f64(1.0) + rhs.0), self.power)
    }
}

/// Subtract a percentage from a unit-aware value.
///
/// Decreases the unit value by the specified percentage while preserving unit power.
/// Formula: `x - (x * p)`
///
impl<N: Numeric, M: Meta> Sub<Percentage<N>> for Unit<N, M> {
    type Output = Unit<N, M>;

    fn sub(self, rhs: Percentage<N>) -> Self::Output {
        // x - (x * p)
        Unit::with_power(self.value * (N::from_f64(1.0) - rhs.0), self.power)
    }
}

// --- Percentage arithmetic ---

impl<T: Numeric> Mul<T> for Percentage<T> {
    type Output = T;
    fn mul(self, rhs: T) -> Self::Output {
        self.0 * rhs
    }
}

// Negation for signed percentages
impl<T: Numeric + Signed> Neg for Percentage<T> {
    type Output = Self;
    fn neg(self) -> Self { Percentage(-self.0) }
}

impl<T: Numeric> Mul<Percentage<T>> for Percentage<T> {
    type Output = Self;
    fn mul(self, rhs: Percentage<T>) -> Self::Output {
        Percentage(self.0 * rhs.0)
    }
}

// Percentage + Percentage
impl<T: Numeric> Add for Percentage<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Percentage(self.0 + rhs.0) }
}
impl<T: Numeric> Sub for Percentage<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { Percentage(self.0 - rhs.0) }
}
impl<T: Numeric> Div for Percentage<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self { Percentage(self.0 / rhs.0) }
}
impl<T: Numeric> Rem for Percentage<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self { Percentage(self.0 % rhs.0) }
}

impl<T: Numeric> AddAssign for Percentage<T> {
    fn add_assign(&mut self, rhs: Self) { self.0 = self.0 + rhs.0; }
}
impl<T: Numeric> SubAssign for Percentage<T> {
    fn sub_assign(&mut self, rhs: Self) { self.0 = self.0 - rhs.0; }
}
impl<T: Numeric> MulAssign for Percentage<T> {
    fn mul_assign(&mut self, rhs: Self) { self.0 = self.0 * rhs.0; }
}
impl<T: Numeric> DivAssign for Percentage<T> {
    fn div_assign(&mut self, rhs: Self) { self.0 = self.0 / rhs.0; }
}
impl<T: Numeric> RemAssign for Percentage<T> {
    fn rem_assign(&mut self, rhs: Self) { self.0 = self.0 % rhs.0; }
}

// ---- Numeric trait ----

impl<T: Numeric> Numeric for Percentage<T> {
    #[inline]
    fn to_f64(self) -> f64 { self.0.to_f64() }
    #[inline]
    fn from_f64(v: f64) -> Self { Percentage(T::from_f64(v)) }
}

impl<T: Numeric + Signed> Signed for Percentage<T> {}

// ---- NumberKind ----

impl<T: Numeric> NumberKind for Percentage<T> {
    fn number_set() -> NumberSet { NumberSet::Real }
    fn is_signed() -> bool { T::from_f64(-1.0) < T::from_f64(0.0) }
    fn is_integer_valued() -> bool { false }
    fn is_float() -> bool { T::from_f64(0.5).to_f64().fract() != 0.0 }
}
