//! Prelude module for convenient imports.
//!
//! This module re-exports the most commonly-used types and traits from `rvmath`,
//! allowing users to bring them all into scope with a single `use rvmath::prelude::*;`.
//!
//!
//!
//! # Included Exports
//!
//! - [`Num`] and [`Numeric`] — core numeric wrapper and trait
//! - [`Percentage`] — percentage arithmetic type
//! - [`Unit`] — unit-aware numeric wrapper
//! - [`Meta`] and [`Dimension`] — unit metadata traits
//! - [`VecN`] — generic N-dimensional vector
//! - [`MatN`] — generic N-dimensional matrix
//! - [`Tensor`] — N-dimensional tensor
//! - [`declare_family`] and [`declare_units`] — macros for defining unit families and units

pub use crate::la::matrix::MatN;
pub use crate::la::tensor::Tensor;
pub use crate::la::vector::VecN;
pub use crate::num::{Complex, Fraction, Num, NumberKind, NumberSet, Numeric, Percentage};
pub use crate::unit::{
    Unit,
    meta::{Dimension, Meta},
};
pub use crate::{declare_family, declare_units};
