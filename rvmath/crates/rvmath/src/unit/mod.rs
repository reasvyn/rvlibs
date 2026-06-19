//! Type-safe dimensional analysis via unit-of-measurement types.
//!
//! This module provides a system for tracking physical dimensions at the type level,
//! preventing unit mismatches at compile time while supporting arithmetic, conversion,
//! and power (exponent) tracking at runtime.
//!
//! # Core Concepts
//!
//! | Concept | Trait/Type | Role |
//! |---------|-----------|------|
//! | **Family** | [`Dimension`](meta::Dimension) | Trait representing a physical dimension (e.g., `Length`, `Mass`). Families group compatible units. |
//! | **Unit** | [`Meta`](meta::Meta) | Trait associating a unit family with its base conversion factor and symbol. |
//! | **Value** | [`Unit<N, T>`] | Generic struct holding a numeric value, a dimensional power (exponent), and a [`PhantomData`] marker for `T: Meta`. |
//!
//! # Declaring Units
//!
//! Use the [`declare_family!`](macros) macro to define a new dimension family and the
//! [`declare_units!`](macros) macro to define concrete unit types within that family:
//!
//! ```ignore
//! declare_family!(Length);
//! declare_units! {
//!     family: Length,
//!     Meter: (1.0, "m"),
//!     Kilometer: (1000.0, "km"),
//! }
//! ```
//!
//! # Unit Conversion
//!
//! Convert between units of the same family with [`convert`](Unit::convert) or
//! [`convert_to`](Unit::convert_to). The conversion factor is raised to the dimensional
//! power, so converting `km² → m²` squares the factor automatically.
//!
//! # Dimensional Arithmetic
//!
//! - **`Mul`**: Powers add (`m * m → m²`).
//! - **`Div`**: Powers subtract (`m² / m → m`).
//! - **`Add` / `Sub`**: Panic at runtime if powers differ (mismatched dimensions).
//!
//! # Power & Floating-Point
//!
//! The `power` field tracks the dimensional exponent as an `N: Numeric` value. Integer
//! powers (1, 2, 3, …) arise from `mul`/`div`/`pow` and give exact conversion. Fractional
//! powers (e.g., `0.5` from `sqrt`) introduce additional floating-point rounding via
//! [`powf`] during conversion.
//!
//! # Example
//!
//! ```ignore
//! use rvmath::unit::prelude::*;
//!
//! let dist = Kilometer::new(1.0);          // 1 km
//! let dist_m = dist.convert::<Meter>();    // 1000 m
//! let area = dist * dist;                  // 1 km²
//! let area_m2 = area.convert::<Meter>();   // 1_000_000 m²
//! ```
pub mod macros;
pub mod meta;

use crate::num::Numeric;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result},
    marker::PhantomData,
};

pub use meta::*;

/// A unit of measurement that wraps a numeric value and tracks its physical dimension (power).
///
/// # Type Parameters
///
/// - `N`: The underlying numeric type (e.g., `f64`, `i32`). Must implement [`Numeric`].
/// - `T`: The metadata defining the unit family (e.g., `Meter`, `Pixel`). Must implement [`Meta`].
///
/// # The `power` Field — Dimensional Exponent
///
/// The `power` field represents the **dimensional exponent** of the unit. It encodes how many
/// times a base dimension has been applied:
///
/// | `power` value | Physical meaning          | Example         |
/// |---------------|---------------------------|-----------------|
/// | `1.0`         | Linear quantity (length)  | `5 m`           |
/// | `2.0`         | Area                      | `25 m²`         |
/// | `3.0`         | Volume                    | `125 m³`        |
/// | `0.5`         | Square root of a quantity | `2.236 m^0.5`   |
///
/// Integer powers arise naturally from multiplication (`m * m → m²`) and from calls to
/// [`Unit::pow`]. Fractional powers can appear after root operations (e.g., `sqrt(area)`) and
/// are supported by this type, but they come with important caveats — see *Floating-Point
/// Behavior* below.
///
/// # Unit Conversion and Floating-Point Behavior
///
/// When converting between units in the same family (e.g., `Kilometer → Meter`), the
/// conversion applies `factor_ratio.powf(power)` to the stored value:
///
///
/// This means:
/// - For **integer powers**, conversion is exact (within normal `f64` precision).
/// - For **fractional powers** (e.g., `power = 1.0/3.0` from a cube-root operation), the
///   `powf` call introduces additional floating-point rounding. Results may differ slightly
///   from the mathematically exact value.
/// - `Display` output for non-integer powers will show the raw `f64` representation, which
///   can look like `m^0.30000000000000004` due to binary floating-point representation.
///
/// # Best Practices
///
/// - **Prefer integer powers** when possible (area, volume, etc.) for predictable behavior and
///   clean display output.
/// - **Be aware of precision** when using fractional powers: if you need high accuracy, verify
///   the result numerically rather than relying on exact equality comparisons.
/// - Use [`Unit::pow`] to raise a unit to a power, which updates `power` correctly.
/// - Use [`Unit::convert_to`] / [`Unit::convert`] for unit conversion.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Unit<N: Numeric, T: Meta> {
    /// The numeric value of the unit.
    pub value: N,
    /// The dimension power (e.g., 1 for length, 2 for area, -1 for reciprocal).
    pub power: N,
    pub _marker: PhantomData<T>,
}

impl<N: Numeric, T: Meta> Unit<N, T> {
    /// Creates a new unit with a default power of 1.0.
    pub fn new(value: N) -> Self {
        Self {
            value,
            power: N::from_f64(1.0),
            _marker: PhantomData,
        }
    }

    /// Creates a new unit with a specific value and dimensional power.
    pub fn with_power(value: N, power: N) -> Self {
        Self {
            value,
            power,
            _marker: PhantomData,
        }
    }

    /// Returns the raw numeric value.
    pub fn value(&self) -> N {
        self.value
    }

    /// Returns the dimensional power (exponent) of the unit.
    pub fn power(&self) -> N {
        self.power
    }

    /// Returns the international symbol for the unit, including power notation if necessary.
    pub fn symbol(&self) -> Cow<'static, str> {
        let symbol = T::SYMBOL;
        let p_f64 = self.power.to_f64();

        if (p_f64 - 1.0).abs() < f64::EPSILON {
            Cow::Borrowed(symbol)
        } else if (p_f64).abs() < f64::EPSILON {
            Cow::Borrowed("")
        } else {
            Cow::Owned(format!("{}^{}", symbol, self.power))
        }
    }

    /// Returns the conversion factor to the base unit of this family.
    pub fn factor_to_base(&self) -> f64 {
        T::FACTOR
    }

    /// Converts this unit to another unit within the same physical family.
    pub fn convert_to<U: Meta<Family = T::Family>>(&self) -> Unit<N, U> {
        let factor_ratio = T::FACTOR / U::FACTOR;
        // Adjust value based on the ratio raised to the current power
        let converted_value = self.value * N::from_f64(factor_ratio.powf(self.power.to_f64()));

        Unit::<N, U> {
            value: converted_value,
            power: self.power,
            _marker: PhantomData,
        }
    }

    /// Alias for convert_to with type inference.
    pub fn convert<U>(&self) -> Unit<N, U>
    where
        U: Meta<Family = T::Family>,
    {
        self.convert_to::<U>()
    }
}

impl<N: Numeric, T: Meta> Default for Unit<N, T> {
    fn default() -> Self {
        Self::new(N::from_f64(0.0))
    }
}

// --- Display & Conversions ---

impl<N: Numeric, T: Meta> Display for Unit<N, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let p_f64 = self.power.to_f64();
        if p_f64 == 0.0 {
            write!(f, "{}", self.value)
        } else if (p_f64 - 1.0).abs() < f64::EPSILON {
            write!(f, "{}{}", self.value, T::SYMBOL)
        } else {
            write!(f, "{}{}^{}", self.value, T::SYMBOL, self.power)
        }
    }
}

impl<N: Numeric, T: Meta> From<f64> for Unit<N, T> {
    fn from(v: f64) -> Self {
        Self::new(N::from_f64(v))
    }
}

impl<N: Numeric, T: Meta> From<Unit<N, T>> for f64 {
    fn from(v: Unit<N, T>) -> f64 {
        v.value.to_f64()
    }
}

// ── Operator Impls ─────────────────────────────────────────

use std::cmp::Ordering;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

impl<N: Numeric, T: Meta> Unit<N, T> {
    /// Raises this unit to the given power.
    pub fn pow<P: Into<f64> + Copy>(self, n: P) -> Self {
        let p = n.into();
        Self {
            value: self.value.powf(p),
            power: self.power * N::from_f64(p),
            _marker: PhantomData,
        }
    }

    /// Returns the square root of this unit.
    pub fn sqrt(self) -> Self { self.pow(0.5) }
    /// Returns the cube root of this unit.
    pub fn cbrt(self) -> Self { self.pow(1.0 / 3.0) }
    /// Returns the nth root of this unit.
    pub fn root<P: Into<f64> + Copy>(self, n: P) -> Self { self.pow(1.0 / n.into()) }
    /// Returns the inverse of this unit.
    pub fn inv(self) -> Self { self.pow(-1.0) }

    pub fn round(self) -> Self { Self::with_power(self.value.round(), self.power) }
    pub fn floor(self) -> Self { Self::with_power(self.value.floor(), self.power) }
    pub fn ceil(self) -> Self { Self::with_power(self.value.ceil(), self.power) }
    pub fn fract(self) -> Self { Self::with_power(self.value.fract(), self.power) }
    pub fn abs(self) -> Self { Self::with_power(self.value.abs(), self.power) }

    pub fn signum(self) -> N where N: crate::num::Signed { self.value.sign() }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        if (self.power.to_f64() - min.power.to_f64()).abs() > f64::EPSILON
            || (self.power.to_f64() - max.power.to_f64()).abs() > f64::EPSILON
        { panic!("Dimensional Error: Clamp requires matching powers"); }
        Self::with_power(self.value.clamp(&min.value, &max.value), self.power)
    }

    pub fn min(self, other: Self) -> Self {
        if (self.power.to_f64() - other.power.to_f64()).abs() > f64::EPSILON
        { panic!("Dimensional Error: Min requires matching powers"); }
        Self::with_power(self.value.min(&other.value), self.power)
    }

    pub fn max(self, other: Self) -> Self {
        if (self.power.to_f64() - other.power.to_f64()).abs() > f64::EPSILON
        { panic!("Dimensional Error: Max requires matching powers"); }
        Self::with_power(self.value.max(&other.value), self.power)
    }

    pub fn lerp(self, other: Self, t: f64) -> Self {
        if (self.power.to_f64() - other.power.to_f64()).abs() > f64::EPSILON
        { panic!("Dimensional Error: Lerp requires matching powers"); }
        Self::with_power(self.value.lerp(&other.value, t), self.power)
    }

    fn ensure_dimensionless(&self, op_name: &str) {
        if self.power.to_f64().abs() > f64::EPSILON {
            panic!("Math Error: Operation '{}' is only valid for dimensionless units (current power: {})", op_name, self.power);
        }
    }

    pub fn ln(self) -> N { self.ensure_dimensionless("ln"); self.value.ln() }
    pub fn log10(self) -> N { self.ensure_dimensionless("log10"); self.value.log(&N::from_f64(10.0)) }
    pub fn log(self, base: N) -> N { self.ensure_dimensionless("log"); self.value.log(&base) }
    pub fn exp(self) -> N { self.ensure_dimensionless("exp"); self.value.exp() }
    pub fn sin(self) -> N { self.ensure_dimensionless("sin"); self.value.sin() }
    pub fn cos(self) -> N { self.ensure_dimensionless("cos"); self.value.cos() }
    pub fn tan(self) -> N { self.ensure_dimensionless("tan"); self.value.tan() }

    pub fn atan2(self, other: Self) -> N {
        if (self.power.to_f64() - other.power.to_f64()).abs() > f64::EPSILON
        { panic!("Dimensional Error: atan2 requires matching powers"); }
        self.value.atan2(&other.value)
    }

    pub fn to_db_power(self) -> N {
        self.ensure_dimensionless("to_db_power");
        N::from_f64(10.0) * self.value.log(&N::from_f64(10.0))
    }

    pub fn to_db_amplitude(self) -> N {
        self.ensure_dimensionless("to_db_amplitude");
        N::from_f64(20.0) * self.value.log(&N::from_f64(10.0))
    }
}

impl<N: Numeric + Neg<Output = N>, T: Meta> Neg for Unit<N, T> {
    type Output = Self;
    fn neg(self) -> Self { Self::with_power(-self.value, self.power) }
}

impl<N: Numeric, T: Meta> Add for Unit<N, T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        if (self.power.to_f64() - rhs.power.to_f64()).abs() > f64::EPSILON
        { panic!("Dimensional Error: Cannot add units with different powers"); }
        Self::with_power(self.value + rhs.value, self.power)
    }
}

impl<N: Numeric, T: Meta> AddAssign for Unit<N, T> {
    fn add_assign(&mut self, rhs: Self) {
        if (self.power.to_f64() - rhs.power.to_f64()).abs() > f64::EPSILON
        { panic!("Dimensional Error: Cannot add_assign units with different powers"); }
        self.value += rhs.value;
    }
}

impl<N: Numeric, T: Meta> Sub for Unit<N, T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        if (self.power.to_f64() - rhs.power.to_f64()).abs() > f64::EPSILON
        { panic!("Dimensional Error: Cannot subtract units with different powers"); }
        Self::with_power(self.value - rhs.value, self.power)
    }
}

impl<N: Numeric, T: Meta> SubAssign for Unit<N, T> {
    fn sub_assign(&mut self, rhs: Self) {
        if (self.power.to_f64() - rhs.power.to_f64()).abs() > f64::EPSILON
        { panic!("Dimensional Error: Cannot sub_assign units with different powers"); }
        self.value -= rhs.value;
    }
}

impl<N: Numeric, T: Meta> RemAssign for Unit<N, T> {
    fn rem_assign(&mut self, rhs: Self) {
        if (self.power.to_f64() - rhs.power.to_f64()).abs() > f64::EPSILON
        { panic!("Dimensional Error: Remainder requires matching powers"); }
        self.value %= rhs.value;
    }
}

impl<N: Numeric, T: Meta> Mul<Unit<N, T>> for Unit<N, T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::with_power(self.value * rhs.value, self.power + rhs.power)
    }
}

impl<N: Numeric, T: Meta> Mul<N> for Unit<N, T> {
    type Output = Self;
    fn mul(self, rhs: N) -> Self { Self::with_power(self.value * rhs, self.power) }
}

impl<N: Numeric, T: Meta> MulAssign<N> for Unit<N, T> {
    fn mul_assign(&mut self, rhs: N) { self.value *= rhs; }
}

impl<N: Numeric, T: Meta> Div<Unit<N, T>> for Unit<N, T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self::with_power(self.value / rhs.value, self.power - rhs.power)
    }
}

impl<N: Numeric, T: Meta> Div<N> for Unit<N, T> {
    type Output = Self;
    fn div(self, rhs: N) -> Self { Self::with_power(self.value / rhs, self.power) }
}

impl<N: Numeric, T: Meta> DivAssign<N> for Unit<N, T> {
    fn div_assign(&mut self, rhs: N) { self.value /= rhs; }
}

impl<N: Numeric, T: Meta> Rem for Unit<N, T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        if (self.power.to_f64() - rhs.power.to_f64()).abs() > f64::EPSILON
        { panic!("Dimensional Error: Remainder requires matching powers"); }
        Self::with_power(self.value % rhs.value, self.power)
    }
}

impl<N: Numeric, T: Meta> PartialEq for Unit<N, T> {
    fn eq(&self, other: &Self) -> bool {
        (self.power.to_f64() - other.power.to_f64()).abs() < f64::EPSILON
            && (self.value.to_f64() - other.value.to_f64()).abs() < f64::EPSILON
    }
}

impl<N: Numeric, T: Meta> PartialEq<N> for Unit<N, T> {
    fn eq(&self, other: &N) -> bool {
        self.power.to_f64().abs() < f64::EPSILON
            && (self.value.to_f64() - other.to_f64()).abs() < f64::EPSILON
    }
}

impl<N: Numeric, T: Meta> PartialOrd for Unit<N, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.power.to_f64() - other.power.to_f64()).abs() > f64::EPSILON { return None; }
        self.value.partial_cmp(&other.value)
    }
}
