//! Unit metadata traits for the unit system.
//!
//! This module defines core traits that specify unit behavior and dimensional relationships.
//! Units must implement the `Meta` trait, which associates them with a unit family
//! via the `Dimension` trait.

/// Metadata trait for a specific unit within a dimension family.
///
/// Every concrete unit type must implement this trait to specify:
/// - Its unit family (dimension)
/// - Its symbol (how it's displayed)
/// - Its conversion factor relative to the base unit
///
/// # Associated Types
///
/// - `Family` - The unit dimension family this unit belongs to
///
/// # Associated Constants
///
/// - `SYMBOL` - A string representation of the unit (e.g., "m" for meters, "km" for kilometers)
/// - `FACTOR` - The conversion factor relative to the base unit (e.g., 1000.0 for kilometers)
///
/// # Examples
///
/// When you use `declare_family!` and `declare_units!` macros, they automatically
/// implement this trait for you:
///
pub trait Meta {
    /// The unit family/dimension this unit belongs to (e.g., Length, Mass, Time).
    type Family: Dimension;

    /// String symbol for this unit (e.g., "m" for meters, "kg" for kilograms).
    const SYMBOL: &'static str;

    /// Conversion factor relative to the base unit in the family.
    /// For the base unit, this is 1.0. For other units, it represents how many base units
    /// equal one of this unit (e.g., 1 km = 1000 m, so FACTOR = 1000.0).
    const FACTOR: f64;
}

/// Marker trait for unit dimension families.
///
/// A dimension family represents a physical quantity category (like Length, Mass, Time).
/// Each dimension family must specify its base unit.
///
/// # Associated Types
///
/// - `Base` - The canonical base unit for this dimension
///
/// # Examples
///
/// The macros automatically create dimension types for you:
///
///
/// You can then access the base unit by implementing the trait:
///
pub trait Dimension {
    /// The base unit for this dimension family.
    /// This is typically the SI base unit or the unit specified as the second argument to `declare_family!`.
    type Base: Meta;
}
