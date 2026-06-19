//! Numeric traits and implementations for primitive types.
//!
//! This module defines the `Numeric` and `Signed` traits which provide
//! a unified interface for various numeric types. It utilizes macros to reduce
//! boilerplate while maintaining comprehensive documentation.

use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

macro_rules! numeric_const {
    ($($(#[$meta:meta])* $name:ident = $val:expr;)*) => {
        $(
            $(#[$meta])*
            fn $name() -> Self {
                Self::from_f64($val)
            }
        )*
    };
}

macro_rules! numeric_unary {
    ($($(#[$meta:meta])* $name:ident;)*) => {
        $(
            $(#[$meta])*
            fn $name(&self) -> Self {
                Self::from_f64(self.to_f64().$name())
            }
        )*
    };
}

macro_rules! numeric_binary {
    ($($(#[$meta:meta])* $name:ident($arg:ident);)*) => {
        $(
            $(#[$meta])*
            fn $name(&self, $arg: &Self) -> Self {
                Self::from_f64(self.to_f64().$name($arg.to_f64()))
            }
        )*
    };
}

/// A trait for types that behave like numbers.
pub trait Numeric:
    Sized
    + Copy
    + std::fmt::Display
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
{
    /// Converts the numeric value to a 64-bit float.
    fn to_f64(self) -> f64;

    /// Creates a numeric value from a 64-bit float.
    fn from_f64(v: f64) -> Self;

    numeric_const! {
        /// Returns the mathematical constant PI.
        pi = std::f64::consts::PI;
        /// Returns the mathematical constant E.
        e = std::f64::consts::E;
        /// Returns the mathematical constant TAU (2 * PI).
        tau = std::f64::consts::TAU;
        /// Returns the golden ratio PHI.
        phi = crate::consts::PHI;
    }

    numeric_unary! {
        /// Returns the natural logarithm (ln) of the number.
        ln;
        /// Returns the base-10 logarithm of the number.
        log10;
        /// Returns e raised to the power of the number.
        exp;
        /// Returns the square root of the number.
        sqrt;
        /// Returns the cube root of the number.
        cbrt;
        /// Returns the sine of the number (in radians).
        sin;
        /// Returns the cosine of the number (in radians).
        cos;
        /// Returns the tangent of the number (in radians).
        tan;
        /// Returns the arc sine of the number.
        asin;
        /// Returns the arc cosine of the number.
        acos;
        /// Returns the arc tangent of the number.
        atan;
        /// Returns the hyperbolic sine of the number.
        sinh;
        /// Returns the hyperbolic cosine of the number.
        cosh;
        /// Returns the hyperbolic tangent of the number.
        tanh;
        /// Converts radians to degrees.
        to_degrees;
        /// Converts degrees to radians.
        to_radians;
        /// Returns ln(1 + n) more accurately than ln(n + 1).
        ln_1p;
        /// Returns e^n - 1 more accurately than exp(n) - 1.
        exp_m1;
        /// Returns the multiplicative inverse of the number (1/n).
        recip;
        /// Returns the nearest integer to the number.
        round;
        /// Returns the largest integer less than or equal to the number.
        floor;
        /// Returns the smallest integer greater than or equal to the number.
        ceil;
        /// Returns the fractional part of the number.
        fract;
        /// Returns the absolute value of the number.
        abs;
    }

    numeric_binary! {
        /// Returns the logarithm with a custom base.
        log(base);
        /// Returns the arc tangent of the number using two arguments (y, x).
        atan2(x);
        /// Returns the length of the hypotenuse (sqrt(x^2 + y^2)).
        hypot(x);
        /// Returns the minimum of two numbers.
        min(other);
        /// Returns the maximum of two numbers.
        max(other);
    }

    /// Alias for recip (1/n).
    fn inv(&self) -> Self {
        self.recip()
    }

    /// Returns the sign of the number (1.0, -1.0, or 0.0).
    fn sign(&self) -> Self {
        let val = self.to_f64();
        if val > 0.0 {
            Self::from_f64(1.0)
        } else if val < 0.0 {
            Self::from_f64(-1.0)
        } else {
            Self::from_f64(0.0)
        }
    }

    /// Returns the power of the number raised to another number.
    fn pow(&self, n: &Self) -> Self {
        Self::from_f64(self.to_f64().powf(n.to_f64()))
    }

    /// Returns the power of the number raised to a floating-point exponent.
    fn powf(&self, n: f64) -> Self {
        Self::from_f64(self.to_f64().powf(n))
    }

    /// Returns the power of the number raised to an integer exponent.
    fn powi(&self, n: i32) -> Self {
        Self::from_f64(self.to_f64().powi(n))
    }

    /// Returns the n-th root of the number.
    fn root(&self, n: &Self) -> Self {
        let val = self.to_f64();
        let exp = n.to_f64();
        if exp == 0.0 {
            return Self::from_f64(f64::NAN);
        }
        if val < 0.0 && exp.fract() == 0.0 && exp as i64 % 2 == 1 {
            Self::from_f64(-(-val).powf(1.0 / exp))
        } else {
            Self::from_f64(val.powf(1.0 / exp))
        }
    }

    /// Clamps the number between a minimum and maximum value.
    fn clamp(&self, min: &Self, max: &Self) -> Self {
        Self::from_f64(self.to_f64().clamp(min.to_f64(), max.to_f64()))
    }

    /// Linearly interpolates between self and other by a factor of t.
    fn lerp(&self, other: &Self, t: f64) -> Self {
        let start = self.to_f64();
        let end = other.to_f64();
        Self::from_f64(start + (end - start) * t)
    }

    /// Maps a value from one range [in_min, in_max] to another [out_min, out_max].
    fn map_range(&self, in_min: &Self, in_max: &Self, out_min: &Self, out_max: &Self) -> Self {
        let val = self.to_f64();
        let imin = in_min.to_f64();
        let imax = in_max.to_f64();
        let omin = out_min.to_f64();
        let omax = out_max.to_f64();
        Self::from_f64(omin + (val - imin) * (omax - omin) / (imax - imin))
    }

    /// Returns `true` if the number is NaN.
    fn is_nan(self) -> bool {
        self.to_f64().is_nan()
    }

    /// Returns `true` if the number is infinite.
    fn is_infinite(self) -> bool {
        self.to_f64().is_infinite()
    }

    /// Returns `true` if the number is finite.
    fn is_finite(self) -> bool {
        self.to_f64().is_finite()
    }
}

/// A trait for numeric types that can be negative.
pub trait Signed: Numeric + Neg<Output = Self> {
    /// Returns true if the value is strictly positive (> 0).
    fn is_positive(&self) -> bool { self.to_f64() > 0.0 }
    /// Returns true if the value is strictly negative (< 0).
    fn is_negative(&self) -> bool { self.to_f64() < 0.0 }
}

macro_rules! impl_numeric_float {
    ($($t:ty),*) => {
        $(
            impl Numeric for $t {
                #[inline]
                fn to_f64(self) -> f64 { self as f64 }
                #[inline]
                fn from_f64(v: f64) -> Self { v as $t }
            }
        )*
    };
}

macro_rules! impl_numeric_int {
    ($($t:ty),*) => {
        $(
            impl Numeric for $t {
                #[inline]
                fn to_f64(self) -> f64 { self as f64 }
                #[inline]
                fn from_f64(v: f64) -> Self { v as $t }

                fn pow(&self, n: &Self) -> Self {
                    let exp = n.to_f64() as u32;
                    Self::from_f64((self.to_f64()).powi(exp as i32))
                }

                fn powi(&self, n: i32) -> Self {
                    Self::from_f64((self.to_f64()).powi(n))
                }

                fn root(&self, n: &Self) -> Self {
                    let val = self.to_f64();
                    let exp = n.to_f64();
                    if exp == 0.0 { return Self::from_f64(f64::NAN); }
                    if val < 0.0 && exp.fract() == 0.0 && exp as i64 % 2 == 1 {
                        Self::from_f64(-(-val).powf(1.0 / exp))
                    } else {
                        Self::from_f64(val.powf(1.0 / exp))
                    }
                }

                fn recip(&self) -> Self {
                    if self.to_f64() == 0.0 { Self::from_f64(0.0) } else { Self::from_f64(1.0 / self.to_f64()) }
                }

                fn round(&self) -> Self { *self }
                fn floor(&self) -> Self { *self }
                fn ceil(&self) -> Self { *self }
                fn fract(&self) -> Self { Self::from_f64(0.0) }
            }
        )*
    };
}

macro_rules! impl_signed {
    ($($t:ty),*) => {
        $( impl Signed for $t {} )*
    };
}

impl_numeric_float!(f32, f64);
impl_numeric_int!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl_signed!(i8, i16, i32, i64, isize, f32, f64);
