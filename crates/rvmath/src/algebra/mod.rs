//! Symbolic algebra on string-based expressions.
//!
//! Key functions:
//! - [`simplify()`] — reduce expressions via algebraic rules
//! - [`derivative()`] — symbolic differentiation (multi-variable)
//! - [`rationalize()`] — rationalise denominators
//! - [`resolve()`] — evaluate with variable substitution → numeric result
//!
//! # Examples
//!
//! ```
//! # use rvmath::algebra::{simplify, derivative, resolve};
//! # use rvmath::num::Num;
//! let s = simplify("2*x + 3*x").unwrap();
//! assert_eq!(s, "5*x");
//!
//! let d = derivative("x*y + x^2", "x").unwrap();
//! assert_eq!(d, "(y+2*x)");
//!
//! let r = resolve("3*x + 1", &[("x", Num::new(2.0))]).unwrap();
//! assert_eq!(r.value, 7.0);
//! ```
//!
//! # Design
//!
//! - **String-based API**: input/output are `String`s for accessibility.
//! - **Multi-variable**: any variable names; differentiation treats non-specified vars as constants.
//! - **AST internally**: `Expr` enum for structured recursive operations.
//! - **Auto-simplify**: `derivative()` and `rationalize()` simplify results iteratively.
//! - **PEMDAS parser**: respects operator precedence and associativity.
//! - **15+ functions**: sin, cos, tan, sqrt, ln, exp, … with chain-rule derivative support.
//!
//! # Limitations
//!
//! - No symbolic integration (use the `calculus` module for numerical integration)
//! - Factorization limited to basic algebraic rules
//! - Some complex expressions may not fully reduce to canonical form

/// Symbolic differentiation of algebraic expressions.
pub mod derivative;

/// Evaluation of algebraic expressions with variable substitution.
pub mod evaluate;

/// Expression AST (Abstract Syntax Tree) for algebraic expressions.
pub mod expr;

/// Parser for algebraic expressions.
pub mod parser;

/// Rationalization of algebraic expressions.
pub mod rationalize;

/// Simplification of algebraic expressions.
pub mod simplify;

use crate::num::Num;
pub use parser::parse;
use std::collections::HashMap;

/// Simplify an algebraic expression given as a string.
///
/// Applies simplification rules iteratively until reaching a fixed point.
///
pub fn simplify(expr_str: &str) -> Result<String, String> {
    let expr = parse(expr_str)?;
    let simplified = simplify::simplify(&expr);
    Ok(simplified.to_string())
}

/// Compute the symbolic derivative of an expression.
///
/// Applies differentiation rules (power, sum, product, quotient, chain) symbolically.
/// Supports both single and multi-variable expressions.
///
///
pub fn derivative(expr_str: &str, var: &str) -> Result<String, String> {
    let expr = parse(expr_str)?;
    let deriv = derivative::derivative(&expr, var);
    Ok(deriv.to_string())
}

/// Rationalize an algebraic expression.
///
/// Rationalizes denominators by multiplying by conjugates.
///
pub fn rationalize(expr_str: &str) -> Result<String, String> {
    let expr = parse(expr_str)?;
    let rationalized = rationalize::rationalize(&expr);
    Ok(rationalized.to_string())
}

/// Evaluate an algebraic expression with variable substitution.
///
/// Substitutes variables with their numeric values and computes the final result.
/// Variables that are not provided in the assignments will cause an error.
///
/// # Arguments
///
/// * `expr_str` - The algebraic expression as a string
/// * `vars` - Slice of `(variable_name, Num<f64>)` tuples for substitution
///
/// # Returns
///
/// - `Ok(Num<f64>)` - The computed numeric result wrapped in Num
/// - `Err(String)` - Error message if evaluation fails
///
pub fn resolve(expr_str: &str, vars: &[(&str, Num<f64>)]) -> Result<Num<f64>, String> {
    let expr = parse(expr_str)?;

    // Convert the slice of tuples into a HashMap
    let mut var_map = HashMap::new();
    for (name, num) in vars {
        var_map.insert(name.to_string(), num.value);
    }

    let result = evaluate::evaluate(&expr, &var_map)?;
    Ok(Num::new(result))
}

/// Evaluate an algebraic expression for multiple values of a single variable.
///
/// This function applies an expression to a series of values for one variable,
/// returning a vector of computed results. Useful for batch processing and
/// vectorized computation.
///
/// # Arguments
///
/// * `expr_str` - The algebraic expression as a string (e.g., "4*x + 2")
/// * `var_name` - The name of the variable to substitute (e.g., "x")
/// * `values` - Slice of `Num<f64>` values to substitute for the variable
///
/// # Returns
///
/// - `Ok(Vec<Num<f64>>)` - Vector of results for each value
/// - `Err(String)` - Error message if parsing or evaluation fails
///
pub fn map_resolve(
    expr_str: &str,
    var_name: &str,
    values: &[Num<f64>],
) -> Result<Vec<Num<f64>>, String> {
    // Parse the expression once, then reuse for all evaluations
    let expr = parse(expr_str)?;

    let mut results = Vec::with_capacity(values.len());

    for num in values {
        let mut var_map = HashMap::new();
        var_map.insert(var_name.to_string(), num.value);

        let result = evaluate::evaluate(&expr, &var_map)?;
        results.push(Num::new(result));
    }

    Ok(results)
}


