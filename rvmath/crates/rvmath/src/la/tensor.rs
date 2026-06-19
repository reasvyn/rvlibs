//! N-dimensional tensor data structure and operations.
//!
//! [`Tensor<T>`] uses dynamic `shape`/`strides` with flat `Vec<T>` storage.
//!
//! # Construction
//!
//! - [`Tensor::new`] — fill with `T::default()`.
//! - [`Tensor::from_data`] — from existing `Vec<T>` + shape (validates size).
//! - [`Tensor::zero`] / [`Tensor::ones`] — for [`Numeric`] types.
//!
//! # Indexing & Reshape
//!
//! - [`get`](Tensor::get), [`get_mut`](Tensor::get_mut), [`try_get`](Tensor::try_get) — multi-dimensional access.
//! - [`reshape`](Tensor::reshape) — changes shape (same total element count).
//! - [`Index`]`<&[usize]>` and [`IndexMut`]`<&[usize]>` for bracket syntax.
//!
//! # Operations
//!
//! - **Element-wise** — `Add`, `Sub`, `Mul`, `Div`, `Rem` (tensor–tensor and tensor–scalar).
//! - **Unit-aware** — [`Tensor::mul_units`], [`Tensor::mul_scalar_units`], [`Tensor::zero_units`].
//! - **Comparison** — [`PartialOrd`] via lexicographic element comparison.
//!
//! # Example
//!
//! ```rust
//! use rvmath::Tensor;
//! let t = Tensor::<f64>::new(vec![2, 3]);
//! assert_eq!(&t.shape[..], &[2, 3]);
//! ```
#![allow(clippy::needless_range_loop)]

use crate::num::Numeric;
use crate::unit::{Unit, meta::Meta};
use std::cmp::Ordering;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub,
    SubAssign,
};

/// A generic n-dimensional Tensor.
///
/// Stores data in a flat `Vec<T>` and manages indexing through `shape` and `strides`.
#[derive(Debug, Clone, PartialEq)]
pub struct Tensor<T> {
    /// Flat container for the tensor data.
    pub data: Vec<T>,
    /// Dimensions of the tensor.
    pub shape: Vec<usize>,
    /// Strides for each dimension to map n-dimensional indices to flat index.
    pub strides: Vec<usize>,
}

impl<T: Default + Copy> Tensor<T> {
    /// Creates a new tensor with a given shape, initialized with default values.
    pub fn new(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        let strides = compute_strides(&shape);
        Self {
            data: vec![T::default(); size],
            shape,
            strides,
        }
    }
}

impl<T: Copy> Tensor<T> {
    /// Creates a new tensor from raw data and a shape.
    pub fn from_data(data: Vec<T>, shape: Vec<usize>) -> Result<Self, String> {
        let size: usize = shape.iter().product();
        if data.len() != size {
            return Err(format!(
                "Data size {} does not match shape {:?} (expected {})",
                data.len(),
                shape,
                size
            ));
        }
        let strides = compute_strides(&shape);
        Ok(Self {
            data,
            shape,
            strides,
        })
    }

    /// Returns the rank (number of dimensions) of the tensor.
    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    /// Maps an n-dimensional index to a flat index.
    pub fn get_index(&self, indices: &[usize]) -> usize {
        assert_eq!(indices.len(), self.shape.len(), "Index rank mismatch");
        let mut flat_idx = 0;
        for (i, &idx) in indices.iter().enumerate() {
            assert!(
                idx < self.shape[i],
                "Index out of bounds at dimension {}",
                i
            );
            flat_idx += idx * self.strides[i];
        }
        flat_idx
    }

    /// Maps an n-dimensional index to a flat index, returning `None` on invalid indices.
    pub fn try_get_index(&self, indices: &[usize]) -> Option<usize> {
        if indices.len() != self.shape.len() {
            return None;
        }
        let mut flat_idx = 0;
        for (i, &idx) in indices.iter().enumerate() {
            if idx >= self.shape[i] {
                return None;
            }
            flat_idx += idx * self.strides[i];
        }
        Some(flat_idx)
    }

    /// Gets a reference to an element at the specified n-dimensional index.
    pub fn get(&self, indices: &[usize]) -> &T {
        let idx = self.get_index(indices);
        &self.data[idx]
    }

    /// Gets a reference to an element at the specified n-dimensional index.
    /// Returns `None` if indices are out of bounds.
    pub fn try_get(&self, indices: &[usize]) -> Option<&T> {
        let idx = self.try_get_index(indices)?;
        Some(&self.data[idx])
    }

    /// Gets a mutable reference to an element at the specified n-dimensional index.
    pub fn get_mut(&mut self, indices: &[usize]) -> &mut T {
        let idx = self.get_index(indices);
        &mut self.data[idx]
    }

    /// Gets a mutable reference to an element at the specified n-dimensional index.
    /// Returns `None` if indices are out of bounds.
    pub fn try_get_mut(&mut self, indices: &[usize]) -> Option<&mut T> {
        let idx = self.try_get_index(indices)?;
        Some(&mut self.data[idx])
    }

    /// Reshapes the tensor to a new shape.
    pub fn reshape(mut self, new_shape: Vec<usize>) -> Result<Self, String> {
        let size: usize = new_shape.iter().product();
        if self.data.len() != size {
            return Err(format!(
                "Cannot reshape: new shape {:?} size {} != current size {}",
                new_shape,
                size,
                self.data.len()
            ));
        }
        self.strides = compute_strides(&new_shape);
        self.shape = new_shape;
        Ok(self)
    }
}

impl<T: Numeric> Tensor<T> {
    /// Creates a tensor of a given shape filled with zeros.
    pub fn zero(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        let strides = compute_strides(&shape);
        Self {
            data: vec![T::from_f64(0.0); size],
            shape,
            strides,
        }
    }

    /// Creates a tensor of a given shape filled with ones.
    pub fn ones(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        let strides = compute_strides(&shape);
        Self {
            data: vec![T::from_f64(1.0); size],
            shape,
            strides,
        }
    }
}

// ── Unit-aware ────────────────────────────────────────────

impl<N: Numeric, U: Meta + Copy> Tensor<Unit<N, U>> {
    /// Creates a tensor where all unit values are zero.
    pub fn zero_units(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        let strides = compute_strides(&shape);
        Self {
            data: vec![Unit::new(N::from_f64(0.0)); size],
            shape,
            strides,
        }
    }

    /// Multiplies this tensor with another tensor (element-wise with units).
    pub fn mul_units(self, other: Self) -> Self {
        assert_eq!(
            self.shape, other.shape,
            "Shape mismatch in tensor mul_units"
        );
        let res_power = self.data[0].power + other.data[0].power;
        let mut data = self.data;
        for i in 0..data.len() {
            data[i] = Unit::with_power(data[i].value * other.data[i].value, res_power);
        }
        Self::from_data(data, self.shape).unwrap()
    }

    /// Multiplies this tensor with a scalar unit.
    pub fn mul_scalar_units(mut self, scalar: Unit<N, U>) -> Self {
        let res_power = self.data[0].power + scalar.power;
        for i in 0..self.data.len() {
            self.data[i] = Unit::with_power(self.data[i].value * scalar.value, res_power);
        }
        self
    }
}

impl<T> Default for Tensor<T> {
    fn default() -> Self {
        Self {
            data: vec![],
            shape: vec![],
            strides: vec![],
        }
    }
}

impl<T: Copy> Index<&[usize]> for Tensor<T> {
    type Output = T;

    fn index(&self, indices: &[usize]) -> &Self::Output {
        self.get(indices)
    }
}

impl<T: Copy> IndexMut<&[usize]> for Tensor<T> {
    fn index_mut(&mut self, indices: &[usize]) -> &mut Self::Output {
        self.get_mut(indices)
    }
}

fn compute_strides(shape: &[usize]) -> Vec<usize> {
    let mut strides = vec![1; shape.len()];
    for i in (0..shape.len() - 1).rev() {
        strides[i] = strides[i + 1] * shape[i + 1];
    }
    strides
}

// ── Arithmetic Operations ──────────────────────────────────

macro_rules! impl_tensor_op {
    ($trait:ident, $method:ident, $assign_trait:ident, $assign_method:ident, $op:tt) => {
        impl<T: Numeric> $trait for Tensor<T> {
            type Output = Self;

            fn $method(mut self, other: Self) -> Self {
                assert_eq!(self.shape, other.shape, "Shape mismatch in tensor operation");
                for i in 0..self.data.len() {
                    self.data[i] $op other.data[i];
                }
                self
            }
        }

        impl<T: Numeric> $assign_trait for Tensor<T> {
            fn $assign_method(&mut self, other: Self) {
                assert_eq!(self.shape, other.shape, "Shape mismatch in tensor operation");
                for i in 0..self.data.len() {
                    self.data[i] $op other.data[i];
                }
            }
        }
    };
}

macro_rules! impl_tensor_scalar_op {
    ($trait:ident, $method:ident, $assign_trait:ident, $assign_method:ident, $op:tt) => {
        impl<T: Numeric> $trait<T> for Tensor<T> {
            type Output = Self;

            fn $method(mut self, scalar: T) -> Self {
                for i in 0..self.data.len() {
                    self.data[i] $op scalar;
                }
                self
            }
        }

        impl<T: Numeric> $assign_trait<T> for Tensor<T> {
            fn $assign_method(&mut self, scalar: T) {
                for i in 0..self.data.len() {
                    self.data[i] $op scalar;
                }
            }
        }
    };
}

impl_tensor_op!(Add, add, AddAssign, add_assign, +=);
impl_tensor_op!(Sub, sub, SubAssign, sub_assign, -=);
impl_tensor_op!(Mul, mul, MulAssign, mul_assign, *=);
impl_tensor_op!(Div, div, DivAssign, div_assign, /=);
impl_tensor_op!(Rem, rem, RemAssign, rem_assign, %=);

impl_tensor_scalar_op!(Mul, mul, MulAssign, mul_assign, *=);
impl_tensor_scalar_op!(Div, div, DivAssign, div_assign, /=);
impl_tensor_scalar_op!(Rem, rem, RemAssign, rem_assign, %=);

// ── Comparison ─────────────────────────────────────────────

impl<T: Numeric> PartialOrd for Tensor<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.shape != other.shape {
            return None;
        }
        self.data.partial_cmp(&other.data)
    }
}
