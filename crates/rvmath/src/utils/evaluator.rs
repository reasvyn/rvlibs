//! RPN (Reverse Polish Notation) evaluator for postfix expressions.

use super::parser::{BinaryOp, Token, UnaryOp};

/// Evaluates a postfix (RPN) expression and returns the result.
pub struct Evaluator {
    tokens: Vec<Token>,
}

impl Evaluator {
    /// Creates a new evaluator with the given postfix tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    /// Evaluates the postfix expression and returns the result.
    pub fn evaluate(&self) -> Result<f64, String> {
        let mut stack: Vec<f64> = Vec::new();

        for token in &self.tokens {
            match token {
                Token::Number(n) => {
                    stack.push(*n);
                }
                Token::BinaryOp(op) => {
                    if stack.len() < 2 {
                        return Err(
                            "Invalid expression: not enough operands for binary operator"
                                .to_string(),
                        );
                    }
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let result = self.apply_binary_op(*op, a, b);
                    stack.push(result);
                }
                Token::UnaryOp(UnaryOp::Neg) => {
                    if stack.is_empty() {
                        return Err("Invalid expression: not enough operands for unary operator"
                            .to_string());
                    }
                    let a = stack.pop().unwrap();
                    stack.push(-a);
                }
                Token::Function(name) => {
                    let arity = function_arity(name);
                    if stack.len() < arity {
                        return Err(format!(
                            "Invalid expression: not enough operands for function '{}' (need {}, have {})",
                            name,
                            arity,
                            stack.len()
                        ));
                    }
                    let mut args: Vec<f64> = Vec::with_capacity(arity);
                    for _ in 0..arity {
                        args.push(stack.pop().unwrap());
                    }
                    args.reverse();
                    let result = self.apply_function(name, &args)?;
                    stack.push(result);
                }
                Token::LParen | Token::RParen | Token::Comma => {
                    return Err("Unexpected token in postfix expression".to_string());
                }
            }
        }

        if stack.len() != 1 {
            return Err("Invalid expression: incorrect number of operands".to_string());
        }

        Ok(stack[0])
    }

    /// Applies a binary operator to two operands.
    fn apply_binary_op(&self, op: BinaryOp, a: f64, b: f64) -> f64 {
        match op {
            BinaryOp::Add => a + b,
            BinaryOp::Sub => a - b,
            BinaryOp::Mul => a * b,
            BinaryOp::Div => {
                if b == 0.0 {
                    f64::NAN
                } else {
                    a / b
                }
            }
            BinaryOp::Mod => {
                if b == 0.0 {
                    f64::NAN
                } else {
                    a % b
                }
            }
            BinaryOp::Pow => a.powf(b),
        }
    }

    /// Applies a mathematical function to its arguments.
    fn apply_function(&self, name: &str, args: &[f64]) -> Result<f64, String> {
        let result = match name {
            // Trigonometric functions
            "sin" => args[0].sin(),
            "cos" => args[0].cos(),
            "tan" => args[0].tan(),
            "asin" => args[0].asin(),
            "acos" => args[0].acos(),
            "atan" => args[0].atan(),

            // Multi-arg trig
            "atan2" => args[0].atan2(args[1]),

            // Hyperbolic functions
            "sinh" => args[0].sinh(),
            "cosh" => args[0].cosh(),
            "tanh" => args[0].tanh(),

            // Exponential and logarithmic functions
            "exp" => args[0].exp(),
            "ln" => {
                if args[0] <= 0.0 {
                    f64::NAN
                } else {
                    args[0].ln()
                }
            }
            "log" => {
                if args[0] <= 0.0 {
                    f64::NAN
                } else if args.len() == 2 {
                    args[0].log(args[1])
                } else {
                    args[0].log10()
                }
            }
            "log10" => {
                if args[0] <= 0.0 {
                    f64::NAN
                } else {
                    args[0].log10()
                }
            }
            "log2" => {
                if args[0] <= 0.0 {
                    f64::NAN
                } else {
                    args[0].log2()
                }
            }

            // Root functions
            "sqrt" => {
                if args[0] < 0.0 {
                    f64::NAN
                } else {
                    args[0].sqrt()
                }
            }
            "cbrt" => args[0].cbrt(),

            // Rounding functions
            "round" => args[0].round(),
            "floor" => args[0].floor(),
            "ceil" => args[0].ceil(),
            "abs" => args[0].abs(),

            // Math utilities
            "min" => args[0].min(args[1]),
            "max" => args[0].max(args[1]),
            "pow" => args[0].powf(args[1]),
            "hypot" => args[0].hypot(args[1]),

            // Other utilities
            "exp_m1" => args[0].exp_m1(),
            "ln_1p" => {
                if args[0] <= -1.0 {
                    f64::NAN
                } else {
                    (1.0 + args[0]).ln()
                }
            }
            "recip" => {
                if args[0] == 0.0 {
                    f64::NAN
                } else {
                    1.0 / args[0]
                }
            }
            "inv" => {
                if args[0] == 0.0 {
                    f64::NAN
                } else {
                    1.0 / args[0]
                }
            }
            "fract" => args[0].fract(),
            "trunc" => args[0].trunc(),
            "sign" => {
                if args[0] > 0.0 {
                    1.0
                } else if args[0] < 0.0 {
                    -1.0
                } else {
                    0.0
                }
            }

            // Angle conversion
            "to_degrees" => args[0].to_degrees(),
            "to_radians" => args[0].to_radians(),
            "deg" => args[0].to_degrees(),
            "rad" => args[0].to_radians(),

            _ => return Err(format!("Unknown function: '{}'", name)),
        };

        Ok(result)
    }
}

/// Returns the number of arguments a function expects.
fn function_arity(name: &str) -> usize {
    match name {
        "atan2" | "min" | "max" | "pow" | "hypot" => 2,
        "log" => 2,                       // log(x, base) — also supports 1-arg as log10
        _ => 1,
    }
}

