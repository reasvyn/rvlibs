use rvtest::spec::describe;
use rvmath::algebra::expr::Expr;
use rvmath::algebra::parse;
use rvmath::algebra::{derivative, rationalize, resolve, simplify};
use rvmath::num::Num;
use std::collections::HashMap;

#[test]
fn algebra_expr_tests() {
    describe("Expr")
        .it("const holds value and is_const returns true", || {
            let expr = Expr::const_val(5.0);
            assert_eq!(expr.as_const(), Some(5.0));
            assert!(expr.is_const());
        })
        .it("var checks identity correctly", || {
            let expr = Expr::var("x");
            assert!(expr.is_var("x"));
            assert!(!expr.is_var("y"));
        })
        .it("contains_var detects nested variables", || {
            let expr = Expr::var("x").add(Expr::const_val(5.0));
            assert!(expr.contains_var("x"));
            assert!(!expr.contains_var("y"));
        })
        .it("variables returns unique list", || {
            let expr = Expr::var("x").add(Expr::var("y")).mul(Expr::var("x"));
            let vars = expr.variables();
            assert_eq!(vars, vec!["x", "y"]);
        })
        .it("display produces readable output", || {
            let expr = Expr::var("x").pow(Expr::const_val(2.0)).add(Expr::const_val(1.0));
            let s = expr.to_string();
            assert!(s.contains("x") && s.contains("^") && s.contains("2"));
        })
        .tag("expr")
        .run();

    describe("Parser")
        .it("parses constant expression", || {
            let expr = parse("42").unwrap();
            assert_eq!(expr, Expr::Const(42.0));
        })
        .it("parses variable", || {
            let expr = parse("x").unwrap();
            assert_eq!(expr, Expr::Var("x".to_string()));
        })
        .it("parses addition", || {
            let expr = parse("x + 2").unwrap();
            assert!(matches!(expr, Expr::Add(_, _)));
        })
        .it("parses multiplication", || {
            let expr = parse("2 * x").unwrap();
            assert!(matches!(expr, Expr::Mul(_, _)));
        })
        .it("parses power", || {
            let expr = parse("x^2").unwrap();
            assert!(matches!(expr, Expr::Pow(_, _)));
        })
        .it("parses function", || {
            let expr = parse("sin(x)").unwrap();
            assert!(matches!(expr, Expr::Func(ref name, _) if name == "sin"));
        })
        .it("parses parentheses", || {
            let expr = parse("(x + 2) * 3").unwrap();
            assert!(matches!(expr, Expr::Mul(_, _)));
        })
        .it("parses negation", || {
            let expr = parse("-x").unwrap();
            assert!(matches!(expr, Expr::Neg(_)));
        })
        .it("parses complex expression", || {
            let expr = parse("2*x^2 + 3*x + 4").unwrap();
            assert!(expr.contains_var("x"));
        })
        .it("rejects unbalanced parentheses", || {
            assert!(parse("(x + 2").is_err());
        })
        .tag("parser")
        .run();

    describe("Simplify")
        .it("folds constant addition", || {
            let expr = Expr::Const(2.0).add(Expr::Const(3.0));
            let result = rvmath::algebra::simplify::simplify(&expr);
            assert_eq!(result, Expr::Const(5.0));
        })
        .it("identity law for add with zero", || {
            let expr = Expr::Const(0.0).add(Expr::var("x"));
            let result = rvmath::algebra::simplify::simplify(&expr);
            assert_eq!(result, Expr::var("x"));
        })
        .it("identity law for mul with one", || {
            let expr = Expr::Const(1.0).mul(Expr::var("x"));
            let result = rvmath::algebra::simplify::simplify(&expr);
            assert_eq!(result, Expr::var("x"));
        })
        .it("zero multiplication", || {
            let expr = Expr::Const(0.0).mul(Expr::var("x"));
            let result = rvmath::algebra::simplify::simplify(&expr);
            assert_eq!(result, Expr::Const(0.0));
        })
        .it("power of one identity", || {
            let expr = Expr::var("x").pow(Expr::Const(1.0));
            let result = rvmath::algebra::simplify::simplify(&expr);
            assert_eq!(result, Expr::var("x"));
        })
        .it("power of zero", || {
            let expr = Expr::var("x").pow(Expr::Const(0.0));
            let result = rvmath::algebra::simplify::simplify(&expr);
            assert_eq!(result, Expr::Const(1.0));
        })
        .it("double negation elimination", || {
            let expr = Expr::var("x").neg().neg();
            let result = rvmath::algebra::simplify::simplify(&expr);
            assert_eq!(result, Expr::var("x"));
        })
        .it("collects like terms", || {
            let expr = Expr::Const(2.0).mul(Expr::var("x"))
                .add(Expr::Const(3.0).mul(Expr::var("x")));
            let result = rvmath::algebra::simplify::simplify(&expr);
            assert!(result.to_string().contains("5"));
        })
        .it("self subtraction yields zero", || {
            let expr = Expr::var("x").sub(Expr::var("x"));
            let result = rvmath::algebra::simplify::simplify(&expr);
            assert_eq!(result, Expr::Const(0.0));
        })
        .it("self division yields one", || {
            let expr = Expr::var("x").div(Expr::var("x"));
            let result = rvmath::algebra::simplify::simplify(&expr);
            assert_eq!(result, Expr::Const(1.0));
        })
        .it("distributive law: 2*(x+3) = 2*x+6", || {
            let result = simplify("2*(x+3)").unwrap();
            assert_eq!(result, "(2*x+6)");
        })
        .it("nested add simplification", || {
            let result = simplify("2*x+3+x").unwrap();
            assert_eq!(result, "(3*x+3)");
        })
        .tag("simplify")
        .run();

    describe("Derivative")
        .it("derivative of constant is zero", || {
            let expr = Expr::Const(5.0);
            let result = rvmath::algebra::derivative::derivative(&expr, "x");
            assert_eq!(result, Expr::Const(0.0));
        })
        .it("derivative of variable is one", || {
            let expr = Expr::var("x");
            let result = rvmath::algebra::derivative::derivative(&expr, "x");
            assert_eq!(result, Expr::Const(1.0));
        })
        .it("derivative wrt different variable is zero", || {
            let expr = Expr::var("y");
            let result = rvmath::algebra::derivative::derivative(&expr, "x");
            assert_eq!(result, Expr::Const(0.0));
        })
        .it("derivative of sum", || {
            let expr = Expr::var("x").add(Expr::Const(2.0));
            let result = rvmath::algebra::derivative::derivative(&expr, "x");
            let simplified = rvmath::algebra::simplify::simplify(&result);
            assert_eq!(simplified, Expr::Const(1.0));
        })
        .it("derivative of power", || {
            let expr = Expr::var("x").pow(Expr::Const(3.0));
            let result = rvmath::algebra::derivative::derivative(&expr, "x");
            let simplified = rvmath::algebra::simplify::simplify(&result);
            assert!(simplified.to_string().contains("3"));
        })
        .it("derivative of product", || {
            let expr = Expr::var("x").mul(Expr::var("x"));
            let result = rvmath::algebra::derivative::derivative(&expr, "x");
            let simplified = rvmath::algebra::simplify::simplify(&result);
            assert!(simplified.to_string().contains("2"));
        })
        .it("derivative of sin is cos", || {
            let expr = Expr::Func("sin".to_string(), vec![Expr::var("x")]);
            let result = rvmath::algebra::derivative::derivative(&expr, "x");
            assert!(result.to_string().contains("cos"));
        })
        .it("derivative of exp is exp", || {
            let expr = Expr::Func("exp".to_string(), vec![Expr::var("x")]);
            let result = rvmath::algebra::derivative::derivative(&expr, "x");
            assert!(result.to_string().contains("exp"));
        })
        .it("derivative of ln", || {
            let expr = Expr::Func("ln".to_string(), vec![Expr::var("x")]);
            let result = rvmath::algebra::derivative::derivative(&expr, "x");
            assert!(result.to_string().contains("1") && result.to_string().contains("x"));
        })
        .tag("derivative")
        .run();

    describe("Rationalize")
        .it("rationalizes 1/sqrt(2)", || {
            let expr = Expr::Const(1.0).div(Expr::Func("sqrt".to_string(), vec![Expr::Const(2.0)]));
            let result = rvmath::algebra::rationalize::rationalize(&expr);
            assert!(result.to_string().contains("sqrt"));
        })
        .it("preserves constants", || {
            let expr = Expr::Const(5.0);
            let result = rvmath::algebra::rationalize::rationalize(&expr);
            assert_eq!(result, Expr::Const(5.0));
        })
        .it("preserves variables", || {
            let expr = Expr::var("x");
            let result = rvmath::algebra::rationalize::rationalize(&expr);
            assert_eq!(result, Expr::var("x"));
        })
        .it("processes simple division", || {
            let expr = Expr::Const(2.0).div(Expr::Const(3.0));
            let result = rvmath::algebra::rationalize::rationalize(&expr);
            assert!(!result.to_string().is_empty());
        })
        .it("preserves variable structure", || {
            let expr = Expr::var("x").add(Expr::Const(1.0));
            let result = rvmath::algebra::rationalize::rationalize(&expr);
            assert!(result.contains_var("x"));
        })
        .tag("rationalize")
        .run();

    describe("Evaluate")
        .it("evaluates constant expression", || {
            let expr = Expr::const_val(42.0);
            let vars = HashMap::new();
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert_eq!(result, 42.0);
        })
        .it("evaluates variable substitution", || {
            let expr = Expr::var("x");
            let mut vars = HashMap::new();
            vars.insert("x".to_string(), 5.0);
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert_eq!(result, 5.0);
        })
        .it("rejects undefined variable", || {
            let expr = Expr::var("x");
            let vars = HashMap::new();
            assert!(rvmath::algebra::evaluate::evaluate(&expr, &vars).is_err());
        })
        .it("evaluates addition", || {
            let expr = Expr::const_val(2.0).add(Expr::const_val(3.0));
            let vars = HashMap::new();
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert_eq!(result, 5.0);
        })
        .it("evaluates subtraction", || {
            let expr = Expr::const_val(7.0).sub(Expr::const_val(3.0));
            let vars = HashMap::new();
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert_eq!(result, 4.0);
        })
        .it("evaluates multiplication", || {
            let expr = Expr::const_val(4.0).mul(Expr::const_val(5.0));
            let vars = HashMap::new();
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert_eq!(result, 20.0);
        })
        .it("evaluates division", || {
            let expr = Expr::const_val(20.0).div(Expr::const_val(4.0));
            let vars = HashMap::new();
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert_eq!(result, 5.0);
        })
        .it("rejects division by zero", || {
            let expr = Expr::const_val(1.0).div(Expr::const_val(0.0));
            let vars = HashMap::new();
            assert!(rvmath::algebra::evaluate::evaluate(&expr, &vars).is_err());
        })
        .it("evaluates power", || {
            let expr = Expr::const_val(2.0).pow(Expr::const_val(3.0));
            let vars = HashMap::new();
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert_eq!(result, 8.0);
        })
        .it("evaluates negation", || {
            let expr = Expr::const_val(5.0).neg();
            let vars = HashMap::new();
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert_eq!(result, -5.0);
        })
        .it("evaluates trig functions", || {
            let vars = HashMap::new();
            let sin0 = Expr::Func("sin".to_string(), vec![Expr::const_val(0.0)]);
            assert!((rvmath::algebra::evaluate::evaluate(&sin0, &vars).unwrap() - 0.0).abs() < 1e-10);
            let cos0 = Expr::Func("cos".to_string(), vec![Expr::const_val(0.0)]);
            assert!((rvmath::algebra::evaluate::evaluate(&cos0, &vars).unwrap() - 1.0).abs() < 1e-10);
        })
        .it("evaluates sqrt", || {
            let expr = Expr::Func("sqrt".to_string(), vec![Expr::const_val(16.0)]);
            let vars = HashMap::new();
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert_eq!(result, 4.0);
        })
        .it("evaluates exp", || {
            let expr = Expr::Func("exp".to_string(), vec![Expr::const_val(0.0)]);
            let vars = HashMap::new();
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert!((result - 1.0).abs() < 1e-10);
        })
        .it("evaluates multi-variable expression", || {
            let expr = Expr::const_val(2.0).mul(Expr::var("x"))
                .add(Expr::const_val(3.0).mul(Expr::var("y")));
            let mut vars = HashMap::new();
            vars.insert("x".to_string(), 4.0);
            vars.insert("y".to_string(), 5.0);
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert_eq!(result, 23.0);
        })
        .it("evaluates polynomial", || {
            let expr = Expr::var("x").pow(Expr::const_val(2.0))
                .add(Expr::const_val(2.0).mul(Expr::var("x")))
                .add(Expr::const_val(1.0));
            let mut vars = HashMap::new();
            vars.insert("x".to_string(), 3.0);
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert_eq!(result, 16.0);
        })
        .it("domain errors return errors", || {
            let vars = HashMap::new();
            let sqrt_neg = Expr::Func("sqrt".to_string(), vec![Expr::const_val(-1.0)]);
            assert!(rvmath::algebra::evaluate::evaluate(&sqrt_neg, &vars).is_err());
            let ln_neg = Expr::Func("ln".to_string(), vec![Expr::const_val(-1.0)]);
            assert!(rvmath::algebra::evaluate::evaluate(&ln_neg, &vars).is_err());
        })
        .it("evaluates abs", || {
            let expr = Expr::Func("abs".to_string(), vec![Expr::const_val(-5.0)]);
            let vars = HashMap::new();
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert_eq!(result, 5.0);
        })
        .it("evaluates cbrt", || {
            let expr = Expr::Func("cbrt".to_string(), vec![Expr::const_val(8.0)]);
            let vars = HashMap::new();
            let result = rvmath::algebra::evaluate::evaluate(&expr, &vars).unwrap();
            assert_eq!(result, 2.0);
        })
        .tag("evaluate")
        .run();

    describe("API")
        .it("simplify string expression", || {
            let result = simplify("2+3").unwrap();
            assert_eq!(result, "5");
        })
        .it("derivative string expression", || {
            let result = derivative("x^2", "x").unwrap();
            assert!(result.contains("2") && result.contains("x"));
        })
        .it("rationalize string expression", || {
            let result = rationalize("1/sqrt(2)").unwrap();
            assert!(result.contains("sqrt"));
        })
        .it("rejects invalid expression", || {
            let result = simplify("x++");
            assert!(result.is_err());
        })
        .it("multi-variable derivative", || {
            let result = derivative("x*y + x^2", "x").unwrap();
            assert!(result.contains("y"));
        })
        .it("multi-variable derivative different var", || {
            let result = derivative("x*y + x^2", "y").unwrap();
            assert!(result.contains("x"));
        })
        .tag("api")
        .run();

    describe("Resolve")
        .it("simple variable substitution", || {
            let result = resolve("2*x + 3", &[("x", Num::new(4.0))]).unwrap();
            assert_eq!(result.value, 11.0);
        })
        .it("multi-variable resolve", || {
            let result = resolve("x*y + 5", &[("x", Num::new(2.0)), ("y", Num::new(3.0))]).unwrap();
            assert_eq!(result.value, 11.0);
        })
        .it("polynomial resolve", || {
            let result = resolve("x^2 + 2*x + 1", &[("x", Num::new(3.0))]).unwrap();
            assert_eq!(result.value, 16.0);
        })
        .it("function resolve", || {
            let result = resolve("sin(x)", &[("x", Num::new(0.0))]).unwrap();
            assert!(result.value.abs() < 1e-10);
        })
        .it("pythagorean resolve", || {
            let result = resolve("sqrt(x^2 + y^2)", &[("x", Num::new(3.0)), ("y", Num::new(4.0))]).unwrap();
            assert_eq!(result.value, 5.0);
        })
        .it("rejects undefined variable", || {
            let result = resolve("x + y", &[("x", Num::new(1.0))]);
            assert!(result.is_err());
        })
        .it("complex expression", || {
            let result = resolve("(a + b) * c - d / e", &[
                ("a", Num::new(2.0)), ("b", Num::new(3.0)),
                ("c", Num::new(2.0)), ("d", Num::new(10.0)), ("e", Num::new(2.0)),
            ]).unwrap();
            assert_eq!(result.value, 5.0);
        })
        .tag("resolve")
        .run();

    describe("MapResolve")
        .it("maps linear expression", || {
            let inputs = vec![Num::new(2.0), Num::new(3.0), Num::new(5.0), Num::new(7.0), Num::new(9.0)];
            let results = rvmath::algebra::map_resolve("4*x + 2", "x", &inputs).unwrap();
            assert_eq!(results.len(), 5);
            assert_eq!(results[0].value, 10.0);
            assert_eq!(results[4].value, 38.0);
        })
        .it("maps polynomial expression", || {
            let inputs = vec![Num::new(1.0), Num::new(2.0), Num::new(3.0)];
            let results = rvmath::algebra::map_resolve("x^2 + 1", "x", &inputs).unwrap();
            assert_eq!(results[2].value, 10.0);
        })
        .it("maps with functions", || {
            let inputs = vec![Num::new(0.0)];
            let results = rvmath::algebra::map_resolve("sin(x)", "x", &inputs).unwrap();
            assert!(results[0].value.abs() < 1e-10);
        })
        .it("handles empty array", || {
            let inputs: Vec<Num<f64>> = vec![];
            let results = rvmath::algebra::map_resolve("2*x", "x", &inputs).unwrap();
            assert!(results.is_empty());
        })
        .it("handles negative values", || {
            let inputs = vec![Num::new(-2.0), Num::new(-1.0), Num::new(0.0), Num::new(1.0), Num::new(2.0)];
            let results = rvmath::algebra::map_resolve("x^2", "x", &inputs).unwrap();
            assert_eq!(results[0].value, 4.0);
            assert_eq!(results[2].value, 0.0);
        })
        .it("rejects invalid expression", || {
            let inputs = vec![Num::new(1.0), Num::new(2.0)];
            assert!(rvmath::algebra::map_resolve("x++", "x", &inputs).is_err());
        })
        .it("rejects undefined variable", || {
            let inputs = vec![Num::new(1.0), Num::new(2.0)];
            assert!(rvmath::algebra::map_resolve("x + y", "x", &inputs).is_err());
        })
        .tag("map_resolve")
        .run()
        .assert_all_pass();
}
