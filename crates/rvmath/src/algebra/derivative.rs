//! Symbolic differentiation of algebraic expressions.
/// Symbolic differentiation of algebraic expressions.

/// Computes the derivative of an expression symbolically using calculus rules.
use super::expr::Expr;
use super::simplify::simplify;

/// Compute the symbolic derivative of an expression.
///
/// Applies differentiation rules:
/// - Constant rule: d/dx(c) = 0
/// - Variable rule: d/dx(x) = 1, d/dx(y) = 0 (for other variables)
/// - Sum rule: d/dx(a + b) = da/dx + db/dx
/// - Difference rule: d/dx(a - b) = da/dx - db/dx
/// - Product rule: d/dx(a * b) = a * db/dx + b * da/dx
/// - Quotient rule: d/dx(a / b) = (b * da/dx - a * db/dx) / b^2
/// - Power rule: d/dx(x^n) = n * x^(n-1)
/// - Chain rule: d/dx(f(g(x))) = f'(g(x)) * g'(x)
///
pub fn derivative(expr: &Expr, var: &str) -> Expr {
    let result = derivative_inner(expr, var);
    simplify(&result)
}

fn derivative_inner(expr: &Expr, var: &str) -> Expr {
    match expr {
        // Constant rule: d/dx(c) = 0
        Expr::Const(_) => Expr::Const(0.0),

        // Variable rule: d/dx(x) = 1, d/dx(y) = 0
        Expr::Var(v) => {
            if v == var {
                Expr::Const(1.0)
            } else {
                Expr::Const(0.0)
            }
        }

        // Sum rule: d/dx(a + b) = da/dx + db/dx
        Expr::Add(a, b) => {
            let da = derivative_inner(a, var);
            let db = derivative_inner(b, var);
            Expr::Add(Box::new(da), Box::new(db))
        }

        // Difference rule: d/dx(a - b) = da/dx - db/dx
        Expr::Sub(a, b) => {
            let da = derivative_inner(a, var);
            let db = derivative_inner(b, var);
            Expr::Sub(Box::new(da), Box::new(db))
        }

        // Product rule: d/dx(a * b) = a * db/dx + b * da/dx
        Expr::Mul(a, b) => {
            let da = derivative_inner(a, var);
            let db = derivative_inner(b, var);

            let left = Expr::Mul(a.clone(), Box::new(db));
            let right = Expr::Mul(b.clone(), Box::new(da));

            Expr::Add(Box::new(left), Box::new(right))
        }

        // Quotient rule: d/dx(a / b) = (b * da/dx - a * db/dx) / b^2
        Expr::Div(a, b) => {
            let da = derivative_inner(a, var);
            let db = derivative_inner(b, var);

            let numerator = Expr::Sub(
                Box::new(Expr::Mul(b.clone(), Box::new(da))),
                Box::new(Expr::Mul(a.clone(), Box::new(db))),
            );

            let denominator = Expr::Pow(b.clone(), Box::new(Expr::Const(2.0)));

            Expr::Div(Box::new(numerator), Box::new(denominator))
        }

        // Power rule: d/dx(x^n) = n * x^(n-1) * dx/dx (chain rule extension)
        // General: d/dx(u^v) requires more complex handling
        Expr::Pow(a, b) => {
            // Check if both are constants
            if !a.contains_var(var) && !b.contains_var(var) {
                // d/dx(c^c) = 0
                return Expr::Const(0.0);
            }

            // Power rule with chain: d/dx(u^n) = n * u^(n-1) * du/dx
            if let Expr::Const(n) = b.as_ref() {
                if !b.contains_var(var) && a.contains_var(var) {
                    let du = derivative_inner(a, var);
                    let new_exp = Expr::Const(n - 1.0);
                    let pow_term = Expr::Pow(a.clone(), Box::new(new_exp));

                    return Expr::Mul(
                        Box::new(Expr::Mul(Box::new(Expr::Const(*n)), Box::new(pow_term))),
                        Box::new(du),
                    );
                }
            }

            // General case: d/dx(u^v) = u^v * (v' * ln(u) + v * u'/u)
            let du = derivative_inner(a, var);
            let dv = derivative_inner(b, var);

            let ln_u = Expr::Func("ln".to_string(), vec![a.as_ref().clone()]);
            let u_inv = Expr::Div(Box::new(du.clone()), a.clone());

            let term1 = Expr::Mul(Box::new(dv), Box::new(ln_u));
            let term2 = Expr::Mul(b.clone(), Box::new(u_inv));
            let sum = Expr::Add(Box::new(term1), Box::new(term2));

            Expr::Mul(Box::new(Expr::Pow(a.clone(), b.clone())), Box::new(sum))
        }

        // Negation: d/dx(-u) = -du/dx
        Expr::Neg(e) => {
            let de = derivative_inner(e, var);
            Expr::Neg(Box::new(de))
        }

        // Function derivatives (chain rule applied)
        Expr::Func(fname, args) => {
            if args.is_empty() {
                return Expr::Const(0.0);
            }

            if args.len() == 1 {
                let u = &args[0];
                let du = derivative_inner(u, var);

                // Standard function derivatives
                let df_du = match fname.as_str() {
                    "sin" => Expr::Func("cos".to_string(), vec![u.clone()]),
                    "cos" => Expr::Neg(Box::new(Expr::Func("sin".to_string(), vec![u.clone()]))),
                    "tan" => {
                        let cos_u = Expr::Func("cos".to_string(), vec![u.clone()]);
                        Expr::Div(
                            Box::new(Expr::Const(1.0)),
                            Box::new(Expr::Pow(Box::new(cos_u), Box::new(Expr::Const(2.0)))),
                        )
                    }
                    "asin" => {
                        let u_sq = Expr::Pow(Box::new(u.clone()), Box::new(Expr::Const(2.0)));
                        let one_minus_u_sq = Expr::Sub(Box::new(Expr::Const(1.0)), Box::new(u_sq));
                        let sqrt_term = Expr::Func("sqrt".to_string(), vec![one_minus_u_sq]);
                        Expr::Div(Box::new(Expr::Const(1.0)), Box::new(sqrt_term))
                    }
                    "acos" => {
                        let u_sq = Expr::Pow(Box::new(u.clone()), Box::new(Expr::Const(2.0)));
                        let one_minus_u_sq = Expr::Sub(Box::new(Expr::Const(1.0)), Box::new(u_sq));
                        let sqrt_term = Expr::Func("sqrt".to_string(), vec![one_minus_u_sq]);
                        Expr::Neg(Box::new(Expr::Div(
                            Box::new(Expr::Const(1.0)),
                            Box::new(sqrt_term),
                        )))
                    }
                    "atan" => {
                        let u_sq = Expr::Pow(Box::new(u.clone()), Box::new(Expr::Const(2.0)));
                        let one_plus_u_sq = Expr::Add(Box::new(Expr::Const(1.0)), Box::new(u_sq));
                        Expr::Div(Box::new(Expr::Const(1.0)), Box::new(one_plus_u_sq))
                    }
                    "sinh" => Expr::Func("cosh".to_string(), vec![u.clone()]),
                    "cosh" => Expr::Func("sinh".to_string(), vec![u.clone()]),
                    "tanh" => {
                        let cosh_u = Expr::Func("cosh".to_string(), vec![u.clone()]);
                        Expr::Div(
                            Box::new(Expr::Const(1.0)),
                            Box::new(Expr::Pow(Box::new(cosh_u), Box::new(Expr::Const(2.0)))),
                        )
                    }
                    "exp" => Expr::Func("exp".to_string(), vec![u.clone()]),
                    "ln" => Expr::Div(Box::new(Expr::Const(1.0)), Box::new(u.clone())),
                    "log" => Expr::Div(
                        Box::new(Expr::Const(1.0)),
                        Box::new(Expr::Mul(
                            Box::new(u.clone()),
                            Box::new(Expr::Const((10.0_f64).ln())),
                        )),
                    ),
                    "log10" => Expr::Div(
                        Box::new(Expr::Const(1.0)),
                        Box::new(Expr::Mul(
                            Box::new(u.clone()),
                            Box::new(Expr::Const(10.0_f64.ln())),
                        )),
                    ),
                    "sqrt" => {
                        let two_sqrt = Expr::Mul(
                            Box::new(Expr::Const(2.0)),
                            Box::new(Expr::Func("sqrt".to_string(), vec![u.clone()])),
                        );
                        Expr::Div(Box::new(Expr::Const(1.0)), Box::new(two_sqrt))
                    }
                    "cbrt" => {
                        let three = Expr::Const(3.0);
                        let u_sq = Expr::Pow(Box::new(u.clone()), Box::new(Expr::Const(2.0 / 3.0)));
                        Expr::Div(
                            Box::new(Expr::Const(1.0)),
                            Box::new(Expr::Mul(Box::new(three), Box::new(u_sq))),
                        )
                    }
                    "abs" => {
                        // d/dx(|u|) = u/|u| * du/dx (undefined at u=0)
                        let abs_u = Expr::Func("abs".to_string(), vec![u.clone()]);
                        Expr::Mul(
                            Box::new(Expr::Div(Box::new(u.clone()), Box::new(abs_u))),
                            Box::new(du.clone()),
                        )
                    }
                    _ => return Expr::Const(0.0), // Unknown function
                };

                // Chain rule: multiply by du/dx
                Expr::Mul(Box::new(df_du), Box::new(du))
            } else {
                // Multi-argument functions not typically supported
                Expr::Const(0.0)
            }
        }
    }
}


