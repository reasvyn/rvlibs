use rvtest::spec::describe;
use rvmath::ops::*;

const EPS: f64 = 1e-12;

fn is_nan_f64(x: impl Into<f64>) -> bool { x.into().is_nan() }
fn is_inf_f64(x: impl Into<f64>) -> bool { x.into().is_infinite() }
fn abs_f64(x: impl Into<f64>) -> f64 { x.into().abs() }

#[test]
fn ops_tests() {
    describe("Basic Arithmetic")
        .it("add", || {
            assert_eq!(arithmetic::add(2.0_f64, 3.0_f64), 5.0);
            assert_eq!(arithmetic::add(2.0_f64, -3.0_f64), -1.0);
            assert_eq!(arithmetic::add(0.0_f64, 0.0_f64), 0.0);
            assert!(is_nan_f64(arithmetic::add(f64::NAN, 1.0_f64)));
            assert_eq!(arithmetic::add(2i32, 3i32), 5);
        })
        .it("sub", || {
            assert_eq!(arithmetic::sub(5.0_f64, 3.0_f64), 2.0);
            assert_eq!(arithmetic::sub(3.0_f64, 5.0_f64), -2.0);
            assert_eq!(arithmetic::sub(0.0_f64, 0.0_f64), 0.0);
            assert!(is_nan_f64(arithmetic::sub(f64::NAN, 1.0_f64)));
            assert_eq!(arithmetic::sub(5i32, 3i32), 2);
        })
        .it("mul", || {
            assert_eq!(arithmetic::mul(4.0_f64, 5.0_f64), 20.0);
            assert_eq!(arithmetic::mul(-2.0_f64, 3.0_f64), -6.0);
            assert_eq!(arithmetic::mul(0.0_f64, 5.0_f64), 0.0);
            assert!(is_nan_f64(arithmetic::mul(f64::INFINITY, 0.0_f64)));
            assert_eq!(arithmetic::mul(4i32, 5i32), 20);
        })
        .it("div", || {
            assert_eq!(arithmetic::div(10.0_f64, 2.0_f64), 5.0);
            assert_eq!(arithmetic::div(5.0_f64, 2.0_f64), 2.5);
            assert!(is_inf_f64(arithmetic::div(1.0_f64, 0.0_f64)));
            assert!(is_nan_f64(arithmetic::div(0.0_f64, 0.0_f64)));
            assert_eq!(arithmetic::div(10i32, 3i32), 3);
        })
        .it("rem", || {
            assert_eq!(arithmetic::rem(10.0_f64, 3.0_f64), 1.0);
            assert_eq!(arithmetic::rem(-10.0_f64, 3.0_f64), -1.0);
            assert!(is_nan_f64(arithmetic::rem(1.0_f64, 0.0_f64)));
            assert_eq!(arithmetic::rem(10i32, 3i32), 1);
        })
        .it("neg", || {
            assert_eq!(arithmetic::neg(5.0_f64), -5.0);
            assert_eq!(arithmetic::neg(-3.0_f64), 3.0);
            assert_eq!(arithmetic::neg(0.0_f64), 0.0);
            assert_eq!(arithmetic::neg(5i32), -5);
        })
        .it("abs", || {
            assert_eq!(arithmetic::abs(5.0_f64), 5.0);
            assert_eq!(arithmetic::abs(-5.0_f64), 5.0);
            assert_eq!(arithmetic::abs(0.0_f64), 0.0);
            assert!(is_nan_f64(arithmetic::abs(f64::NAN)));
            assert_eq!(arithmetic::abs(-5i32), 5);
        })
        .tag("basic")
        .run();

    describe("Power and Roots")
        .it("pow", || {
            assert_eq!(arithmetic::pow(2.0_f64, 3.0_f64), 8.0);
            assert_eq!(arithmetic::pow(4.0_f64, 0.5_f64), 2.0);
            assert!(abs_f64(arithmetic::pow(2.0_f64, -1.0_f64) - 0.5_f64) < EPS);
            assert_eq!(arithmetic::pow(0.0_f64, 0.0_f64), 1.0);
            assert_eq!(arithmetic::pow(2i32, 3i32), 8);
        })
        .it("powf", || {
            assert_eq!(arithmetic::powf(2.0_f64, 3.0_f64), 8.0);
            assert!(abs_f64(arithmetic::powf(2.0_f64, -1.0_f64) - 0.5_f64) < EPS);
        })
        .it("powi", || {
            assert_eq!(arithmetic::powi(2.0_f64, 3), 8.0);
            assert!(abs_f64(arithmetic::powi(2.0_f64, -1) - 0.5_f64) < EPS);
            assert_eq!(arithmetic::powi(4.0_f64, 0), 1.0);
            assert_eq!(arithmetic::powi(2i32, 3), 8);
        })
        .it("sqrt", || {
            assert_eq!(arithmetic::sqrt(4.0_f64), 2.0);
            assert!(is_nan_f64(arithmetic::sqrt(-1.0_f64)));
            assert_eq!(arithmetic::sqrt(0.0_f64), 0.0);
            assert!(abs_f64(arithmetic::sqrt(2.0_f64) - 2.0_f64.sqrt()) < EPS);
        })
        .it("cbrt", || {
            assert_eq!(arithmetic::cbrt(8.0_f64), 2.0);
            assert_eq!(arithmetic::cbrt(-8.0_f64), -2.0);
            assert_eq!(arithmetic::cbrt(0.0_f64), 0.0);
        })
        .it("root", || {
            assert_eq!(arithmetic::root(8.0_f64, 3.0_f64), 2.0);
            assert!(is_nan_f64(arithmetic::root(2.0_f64, 0.0_f64)));
        })
        .it("recip", || {
            assert_eq!(arithmetic::recip(2.0_f64), 0.5);
            assert!(is_inf_f64(arithmetic::recip(0.0_f64)));
            assert_eq!(arithmetic::recip(-2.0_f64), -0.5);
            assert_eq!(arithmetic::recip(5i32), 0);
        })
        .it("exp / exp_m1", || {
            assert_eq!(arithmetic::exp(0.0_f64), 1.0);
            assert!(abs_f64(arithmetic::exp(1.0_f64) - std::f64::consts::E) < EPS);
            assert_eq!(arithmetic::exp_m1(0.0_f64), 0.0);
            assert!(abs_f64(arithmetic::exp_m1(1.0_f64) - (std::f64::consts::E - 1.0)) < EPS);
        })
        .it("hypot", || {
            assert_eq!(arithmetic::hypot(3.0_f64, 4.0_f64), 5.0);
            assert!(is_inf_f64(arithmetic::hypot(f64::INFINITY, 1.0_f64)));
            assert_eq!(arithmetic::hypot(0.0_f64, 0.0_f64), 0.0);
        })
        .tag("power_roots")
        .run();

    describe("Sign and Comparison")
        .it("sign", || {
            assert_eq!(arithmetic::sign(5.0_f64), 1.0);
            assert_eq!(arithmetic::sign(-5.0_f64), -1.0);
            assert_eq!(arithmetic::sign(0.0_f64), 0.0);
            assert_eq!(arithmetic::sign(5i32), 1);
            assert_eq!(arithmetic::sign(-5i32), -1);
        })
        .it("min / max", || {
            assert_eq!(arithmetic::min(3.0_f64, 5.0_f64), 3.0);
            assert_eq!(arithmetic::max(3.0_f64, 5.0_f64), 5.0);
            assert_eq!(arithmetic::min(3i32, 5i32), 3);
            assert_eq!(arithmetic::max(3i32, 5i32), 5);
        })
        .it("clamp", || {
            assert_eq!(arithmetic::clamp(5.0_f64, 0.0_f64, 10.0_f64), 5.0);
            assert_eq!(arithmetic::clamp(-1.0_f64, 0.0_f64, 10.0_f64), 0.0);
            assert_eq!(arithmetic::clamp(15.0_f64, 0.0_f64, 10.0_f64), 10.0);
            assert_eq!(arithmetic::clamp(5i32, 0i32, 10i32), 5);
        })
        .tag("sign_compare")
        .run();

    describe("Rounding")
        .it("round", || {
            assert_eq!(arithmetic::round(2.5_f64), 3.0);
            assert_eq!(arithmetic::round(2.4_f64), 2.0);
            assert_eq!(arithmetic::round(-2.5_f64), -3.0);
            assert_eq!(arithmetic::round(5i32), 5);
        })
        .it("floor", || {
            assert_eq!(arithmetic::floor(2.7_f64), 2.0);
            assert_eq!(arithmetic::floor(-2.3_f64), -3.0);
            assert_eq!(arithmetic::floor(5i32), 5);
        })
        .it("ceil", || {
            assert_eq!(arithmetic::ceil(2.3_f64), 3.0);
            assert_eq!(arithmetic::ceil(-2.7_f64), -2.0);
            assert_eq!(arithmetic::ceil(5i32), 5);
        })
        .it("fract", || {
            assert!(abs_f64(arithmetic::fract(2.5_f64) - 0.5_f64) < EPS);
            assert!(abs_f64(arithmetic::fract(-1.3_f64) - (-0.3_f64)) < EPS);
            assert_eq!(arithmetic::fract(0.0_f64), 0.0);
            assert_eq!(arithmetic::fract(5i32), 0);
        })
        .tag("rounding")
        .run();

    describe("Utility")
        .it("lerp", || {
            assert_eq!(arithmetic::lerp(0.0_f64, 10.0_f64, 0.5_f64), 5.0);
            assert_eq!(arithmetic::lerp(0.0_f64, 10.0_f64, 0.0_f64), 0.0);
            assert_eq!(arithmetic::lerp(0.0_f64, 10.0_f64, 1.0_f64), 10.0);
        })
        .it("map_range", || {
            assert_eq!(arithmetic::map_range(5.0_f64, 0.0_f64, 10.0_f64, 0.0_f64, 100.0_f64), 50.0);
            assert_eq!(arithmetic::map_range(0.0_f64, 0.0_f64, 10.0_f64, 0.0_f64, 100.0_f64), 0.0);
            assert_eq!(arithmetic::map_range(10.0_f64, 0.0_f64, 10.0_f64, 0.0_f64, 100.0_f64), 100.0);
        })
        .it("to_degrees / to_radians", || {
            assert_eq!(arithmetic::to_degrees(std::f64::consts::PI), 180.0);
            assert!(abs_f64(arithmetic::to_radians(180.0_f64) - std::f64::consts::PI) < EPS);
        })
        .tag("utility")
        .run();

    describe("Classification")
        .it("is_nan / is_infinite / is_finite", || {
            assert!(is_nan_f64(arithmetic::is_nan(f64::NAN)));
            assert!(!is_nan_f64(arithmetic::is_nan(1.0_f64)));
            assert!(is_inf_f64(arithmetic::is_infinite(f64::INFINITY)));
            assert!(is_inf_f64(arithmetic::is_infinite(f64::NEG_INFINITY)));
            assert!(!is_inf_f64(arithmetic::is_infinite(1.0_f64)));
            assert!(arithmetic::is_finite(1.0_f64));
            assert!(!arithmetic::is_finite(f64::INFINITY));
        })
        .tag("classification")
        .run();

    describe("Constants")
        .it("pi, e, tau, phi", || {
            assert!(abs_f64(arithmetic::pi::<f64>() - std::f64::consts::PI) < EPS);
            assert!(abs_f64(arithmetic::e::<f64>() - std::f64::consts::E) < EPS);
            assert!(abs_f64(arithmetic::tau::<f64>() - std::f64::consts::TAU) < EPS);
            assert!(abs_f64(arithmetic::phi::<f64>() - 1.618033988749895) < EPS);
            assert_eq!(arithmetic::pi::<i32>(), 3);
        })
        .tag("constants")
        .run();

    describe("Trigonometry")
        .it("sin", || {
            assert_eq!(trig::sin(0.0_f64), 0.0);
            assert!(abs_f64(trig::sin(std::f64::consts::FRAC_PI_2) - 1.0_f64) < EPS);
            assert!(is_nan_f64(trig::sin(f64::NAN)));
        })
        .it("cos", || {
            assert_eq!(trig::cos(0.0_f64), 1.0);
            assert!(abs_f64(trig::cos(std::f64::consts::PI) - (-1.0_f64)) < EPS);
        })
        .it("tan", || {
            assert_eq!(trig::tan(0.0_f64), 0.0);
            assert!(abs_f64(trig::tan(std::f64::consts::FRAC_PI_4) - 1.0_f64) < EPS);
        })
        .it("asin", || {
            assert_eq!(trig::asin(0.0_f64), 0.0);
            assert!(abs_f64(trig::asin(1.0_f64) - std::f64::consts::FRAC_PI_2) < EPS);
            assert!(is_nan_f64(trig::asin(2.0_f64)));
        })
        .it("acos", || {
            assert!(abs_f64(trig::acos(1.0_f64) - 0.0_f64) < EPS);
            assert!(is_nan_f64(trig::acos(2.0_f64)));
        })
        .it("atan", || {
            assert_eq!(trig::atan(0.0_f64), 0.0);
            assert!(abs_f64(trig::atan(1.0_f64) - std::f64::consts::FRAC_PI_4) < EPS);
        })
        .it("atan2", || {
            assert_eq!(trig::atan2(0.0_f64, 1.0_f64), 0.0);
            assert!(abs_f64(trig::atan2(1.0_f64, 1.0_f64) - std::f64::consts::FRAC_PI_4) < EPS);
        })
        .tag("trig")
        .run();

    describe("Hyperbolic")
        .it("sinh", || {
            assert_eq!(hyperbolic::sinh(0.0_f64), 0.0);
            assert!(abs_f64(hyperbolic::sinh(1.0_f64) - 1.0_f64.sinh()) < EPS);
            assert_eq!(hyperbolic::sinh(-1.0_f64), -hyperbolic::sinh(1.0_f64));
        })
        .it("cosh", || {
            assert_eq!(hyperbolic::cosh(0.0_f64), 1.0);
            assert_eq!(hyperbolic::cosh(-1.0_f64), hyperbolic::cosh(1.0_f64));
        })
        .it("tanh", || {
            assert_eq!(hyperbolic::tanh(0.0_f64), 0.0);
            assert!(abs_f64(hyperbolic::tanh(f64::INFINITY) - 1.0_f64) < EPS);
        })
        .tag("hyperbolic")
        .run();

    describe("Logarithm")
        .it("ln", || {
            assert_eq!(logarithm::ln(1.0_f64), 0.0);
            assert!(abs_f64(logarithm::ln(std::f64::consts::E) - 1.0_f64) < EPS);
            assert!(is_inf_f64(logarithm::ln(0.0_f64)));
            assert!(is_nan_f64(logarithm::ln(-1.0_f64)));
        })
        .it("log", || {
            assert_eq!(logarithm::log(100.0_f64, 10.0_f64), 2.0);
            assert_eq!(logarithm::log(8.0_f64, 2.0_f64), 3.0);
            assert!(is_nan_f64(logarithm::log(-1.0_f64, 10.0_f64)));
        })
        .it("log10", || {
            assert_eq!(logarithm::log10(100.0_f64), 2.0);
            assert_eq!(logarithm::log10(1.0_f64), 0.0);
            assert!(is_inf_f64(logarithm::log10(0.0_f64)));
        })
        .it("log2", || {
            assert_eq!(logarithm::log2(8.0_f64), 3.0);
            assert_eq!(logarithm::log2(1.0_f64), 0.0);
        })
        .it("ln_1p", || {
            assert_eq!(logarithm::ln_1p(0.0_f64), 0.0);
            assert!(abs_f64(logarithm::ln_1p(1.0_f64) - 0.6931471805599453_f64) < 1e-10);
            assert!(is_inf_f64(logarithm::ln_1p(-1.0_f64)));
        })
        .tag("logarithm")
        .run()
        .assert_all_pass();
}
