//! Three-dimensional shape formulas.
//!
//! Provides volume and surface area calculations for common 3D shapes.

use crate::num::Numeric;

/// Calculates the volume of a sphere.
///
/// Formula: V = (4/3)πr³
///
/// # Arguments
/// * `radius` - The radius of the sphere
///
/// # Returns
/// The volume of the sphere
///
///
/// Unit-aware example:
pub fn sphere_volume<T: Numeric>(radius: T) -> T {
    let four_thirds = T::from_f64(4.0 / 3.0);
    let pi = T::pi();
    four_thirds * pi * radius * radius * radius
}

/// Calculates the surface area of a sphere.
///
/// Formula: A = 4πr²
///
/// # Arguments
/// * `radius` - The radius of the sphere
///
/// # Returns
/// The surface area of the sphere
///
pub fn sphere_surface<T: Numeric>(radius: T) -> T {
    let four = T::from_f64(4.0);
    let pi = T::pi();
    four * pi * radius * radius
}

/// Calculates the volume of a cylinder.
///
/// Formula: V = πr²h
///
/// # Arguments
/// * `radius` - The radius of the cylinder base
/// * `height` - The height of the cylinder
///
/// # Returns
/// The volume of the cylinder
///
pub fn cylinder_volume<T: Numeric>(radius: T, height: T) -> T {
    let pi = T::pi();
    pi * radius * radius * height
}

/// Calculates the surface area of a cylinder (including both circular bases).
///
/// Formula: A = 2πr(r + h) = 2πr² + 2πrh
///
/// # Arguments
/// * `radius` - The radius of the cylinder base
/// * `height` - The height of the cylinder
///
/// # Returns
/// The total surface area of the cylinder
///
pub fn cylinder_surface<T: Numeric>(radius: T, height: T) -> T {
    let two = T::from_f64(2.0);
    let pi = T::pi();
    two * pi * radius * (radius + height)
}

/// Calculates the volume of a cone.
///
/// Formula: V = (1/3)πr²h
///
/// # Arguments
/// * `radius` - The radius of the cone base
/// * `height` - The height of the cone
///
/// # Returns
/// The volume of the cone
///
pub fn cone_volume<T: Numeric>(radius: T, height: T) -> T {
    let one_third = T::from_f64(1.0 / 3.0);
    let pi = T::pi();
    one_third * pi * radius * radius * height
}

/// Calculates the lateral surface area of a cone (excluding the base).
///
/// Formula: A = πr√(r² + h²) = πr(r + l) where l is slant height
///
/// # Arguments
/// * `radius` - The radius of the cone base
/// * `height` - The height of the cone
///
/// # Returns
/// The lateral surface area (curved surface only, not including circular base)
///
pub fn cone_surface_lateral<T: Numeric>(radius: T, height: T) -> T {
    let pi = T::pi();
    let slant_height = (radius * radius + height * height).sqrt();
    pi * radius * slant_height
}

/// Calculates the total surface area of a cone (including circular base).
///
/// Formula: A = πr² + πr√(r² + h²)
///
/// # Arguments
/// * `radius` - The radius of the cone base
/// * `height` - The height of the cone
///
/// # Returns
/// The total surface area (lateral surface + circular base)
///
pub fn cone_surface<T: Numeric>(radius: T, height: T) -> T {
    let pi = T::pi();
    let base_area = pi * radius * radius;
    let lateral_area = cone_surface_lateral(radius, height);
    base_area + lateral_area
}

/// Calculates the volume of a torus (donut).
///
/// Formula: V = 2π²Rr² where R = major radius, r = minor radius
///
/// # Arguments
/// * `major_radius` - The distance from center of torus to center of tube
/// * `minor_radius` - The radius of the tube itself
///
/// # Returns
/// The volume of the torus
///
pub fn torus_volume<T: Numeric>(major_radius: T, minor_radius: T) -> T {
    let two = T::from_f64(2.0);
    let pi = T::pi();
    two * pi * pi * major_radius * minor_radius * minor_radius
}

/// Calculates the surface area of a torus (donut).
///
/// Formula: A = 4π²Rr where R = major radius, r = minor radius
///
/// # Arguments
/// * `major_radius` - The distance from center of torus to center of tube
/// * `minor_radius` - The radius of the tube itself
///
/// # Returns
/// The surface area of the torus
///
pub fn torus_surface<T: Numeric>(major_radius: T, minor_radius: T) -> T {
    let four = T::from_f64(4.0);
    let pi = T::pi();
    four * pi * pi * major_radius * minor_radius
}


