use rvtest::spec::describe;
use rvmath::num::{Complex, Num, NumberKind, NumberSet, Numeric};

#[test]
fn complex_tests() {
    describe("Construction")
        .it("new", || {
            assert_eq!(Complex::new(3.0, 4.0), Complex { re: 3.0, im: 4.0 });
        })
        .it("from_real", || {
            let z = Complex::<f64>::from_real(5.0);
            assert_eq!(z.re, 5.0);
            assert_eq!(z.im, 0.0);
        })
        .it("from_imag", || {
            let z = Complex::<f64>::from_imag(3.0);
            assert_eq!(z.re, 0.0);
            assert_eq!(z.im, 3.0);
        })
        .it("from_polar", || {
            let z = Complex::<f64>::from_polar(1.0, std::f64::consts::FRAC_PI_2);
            assert!((z.re - 0.0).abs() < 1e-10);
            assert!((z.im - 1.0).abs() < 1e-10);
        })
        .it("zero, one, i", || {
            assert_eq!(Complex::<f64>::zero(), Complex::new(0.0, 0.0));
            assert_eq!(Complex::<f64>::one(), Complex::new(1.0, 0.0));
            assert_eq!(Complex::<f64>::i(), Complex::new(0.0, 1.0));
        })
        .it("default is zero", || {
            let z: Complex<f64> = Default::default();
            assert_eq!(z.re, 0.0);
            assert_eq!(z.im, 0.0);
        })
        .tag("construction")
        .run();

    describe("Properties")
        .it("is_real / is_imag", || {
            assert!(Complex::<f64>::new(5.0, 0.0).is_real());
            assert!(!Complex::new(5.0, 1.0).is_real());
            assert!(Complex::<f64>::new(0.0, 3.0).is_imag());
            assert!(!Complex::new(1.0, 3.0).is_imag());
        })
        .it("conj", || {
            assert_eq!(Complex::new(3.0, 4.0).conj(), Complex::new(3.0, -4.0));
        })
        .it("norm_sqr and norm", || {
            assert_eq!(Complex::new(3.0, 4.0).norm_sqr(), 25.0);
            assert_eq!(Complex::new(3.0, 4.0).norm(), 5.0);
        })
        .it("arg", || {
            let z = Complex::new(1.0, 1.0);
            assert!((z.arg() - std::f64::consts::FRAC_PI_4).abs() < 1e-10);
        })
        .tag("properties")
        .run();

    describe("Display")
        .it("real only", || {
            assert_eq!(format!("{}", Complex::new(3.0, 0.0)), "3");
            assert_eq!(format!("{}", Complex::new(-3.0, 0.0)), "-3");
        })
        .it("imag only", || {
            assert_eq!(format!("{}", Complex::new(0.0, 4.0)), "4i");
            assert_eq!(format!("{}", Complex::new(0.0, -4.0)), "-4i");
        })
        .it("complex", || {
            assert_eq!(format!("{}", Complex::new(3.0, 4.0)), "3 + 4i");
            assert_eq!(format!("{}", Complex::new(3.0, -4.0)), "3 - 4i");
            assert_eq!(format!("{}", Complex::new(-3.0, -4.0)), "-3 - 4i");
        })
        .tag("display")
        .run();

    describe("Arithmetic")
        .it("neg", || {
            assert_eq!(-Complex::new(3.0, -4.0), Complex::new(-3.0, 4.0));
            assert_eq!(-Complex::new(0.0, 0.0), Complex::new(0.0, 0.0));
        })
        .it("add", || {
            assert_eq!(Complex::new(1.0, 2.0) + Complex::new(3.0, 4.0), Complex::new(4.0, 6.0));
            assert_eq!(Complex::new(1.0, 2.0) + 5.0, Complex::new(6.0, 2.0));
        })
        .it("sub", || {
            assert_eq!(Complex::new(5.0, 6.0) - Complex::new(3.0, 4.0), Complex::new(2.0, 2.0));
            assert_eq!(Complex::new(5.0, 6.0) - 3.0, Complex::new(2.0, 6.0));
        })
        .it("mul", || {
            assert_eq!(Complex::new(1.0, 2.0) * Complex::new(3.0, 4.0), Complex::new(-5.0, 10.0));
            assert_eq!(Complex::new(1.0, 2.0) * 3.0, Complex::new(3.0, 6.0));
        })
        .it("i * i = -1", || {
            assert_eq!(Complex::<f64>::i() * Complex::<f64>::i(), Complex::new(-1.0, 0.0));
        })
        .it("div", || {
            let r = Complex::new(1.0, 2.0) / Complex::new(3.0, 4.0);
            assert!((r.re - 0.44).abs() < 1e-10);
            assert!((r.im - 0.08).abs() < 1e-10);
            assert_eq!(Complex::new(3.0, 6.0) / 3.0, Complex::new(1.0, 2.0));
        })
        .it("div by zero", || {
            assert!((Complex::new(1.0, 2.0) / 0.0).is_nan());
            assert!((Complex::new(1.0, 2.0) / Complex::new(0.0, 0.0)).is_nan());
        })
        .it("rem", || {
            let r = Complex::new(7.0, 7.0) % Complex::new(3.0, 4.0);
            assert_eq!(r.re, 7.0 % 3.0);
            assert_eq!(r.im, 7.0 % 4.0);
        })
        .it("add_assign / sub_assign / mul_assign / div_assign / rem_assign", || {
            let mut a = Complex::new(1.0, 2.0);
            a += Complex::new(3.0, 4.0);
            assert_eq!(a, Complex::new(4.0, 6.0));
            a -= Complex::new(3.0, 4.0);
            assert_eq!(a, Complex::new(1.0, 2.0));
            a *= Complex::new(3.0, 4.0);
            assert_eq!(a, Complex::new(-5.0, 10.0));
            a /= Complex::new(1.0, 1.0);
            assert!((a.re - 2.5).abs() < 1e-10);
            let mut b = Complex::new(5.0, 6.0);
            b -= 3.0;
            assert_eq!(b, Complex::new(2.0, 6.0));
            let mut c = Complex::new(1.0, 2.0);
            c += 5.0;
            assert_eq!(c, Complex::new(6.0, 2.0));
        })
        .it("partial_cmp", || {
            assert!(Complex::new(1.0, 2.0) < Complex::new(2.0, 1.0));
        })
        .tag("arithmetic")
        .run();

    describe("Integer Complex")
        .it("i32 complex arithmetic", || {
            let a = Complex::new(1i32, 2i32);
            let b = Complex::new(3i32, 4i32);
            assert_eq!((a + b).re, 4);
            assert_eq!((a * b).re, -5);
        })
        .it("i32 default is (0,0)", || {
            let z: Complex<i32> = Default::default();
            assert_eq!(z.re, 0);
            assert_eq!(z.im, 0);
        })
        .tag("integer")
        .run();

    describe("Conversions")
        .it("from_f64", || {
            let z = Complex::<f64>::from_f64(5.0);
            assert_eq!(z.re, 5.0);
            assert_eq!(z.im, 0.0);
        })
        .it("from T (real)", || {
            let z: Complex<f64> = 3.0.into();
            assert_eq!(z.re, 3.0);
            assert_eq!(z.im, 0.0);
        })
        .it("from tuple", || {
            let z: Complex<f64> = (3.0, 4.0).into();
            assert_eq!(z.re, 3.0);
            assert_eq!(z.im, 4.0);
        })
        .it("from Num", || {
            let n = Num::new(5.0);
            let z: Complex<f64> = n.into();
            assert_eq!(z.re, 5.0);
            assert_eq!(z.im, 0.0);
        })
        .it("to_f64 returns norm", || {
            let z = Complex::new(3.0, 4.0);
            assert_eq!(z.to_f64(), 5.0);
        })
        .it("from Fraction", || {
            let z: Complex<f64> = rvmath::Fraction::new(3, 4).to_f64().into();
            assert!((z.re - 0.75).abs() < 1e-10);
        })
        .tag("conversions")
        .run();

    describe("Numeric Trait")
        .it("abs", || {
            let z = Complex::new(3.0, 4.0);
            let a = z.abs();
            assert!((a.re - 5.0).abs() < 1e-10);
        })
        .it("sign returns unit vector", || {
            let z = Complex::new(3.0, 4.0);
            assert!((z.sign().norm() - 1.0).abs() < 1e-10);
        })
        .it("sqrt of -1 is i", || {
            let s = Complex::new(-1.0, 0.0).sqrt();
            assert!((s.re.abs() - 0.0) < 1e-10);
            assert!((s.im.abs() - 1.0) < 1e-10);
        })
        .it("exp(iπ) = -1", || {
            let e = Complex::new(0.0, std::f64::consts::PI).exp();
            assert!((e.re + 1.0).abs() < 1e-10);
            assert!(e.im.abs() < 1e-10);
        })
        .it("euler identity e^(iπ) + 1 = 0", || {
            let pi = Complex::new(std::f64::consts::PI, 0.0);
            let i = Complex::<f64>::i();
            let result = (i * pi).exp() + Complex::one();
            assert!(result.re.abs() < 1e-10);
            assert!(result.im.abs() < 1e-10);
        })
        .it("sin/sinh/cos/cosh/tanh", || {
            let z = Complex::<f64>::from_imag(1.0);
            let s = z.sin();
            assert!(s.re.abs() < 1e-10);
            assert!((s.im - 1.0_f64.sinh()).abs() < 1e-10);
            let c = z.cos();
            assert!((c.re - 1.0_f64.cosh()).abs() < 1e-10);
            assert!(c.im.abs() < 1e-10);
            let z2 = Complex::new(0.0, std::f64::consts::FRAC_PI_2);
            assert!((z2.sinh().im - 1.0).abs() < 1e-10);
            assert!(z2.cosh().re.abs() < 1e-10);
            let z3 = Complex::new(0.0, std::f64::consts::FRAC_PI_4);
            assert!((z3.tanh().im - 1.0).abs() < 1e-10);
        })
        .it("ln of -1 is iπ", || {
            let l = Complex::new(-1.0, 0.0).ln();
            assert!(l.re.abs() < 1e-10);
            assert!((l.im - std::f64::consts::PI).abs() < 1e-10);
        })
        .it("powi", || {
            let z = Complex::new(1.0, 1.0);
            assert!((z.powi(2).re - 0.0).abs() < 1e-10);
            assert!((z.powi(2).im - 2.0).abs() < 1e-10);
            assert_eq!(z.powi(0), Complex::one());
            let r = z.powi(-1);
            assert!((r.re - 0.5).abs() < 1e-10);
            assert!((r.im + 0.5).abs() < 1e-10);
        })
        .it("recip", || {
            let r = Complex::new(1.0, 1.0).recip();
            assert!((r.re - 0.5).abs() < 1e-10);
            assert!((r.im + 0.5).abs() < 1e-10);
            assert!(Complex::new(0.0, 0.0).recip().is_nan());
        })
        .it("round/floor/ceil", || {
            let z = Complex::new(1.5, -2.7);
            assert_eq!(z.round(), Complex::new(2.0, -3.0));
            assert_eq!(z.floor(), Complex::new(1.0, -3.0));
            assert_eq!(z.ceil(), Complex::new(2.0, -2.0));
        })
        .it("lerp", || {
            let r = Complex::new(0.0, 0.0).lerp(&Complex::new(10.0, 20.0), 0.5);
            assert_eq!(r, Complex::new(5.0, 10.0));
        })
        .it("clamp", || {
            let z = Complex::new(5.0, 5.0);
            let min = Complex::new(2.0, 2.0);
            let max = Complex::new(4.0, 4.0);
            assert_eq!(z.clamp(&min, &max), Complex::new(4.0, 4.0));
        })
        .it("cbrt", || {
            let r = Complex::new(-8.0, 0.0).cbrt();
            assert!((r.norm() - 2.0).abs() < 1e-10);
        })
        .it("is_nan / is_finite / is_infinite", || {
            assert!(Complex::new(f64::NAN, 0.0).is_nan());
            assert!(Complex::new(0.0, f64::NAN).is_nan());
            assert!(!Complex::new(1.0, 2.0).is_nan());
            assert!(Complex::new(1.0, 2.0).is_finite());
            assert!(!Complex::new(f64::INFINITY, 0.0).is_finite());
            assert!(Complex::new(f64::INFINITY, 0.0).is_infinite());
            assert!(!Complex::new(1.0, 2.0).is_infinite());
        })
        .it("to_degrees/to_radians", || {
            let z = Complex::new(std::f64::consts::PI, std::f64::consts::FRAC_PI_2);
            let d = z.to_degrees();
            assert!((d.re - 180.0).abs() < 1e-10);
            assert!((d.im - 90.0).abs() < 1e-10);
        })
        .it("tan", || {
            let t = Complex::new(0.0, 0.0).tan();
            assert!((t.re - 0.0).abs() < 1e-10);
            let t = Complex::new(std::f64::consts::FRAC_PI_4, 0.0).tan();
            assert!((t.re - 1.0).abs() < 1e-10);
        })
        .it("asin/acos/atan", || {
            let a = Complex::new(0.0, 0.0).asin();
            assert!((a.re - 0.0).abs() < 1e-10);
            let a = Complex::new(1.0, 0.0).acos();
            assert!((a.re - 0.0).abs() < 1e-10);
            let a = Complex::new(0.0, 0.0).atan();
            assert!((a.re - 0.0).abs() < 1e-10);
        })
        .it("pow with complex exponent", || {
            let r = Complex::new(2.0, 0.0).pow(&Complex::new(3.0, 0.0));
            assert!((r.re - 8.0).abs() < 1e-10);
        })
        .it("powf", || {
            let r = Complex::new(2.0, 0.0).powf(3.0);
            assert!((r.re - 8.0).abs() < 1e-10);
        })
        .it("root", || {
            let r = Complex::new(8.0, 0.0).root(&Complex::new(3.0, 0.0));
            assert!((r.re - 2.0).abs() < 1e-10);
        })
        .it("log10", || {
            let l = Complex::new(100.0, 0.0).log10();
            assert!((l.re - 2.0).abs() < 1e-10);
        })
        .it("hypot", || {
            let h = Complex::new(3.0, 0.0).hypot(&Complex::new(4.0, 0.0));
            assert!((h.re - 5.0).abs() < 1e-10);
        })
        .it("log with custom base", || {
            let r = Complex::new(100.0, 0.0).log(&Complex::new(10.0, 0.0));
            assert!((r.re - 2.0).abs() < 1e-10);
        })
        .tag("numeric")
        .run();

    describe("NumberKind and Constants")
        .it("number_kind is Complex", || {
            assert_eq!(<Complex<f64> as NumberKind>::number_set(), NumberSet::Complex);
            assert!(<Complex<f64> as NumberKind>::is_signed());
            assert!(!<Complex<f64> as NumberKind>::is_integer_valued());
        })
        .it("numeric constants", || {
            let pi = Complex::<f64>::pi();
            assert!((pi.re - std::f64::consts::PI).abs() < 1e-10);
            assert_eq!(pi.im, 0.0);
            assert!((Complex::<f64>::e().re - std::f64::consts::E).abs() < 1e-10);
            assert!((Complex::<f64>::tau().re - std::f64::consts::TAU).abs() < 1e-10);
            assert!((Complex::<f64>::phi().re - 1.618033988749895).abs() < 1e-10);
        })
        .tag("misc")
        .run();

    describe("Copy and Conversions")
        .it("copy", || {
            let a = Complex::new(1.0, 2.0);
            let b = a;
            assert_eq!(a, b);
        })
        .it("from tuple explicit", || {
            let z = Complex::<f64>::from((3.0, 4.0));
            assert_eq!(z.re, 3.0);
            assert_eq!(z.im, 4.0);
        })
        .it("into Num wrapper", || {
            let z = Complex::new(3.0, 4.0);
            let n: Num<Complex<f64>> = z.into();
            assert_eq!(n.value.re, 3.0);
        })
        .tag("copy_convert")
        .run();

    describe("Fraction and Percentage Integration")
        .it("fraction integration", || {
            let z = Complex::new(1.0, 2.0);
            let scalar = rvmath::Fraction::new(1, 2).to_f64();
            let r = z * scalar;
            assert!((r.re - 0.5).abs() < 1e-10);
        })
        .it("percentage integration", || {
            let z = Complex::new(100.0, 200.0);
            let scalar = rvmath::Percentage::<f64>::from_percent(50.0).to_f64();
            let r = z * scalar;
            assert!((r.re - 50.0).abs() < 1e-10);
        })
        .tag("integration")
        .run()
        .assert_all_pass();
}
