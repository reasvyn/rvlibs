use rvtest::spec::describe;
use rvmath::utils::evaluate;

#[test]
fn evaluator_tests() {
    describe("Basic Arithmetic")
        .it("addition", || {
            let result = evaluate("2 + 3");
            assert_eq!(result.value, 5.0);
        })
        .it("subtraction", || {
            let result = evaluate("10 - 4");
            assert_eq!(result.value, 6.0);
        })
        .it("multiplication", || {
            let result = evaluate("5 * 3");
            assert_eq!(result.value, 15.0);
        })
        .it("division", || {
            let result = evaluate("20 / 4");
            assert_eq!(result.value, 5.0);
        })
        .tag("basic_arithmetic")
        .run();

    describe("Operator Precedence")
        .it("multiplication before addition", || {
            let result = evaluate("2 + 3 * 4");
            assert_eq!(result.value, 14.0);
        })
        .it("division before subtraction", || {
            let result = evaluate("20 - 10 / 2");
            assert_eq!(result.value, 15.0);
        })
        .it("parentheses override precedence", || {
            let result = evaluate("(2 + 3) * 4");
            assert_eq!(result.value, 20.0);
        })
        .it("nested parentheses", || {
            let result = evaluate("((2 + 3) * 4) / 2");
            assert_eq!(result.value, 10.0);
        })
        .it("exponentiation", || {
            let result = evaluate("2 ^ 3");
            assert_eq!(result.value, 8.0);
        })
        .it("exponent before multiplication", || {
            let result = evaluate("2 * 3 ^ 2");
            assert_eq!(result.value, 18.0);
        })
        .it("right associativity of power", || {
            let result = evaluate("2 ^ 3 ^ 2");
            assert_eq!(result.value, 512.0);  // 2^(3^2) = 2^9 = 512
        })
        .tag("precedence")
        .run();

    describe("Unary and Edge Cases")
        .it("unary negation", || {
            let result = evaluate("-5 + 3");
            assert_eq!(result.value, -2.0);
        })
        .it("division by zero returns nan", || {
            let result = evaluate("1 / 0");
            assert!(result.value.is_nan());
        })
        .it("sqrt of negative returns nan", || {
            let result = evaluate("sqrt(-4)");
            assert!(result.value.is_nan());
        })
        .it("whitespace handling", || {
            let result = evaluate("  2  +  3  *  4  ");
            assert_eq!(result.value, 14.0);
        })
        .it("decimal numbers", || {
            let result = evaluate("3.5 * 2");
            assert_eq!(result.value, 7.0);
        })
        .it("invalid expression returns nan", || {
            let result = evaluate("2 +");
            assert!(result.value.is_nan());
        })
        .it("mismatched parentheses returns nan", || {
            let result = evaluate("(2 + 3) * 4)");
            assert!(result.value.is_nan());
        })
        .it("chained operations", || {
            let result = evaluate("10 + 20 - 5 * 2 / 2 + 3");
            assert_eq!(result.value, 28.0);
        })
        .tag("edge")
        .run();

    describe("Functions")
        .it("sqrt", || {
            let result = evaluate("sqrt(16)");
            assert_eq!(result.value, 4.0);
        })
        .it("sqrt in expression", || {
            let result = evaluate("sqrt(16) + 6");
            assert_eq!(result.value, 10.0);
        })
        .it("sin(0) = 0", || {
            let result = evaluate("sin(0)");
            assert_eq!(result.value, 0.0);
        })
        .it("cos(0) = 1", || {
            let result = evaluate("cos(0)");
            assert_eq!(result.value, 1.0);
        })
        .it("tan(0) = 0", || {
            let result = evaluate("tan(0)");
            assert_eq!(result.value, 0.0);
        })
        .it("cbrt", || {
            let result = evaluate("cbrt(8)");
            assert_eq!(result.value, 2.0);
        })
        .it("abs", || {
            let result = evaluate("abs(-5)");
            assert_eq!(result.value, 5.0);
        })
        .it("ln", || {
            let result = evaluate("ln(2.718281828)");
            assert!((result.value - 1.0).abs() < 0.0001);
        })
        .it("exp", || {
            let result = evaluate("exp(0)");
            assert_eq!(result.value, 1.0);
        })
        .it("floor", || {
            let result = evaluate("floor(3.7)");
            assert_eq!(result.value, 3.0);
        })
        .it("ceil", || {
            let result = evaluate("ceil(3.2)");
            assert_eq!(result.value, 4.0);
        })
        .it("round", || {
            let result = evaluate("round(3.5)");
            assert_eq!(result.value, 4.0);
        })
        .it("to_degrees", || {
            let result = evaluate("to_degrees(3.141592653589793)");
            assert!((result.value - 180.0).abs() < 0.001);
        })
        .it("to_radians", || {
            let result = evaluate("to_radians(180)");
            assert!((result.value - std::f64::consts::PI).abs() < 0.001);
        })
        .it("recip", || {
            let result = evaluate("recip(2)");
            assert_eq!(result.value, 0.5);
        })
        .it("inv", || {
            let result = evaluate("inv(4)");
            assert_eq!(result.value, 0.25);
        })
        .it("sinh", || {
            let result = evaluate("sinh(0)");
            assert_eq!(result.value, 0.0);
        })
        .it("cosh", || {
            let result = evaluate("cosh(0)");
            assert_eq!(result.value, 1.0);
        })
        .it("tanh", || {
            let result = evaluate("tanh(0)");
            assert_eq!(result.value, 0.0);
        })
        .it("fract", || {
            let result = evaluate("fract(3.7)");
            assert!((result.value - 0.7).abs() < 0.0001);
        })
        .it("sign", || {
            let result = evaluate("sign(5)");
            assert_eq!(result.value, 1.0);
        })
        .it("log10", || {
            let result = evaluate("log10(100)");
            assert_eq!(result.value, 2.0);
        })
        .it("log2", || {
            let result = evaluate("log2(8)");
            assert_eq!(result.value, 3.0);
        })
        .it("atan2", || {
            let result = evaluate("atan2(0, 1)");
            assert!((result.value - 0.0).abs() < 1e-10);
        })
        .it("min", || {
            let result = evaluate("min(3, 7)");
            assert_eq!(result.value, 3.0);
        })
        .it("max", || {
            let result = evaluate("max(3, 7)");
            assert_eq!(result.value, 7.0);
        })
        .it("pow", || {
            let result = evaluate("pow(2, 3)");
            assert_eq!(result.value, 8.0);
        })
        .it("hypot(3,4) = 5", || {
            let result = evaluate("hypot(3, 4)");
            assert!((result.value - 5.0).abs() < 1e-10);
        })
        .it("modulo", || {
            let result = evaluate("17 % 5");
            assert_eq!(result.value, 2.0);
        })
        .tag("functions")
        .run()
        .assert_all_pass();
}
