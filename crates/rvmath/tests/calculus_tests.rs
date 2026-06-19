use rvtest::spec::describe;
use rvmath::calculus::{
    arccos_derivative, arcsin_derivative, arcsin_integral, arctan_derivative, arctan_integral,
    binomial_series, bisection, central_difference, cos_derivative, cos_integral, cosh_derivative,
    exp_base_derivative, exp_base_integral, exp_derivative, exp_integral, factorial,
    forward_difference, ln_derivative, ln_integral, log_base_derivative, maclaurin_arctan,
    maclaurin_cos, maclaurin_exp, maclaurin_ln1p, maclaurin_sin, newton_raphson, power_integral,
    power_rule, sec2_integral, second_derivative, simpsons_rule, sin_derivative, sin_integral,
    sinh_derivative, tan_derivative, tanh_derivative, trapezoidal_rule,
};
use rvmath::calculus_constants::{
    APERY, CATALAN, EULER_MASCHERONI, EULER_MASCHERONI_RECIP, FEIGENBAUM_ALPHA, FEIGENBAUM_DELTA,
};

const EPS: f64 = 1e-10;

#[test]
fn calculus_tests() {
    describe("Numerical Methods")
        .it("newton_raphson finds root of x²-4", || {
            let f = |x: f64| x * x - 4.0;
            let df = |x: f64| 2.0 * x;
            let root = newton_raphson(f, df, 1.0, 1e-10, 100);
            assert!((root - 2.0).abs() < 1e-8);
        })
        .it("bisection finds root of x²-4", || {
            let f = |x: f64| x * x - 4.0;
            let root = bisection(f, 1.0, 3.0, 1e-10, 100);
            assert!((root - 2.0).abs() < 1e-8);
        })
        .it("forward_difference approximates derivative of x²", || {
            let f = |x: f64| x * x;
            let deriv = forward_difference(f, 3.0, 1.5e-8);
            assert!((deriv - 6.0).abs() < 1e-5);
        })
        .it("central_difference approximates derivative of x²", || {
            let f = |x: f64| x * x;
            let deriv = central_difference(f, 3.0, 1.5e-8);
            assert!((deriv - 6.0).abs() < 1e-5);
        })
        .it("second_derivative of x² is 2", || {
            let f = |x: f64| x * x;
            let deriv2 = second_derivative(f, 3.0, 1.5e-5);
            assert!((deriv2 - 2.0).abs() < 1e-3);
        })
        .tag("numerical")
        .run();

    describe("Derivative Formulas")
        .it("power_rule", || {
            assert!((power_rule::<f64>(3.0, 2.0) - 6.0).abs() < EPS);
            assert!((power_rule::<f64>(2.0, 3.0) - 12.0).abs() < EPS);
            assert!((power_rule::<f64>(5.0, 1.0) - 1.0).abs() < EPS);
        })
        .it("exp_derivative", || {
            assert!((exp_derivative::<f64>(0.0) - 1.0).abs() < EPS);
            assert!((exp_derivative::<f64>(1.0) - std::f64::consts::E).abs() < EPS);
        })
        .it("ln_derivative", || {
            assert!((ln_derivative::<f64>(1.0) - 1.0).abs() < EPS);
            assert!((ln_derivative::<f64>(2.0) - 0.5).abs() < EPS);
        })
        .it("trig derivatives at zero", || {
            assert!((sin_derivative::<f64>(0.0) - 1.0).abs() < EPS);
            assert!((cos_derivative::<f64>(0.0) - 0.0).abs() < EPS);
            assert!((tan_derivative::<f64>(0.0) - 1.0).abs() < EPS);
        })
        .it("hyperbolic derivatives at zero", || {
            assert!((sinh_derivative::<f64>(0.0) - 1.0).abs() < EPS);
            assert!((cosh_derivative::<f64>(0.0) - 0.0).abs() < EPS);
            assert!((tanh_derivative::<f64>(0.0) - 1.0).abs() < EPS);
        })
        .it("inverse trig derivatives at zero", || {
            assert!((arcsin_derivative::<f64>(0.0) - 1.0).abs() < EPS);
            assert!((arccos_derivative::<f64>(0.0) - (-1.0)).abs() < EPS);
            assert!((arctan_derivative::<f64>(0.0) - 1.0).abs() < EPS);
        })
        .tag("derivatives")
        .run();

    describe("Integral Formulas")
        .it("power_integral", || {
            assert!((power_integral::<f64>(3.0, 2.0) - 9.0).abs() < EPS);
            assert!((power_integral::<f64>(2.0, 3.0) - 4.0).abs() < EPS);
        })
        .it("exp_integral", || {
            assert!((exp_integral::<f64>(0.0) - 1.0).abs() < EPS);
        })
        .it("ln_integral at 1 is 0", || {
            assert!(ln_integral::<f64>(1.0).abs() < EPS);
        })
        .it("trig integrals at zero", || {
            assert!((sin_integral::<f64>(0.0) - (-1.0)).abs() < EPS);
            assert!(cos_integral::<f64>(0.0).abs() < EPS);
        })
        .it("sec2_integral", || {
            assert!((sec2_integral(0.0) - 0.0_f64).abs() < EPS);
            assert!((sec2_integral(std::f64::consts::FRAC_PI_4) - 1.0).abs() < EPS);
        })
        .it("arctan_integral", || {
            assert!((arctan_integral(0.0) - 0.0_f64).abs() < EPS);
            assert!((arctan_integral(1.0) - std::f64::consts::FRAC_PI_4).abs() < EPS);
        })
        .it("arcsin_integral", || {
            assert!((arcsin_integral(0.0) - 0.0_f64).abs() < EPS);
            assert!((arcsin_integral(1.0) - std::f64::consts::FRAC_PI_2).abs() < EPS);
        })
        .tag("integrals")
        .run();

    describe("Numerical Integration")
        .it("simpsons_rule integrates x² from 0 to 3", || {
            let result = simpsons_rule(|x: f64| x * x, 0.0, 3.0, 100);
            assert!((result - 9.0).abs() < 0.01);
        })
        .it("trapezoidal_rule integrates x² from 0 to 3", || {
            let result = trapezoidal_rule(|x: f64| x * x, 0.0, 3.0, 100);
            assert!((result - 9.0).abs() < 0.1);
        })
        .tag("numerical_integration")
        .run();

    describe("Constants")
        .it("euler_mascheroni value", || {
            assert!(EULER_MASCHERONI > 0.5 && EULER_MASCHERONI < 0.6);
            assert!(f64::abs(EULER_MASCHERONI - 0.5772156649) < 1e-10);
        })
        .it("catalan value", || {
            assert!(CATALAN > 0.9 && CATALAN < 1.0);
            assert!(f64::abs(CATALAN - 0.915965594) < 1e-9);
        })
        .it("apery value", || {
            assert!(APERY > 1.2 && APERY < 1.3);
            assert!(f64::abs(APERY - 1.202056903) < 1e-9);
        })
        .it("feigenbaum constants range", || {
            assert!(FEIGENBAUM_DELTA > 4.6 && FEIGENBAUM_DELTA < 4.7);
            assert!(FEIGENBAUM_ALPHA > 2.5 && FEIGENBAUM_ALPHA < 2.51);
        })
        .it("euler_mascheroni_recip matches 1/γ", || {
            let recalc = 1.0 / EULER_MASCHERONI;
            assert!(f64::abs(EULER_MASCHERONI_RECIP - recalc) < 1e-15);
        })
        .tag("constants")
        .run();

    describe("Series")
        .it("factorial of small integers", || {
            assert_eq!(factorial(0), 1.0);
            assert_eq!(factorial(1), 1.0);
            assert_eq!(factorial(5), 120.0);
            assert_eq!(factorial(10), 3628800.0);
        })
        .it("maclaurin_exp converges to e", || {
            assert!((maclaurin_exp(0.0, 20) - 1.0_f64).abs() < EPS);
            assert!((maclaurin_exp(1.0, 20) - std::f64::consts::E).abs() < EPS);
        })
        .it("maclaurin_sin at π/2 is 1", || {
            assert!(f64::abs(maclaurin_sin(0.0, 20)) < EPS);
            assert!((maclaurin_sin(std::f64::consts::PI / 2.0, 20) - 1.0).abs() < EPS);
        })
        .it("maclaurin_cos at π is -1", || {
            assert!((maclaurin_cos(0.0, 20) - 1.0_f64).abs() < EPS);
            assert!((maclaurin_cos(std::f64::consts::PI, 20) - (-1.0_f64)).abs() < EPS);
        })
        .it("maclaurin_arctan at 0 is 0", || {
            assert!(f64::abs(maclaurin_arctan(0.0, 50)) < EPS);
        })
        .it("maclaurin_ln1p converges", || {
            assert!(f64::abs(maclaurin_ln1p(0.0, 20)) < EPS);
            assert!((maclaurin_ln1p(0.5, 200) - 0.5_f64.ln_1p()).abs() < 1e-6);
        })
        .it("binomial_series at 0 is 1", || {
            assert!((binomial_series(0.0, 0.5, 50) - 1.0_f64).abs() < EPS);
        })
        .tag("series")
        .run();

    describe("Base Derivative/Integral")
        .it("exp_base_derivative", || {
            let d = exp_base_derivative(0.0, 2.0);
            assert!((d - 2.0_f64.ln()).abs() < EPS);
            let d = exp_base_derivative(1.0, 2.0);
            assert!((d - (2.0 * 2.0_f64.ln())).abs() < EPS);
        })
        .it("log_base_derivative", || {
            let d = log_base_derivative(1.0, 10.0);
            assert!((d - 1.0 / 10.0_f64.ln()).abs() < EPS);
        })
        .it("exp_base_integral", || {
            let i = exp_base_integral(0.0, 2.0);
            assert!((i - 1.0 / 2.0_f64.ln()).abs() < EPS);
        })
        .it("maclaurin_ln1p at 0.5", || {
            let approx = maclaurin_ln1p(0.5, 200);
            assert!((approx - 0.5_f64.ln_1p()).abs() < 1e-6);
        })
        .tag("base")
        .run()
        .assert_all_pass();
}
