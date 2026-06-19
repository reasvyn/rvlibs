//! Generic N-dimensional vector types with unit-aware arithmetic.
//!
//! Core type is [`VecN<T, N>`] — a fixed-size vector backed by `[T; N]`.
//! Common aliases: [`Vec2`], [`Vec3`], [`Vec4`] for 2D, 3D, and 4D vectors.
//!
//! # Operations
//!
//! - **Dot product** — [`VecN::dot`] returns a scalar; [`VecN::dot_units`] for unit-aware vectors.
//! - **Length & normalize** — [`VecN::length`], [`VecN::normalize`]; unit variants [`VecN::length_units`], [`VecN::normalize_units`].
//! - **Element-wise** — via standard `+`, `-`, `*`, `/` operators.
//! - **Indexing** — [`Index`]`<usize>` and [`IndexMut`]`<usize>` for direct component access.
//! - **Serialization** — `Serialize`/`Deserialize` behind the `serde` feature gate.
//!
//! # Example
//!
//! ```rust
//! use rvmath::Vec3;
//! let v = Vec3::new([1.0, 2.0, 3.0]);
//! let w = Vec3::new([4.0, 5.0, 6.0]);
//! assert_eq!(v.dot(w), 32.0);
//! ```
#![allow(clippy::needless_range_loop)]

use crate::num::Numeric;
use crate::unit::{Unit, meta::Meta};
#[cfg(feature = "serde")]
use serde::de::{Error, SeqAccess, Visitor};
#[cfg(feature = "serde")]
use serde::ser::SerializeTuple;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

// ── VecN Struct ──────────────────────────────────────────────

/// A generic N-dimensional vector.
///
/// Backed by `[T; N_DIM]` on the stack. Works with raw numeric types
/// and [`Unit<N, U>`](crate::Unit) for dimensionally-aware calculations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VecN<T, const N_DIM: usize> {
    /// Internal array of components.
    pub data: [T; N_DIM],
}

#[cfg(feature = "serde")]
impl<T: Serialize, const N_DIM: usize> Serialize for VecN<T, N_DIM> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(N_DIM)?;
        for element in &self.data {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de> + Copy + Default, const N_DIM: usize> Deserialize<'de>
    for VecN<T, N_DIM>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VecVisitor<T, const N: usize> {
            _marker: std::marker::PhantomData<T>,
        }

        impl<'de, T: Deserialize<'de> + Copy + Default, const N: usize> Visitor<'de>
            for VecVisitor<T, N>
        {
            type Value = VecN<T, N>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a sequence of {} elements", N)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut data = [T::default(); N];
                for i in 0..N {
                    data[i] = seq
                        .next_element()?
                        .ok_or_else(|| Error::invalid_length(i, &self))?;
                }
                Ok(VecN::new(data))
            }
        }

        deserializer.deserialize_tuple(
            N_DIM,
            VecVisitor {
                _marker: std::marker::PhantomData,
            },
        )
    }
}

impl<T, const N_DIM: usize> VecN<T, N_DIM> {
    /// Creates a new vector from an array of components.
    pub fn new(data: [T; N_DIM]) -> Self {
        Self { data }
    }

    /// Returns the internal array of components.
    pub fn components(self) -> [T; N_DIM] {
        self.data
    }
}

// ── Numeric Impls ──────────────────────────────────────────

impl<T: Numeric, const N_DIM: usize> VecN<T, N_DIM> {
    /// Returns a zero vector (all components set to `T::from_f64(0.0)`).
    pub fn zero() -> Self {
        Self {
            data: std::array::from_fn(|_| T::from_f64(0.0)),
        }
    }

    /// Dot product of two vectors.
    pub fn dot(self, other: Self) -> T {
        let mut sum = T::from_f64(0.0);
        for i in 0..N_DIM {
            sum += self.data[i] * other.data[i];
        }
        sum
    }

    /// Euclidean length.
    pub fn length(self) -> T {
        self.dot(self).sqrt()
    }

    /// Euclidean distance between two vectors.
    pub fn distance(self, other: Self) -> T {
        let mut sum_sq = T::from_f64(0.0);
        for i in 0..N_DIM {
            let diff = self.data[i] - other.data[i];
            sum_sq += diff * diff;
        }
        sum_sq.sqrt()
    }

    /// Normalizes to unit length. Returns self if length is zero.
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len.to_f64() == 0.0 {
            self
        } else {
            let mut data = self.data;
            for i in 0..N_DIM {
                data[i] /= len;
            }
            Self::new(data)
        }
    }
}

// ── Unit-aware Impls ───────────────────────────────────────

impl<N: Numeric, U: Meta + Copy, const N_DIM: usize> VecN<Unit<N, U>, N_DIM> {
    /// Returns a zero vector with units.
    pub fn zero_units() -> Self {
        Self {
            data: std::array::from_fn(|_| Unit::new(N::from_f64(0.0))),
        }
    }

    /// Dot product with unit tracking.
    pub fn dot_units(self, other: Self) -> Unit<N, U> {
        let mut sum_val = N::from_f64(0.0);
        let res_power = self.data[0].power + other.data[0].power;
        for i in 0..N_DIM {
            sum_val += self.data[i].value * other.data[i].value;
        }
        Unit::with_power(sum_val, res_power)
    }

    /// Magnitude with unit tracking.
    pub fn length_units(self) -> Unit<N, U> {
        self.dot_units(self).sqrt()
    }

    /// Normalizes a unit vector.
    pub fn normalize_units(self) -> Self {
        let len = self.length_units();
        if len.value.to_f64() == 0.0 {
            self
        } else {
            let mut data = self.data;
            for i in 0..N_DIM {
                data[i].value /= len.value;
            }
            Self::new(data)
        }
    }

    /// Extracts raw numeric values, discarding units.
    pub fn values(self) -> [N; N_DIM] {
        std::array::from_fn(|i| self.data[i].value)
    }
}

// ── Arithmetic Operators ───────────────────────────────────

impl<T: Add<Output = T> + Copy, const N_DIM: usize> Add for VecN<T, N_DIM> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(std::array::from_fn(|i| self.data[i] + rhs.data[i]))
    }
}

impl<T: Sub<Output = T> + Copy, const N_DIM: usize> Sub for VecN<T, N_DIM> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(std::array::from_fn(|i| self.data[i] - rhs.data[i]))
    }
}

impl<T: Mul<S, Output = T> + Copy, S: Copy, const N_DIM: usize> Mul<S> for VecN<T, N_DIM> {
    type Output = Self;
    fn mul(self, rhs: S) -> Self {
        Self::new(std::array::from_fn(|i| self.data[i] * rhs))
    }
}

impl<T: Div<S, Output = T> + Copy, S: Copy, const N_DIM: usize> Div<S> for VecN<T, N_DIM> {
    type Output = Self;
    fn div(self, rhs: S) -> Self {
        Self::new(std::array::from_fn(|i| self.data[i] / rhs))
    }
}

// ── Indexing ───────────────────────────────────────────────

impl<T, const N_DIM: usize> Index<usize> for VecN<T, N_DIM> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N_DIM: usize> IndexMut<usize> for VecN<T, N_DIM> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// ── Aliases (Vec2, Vec3, Vec4) ─────────────────────────────

macro_rules! impl_vec_alias {
    ($($vis:vis $name:ident [$n:expr] ($($idx:tt : $comp_name:ident),*);)*) => {
        $(
            $vis type $name<T> = VecN<T, $n>;

            impl<T: Copy> $name<T> {
                pub fn splat(val: T) -> Self {
                    Self::new([ $( { let _ = stringify!($idx); val } ),* ])
                }

                pub fn new_coords($( $comp_name : T ),*) -> Self {
                    Self::new([ $( $comp_name ),* ])
                }

                $(
                    pub fn $comp_name(&self) -> T {
                        self.data[$idx]
                    }
                )*
            }

            impl<T: Copy> From<($(repeat_type!($idx, T)),*)> for $name<T> {
                fn from(tuple: ($(repeat_type!($idx, T)),*)) -> Self {
                    Self::new([ $( tuple.$idx ),* ])
                }
            }
        )*
    };
}

macro_rules! repeat_type {
    ($i:tt, $t:ty) => { $t };
}

impl_vec_alias! {
    pub Vec2 [2] (0: x, 1: y);
    pub Vec3 [3] (0: x, 1: y, 2: z);
    pub Vec4 [4] (0: x, 1: y, 2: z, 3: w);
}

// ── Default ────────────────────────────────────────────────

impl<T: Default + Copy, const N_DIM: usize> Default for VecN<T, N_DIM> {
    fn default() -> Self {
        Self {
            data: [T::default(); N_DIM],
        }
    }
}

// ── Conversions ────────────────────────────────────────────

impl<T, const N_DIM: usize> From<[T; N_DIM]> for VecN<T, N_DIM> {
    fn from(data: [T; N_DIM]) -> Self {
        Self { data }
    }
}

impl<T: Copy, const N_DIM: usize> From<VecN<T, N_DIM>> for Vec<T> {
    fn from(vector: VecN<T, N_DIM>) -> Self {
        vector.data.to_vec()
    }
}

impl<T: Copy + Default, const N_DIM: usize> TryFrom<Vec<T>> for VecN<T, N_DIM> {
    type Error = String;

    fn try_from(dynamic_vec: Vec<T>) -> Result<Self, Self::Error> {
        if dynamic_vec.len() != N_DIM {
            return Err(format!(
                "Dimension mismatch: expected {} elements, found {}",
                N_DIM,
                dynamic_vec.len()
            ));
        }
        let mut data = [T::default(); N_DIM];
        data[..N_DIM].copy_from_slice(&dynamic_vec[..N_DIM]);
        Ok(Self::new(data))
    }
}
