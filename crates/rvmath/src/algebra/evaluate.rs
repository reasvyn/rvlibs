//! Evaluation of algebraic expressions with variable substitution.
/// Evaluation of algebraic expressions with variable substitution.

/// Evaluates an expression AST with specific variable values, computing
/// the final numeric result.
use super::expr::Expr;
use std::collections::HashMap;

/// Evaluate an expression with variable substitution.
///
/// Recursively evaluates the expression tree, substituting variables
/// with their assigned numeric values and computing the result.
///
/// # Arguments
///
/// * `expr` - The expression AST to evaluate
/// * `vars` - HashMap mapping variable names to their numeric values
///
/// # Returns
///
/// - `Ok(f64)` - The computed numeric result
/// - `Err(String)` - Error message if evaluation fails (e.g., undefined variable, division by zero)
///
///
pub fn evaluate(expr: &Expr, vars: &HashMap<String, f64>) -> Result<f64, String> {
    match expr {
        // Constants evaluate to their numeric value
        Expr::Const(n) => Ok(*n),

        // Variables look up their value in the map
        Expr::Var(name) => vars
            .get(name)
            .copied()
            .ok_or_else(|| format!("Undefined variable: {}", name)),

        // Addition: evaluate both sides and add
        Expr::Add(a, b) => {
            let a_val = evaluate(a, vars)?;
            let b_val = evaluate(b, vars)?;
            Ok(a_val + b_val)
        }

        // Subtraction: evaluate both sides and subtract
        Expr::Sub(a, b) => {
            let a_val = evaluate(a, vars)?;
            let b_val = evaluate(b, vars)?;
            Ok(a_val - b_val)
        }

        // Multiplication: evaluate both sides and multiply
        Expr::Mul(a, b) => {
            let a_val = evaluate(a, vars)?;
            let b_val = evaluate(b, vars)?;
            Ok(a_val * b_val)
        }

        // Division: evaluate both sides and divide (check for zero)
        Expr::Div(a, b) => {
            let a_val = evaluate(a, vars)?;
            let b_val = evaluate(b, vars)?;
            if b_val == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a_val / b_val)
            }
        }

        // Power: evaluate both sides and compute power
        Expr::Pow(base, exp) => {
            let base_val = evaluate(base, vars)?;
            let exp_val = evaluate(exp, vars)?;
            Ok(base_val.powf(exp_val))
        }

        // Negation: evaluate the inner expression and negate
        Expr::Neg(e) => {
            let val = evaluate(e, vars)?;
            Ok(-val)
        }

        // Functions: evaluate arguments and apply function
        Expr::Func(name, args) => evaluate_function(name, args, vars),
    }
}

/// Evaluate mathematical functions.
///
/// Supports 15+ mathematical functions including trigonometric,
/// hyperbolic, exponential, logarithmic, and root functions.
fn evaluate_function(
    fname: &str,
    args: &[Expr],
    vars: &HashMap<String, f64>,
) -> Result<f64, String> {
    // Most functions take one argument
    if args.is_empty() {
        return Err(format!("Function {} requires at least one argument", fname));
    }

    let arg = evaluate(&args[0], vars)?;

    match fname {
        // Trigonometric functions
        "sin" => Ok(arg.sin()),
        "cos" => Ok(arg.cos()),
        "tan" => Ok(arg.tan()),

        // Inverse trigonometric functions
        "asin" => {
            if !(-1.0..=1.0).contains(&arg) {
                Err(format!("asin domain error: {} not in [-1, 1]", arg))
            } else {
                Ok(arg.asin())
            }
        }
        "acos" => {
            if !(-1.0..=1.0).contains(&arg) {
                Err(format!("acos domain error: {} not in [-1, 1]", arg))
            } else {
                Ok(arg.acos())
            }
        }
        "atan" => Ok(arg.atan()),

        // Hyperbolic functions
        "sinh" => Ok(arg.sinh()),
        "cosh" => Ok(arg.cosh()),
        "tanh" => Ok(arg.tanh()),

        // Exponential and logarithmic functions
        "exp" => Ok(arg.exp()),
        "ln" => {
            if arg <= 0.0 {
                Err(format!("ln domain error: {} must be positive", arg))
            } else {
                Ok(arg.ln())
            }
        }
        "log" => {
            // log(arg) using change of base: log_10(arg) = ln(arg) / ln(10)
            if arg <= 0.0 {
                Err(format!("log domain error: {} must be positive", arg))
            } else {
                Ok(arg.log10())
            }
        }
        "log10" => {
            if arg <= 0.0 {
                Err(format!("log10 domain error: {} must be positive", arg))
            } else {
                Ok(arg.log10())
            }
        }

        // Root functions
        "sqrt" => {
            if arg < 0.0 {
                Err(format!("sqrt domain error: {} is negative", arg))
            } else {
                Ok(arg.sqrt())
            }
        }
        "cbrt" => Ok(arg.cbrt()), // Cube root works for negative numbers

        // Absolute value
        "abs" => Ok(arg.abs()),

        // Unknown function
        _ => Err(format!("Unknown function: {}", fname)),
    }
}


