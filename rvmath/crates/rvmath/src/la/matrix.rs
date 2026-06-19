//! Generic M×N matrix types with unit-aware arithmetic.
//!
//! Core type is [`MatN<T, ROWS, COLS>`] — backed by `[[T; COLS]; ROWS]`.
//! Common aliases: [`Mat2x2`], [`Mat3x3`], [`Mat4x4`].
//!
//! # Operations
//!
//! - **Element-wise** — via standard `*` operator (Hadamard product).
//! - **Matrix multiply** — [`MatN::mul_mat`] for general M×N × N×P multiplication.
//! - **Transpose** — [`MatN::transpose`] swaps rows and columns.
//! - **Determinant** — [`det2`](MatN::det2) for 2×2, [`det3`](MatN::det3) for 3×3.
//! - **Inverse** — [`MatN::inv2`] returns `Option<Mat2x2>`.
//! - **Matrix–vector** — `MatN * VecN` via [`Mul`] (unit-aware variant included).
//! - **Unit-aware** — [`MatN::mul_mat_units`], [`MatN::zero_units`], and unit-aware `Mul<VecN>`.
//!
//! # Example
//!
//! ```rust
//! use rvmath::Mat2x2;
//! let a = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
//! let b = Mat2x2::new([[0.0, 1.0], [1.0, 0.0]]);
//! let c = a.mul_mat(b);
//! assert_eq!(c.components(), [[2.0, 1.0], [4.0, 3.0]]);
//! ```
#![allow(clippy::needless_range_loop)]

use crate::la::vector::VecN;
use crate::num::Numeric;
use crate::unit::{Unit, meta::Meta};
use std::cmp::Ordering;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};

// ── MatN Struct ──────────────────────────────────────────────

/// A generic ROWS × COLS matrix.
///
/// Backed by `[[T; COLS]; ROWS]` on the stack. Works with raw numeric types
/// and [`Unit<N, U>`](crate::Unit) for dimensionally-aware calculations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatN<T, const ROWS: usize, const COLS: usize> {
    /// Internal 2D array of components.
    pub data: [[T; COLS]; ROWS],
}

impl<T, const ROWS: usize, const COLS: usize> MatN<T, ROWS, COLS> {
    /// Creates a new matrix from a 2D array.
    pub fn new(data: [[T; COLS]; ROWS]) -> Self {
        Self { data }
    }

    /// Returns the internal 2D array.
    pub fn components(self) -> [[T; COLS]; ROWS] {
        self.data
    }

    /// Returns a specific row as an array.
    pub fn row(&self, index: usize) -> [T; COLS]
    where
        T: Copy,
    {
        self.data[index]
    }

    /// Returns a specific column as an array.
    pub fn col(&self, index: usize) -> [T; ROWS]
    where
        T: Copy + Default,
    {
        let mut col = [T::default(); ROWS];
        for r in 0..ROWS {
            col[r] = self.data[r][index];
        }
        col
    }

    /// Returns a specific row as a [`VecN`].
    pub fn row_vec(&self, index: usize) -> VecN<T, COLS>
    where
        T: Copy,
    {
        VecN::new(self.data[index])
    }

    /// Returns a specific column as a [`VecN`].
    pub fn col_vec(&self, index: usize) -> VecN<T, ROWS>
    where
        T: Copy + Default,
    {
        VecN::new(self.col(index))
    }

    /// Transpose the matrix (swap rows and columns).
    pub fn transpose(self) -> MatN<T, COLS, ROWS>
    where
        T: Copy,
    {
        let mut data = [[self.data[0][0]; ROWS]; COLS];
        for r in 0..ROWS {
            for c in 0..COLS {
                data[c][r] = self.data[r][c];
            }
        }
        MatN::new(data)
    }
}

// ── Numeric Impls ──────────────────────────────────────────

impl<T: Numeric, const ROWS: usize, const COLS: usize> MatN<T, ROWS, COLS> {
    /// Zero matrix.
    pub fn zero() -> Self {
        Self {
            data: std::array::from_fn(|_| std::array::from_fn(|_| T::from_f64(0.0))),
        }
    }

    /// Matrix multiplication: `self` (ROWS×COLS) × `other` (COLS×C2) → (ROWS×C2).
    pub fn mul_mat<const C2: usize>(self, other: MatN<T, COLS, C2>) -> MatN<T, ROWS, C2> {
        let mut data = [[T::from_f64(0.0); C2]; ROWS];
        for i in 0..ROWS {
            for j in 0..C2 {
                let mut sum = T::from_f64(0.0);
                for k in 0..COLS {
                    sum += self.data[i][k] * other.data[k][j];
                }
                data[i][j] = sum;
            }
        }
        MatN::new(data)
    }
}

impl<T: Numeric> MatN<T, 2, 2> {
    /// 2×2 identity matrix.
    pub fn identity() -> Self {
        Self::new([
            [T::from_f64(1.0), T::from_f64(0.0)],
            [T::from_f64(0.0), T::from_f64(1.0)],
        ])
    }

    /// Determinant of a 2×2 matrix.
    pub fn det2(&self) -> T {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }

    /// Inverse of a 2×2 matrix. Returns `None` if singular.
    pub fn inv2(&self) -> Option<Self> {
        let det = self.det2();
        if det.to_f64() == 0.0 {
            return None;
        }
        let inv_det = T::from_f64(1.0) / det;
        let mut data = [[T::from_f64(0.0); 2]; 2];
        data[0][0] = self.data[1][1] * inv_det;
        data[0][1] = (T::from_f64(0.0) - self.data[0][1]) * inv_det;
        data[1][0] = (T::from_f64(0.0) - self.data[1][0]) * inv_det;
        data[1][1] = self.data[0][0] * inv_det;
        Some(Self::new(data))
    }
}

impl<T: Numeric> MatN<T, 3, 3> {
    /// 3×3 identity matrix.
    pub fn identity() -> Self {
        Self::new([
            [T::from_f64(1.0), T::from_f64(0.0), T::from_f64(0.0)],
            [T::from_f64(0.0), T::from_f64(1.0), T::from_f64(0.0)],
            [T::from_f64(0.0), T::from_f64(0.0), T::from_f64(1.0)],
        ])
    }

    /// Determinant of a 3×3 matrix.
    pub fn det3(&self) -> T {
        let a = self.data[0][0];
        let b = self.data[0][1];
        let c = self.data[0][2];
        let d = self.data[1][0];
        let e = self.data[1][1];
        let f = self.data[1][2];
        let g = self.data[2][0];
        let h = self.data[2][1];
        let i = self.data[2][2];
        a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }
}

// ── Unit-aware Impls ───────────────────────────────────────

impl<N: Numeric, U: Meta + Copy, const ROWS: usize, const COLS: usize>
    MatN<Unit<N, U>, ROWS, COLS>
{
    /// Zero matrix with units.
    pub fn zero_units() -> Self {
        Self {
            data: std::array::from_fn(|_| std::array::from_fn(|_| Unit::new(N::from_f64(0.0)))),
        }
    }

    /// Matrix multiplication with unit tracking.
    pub fn mul_mat_units<const C2: usize>(
        self,
        other: MatN<Unit<N, U>, COLS, C2>,
    ) -> MatN<Unit<N, U>, ROWS, C2> {
        let mut data = [[Unit::new(N::from_f64(0.0)); C2]; ROWS];
        let res_power = self.data[0][0].power + other.data[0][0].power;
        for i in 0..ROWS {
            for j in 0..C2 {
                let mut sum_val = N::from_f64(0.0);
                for k in 0..COLS {
                    sum_val += self.data[i][k].value * other.data[k][j].value;
                }
                data[i][j] = Unit::with_power(sum_val, res_power);
            }
        }
        MatN::new(data)
    }
}

impl<N: Numeric, U: Meta + Copy, const R: usize, const C: usize> Mul<VecN<Unit<N, U>, C>>
    for MatN<Unit<N, U>, R, C>
{
    type Output = VecN<Unit<N, U>, R>;

    fn mul(self, vec: VecN<Unit<N, U>, C>) -> VecN<Unit<N, U>, R> {
        let mut data = [Unit::new(N::from_f64(0.0)); R];
        let res_power = self.data[0][0].power + vec.data[0].power;
        for i in 0..R {
            let mut sum_val = N::from_f64(0.0);
            for j in 0..C {
                sum_val += self.data[i][j].value * vec.data[j].value;
            }
            data[i] = Unit::with_power(sum_val, res_power);
        }
        VecN::new(data)
    }
}

// ── Arithmetic Operators ───────────────────────────────────

impl<T: Numeric, const R: usize, const C: usize> Add for MatN<T, R, C> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut data = self.data;
        for i in 0..R {
            for j in 0..C {
                data[i][j] += other.data[i][j];
            }
        }
        Self::new(data)
    }
}

impl<T: Numeric, const R: usize, const C: usize> AddAssign for MatN<T, R, C> {
    fn add_assign(&mut self, other: Self) {
        for i in 0..R {
            for j in 0..C {
                self.data[i][j] += other.data[i][j];
            }
        }
    }
}

impl<T: Numeric, const R: usize, const C: usize> Sub for MatN<T, R, C> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut data = self.data;
        for i in 0..R {
            for j in 0..C {
                data[i][j] -= other.data[i][j];
            }
        }
        Self::new(data)
    }
}

impl<T: Numeric, const R: usize, const C: usize> SubAssign for MatN<T, R, C> {
    fn sub_assign(&mut self, other: Self) {
        for i in 0..R {
            for j in 0..C {
                self.data[i][j] -= other.data[i][j];
            }
        }
    }
}

impl<T: Numeric, const R: usize, const C: usize> Mul for MatN<T, R, C> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut data = self.data;
        for i in 0..R {
            for j in 0..C {
                data[i][j] *= other.data[i][j];
            }
        }
        Self::new(data)
    }
}

impl<T: Numeric, const R: usize, const C: usize> MulAssign for MatN<T, R, C> {
    fn mul_assign(&mut self, other: Self) {
        for i in 0..R {
            for j in 0..C {
                self.data[i][j] *= other.data[i][j];
            }
        }
    }
}

impl<T: Numeric, const R: usize, const C: usize> Div for MatN<T, R, C> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let mut data = self.data;
        for i in 0..R {
            for j in 0..C {
                data[i][j] /= other.data[i][j];
            }
        }
        Self::new(data)
    }
}

impl<T: Numeric, const R: usize, const C: usize> DivAssign for MatN<T, R, C> {
    fn div_assign(&mut self, other: Self) {
        for i in 0..R {
            for j in 0..C {
                self.data[i][j] /= other.data[i][j];
            }
        }
    }
}

impl<T: Numeric, const R: usize, const C: usize> Rem for MatN<T, R, C> {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        let mut data = self.data;
        for i in 0..R {
            for j in 0..C {
                data[i][j] %= other.data[i][j];
            }
        }
        Self::new(data)
    }
}

impl<T: Numeric, const R: usize, const C: usize> RemAssign for MatN<T, R, C> {
    fn rem_assign(&mut self, other: Self) {
        for i in 0..R {
            for j in 0..C {
                self.data[i][j] %= other.data[i][j];
            }
        }
    }
}

// ── Scalar Operators ───────────────────────────────────────

impl<T: Numeric, const R: usize, const C: usize> Mul<T> for MatN<T, R, C> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        let mut data = self.data;
        for i in 0..R {
            for j in 0..C {
                data[i][j] *= scalar;
            }
        }
        Self::new(data)
    }
}

impl<T: Numeric, const R: usize, const C: usize> MulAssign<T> for MatN<T, R, C> {
    fn mul_assign(&mut self, scalar: T) {
        for i in 0..R {
            for j in 0..C {
                self.data[i][j] *= scalar;
            }
        }
    }
}

impl<T: Numeric, const R: usize, const C: usize> Div<T> for MatN<T, R, C> {
    type Output = Self;
    fn div(self, scalar: T) -> Self {
        let mut data = self.data;
        for i in 0..R {
            for j in 0..C {
                data[i][j] /= scalar;
            }
        }
        Self::new(data)
    }
}

impl<T: Numeric, const R: usize, const C: usize> DivAssign<T> for MatN<T, R, C> {
    fn div_assign(&mut self, scalar: T) {
        for i in 0..R {
            for j in 0..C {
                self.data[i][j] /= scalar;
            }
        }
    }
}

impl<T: Numeric, const R: usize, const C: usize> Rem<T> for MatN<T, R, C> {
    type Output = Self;
    fn rem(self, scalar: T) -> Self {
        let mut data = self.data;
        for i in 0..R {
            for j in 0..C {
                data[i][j] %= scalar;
            }
        }
        Self::new(data)
    }
}

impl<T: Numeric, const R: usize, const C: usize> RemAssign<T> for MatN<T, R, C> {
    fn rem_assign(&mut self, scalar: T) {
        for i in 0..R {
            for j in 0..C {
                self.data[i][j] %= scalar;
            }
        }
    }
}

// ── Matrix–Vector Multiply ─────────────────────────────────

impl<T: Numeric, const R: usize, const C: usize> Mul<VecN<T, C>> for MatN<T, R, C> {
    type Output = VecN<T, R>;

    fn mul(self, vec: VecN<T, C>) -> VecN<T, R> {
        let mut data = [T::from_f64(0.0); R];
        for i in 0..R {
            let mut sum = T::from_f64(0.0);
            for j in 0..C {
                sum += self.data[i][j] * vec.data[j];
            }
            data[i] = sum;
        }
        VecN::new(data)
    }
}

// ── Comparison ─────────────────────────────────────────────

impl<T: Numeric, const R: usize, const C: usize> PartialOrd for MatN<T, R, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

// ── Default & Conversions ──────────────────────────────────

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Default for MatN<T, ROWS, COLS> {
    fn default() -> Self {
        Self {
            data: [[T::default(); COLS]; ROWS],
        }
    }
}

impl<T, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]> for MatN<T, ROWS, COLS> {
    fn from(data: [[T; COLS]; ROWS]) -> Self {
        Self { data }
    }
}

impl<T: Copy, const ROWS: usize, const COLS: usize> From<[VecN<T, COLS>; ROWS]>
    for MatN<T, ROWS, COLS>
{
    fn from(rows: [VecN<T, COLS>; ROWS]) -> Self {
        let mut data = [[rows[0].data[0]; COLS]; ROWS];
        for r in 0..ROWS {
            data[r] = rows[r].data;
        }
        Self::new(data)
    }
}

// ── Aliases ────────────────────────────────────────────────

/// 2×2 matrix.
pub type Mat2x2<T = f64> = MatN<T, 2, 2>;

/// 3×3 matrix.
pub type Mat3x3<T = f64> = MatN<T, 3, 3>;

/// 4×4 matrix.
pub type Mat4x4<T = f64> = MatN<T, 4, 4>;

/// 2×3 matrix (2 rows, 3 columns).
pub type Mat2x3<T = f64> = MatN<T, 2, 3>;

/// 3×2 matrix (3 rows, 2 columns).
pub type Mat3x2<T = f64> = MatN<T, 3, 2>;
