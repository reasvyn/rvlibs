//! Two-dimensional shape formulas.
//!
//! Provides area and perimeter calculations for common 2D shapes.

use crate::num::Numeric;

/// Calculates the area of an ellipse.
///
/// Formula: A = πab where a and b are semi-major and semi-minor axes
///
/// # Arguments
/// * `semi_major` - The semi-major axis (half the longer axis)
/// * `semi_minor` - The semi-minor axis (half the shorter axis)
///
/// # Returns
/// The area of the ellipse
///
pub fn ellipse_area<T: Numeric>(semi_major: T, semi_minor: T) -> T {
    let pi = T::pi();
    pi * semi_major * semi_minor
}

/// Calculates the perimeter of an ellipse using Ramanujan's approximation.
///
/// Formula (Ramanujan): P ≈ π(a + b)(1 + 3h/(10 + √(4 - 3h)))
/// where h = (a - b)² / (a + b)²
///
/// This approximation has an error < 0.01% for most ellipses.
///
/// # Arguments
/// * `semi_major` - The semi-major axis
/// * `semi_minor` - The semi-minor axis
///
/// # Returns
/// The approximate perimeter of the ellipse
///
pub fn ellipse_perimeter<T: Numeric>(semi_major: T, semi_minor: T) -> T {
    let pi = T::pi();
    let a = semi_major;
    let b = semi_minor;

    // h = (a - b)² / (a + b)²
    let h = {
        let diff = a - b;
        let sum = a + b;
        (diff * diff) / (sum * sum)
    };

    // P = π(a + b)(1 + 3h/(10 + √(4 - 3h)))
    let sqrt_term = (T::from_f64(4.0) - T::from_f64(3.0) * h).sqrt();
    let denominator = T::from_f64(10.0) + sqrt_term;
    let numerator = T::from_f64(3.0) * h;

    pi * (a + b) * (T::from_f64(1.0) + numerator / denominator)
}

/// Calculates the area of a triangle using Heron's formula.
///
/// This formula is numerically stable for all triangle types,
/// including very flat triangles.
///
/// Formula:
/// - s = (a + b + c) / 2  (semi-perimeter)
/// - A = √[s(s-a)(s-b)(s-c)]
///
/// # Arguments
/// * `a` - Length of side a
/// * `b` - Length of side b
/// * `c` - Length of side c
///
/// # Returns
/// The area of the triangle, or NaN if inputs don't form a valid triangle
///
pub fn triangle_area_heron<T: Numeric>(a: T, b: T, c: T) -> T {
    // Semi-perimeter
    let s = (a + b + c) / T::from_f64(2.0);

    // A = √[s(s-a)(s-b)(s-c)]
    let product = s * (s - a) * (s - b) * (s - c);

    // Return NaN if triangle inequality is violated
    if product.to_f64() < 0.0 {
        T::from_f64(f64::NAN)
    } else {
        product.sqrt()
    }
}

/// Calculates the area of a triangle given base and height.
///
/// Formula: A = (1/2) * base * height
///
/// # Arguments
/// * `base` - The base of the triangle
/// * `height` - The perpendicular height from base to opposite vertex
///
/// # Returns
/// The area of the triangle
///
pub fn triangle_area<T: Numeric>(base: T, height: T) -> T {
    (base * height) / T::from_f64(2.0)
}

/// Calculates the area of a regular polygon.
///
/// Formula: A = (n * r² * sin(2π/n)) / 2
/// where n is the number of sides and r is the circumradius
///
/// # Arguments
/// * `num_sides` - The number of sides of the polygon
/// * `circumradius` - The radius of the circumscribed circle
///
/// # Returns
/// The area of the regular polygon
///
pub fn polygon_area<T: Numeric>(num_sides: u32, circumradius: T) -> T {
    if num_sides < 3 {
        return T::from_f64(0.0);
    }

    let n = T::from_f64(num_sides as f64);
    let two_pi_over_n = T::from_f64(2.0) * T::pi() / n;

    (n * circumradius * circumradius * two_pi_over_n.sin()) / T::from_f64(2.0)
}

/// Calculates the perimeter of a regular polygon.
///
/// Formula: P = 2 * n * r * sin(π/n)
/// where n is the number of sides and r is the circumradius
///
/// # Arguments
/// * `num_sides` - The number of sides of the polygon
/// * `circumradius` - The radius of the circumscribed circle
///
/// # Returns
/// The perimeter of the regular polygon
///
pub fn polygon_perimeter<T: Numeric>(num_sides: u32, circumradius: T) -> T {
    if num_sides < 3 {
        return T::from_f64(0.0);
    }

    let n = T::from_f64(num_sides as f64);
    let pi_over_n = T::pi() / n;

    T::from_f64(2.0) * n * circumradius * pi_over_n.sin()
}


