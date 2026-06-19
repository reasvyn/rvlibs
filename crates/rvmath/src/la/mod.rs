//! Linear algebra: vectors, matrices, and tensors.
//!
//! This module groups all linear algebra types under one parent:
//!
//! | Sub-module | Types | Description |
//! |------------|-------|-------------|
//! | [`vector`] | [`VecN`], [`Vec2`], [`Vec3`], [`Vec4`] | Fixed-size N-dimensional vectors |
//! | [`matrix`] | [`MatN`], [`Mat2x2`], [`Mat3x3`], [`Mat4x4`] | Fixed-size M×N matrices |
//! | [`tensor`] | [`Tensor`] | Dynamic N-dimensional arrays |
//!
//! # Re-exports
//!
//! All major types are re-exported at this level for convenience:
//! - `use crate::la::VecN` or `crate::la::vector::VecN`
//! - `use crate::la::MatN` or `crate::la::matrix::MatN`
//! - `use crate::la::Tensor` or `crate::la::tensor::Tensor`

pub mod vector;
pub mod matrix;
pub mod tensor;

pub use vector::VecN;
pub use vector::{Vec2, Vec3, Vec4};
pub use matrix::MatN;
pub use matrix::{Mat2x2, Mat3x3, Mat4x4, Mat2x3, Mat3x2};
pub use tensor::Tensor;
