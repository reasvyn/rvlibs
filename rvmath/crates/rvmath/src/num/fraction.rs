//! Fraction (rational number) arithmetic with automatic reduction.
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

use crate::num::{Num, NumberKind, NumberSet, Numeric, Percentage, Signed};
use crate::unit::{meta::Meta, Unit};

/// A fraction type with generic numerator and denominator.
///
/// Fractions are automatically reduced to lowest terms upon construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction<T: Numeric> {
    /// The numerator of the fraction.
    numerator: T,
    /// The denominator of the fraction (must be non-zero).
    denominator: T,
}

// ---- GCD helper ----

fn gcd<T: Numeric>(a: T, b: T) -> T {
    let zero = T::from_f64(0.0);
    if b == zero {
        let g = a;
        if g < zero { zero - g } else { g }
    } else {
        gcd(b, a % b)
    }
}

// ---- Constructors ----

impl<T: Numeric> Fraction<T> {
    /// Creates a new fraction, automatically reduced to lowest terms.
    /// The denominator is guaranteed to be positive after reduction
    /// when `T` is a signed type. For unsigned types the denominator
    /// is always non-negative.
    ///
    /// # Panics
    ///
    /// Panics if the denominator is zero.
    pub fn new(numerator: T, denominator: T) -> Self {
        Self::try_new(numerator, denominator)
            .expect("fraction denominator cannot be zero")
    }

    /// Tries to create a new fraction. Returns `None` if the denominator is zero.
    pub fn try_new(numerator: T, denominator: T) -> Option<Self> {
        let zero = T::from_f64(0.0);
        if denominator == zero {
            return None;
        }
        let g = gcd(numerator, denominator);
        let num = numerator / g;
        let den = denominator / g;
        // Sign normalization for signed types is handled in dedicated methods
        Some(Self {
            numerator: num,
            denominator: den,
        })
    }

    /// Creates a fraction representing zero.
    pub fn zero() -> Self {
        Self {
            numerator: T::from_f64(0.0),
            denominator: T::from_f64(1.0),
        }
    }

    /// Creates a fraction representing one.
    pub fn one() -> Self {
        Self {
            numerator: T::from_f64(1.0),
            denominator: T::from_f64(1.0),
        }
    }

    /// Creates a fraction from an integer value.
    pub fn from_integer(value: T) -> Self {
        Self {
            numerator: value,
            denominator: T::from_f64(1.0),
        }
    }

    /// Approximates a floating-point value as a fraction using
    /// the continued fraction algorithm.
    pub fn from_float(value: f64, max_denominator: i64) -> Self {
        if value == 0.0 {
            return Self::zero();
        }

        let sign = if value < 0.0 { -1.0 } else { 1.0 };
        let abs_val = value.abs();

        // Continued fraction algorithm
        // Standard initial values for convergents h/k:
        //   h_{-2}=0, h_{-1}=1, k_{-2}=1, k_{-1}=0
        let mut h_prev2 = 0i64;  // h_{n-2}
        let mut h_prev1 = 1i64;  // h_{n-1}
        let mut k_prev2 = 1i64;  // k_{n-2}
        let mut k_prev1 = 0i64;  // k_{n-1}
        let mut x = abs_val;

        for _ in 0..20 {
            let n = x.floor() as i64;
            let h = n * h_prev1 + h_prev2;
            let k = n * k_prev1 + k_prev2;

            if k > max_denominator {
                break;
            }

            h_prev2 = h_prev1;
            h_prev1 = h;
            k_prev2 = k_prev1;
            k_prev1 = k;

            let diff = x - n as f64;
            if diff.abs() < 1e-15 {
                break;
            }
            x = diff.recip();
        }

        let num = T::from_f64(sign * h_prev1 as f64);
        let den = T::from_f64(k_prev1 as f64);
        let g = gcd(num, den);
        Self {
            numerator: num / g,
            denominator: den / g,
        }
    }
}

impl<T: Numeric> Default for Fraction<T> {
    fn default() -> Self {
        Self::zero()
    }
}

// ---- Accessors ----

impl<T: Numeric> Fraction<T> {
    /// Returns the numerator.
    pub fn numerator(&self) -> T {
        self.numerator
    }

    /// Returns the denominator.
    pub fn denominator(&self) -> T {
        self.denominator
    }
}

// ---- Methods ----

impl<T: Numeric> Fraction<T> {
    /// Returns the reciprocal of this fraction.
    ///
    /// # Panics
    ///
    /// Panics if the numerator is zero.
    pub fn recip(self) -> Self {
        assert!(
            self.numerator != T::from_f64(0.0),
            "cannot take reciprocal of zero fraction"
        );
        Self {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }

    /// Reduces this fraction to lowest terms.
    /// The fraction is already reduced on construction, so this is
    /// useful after direct field mutation.
    pub fn reduce(self) -> Self {
        let g = gcd(self.numerator, self.denominator);
        Self {
            numerator: self.numerator / g,
            denominator: self.denominator / g,
        }
    }

    /// Returns `true` if the fraction is proper (|numerator| < denominator).
    pub fn is_proper(&self) -> bool {
        let zero = T::from_f64(0.0);
        let num = if self.numerator < zero {
            zero - self.numerator
        } else {
            self.numerator
        };
        num < self.denominator
    }

    /// Returns `true` if the fraction is improper (|numerator| >= denominator).
    pub fn is_improper(&self) -> bool {
        !self.is_proper()
    }

    /// Returns `true` if the fraction represents an integer (denominator == 1).
    pub fn is_integer(&self) -> bool {
        self.denominator == T::from_f64(1.0)
    }

    /// Converts this fraction to `f64`.
    pub fn to_f64(&self) -> f64 {
        self.numerator.to_f64() / self.denominator.to_f64()
    }

    /// Converts this fraction to `Num<T>`.
    pub fn to_num(self) -> Num<T> {
        Num::new(self.numerator / self.denominator)
    }

    /// Returns the integer part (truncation toward zero).
    pub fn trunc(self) -> T {
        self.numerator / self.denominator
    }

    /// Returns the fractional part (proper fraction remainder).
    pub fn fract(self) -> Self {
        let whole = self.trunc();
        let remainder = self.numerator - whole * self.denominator;
        Self {
            numerator: remainder,
            denominator: self.denominator,
        }
    }
}

// ---- Signed methods ----

impl<T: Numeric + Signed> Fraction<T> {
    /// Normalizes the sign so the denominator is always positive.
    pub fn normalize_sign(self) -> Self {
        let zero = T::from_f64(0.0);
        if self.denominator < zero {
            Self {
                numerator: -self.numerator,
                denominator: -self.denominator,
            }
        } else {
            self
        }
    }

    /// Returns the absolute value of this fraction.
    pub fn abs(self) -> Self {
        let zero = T::from_f64(0.0);
        Self {
            numerator: if self.numerator < zero {
                -self.numerator
            } else {
                self.numerator
            },
            denominator: if self.denominator < zero {
                -self.denominator
            } else {
                self.denominator
            },
        }
    }

    /// Splits an improper fraction into a mixed number (whole part, proper fraction).
    pub fn mixed(self) -> (Num<T>, Self) {
        let whole_num = self.trunc();
        let remainder = self.numerator - whole_num * self.denominator;
        (
            Num::new(whole_num),
            Self {
                numerator: remainder,
                denominator: self.denominator,
            },
        )
    }
}

// ---- Display ----

impl<T: Numeric> std::fmt::Display for Fraction<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denominator == T::from_f64(1.0) {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

// ---- Neg ----

impl<T: Numeric + Signed> Neg for Fraction<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

// ---- Add ----

impl<T: Numeric> Add for Fraction<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let num = self.numerator * rhs.denominator + rhs.numerator * self.denominator;
        let den = self.denominator * rhs.denominator;
        let g = gcd(num, den);
        Self {
            numerator: num / g,
            denominator: den / g,
        }
    }
}

impl<T: Numeric> Add<T> for Fraction<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        self + Self::from_integer(rhs)
    }
}

// ---- AddAssign ----

impl<T: Numeric> AddAssign for Fraction<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T: Numeric> AddAssign<T> for Fraction<T> {
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

// ---- Sub ----

impl<T: Numeric> Sub for Fraction<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let num = self.numerator * rhs.denominator - rhs.numerator * self.denominator;
        let den = self.denominator * rhs.denominator;
        let g = gcd(num, den);
        Self {
            numerator: num / g,
            denominator: den / g,
        }
    }
}

impl<T: Numeric> Sub<T> for Fraction<T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        self - Self::from_integer(rhs)
    }
}

// ---- SubAssign ----

impl<T: Numeric> SubAssign for Fraction<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T: Numeric> SubAssign<T> for Fraction<T> {
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

// ---- Mul ----

impl<T: Numeric> Mul for Fraction<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let num = self.numerator * rhs.numerator;
        let den = self.denominator * rhs.denominator;
        let g = gcd(num, den);
        Self {
            numerator: num / g,
            denominator: den / g,
        }
    }
}

impl<T: Numeric> Mul<T> for Fraction<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let num = self.numerator * rhs;
        let den = self.denominator;
        let g = gcd(num, den);
        Self {
            numerator: num / g,
            denominator: den / g,
        }
    }
}

// ---- MulAssign ----

impl<T: Numeric> MulAssign for Fraction<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: Numeric> MulAssign<T> for Fraction<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

// ---- Div ----

impl<T: Numeric> Div for Fraction<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert!(
            rhs.numerator != T::from_f64(0.0),
            "cannot divide fraction by zero"
        );
        self * rhs.recip()
    }
}

impl<T: Numeric> Div<T> for Fraction<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let zero = T::from_f64(0.0);
        assert!(rhs != zero, "cannot divide fraction by zero");
        let num = self.numerator;
        let den = self.denominator * rhs;
        let g = gcd(num, den);
        Self {
            numerator: num / g,
            denominator: den / g,
        }
    }
}

// ---- DivAssign ----

impl<T: Numeric> DivAssign for Fraction<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T: Numeric> DivAssign<T> for Fraction<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

// ---- Rem ----

impl<T: Numeric> RemAssign for Fraction<T> {
    fn rem_assign(&mut self, rhs: Self) { *self = *self % rhs; }
}

impl<T: Numeric> Rem for Fraction<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        assert!(
            rhs.numerator != T::from_f64(0.0),
            "cannot take remainder with zero fraction"
        );
        let quotient = (self.numerator * rhs.denominator)
            / (self.denominator * rhs.numerator);
        let remainder_num = self.numerator * rhs.denominator
            - quotient * self.denominator * rhs.numerator;
        let den = self.denominator * rhs.denominator;
        let g = gcd(remainder_num, den);
        Self {
            numerator: remainder_num / g,
            denominator: den / g,
        }
    }
}

// ---- PartialOrd ----

impl<T: Numeric> PartialOrd for Fraction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Cross-multiplication avoids floating-point precision loss.
        //   a/b  <  c/d   iff   a*d  <  c*b   (for b,d > 0)
        let lhs = self.numerator * other.denominator;
        let rhs = other.numerator * self.denominator;
        lhs.partial_cmp(&rhs)
    }
}

// ---- Conversions ----

impl<T: Numeric> From<T> for Fraction<T> {
    fn from(value: T) -> Self {
        Self::from_integer(value)
    }
}

impl<T: Numeric> From<Fraction<T>> for f64 {
    fn from(f: Fraction<T>) -> Self {
        f.to_f64()
    }
}

impl<T: Numeric> From<Fraction<T>> for Num<T> {
    fn from(f: Fraction<T>) -> Self {
        f.to_num()
    }
}

impl<T: Numeric> From<(T, T)> for Fraction<T> {
    fn from(tuple: (T, T)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

// ---- Percentage integration ----

impl<T: Numeric> From<Fraction<T>> for Percentage<T> {
    fn from(f: Fraction<T>) -> Self {
        Percentage(T::from_f64(f.to_f64()))
    }
}

impl<T: Numeric> From<Percentage<T>> for Fraction<T> {
    fn from(p: Percentage<T>) -> Self {
        Self::new(p.ratio(), T::from_f64(1.0))
    }
}

impl<T: Numeric> Mul<Fraction<T>> for Percentage<T> {
    type Output = Percentage<T>;
    fn mul(self, rhs: Fraction<T>) -> Self::Output {
        Percentage(self.0 * (rhs.numerator / rhs.denominator))
    }
}

// ---- Unit integration ----

impl<N: Numeric, M: Meta> Mul<Fraction<N>> for Unit<N, M> {
    type Output = Unit<N, M>;
    fn mul(self, rhs: Fraction<N>) -> Self::Output {
        let value = self.value * rhs.numerator / rhs.denominator;
        Unit::with_power(value, self.power)
    }
}

impl<N: Numeric, M: Meta> Div<Fraction<N>> for Unit<N, M> {
    type Output = Unit<N, M>;
    fn div(self, rhs: Fraction<N>) -> Self::Output {
        let value = self.value * rhs.denominator / rhs.numerator;
        Unit::with_power(value, self.power)
    }
}

// ---- NumberKind ----

impl<T: Numeric> NumberKind for Fraction<T> {
    fn number_set() -> NumberSet {
        NumberSet::Rational
    }

    fn is_signed() -> bool {
        T::from_f64(-1.0) < T::from_f64(0.0)
    }

    fn is_integer_valued() -> bool {
        false
    }

    fn is_float() -> bool {
        false
    }
}

// ---- Numeric trait ----

impl<T: Numeric> Numeric for Fraction<T> {
    #[inline]
    fn to_f64(self) -> f64 {
        self.numerator.to_f64() / self.denominator.to_f64()
    }

    #[inline]
    fn from_f64(v: f64) -> Self {
        Self::from_float(v, 1_000_000)
    }
}

impl<T: Numeric + Signed> Signed for Fraction<T> {}

// ---- Ensure operator trait bounds for Numeric ----
// The bounds Add<Output=Self>, Sub<Output=Self>, etc. are already satisfied
// by Fraction's existing operator impls earlier in this file.


