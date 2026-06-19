//! Utility functions for mathematical expression evaluation.
//!
//! This module provides tools for parsing and evaluating mathematical expressions
//! from strings, supporting operators, functions, and proper precedence handling.
//!
//! # Why Raw Numeric Only?
//!
//! The expression evaluator intentionally supports raw numeric types (`f64`) only, not user-defined
//! units from the unit system. This design decision reflects a fundamental constraint in Rust's
//! type system:
//!
//! - **Units are user-defined** via macros (e.g., `declare_units!`), not built-in
//! - **Rust requires type-safety at compile time**, but string parsing is **runtime**
//! - String expressions like `"2m + 3km"` cannot know the unit types (`Meter`, `Kilometer`) at parse time
//!
//! **Workaround for unit-aware calculations:**
//!
//! # Future Enhancement
//!
//! If the library adds built-in standard units (e.g., `Meter`, `Kilometer`, `Inch`), unit-aware
//! evaluation becomes feasible. The current implementation is designed to be extensible for this
//! use case.

pub mod evaluator;
pub mod parser;

use crate::num::Num;

pub use evaluator::Evaluator;
pub use parser::Parser;

/// Evaluates a mathematical expression string and returns the result.
///
/// This function parses a string-based mathematical expression and evaluates it,
/// respecting proper operator precedence and associativity. It returns the result
/// as a `Num<f64>` for convenient integration with the library's numeric system.
///
/// # Arguments
/// * `expr` - A string containing a mathematical expression
///
/// # Returns
/// A `Num<f64>` containing the result of the evaluation.
/// Returns NaN if the expression is invalid or contains undefined operations.
///
/// # Operator Precedence (PEMDAS)
/// 1. Parentheses: `()`
/// 2. Exponentiation: `^` (right-associative)
/// 3. Unary minus: `-x`
/// 4. Multiplication, Division, Modulo: `*`, `/`, `%`
/// 5. Addition, Subtraction: `+`, `-`
///
/// # Supported Operators
/// - **Arithmetic**: `+`, `-`, `*`, `/`, `%` (modulo)
/// - **Exponentiation**: `^` (e.g., `2^3 = 8`)
/// - **Unary Negation**: `-x` (e.g., `-5`)
///
/// # Supported Functions
/// - **Trigonometric**: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`
/// - **Hyperbolic**: `sinh`, `cosh`, `tanh`
/// - **Exponential & Logarithmic**: `exp`, `ln`, `log`, `log10`, `log2`
/// - **Root Functions**: `sqrt`, `cbrt`
/// - **Rounding**: `round`, `floor`, `ceil`, `abs`, `fract`, `trunc`
/// - **Angle Conversion**: `to_degrees`, `to_radians`, `deg`, `rad`
/// - **Utilities**: `recip`, `inv`, `sign`, `exp_m1`, `ln_1p`
///
/// # Error Handling
/// Invalid expressions and undefined operations are handled gracefully:
/// - **Division by zero**: Returns `NaN`
/// - **Square root of negative**: Returns `NaN`
/// - **Logarithm of zero/negative**: Returns `NaN`
/// - **Invalid syntax**: Returns `NaN`
/// - **Unknown functions**: Returns `NaN`
/// - **Mismatched parentheses**: Returns `NaN`
///
/// # Examples
///
/// Basic arithmetic:
///
/// ```rust
/// # use rvmath::utils::evaluate;
/// assert_eq!(evaluate("2 + 3").value, 5.0);
/// ```
///
/// Operator precedence:
///
/// ```rust
/// # use rvmath::utils::evaluate;
/// assert_eq!(evaluate("2 + 3 * 4").value, 14.0);
/// ```
///
/// Parentheses:
///
/// ```rust
/// # use rvmath::utils::evaluate;
/// assert_eq!(evaluate("(2 + 3) * 4").value, 20.0);
/// ```
///
/// Mathematical functions:
///
/// ```rust
/// # use rvmath::utils::evaluate;
/// assert!((evaluate("sqrt(16)").value - 4.0).abs() < 1e-12);
/// ```
///
/// Trigonometric functions:
///
/// ```rust
/// # use rvmath::utils::evaluate;
/// let result = evaluate("sin(0)").value;
/// assert!(result.abs() < 1e-12);
/// ```
///
/// Complex expressions:
///
/// ```rust
/// # use rvmath::utils::evaluate;
/// let result = evaluate("sqrt(16) + 6").value;
/// assert!((result - 10.0).abs() < 1e-12);
/// ```
///
/// Exponentiation (right-associative):
///
/// ```rust
/// # use rvmath::utils::evaluate;
/// // 2^(3^2) = 2^9 = 512
/// assert_eq!(evaluate("2^3^2").value, 512.0);
/// ```
///
/// Angle conversion:
///
/// ```rust
/// # use rvmath::utils::evaluate;
/// let deg = evaluate("to_degrees(1.5707963267948966)").value;
/// assert!((deg - 90.0).abs() < 1e-12);
/// ```
pub fn evaluate(expr: &str) -> Num<f64> {
    let parser = Parser::new(expr);
    match parser.parse() {
        Ok(tokens) => {
            let evaluator = Evaluator::new(tokens);
            match evaluator.evaluate() {
                Ok(result) => Num::new(result),
                Err(_) => Num::new(f64::NAN),
            }
        }
        Err(_) => Num::new(f64::NAN),
    }
}
