//! Geometric constants and formulas for 2D/3D shapes.
//!
//! Provides high-precision constants (π, φ, √2, …) and formulas for:
//! - **3D shapes**: Sphere, cylinder, cone, torus (volume & surface area)
//! - **2D shapes**: Ellipse, triangle (Heron), regular polygons
//!
//! All formulas are generic over [`Numeric`], enabling automatic unit handling.
//!
//! # Examples
//!
//! ```
//! # use rvmath::geometry::shapes_3d::sphere_volume;
//! # use rvmath::geometry::shapes_2d::ellipse_area;
//! let v = sphere_volume(3.0_f64);
//! let a = ellipse_area(5.0_f64, 3.0_f64);
//! assert!((v - 113.097335529232_f64).abs() < 1e-12);
//! assert!((a - 47.123889803846_f64).abs() < 1e-12);
//! ```
//!
//! # Design Philosophy
//!
//! The module focuses on formulas that are difficult to hand-write,
//! frequently used, or mathematically complex (Ramanujan, Heron …).
//!
//! # Accuracy
//!
//! - Constants use maximum `f64` precision
//! - Ellipse perimeter uses Ramanujan's approximation (< 0.01 % error)
//! - Triangle area uses Heron's formula (stable for all triangle types)

pub mod constants;
pub mod shapes_2d;
pub mod shapes_3d;

pub use constants::*;
pub use shapes_2d::*;
pub use shapes_3d::*;
