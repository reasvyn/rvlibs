//! Complex number arithmetic and mathematical functions.
use std::cmp::Ordering;
use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::num::{Num, NumberKind, NumberSet, Numeric, Signed};

// ---- Complex Struct ----

/// A complex number type `a + bi` with generic numeric component type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex<T: Numeric> {
    /// The real part of the complex number.
    pub re: T,
    /// The imaginary part of the complex number.
    pub im: T,
}

// ---- Constructors ----

impl<T: Numeric> Complex<T> {
    /// Create a new complex number from real and imaginary parts.
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }

    /// Create a complex number from a real value (imaginary part = 0).
    pub fn from_real(re: T) -> Self {
        Self::new(re, T::from_f64(0.0))
    }

    /// Create a complex number from an imaginary value (real part = 0).
    pub fn from_imag(im: T) -> Self {
        Self::new(T::from_f64(0.0), im)
    }

    /// Create a complex number from polar coordinates (magnitude, angle in radians).
    pub fn from_polar(r: T, theta: T) -> Self {
        Self::new(r * theta.cos(), r * theta.sin())
    }

    /// The complex number 0 + 0i.
    pub fn zero() -> Self {
        Self::new(T::from_f64(0.0), T::from_f64(0.0))
    }

    /// The complex number 1 + 0i.
    pub fn one() -> Self {
        Self::new(T::from_f64(1.0), T::from_f64(0.0))
    }

    /// The imaginary unit 0 + 1i.
    pub fn i() -> Self {
        Self::new(T::from_f64(0.0), T::from_f64(1.0))
    }
}

impl<T: Numeric> Default for Complex<T> {
    fn default() -> Self {
        Self::zero()
    }
}

// ---- Methods ----

impl<T: Numeric> Complex<T> {
    /// Returns the real part of the complex number.
    pub fn re(self) -> T {
        self.re
    }

    /// Returns the imaginary part of the complex number.
    pub fn im(self) -> T {
        self.im
    }

    /// Returns `true` if the imaginary part is zero.
    pub fn is_real(self) -> bool {
        self.im == T::from_f64(0.0)
    }

    /// Returns `true` if the real part is zero.
    pub fn is_imag(self) -> bool {
        self.re == T::from_f64(0.0)
    }

    /// Returns the squared magnitude `|z|²`.
    pub fn norm_sqr(self) -> T {
        self.re * self.re + self.im * self.im
    }

    /// Returns the magnitude `|z|`.
    pub fn norm(self) -> T {
        T::from_f64(self.re.to_f64().hypot(self.im.to_f64()))
    }

    /// Returns the argument (angle in radians).
    pub fn arg(self) -> T {
        T::from_f64(self.im.to_f64().atan2(self.re.to_f64()))
    }
}

impl<T: Numeric + Signed> Complex<T> {
    /// Returns the complex conjugate.
    pub fn conj(self) -> Self {
        Self::new(self.re, -self.im)
    }
}

// ---- Display ----

impl<T: Numeric> fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let zero = T::from_f64(0.0);
        if self.im == zero {
            write!(f, "{}", self.re)
        } else if self.re == zero {
            write!(f, "{}i", self.im)
        } else {
            let sign = if self.im < zero { "-" } else { "+" };
            let im_abs = if self.im < zero {
                zero - self.im
            } else {
                self.im
            };
            write!(f, "{} {} {}i", self.re, sign, im_abs)
        }
    }
}

// ---- PartialOrd (Lexicographic) ----
//
// Note: Complex numbers have no natural total order.
// This implementation provides a lexicographic comparison
// (real part first, then imaginary) for sorting purposes only.
// It does NOT represent a mathematical ordering of complex numbers.

impl<T: Numeric> PartialOrd for Complex<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.re.partial_cmp(&other.re) {
            Some(Ordering::Equal) => self.im.partial_cmp(&other.im),
            result => result,
        }
    }
}

// ---- Neg ----

impl<T: Numeric + Signed> Neg for Complex<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.re, -self.im)
    }
}

// ---- Add ----

impl<T: Numeric> Add for Complex<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<T: Numeric> Add<T> for Complex<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        Self::new(self.re + rhs, self.im)
    }
}

impl<T: Numeric> AddAssign for Complex<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.re = self.re + rhs.re;
        self.im = self.im + rhs.im;
    }
}

impl<T: Numeric> AddAssign<T> for Complex<T> {
    fn add_assign(&mut self, rhs: T) {
        self.re = self.re + rhs;
    }
}

// ---- Sub ----

impl<T: Numeric> Sub for Complex<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl<T: Numeric> Sub<T> for Complex<T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        Self::new(self.re - rhs, self.im)
    }
}

impl<T: Numeric> SubAssign for Complex<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.re = self.re - rhs.re;
        self.im = self.im - rhs.im;
    }
}

impl<T: Numeric> SubAssign<T> for Complex<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.re = self.re - rhs;
    }
}

// ---- Mul ----

impl<T: Numeric> Mul for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        // (a+bi)(c+di) = (ac-bd) + (ad+bc)i
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl<T: Numeric> Mul<T> for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self::new(self.re * rhs, self.im * rhs)
    }
}

impl<T: Numeric> MulAssign for Complex<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: Numeric> MulAssign<T> for Complex<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.re = self.re * rhs;
        self.im = self.im * rhs;
    }
}

// ---- Div ----

impl<T: Numeric> Div for Complex<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        if denom.to_f64() == 0.0 {
            return Self::new(T::from_f64(f64::NAN), T::from_f64(f64::NAN));
        }
        Self::new(
            (self.re * rhs.re + self.im * rhs.im) / denom,
            (self.im * rhs.re - self.re * rhs.im) / denom,
        )
    }
}

impl<T: Numeric> Div<T> for Complex<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        if rhs.to_f64() == 0.0 {
            return Self::new(T::from_f64(f64::NAN), T::from_f64(f64::NAN));
        }
        Self::new(self.re / rhs, self.im / rhs)
    }
}

impl<T: Numeric> DivAssign for Complex<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T: Numeric> DivAssign<T> for Complex<T> {
    fn div_assign(&mut self, rhs: T) {
        self.re = self.re / rhs;
        self.im = self.im / rhs;
    }
}

// ---- Rem ----

impl<T: Numeric> Rem for Complex<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        Self::new(self.re % rhs.re, self.im % rhs.im)
    }
}

impl<T: Numeric> RemAssign for Complex<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.re = self.re % rhs.re;
        self.im = self.im % rhs.im;
    }
}



// ---- Numeric implementation ----

impl<T: Numeric + Signed> Numeric for Complex<T> {
    #[inline]
    fn to_f64(self) -> f64 {
        self.norm().to_f64()
    }

    #[inline]
    fn from_f64(v: f64) -> Self {
        Self::new(T::from_f64(v), T::from_f64(0.0))
    }

    fn ln(&self) -> Self {
        let n = self.norm();
        Self::new(n.ln(), self.arg())
    }

    fn log10(&self) -> Self {
        self.ln() / Self::new(T::from_f64(10.0_f64.ln()), T::from_f64(0.0))
    }

    fn exp(&self) -> Self {
        let ea = self.re.exp();
        Self::new(ea * self.im.cos(), ea * self.im.sin())
    }

    fn sqrt(&self) -> Self {
        let n = self.norm();
        let two = T::from_f64(2.0);
        let zero = T::from_f64(0.0);
        let re_part = ((n + self.re) / two).sqrt();
        let im_part = ((n - self.re) / two).sqrt();
        let im_sign = if self.im < zero { -T::from_f64(1.0) } else { T::from_f64(1.0) };
        Self::new(re_part, im_part * im_sign)
    }

    fn cbrt(&self) -> Self {
        let one_third = T::from_f64(1.0 / 3.0);
        (self.ln() * one_third).exp()
    }

    fn sin(&self) -> Self {
        Self::new(
            self.re.sin() * self.im.cosh(),
            self.re.cos() * self.im.sinh(),
        )
    }

    fn cos(&self) -> Self {
        Self::new(
            self.re.cos() * self.im.cosh(),
            -(self.re.sin() * self.im.sinh()),
        )
    }

    fn tan(&self) -> Self {
        self.sin() / self.cos()
    }

    fn asin(&self) -> Self {
        let i = Self::i();
        let one = Self::one();
        let iz = i * *self;
        let sqrt_term = (one - *self * *self).sqrt();
        -(iz + sqrt_term).ln() * i
    }

    fn acos(&self) -> Self {
        let i = Self::i();
        let one = Self::one();
        let sqrt_term = (*self * *self - one).sqrt();
        -(*self + sqrt_term).ln() * i
    }

    fn atan(&self) -> Self {
        let i = Self::i();
        let two = T::from_f64(2.0);
        let numerator = i + *self;
        let denominator = i - *self;
        (i / two) * (numerator / denominator).ln()
    }

    fn sinh(&self) -> Self {
        Self::new(
            self.re.sinh() * self.im.cos(),
            self.re.cosh() * self.im.sin(),
        )
    }

    fn cosh(&self) -> Self {
        Self::new(
            self.re.cosh() * self.im.cos(),
            self.re.sinh() * self.im.sin(),
        )
    }

    fn tanh(&self) -> Self {
        self.sinh() / self.cosh()
    }

    fn to_degrees(&self) -> Self {
        Self::new(self.re.to_degrees(), self.im.to_degrees())
    }

    fn to_radians(&self) -> Self {
        Self::new(self.re.to_radians(), self.im.to_radians())
    }

    fn ln_1p(&self) -> Self {
        (Self::one() + *self).ln()
    }

    fn exp_m1(&self) -> Self {
        self.exp() - Self::one()
    }

    fn recip(&self) -> Self {
        let norm_sq = self.norm_sqr();
        if norm_sq.to_f64() == 0.0 {
            return Self::new(T::from_f64(f64::NAN), T::from_f64(f64::NAN));
        }
        Self::new(self.re / norm_sq, -self.im / norm_sq)
    }

    fn round(&self) -> Self {
        Self::new(self.re.round(), self.im.round())
    }

    fn floor(&self) -> Self {
        Self::new(self.re.floor(), self.im.floor())
    }

    fn ceil(&self) -> Self {
        Self::new(self.re.ceil(), self.im.ceil())
    }

    fn fract(&self) -> Self {
        Self::new(self.re.fract(), self.im.fract())
    }

    fn abs(&self) -> Self {
        Self::new(self.norm(), T::from_f64(0.0))
    }

    fn sign(&self) -> Self {
        let magnitude = self.norm();
        let zero = T::from_f64(0.0);
        if magnitude == zero {
            Self::zero()
        } else {
            *self / Self::new(magnitude, zero)
        }
    }

    fn pow(&self, n: &Self) -> Self {
        let zero = Self::zero();
        if *self == zero {
            return zero;
        }
        (*n * self.ln()).exp()
    }

    fn powf(&self, n: f64) -> Self {
        let n_c = Self::new(T::from_f64(n), T::from_f64(0.0));
        self.pow(&n_c)
    }

    fn powi(&self, n: i32) -> Self {
        if n == 0 {
            return Self::one();
        }
        if n < 0 {
            return self.recip().powi(-n);
        }
        let mut result = Self::one();
        let mut base = *self;
        let mut exp = n;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base;
            }
            base = base * base;
            exp >>= 1;
        }
        result
    }

    fn log(&self, base: &Self) -> Self {
        self.ln() / base.ln()
    }

    fn atan2(&self, x: &Self) -> Self {
        Self::new(
            T::from_f64(self.re.to_f64().atan2(x.re.to_f64())),
            T::from_f64(self.im.to_f64().atan2(x.im.to_f64())),
        )
    }

    fn clamp(&self, min: &Self, max: &Self) -> Self {
        Self::new(
            T::from_f64(self.re.to_f64().clamp(min.re.to_f64(), max.re.to_f64())),
            T::from_f64(self.im.to_f64().clamp(min.im.to_f64(), max.im.to_f64())),
        )
    }

    fn lerp(&self, other: &Self, t: f64) -> Self {
        Self::new(
            self.re.lerp(&other.re, t),
            self.im.lerp(&other.im, t),
        )
    }

    fn map_range(&self, in_min: &Self, in_max: &Self, out_min: &Self, out_max: &Self) -> Self {
        Self::new(
            self.re.map_range(&in_min.re, &in_max.re, &out_min.re, &out_max.re),
            self.im.map_range(&in_min.im, &in_max.im, &out_min.im, &out_max.im),
        )
    }

    fn is_nan(self) -> bool {
        self.re.is_nan() || self.im.is_nan()
    }

    fn is_infinite(self) -> bool {
        self.re.is_infinite() || self.im.is_infinite()
    }

    fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }
}

// ---- NumberKind ----

impl<T: Numeric> NumberKind for Complex<T> {
    fn number_set() -> NumberSet {
        NumberSet::Complex
    }

    fn is_signed() -> bool {
        true
    }

    fn is_integer_valued() -> bool {
        false
    }

    fn is_float() -> bool {
        false
    }
}

// ---- Conversions ----

impl<T: Numeric> From<T> for Complex<T> {
    fn from(value: T) -> Self {
        Self::from_real(value)
    }
}

impl<T: Numeric> From<(T, T)> for Complex<T> {
    fn from(tuple: (T, T)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl<T: Numeric + Signed> From<Complex<T>> for Num<Complex<T>> {
    fn from(c: Complex<T>) -> Self {
        Num::new(c)
    }
}

impl<T: Numeric> From<Num<T>> for Complex<T> {
    fn from(n: Num<T>) -> Self {
        Self::from_real(n.value)
    }
}




