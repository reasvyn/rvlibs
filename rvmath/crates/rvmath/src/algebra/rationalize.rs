//! Rationalization of algebraic expressions.
/// Rationalization of algebraic expressions.

/// Rationalizes denominators and simplifies expressions with radicals.
use super::expr::Expr;
use super::simplify::simplify;

/// Rationalize an algebraic expression.
///
/// Attempts to rationalize denominators by multiplying by conjugates.
/// Handles simple cases like 1/√2 → √2/2.
///
pub fn rationalize(expr: &Expr) -> Expr {
    let result = rationalize_inner(expr);
    simplify(&result)
}

/// Recursively rationalize expressions.
fn rationalize_inner(expr: &Expr) -> Expr {
    match expr {
        Expr::Const(_) | Expr::Var(_) => expr.clone(),

        Expr::Add(a, b) => Expr::Add(
            Box::new(rationalize_inner(a)),
            Box::new(rationalize_inner(b)),
        ),

        Expr::Sub(a, b) => Expr::Sub(
            Box::new(rationalize_inner(a)),
            Box::new(rationalize_inner(b)),
        ),

        Expr::Mul(a, b) => Expr::Mul(
            Box::new(rationalize_inner(a)),
            Box::new(rationalize_inner(b)),
        ),

        Expr::Div(a, b) => {
            let b_rationalized = match b.as_ref() {
                // 1/√x → √x/x
                Expr::Func(fname, args) if fname == "sqrt" && args.len() == 1 => {
                    let sqrt_expr = &args[0];
                    let conjugate = Expr::Func("sqrt".to_string(), vec![sqrt_expr.clone()]);
                    return Expr::Div(
                        Box::new(Expr::Mul(a.clone(), Box::new(conjugate.clone()))),
                        Box::new(Expr::Mul(b.clone(), Box::new(conjugate))),
                    );
                }

                _ => rationalize_inner(b),
            };

            Expr::Div(Box::new(rationalize_inner(a)), Box::new(b_rationalized))
        }

        Expr::Pow(a, b) => Expr::Pow(
            Box::new(rationalize_inner(a)),
            Box::new(rationalize_inner(b)),
        ),

        Expr::Neg(e) => Expr::Neg(Box::new(rationalize_inner(e))),

        Expr::Func(name, args) => {
            let rationalized_args: Vec<Expr> = args.iter().map(rationalize_inner).collect();
            Expr::Func(name.clone(), rationalized_args)
        }
    }
}


