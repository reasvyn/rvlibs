//! # rvmath
//!
//! A comprehensive, lightweight, and type-safe mathematics library for Rust.
//!
//! ## Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`algebra`] | Algebraic operations: derivative, simplify, resolve, rationalize |
//! | [`calculus`] | Calculus: differentiation, integration, series, root-finding |
//! | [`consts`] | Mathematical and physical constants |
//! | [`geometry`] | Geometric formulas: area, volume, surface area |
//! | [`la`] | Linear algebra: vectors, matrices, tensors |
//! | [`num`] | Numeric traits and number types |
//! | [`ops`] | Functional-style arithmetic, log, trig, hyperbolic ops |
//! | [`prelude`] | Convenience re-exports for common types and traits |
//! | [`unit`] | Unit/quantity types with compile-time dimensional analysis |
//! | [`utils`] | Utility functions and helpers |
//!
//! ## Quick start
//!
//! ```rust
//! use rvmath::prelude::*;
//!
//! let a = Num::new(10.0);
//! let b = Num::new(3.0);
//! assert_eq!((a + b).value, 13.0);
//! ```
//!
//! See the [`prelude`] module for a full list of exported items.
//!
//! ## Re-exports
//!
//! Commonly used items from [`algebra`], [`calculus`], [`geometry`], [`la`], [`num`],
//! [`ops`], [`unit`], and [`utils`] are re-exported at the crate root for convenience.
//! Use `use rvmath::*` to bring them all into scope.
//!
//! ## Feature flags
//!
//! * `serde` — Enables [`Serialize`](serde::Serialize) / [`Deserialize`](serde::Deserialize)
//!   derives on public types (optional).
//!
//! ## `std` usage
//!
//! This crate requires the standard library; it does **not** support `#![no_std]`.

pub mod algebra;
pub mod calculus;
pub mod consts;
pub mod geometry;
pub mod la;
pub mod num;
pub mod ops;
pub mod prelude;
pub mod unit;
pub mod utils;

pub use crate::algebra::{derivative, map_resolve, rationalize, resolve, simplify};
pub use crate::calculus::constants as calculus_constants;
pub use crate::calculus::{
    arccos_derivative, arcsin_derivative, arcsin_integral, arctan_derivative, arctan_integral,
    binomial_series, bisection, central_difference, cos_derivative, cos_integral, cosh_derivative,
    exp_base_derivative, exp_base_integral, exp_derivative, exp_integral, factorial,
    forward_difference, ln_derivative, ln_integral, log_base_derivative, maclaurin_arctan,
    maclaurin_cos, maclaurin_exp, maclaurin_ln1p, maclaurin_sin, newton_raphson, power_integral,
    power_rule, sec2_integral, second_derivative, simpsons_rule, sin_derivative, sin_integral,
    sinh_derivative, tan_derivative, tanh_derivative, trapezoidal_rule,
};
pub use crate::geometry::constants as geometry_constants;
pub use crate::geometry::{
    cone_surface, cone_surface_lateral, cone_volume, cylinder_surface, cylinder_volume,
    ellipse_area, ellipse_perimeter, polygon_area, polygon_perimeter, sphere_surface,
    sphere_volume, torus_surface, torus_volume, triangle_area, triangle_area_heron,
};
pub use crate::la::*;
pub use crate::num::*;
pub use crate::unit::*;
pub use crate::utils::*;
