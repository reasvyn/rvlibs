//! Natural number type (ℕ).
//!
//! Provides a `Natural<T>` newtype that guarantees the wrapped value is positive (greater than zero).
//! This corresponds to the mathematical set of natural numbers ℕ = {1, 2, 3, ...}.

use crate::num::{NumberKind, NumberSet, Numeric};
use std::ops::{Add, Deref, Div, Mul, Rem, Sub};

/// A natural number (ℕ) — a positive integer greater than zero.
///
/// Construction returns `None` if the value is zero or negative, enforcing
/// the mathematical definition of natural numbers at runtime.
///
/// `Natural<T>` dereferences to `T`, so all `Numeric` methods are available directly.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Natural<T: Numeric>(T);

impl<T: Numeric> Natural<T> {
    /// Attempts to create a `Natural` from a value.
    ///
    /// Returns `Some(Natural)` if the value is strictly greater than zero,
    /// and `None` otherwise.
    pub fn new(value: T) -> Option<Self> {
        if value > T::from_f64(0.0) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Unwraps the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Successor: returns `n + 1`.
    ///
    /// Since the result is always positive, this is infallible.
    pub fn succ(self) -> Self {
        Self(self.0 + T::from_f64(1.0))
    }

    /// Predecessor: returns `n - 1` if the result is still positive.
    ///
    /// Returns `None` if `n == 1` (since `0` is not in ℕ).
    pub fn pred(self) -> Option<Self> {
        let result = self.0 - T::from_f64(1.0);
        if result > T::from_f64(0.0) {
            Some(Self(result))
        } else {
            None
        }
    }

    /// Checked addition — always succeeds for naturals.
    pub fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self(self.0 + other.0))
    }

    /// Checked multiplication — always succeeds for naturals.
    pub fn checked_mul(self, other: Self) -> Option<Self> {
        Some(Self(self.0 * other.0))
    }

    /// Checked subtraction — returns `None` if the result would not be a natural.
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        if self.0 > other.0 {
            Some(Self(self.0 - other.0))
        } else {
            None
        }
    }

    /// Checked division — returns `None` if the result would be zero.
    pub fn checked_div(self, other: Self) -> Option<Self> {
        let result = self.0 / other.0;
        if result > T::from_f64(0.0) {
            Some(Self(result))
        } else {
            None
        }
    }
}

impl<T: Numeric> Deref for Natural<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Numeric> NumberKind for Natural<T> {
    fn number_set() -> NumberSet {
        NumberSet::Natural
    }

    fn is_signed() -> bool {
        false
    }

    fn is_integer_valued() -> bool {
        true
    }

    fn is_float() -> bool {
        false
    }
}

// ---- Arithmetic: Natural + Natural = Natural ----

impl<T: Numeric> Add for Natural<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl<T: Numeric> Sub for Natural<T> {
    type Output = Option<Self>;

    fn sub(self, other: Self) -> Option<Self> {
        if self.0 > other.0 {
            Some(Self(self.0 - other.0))
        } else if self.0 == other.0 {
            None
        } else {
            None
        }
    }
}

impl<T: Numeric> Mul for Natural<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl<T: Numeric> Div for Natural<T> {
    type Output = Option<Self>;

    fn div(self, other: Self) -> Option<Self> {
        let result = self.0 / other.0;
        if result > T::from_f64(0.0) {
            Some(Self(result))
        } else {
            None
        }
    }
}

impl<T: Numeric> Rem for Natural<T> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self(self.0 % other.0)
    }
}

impl<T: Numeric> std::fmt::Display for Natural<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
