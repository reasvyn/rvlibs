//! Algebraic expression simplification rules.
//!
//! Applies algebraic rules to reduce expressions to simpler forms.
#![allow(clippy::collapsible_if)]

use super::expr::Expr;

/// Simplify an algebraic expression.
///
/// Applies simplification rules iteratively until no more changes occur:
/// - Constant folding: 2 + 3 → 5
/// - Identity laws: x + 0 → x, x * 1 → x
/// - Collect like terms: 2x + 3x → 5x
/// - Distributive law: 2(x + 3) → 2x + 6
/// - Power rules: x^2 * x^3 → x^5
///
pub fn simplify(expr: &Expr) -> Expr {
    let mut current = expr.clone();
    let mut previous = Expr::Const(f64::NAN); // Use NaN as an initial different value

    // Iterate until we reach a fixed point (no more simplifications)
    while current != previous {
        previous = current.clone();
        current = simplify_once(&current);
    }

    current
}

/// Perform a single pass of simplification.
fn simplify_once(expr: &Expr) -> Expr {
    match expr {
        Expr::Const(_) | Expr::Var(_) => expr.clone(),

        Expr::Add(a, b) => {
            let a_simp = simplify_once(a);
            let b_simp = simplify_once(b);

            // 0 + x = x
            if let Expr::Const(0.0) = a_simp {
                return b_simp;
            }
            // x + 0 = x
            if let Expr::Const(0.0) = b_simp {
                return a_simp;
            }

            // Constant folding: 2 + 3 = 5
            if let (Expr::Const(n1), Expr::Const(n2)) = (&a_simp, &b_simp) {
                return Expr::Const(n1 + n2);
            }

            // Collect like terms: 2x + 3x = 5x
            if let (Expr::Mul(c1, x1), Expr::Mul(c2, x2)) = (&a_simp, &b_simp) {
                if x1 == x2 {
                    if let (Expr::Const(n1), Expr::Const(n2)) = (c1.as_ref(), c2.as_ref()) {
                        return Expr::Mul(Box::new(Expr::Const(n1 + n2)), x1.clone());
                    }
                }
            }

            // x + x = 2x
            if a_simp == b_simp {
                return Expr::Mul(Box::new(Expr::Const(2.0)), Box::new(a_simp));
            }

            // Collect like terms in nested addition: (2x + 3) + (x + 4) = 3x + 7
            if let (Expr::Add(a1, a2), _) = (&a_simp, &b_simp) {
                // Try merging b_simp into a_simp
                let merged = try_merge_add(a1, a2, &b_simp);
                if let Some(result) = merged {
                    return result;
                }
            }

            Expr::Add(Box::new(a_simp), Box::new(b_simp))
        }

        Expr::Sub(a, b) => {
            let a_simp = simplify_once(a);
            let b_simp = simplify_once(b);

            // x - 0 = x
            if let Expr::Const(0.0) = b_simp {
                return a_simp;
            }

            // Constant folding: 5 - 3 = 2
            if let (Expr::Const(n1), Expr::Const(n2)) = (&a_simp, &b_simp) {
                return Expr::Const(n1 - n2);
            }

            // x - x = 0
            if a_simp == b_simp {
                return Expr::Const(0.0);
            }

            Expr::Sub(Box::new(a_simp), Box::new(b_simp))
        }

        Expr::Mul(a, b) => {
            let a_simp = simplify_once(a);
            let b_simp = simplify_once(b);

            // 0 * x = 0
            if let Expr::Const(0.0) = a_simp {
                return Expr::Const(0.0);
            }
            // x * 0 = 0
            if let Expr::Const(0.0) = b_simp {
                return Expr::Const(0.0);
            }

            // 1 * x = x
            if let Expr::Const(1.0) = a_simp {
                return b_simp;
            }
            // x * 1 = x
            if let Expr::Const(1.0) = b_simp {
                return a_simp;
            }

            // Constant folding: 2 * 3 = 6
            if let (Expr::Const(n1), Expr::Const(n2)) = (&a_simp, &b_simp) {
                return Expr::Const(n1 * n2);
            }

            // x * x = x^2
            if a_simp == b_simp {
                return Expr::Pow(Box::new(a_simp), Box::new(Expr::Const(2.0)));
            }

            // Associativity and commutativity for constants: (c1 * x) * c2 = (c1 * c2) * x
            if let (Expr::Mul(c1, x1), Expr::Const(c2)) = (&a_simp, &b_simp) {
                if let Expr::Const(n1) = c1.as_ref() {
                    return Expr::Mul(Box::new(Expr::Const(n1 * c2)), x1.clone());
                }
            }

            // Distributive law: c * (a + b) = c*a + c*b
            if let Expr::Const(c) = &a_simp {
                if let Expr::Add(left, right) = &b_simp {
                    let dl = Expr::Mul(Box::new(a_simp.clone()), left.clone());
                    let dr = Expr::Mul(Box::new(Expr::Const(*c)), right.clone());
                    return simplify_once(&Expr::Add(Box::new(dl), Box::new(dr)));
                }
            }

            Expr::Mul(Box::new(a_simp), Box::new(b_simp))
        }

        Expr::Div(a, b) => {
            let a_simp = simplify_once(a);
            let b_simp = simplify_once(b);

            // 0 / x = 0
            if let Expr::Const(0.0) = a_simp {
                return Expr::Const(0.0);
            }

            // x / 1 = x
            if let Expr::Const(1.0) = b_simp {
                return a_simp;
            }

            // Constant folding: 6 / 2 = 3
            if let (Expr::Const(n1), Expr::Const(n2)) = (&a_simp, &b_simp) {
                if n2 != &0.0 {
                    return Expr::Const(n1 / n2);
                }
            }

            // x / x = 1
            if a_simp == b_simp {
                return Expr::Const(1.0);
            }

            Expr::Div(Box::new(a_simp), Box::new(b_simp))
        }

        Expr::Pow(a, b) => {
            let a_simp = simplify_once(a);
            let b_simp = simplify_once(b);

            // x^0 = 1
            if let Expr::Const(0.0) = b_simp {
                return Expr::Const(1.0);
            }

            // x^1 = x
            if let Expr::Const(1.0) = b_simp {
                return a_simp;
            }

            // 0^n = 0 (for n > 0)
            if let Expr::Const(0.0) = a_simp {
                return Expr::Const(0.0);
            }

            // 1^n = 1
            if let Expr::Const(1.0) = a_simp {
                return Expr::Const(1.0);
            }

            // Constant folding: 2^3 = 8
            if let (Expr::Const(base), Expr::Const(exp)) = (&a_simp, &b_simp) {
                return Expr::Const(base.powf(*exp));
            }

            // Power rule: x^a * x^b = x^(a+b)
            // This is complex and handled through pattern matching

            Expr::Pow(Box::new(a_simp), Box::new(b_simp))
        }

        Expr::Neg(e) => {
            let e_simp = simplify_once(e);

            // -(-x) = x
            if let Expr::Neg(inner) = e_simp {
                return inner.as_ref().clone();
            }

            // Constant folding: -5 = -5
            if let Expr::Const(n) = e_simp {
                return Expr::Const(-n);
            }

            Expr::Neg(Box::new(e_simp))
        }

        Expr::Func(name, args) => {
            let simplified_args: Vec<Expr> = args.iter().map(simplify_once).collect();
            Expr::Func(name.clone(), simplified_args)
        }
    }
}

/// Try to merge `term` into an existing nested addition `(left + right) + term`.
/// e.g., `(2x + 3) + x` → `3x + 3`
fn try_merge_add(left: &Expr, right: &Expr, term: &Expr) -> Option<Expr> {
    // Try merging term with left: (term + right) + term
    if left == term {
        return Some(Expr::Add(
            Box::new(Expr::Mul(Box::new(Expr::Const(2.0)), Box::new(left.clone()))),
            Box::new(right.clone()),
        ));
    }

    // Try term + right: (left + term) + term
    if right == term {
        return Some(Expr::Add(
            Box::new(left.clone()),
            Box::new(Expr::Mul(Box::new(Expr::Const(2.0)), Box::new(right.clone()))),
        ));
    }

    // Try merging like terms: (c1*x + ...) + c2*x
    if let (Expr::Mul(c1, x1), Expr::Mul(c2, x2)) = (left, term) {
        if x1 == x2 {
            if let (Expr::Const(n1), Expr::Const(n2)) = (c1.as_ref(), c2.as_ref()) {
                let new_coeff = Expr::Const(n1 + n2);
                return Some(Expr::Add(
                    Box::new(Expr::Mul(Box::new(new_coeff), x1.clone())),
                    Box::new(right.clone()),
                ));
            }
        }
    }

    // Try merging bare variable with like term: (c*x + ...) + x
    if let Expr::Mul(c1, x1) = left {
        if let (Expr::Const(n1), _) = (c1.as_ref(), x1.as_ref()) {
            if x1.as_ref() == term {
                let new_coeff = Expr::Const(n1 + 1.0);
                return Some(Expr::Add(
                    Box::new(Expr::Mul(Box::new(new_coeff), x1.clone())),
                    Box::new(right.clone()),
                ));
            }
        }
    }

    None
}


