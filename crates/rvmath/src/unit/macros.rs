//! Macros for declaring unit families and associated unit aliases.
/// Declares a family of units with a base unit.
///
/// Creates a new unit family (dimension) and associates it with a base unit.
/// The family acts as a marker type to group related units together.
///
/// # Arguments
///
/// - `$vis` - Visibility modifier (`pub`, `pub(crate)`, etc.)
/// - `$family` - Name of the unit family (e.g., `Length`, `Mass`, `Time`)
/// - `$base` - Name of the base unit for this family (e.g., `Meter`, `Kilogram`, `Second`)
///
///
#[macro_export]
macro_rules! declare_family {
    ($vis:vis $family:ident, $base:ident) => {
        $vis struct $family;
        impl $crate::unit::meta::Dimension for $family {
            type Base = $base;
        }
    };
}

/// Declares a set of units within a family, each with a symbol and conversion factor.
///
/// Creates struct types for each unit and implements the `Meta` trait with appropriate
/// metadata. Also generates `new()` and `with_power()` constructor methods for each unit.
///
/// # Arguments
///
/// - `$family` - The unit family these units belong to
/// - `$name` - Name of the unit struct (e.g., `Meter`, `Kilometer`, `Centimeter`)
/// - `$symbol` - String symbol for the unit (e.g., `"m"`, `"km"`, `"cm"`)
/// - `$factor` - Conversion factor relative to the base unit (e.g., 1.0 for base, 1000.0 for kilo)
///
/// # Generated Methods
///
/// For each unit, the macro generates:
/// - `new(value: f64) -> Unit<f64, UnitType>` - Create a new unit with the given value
/// - `with_power(value: f64, power: f64) -> Unit<f64, UnitType>` - Create with explicit power
///
///
///
/// # Unit Naming Convention
///
/// By convention:
/// - Base unit has FACTOR = 1.0 (e.g., Meter, Kilogram, Second)
/// - Larger units have FACTOR > 1.0 (e.g., Kilometer = 1000.0)
/// - Smaller units have FACTOR < 1.0 (e.g., Centimeter = 0.01)
#[macro_export]
macro_rules! declare_units {
    ($family:ident {
        $(
            $(#[$attr:meta])* $vis:vis $name:ident ($symbol:expr, $factor:expr)
        ),* $(,)?
    }) => {
        $(
            $(#[$attr])*
            #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            $vis struct $name;

            /// Represents a unit within the family, with a symbol and conversion factor.
            impl $crate::unit::meta::Meta for $name {
                type Family = $family;
                const SYMBOL: &'static str = $symbol;
                const FACTOR: f64 = $factor;
            }

            impl $name {
                /// Constructs a new unit with the given value.
                ///
                /// # Arguments
                /// * `value` - The numeric value in this unit
                ///
                pub fn new(value: f64) -> $crate::unit::Unit<f64, $name> {
                    $crate::unit::Unit::<f64, $name>::new(value)
                }

                /// Constructs a new unit with the given value and power (dimensional exponent).
                ///
                /// # Arguments
                /// * `value` - The numeric value in this unit
                /// * `power` - The dimensional exponent (e.g., 1.0 for length, 2.0 for area)
                ///
                pub fn with_power(value: f64, power: f64) -> $crate::unit::Unit<f64, $name> {
                    $crate::unit::Unit::<f64, $name>::with_power(value, power)
                }
            }
        )*
    };
}
