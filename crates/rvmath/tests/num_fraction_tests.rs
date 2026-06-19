use rvtest::spec::describe;
use rvmath::num::{Fraction, Percentage};
use rvmath::Numeric;

#[test]
fn fraction_tests() {
    describe("Construction")
        .it("new creates reduced fraction", || {
            let f = Fraction::new(1, 2);
            assert_eq!(f.numerator(), 1);
            assert_eq!(f.denominator(), 2);
        })
        .it("auto_reduce 4/6 → 2/3", || {
            let f = Fraction::new(4, 6);
            assert_eq!(f.numerator(), 2);
            assert_eq!(f.denominator(), 3);
        })
        .it("auto_reduce 12/18 → 2/3", || {
            let f = Fraction::new(12, 18);
            assert_eq!(f.numerator(), 2);
            assert_eq!(f.denominator(), 3);
        })
        .it("negative numerator", || {
            let f = Fraction::new(-3, 4);
            assert_eq!(f.numerator(), -3);
            assert_eq!(f.denominator(), 4);
        })
        .it("negative denominator becomes negative numerator", || {
            let f = Fraction::new(3, -4);
            assert!(f64::abs(f.to_f64() - (-0.75)) < 1e-10);
        })
        .it("both negative reduced", || {
            let f = Fraction::new(-3, -4);
            let n = f.normalize_sign();
            assert_eq!(n.numerator(), 3);
            assert_eq!(n.denominator(), 4);
        })
        .it("normalize_sign moves negative to numerator", || {
            let f = Fraction::new(3, -4);
            let n = f.normalize_sign();
            assert_eq!(n.numerator(), -3);
            assert_eq!(n.denominator(), 4);
        })
        .it("try_new rejects zero denominator", || {
            assert!(Fraction::<i32>::try_new(1, 0).is_none());
        })
        .it("zero fraction", || {
            let f = Fraction::<i32>::zero();
            assert_eq!(f.numerator(), 0);
            assert_eq!(f.denominator(), 1);
        })
        .it("one fraction", || {
            let f = Fraction::<i32>::one();
            assert_eq!(f.numerator(), 1);
            assert_eq!(f.denominator(), 1);
        })
        .it("from_integer", || {
            let f = Fraction::from_integer(5);
            assert_eq!(f.numerator(), 5);
            assert_eq!(f.denominator(), 1);
        })
        .tag("construction")
        .run();

    describe("From Float")
        .it("exact 0.5 → 1/2", || {
            let f = Fraction::<i32>::from_float(0.5, 1000);
            assert_eq!(f.numerator(), 1);
            assert_eq!(f.denominator(), 2);
        })
        .it("0.3333 → 1/3 within bound", || {
            let f = Fraction::<i32>::from_float(0.3333, 1000);
            assert_eq!(f.numerator(), 1);
            assert_eq!(f.denominator(), 3);
        })
        .it("negative float", || {
            let f = Fraction::<i32>::from_float(-0.5, 1000);
            assert_eq!(f.numerator(), -1);
            assert_eq!(f.denominator(), 2);
        })
        .it("zero", || {
            let f = Fraction::<i32>::from_float(0.0, 1000);
            assert_eq!(f.numerator(), 0);
            assert_eq!(f.denominator(), 1);
        })
        .it("approximate pi", || {
            let f = Fraction::<i64>::from_float(3.1415926535, 100000);
            assert!(f64::abs(f.to_f64() - 3.1415926535) < 1e-6);
        })
        .tag("from_float")
        .run();

    describe("Properties and Conversions")
        .it("recip", || {
            let r = Fraction::new(2, 3).recip();
            assert_eq!(r.numerator(), 3);
            assert_eq!(r.denominator(), 2);
        })
        .it("is_proper / is_improper", || {
            assert!(Fraction::new(1, 2).is_proper());
            assert!(!Fraction::new(3, 2).is_proper());
            assert!(Fraction::new(3, 2).is_improper());
            assert!(!Fraction::new(1, 2).is_improper());
        })
        .it("is_integer", || {
            assert!(Fraction::from_integer(5).is_integer());
            assert!(!Fraction::new(3, 4).is_integer());
        })
        .it("to_f64", || {
            assert!((Fraction::new(1.0, 3.0).to_f64() - 0.3333333333333333).abs() < 1e-15);
        })
        .it("to_num", || {
            let n = Fraction::new(3.0, 2.0).to_num();
            assert!((n.value - 1.5).abs() < 1e-15);
        })
        .it("trunc", || {
            assert_eq!(Fraction::<i32>::new(7, 3).trunc().to_f64(), 2.0);
            assert_eq!(Fraction::<i32>::new(-7, 3).trunc().to_f64(), -2.0);
        })
        .it("fract", || {
            let rem = Fraction::new(7, 3).fract();
            assert_eq!(rem.numerator(), 1);
            assert_eq!(rem.denominator(), 3);
        })
        .it("abs", || {
            let f = Fraction::new(-3, 4).abs();
            assert_eq!(f.numerator(), 3);
            assert_eq!(f.denominator(), 4);
        })
        .it("mixed", || {
            let f = Fraction::<i32>::new(7, 3);
            let (whole, frac) = f.mixed();
            assert_eq!(whole.value.to_f64(), 2.0);
            assert_eq!(frac.numerator(), 1);
            assert_eq!(frac.denominator(), 3);
        })
        .it("default is 0/1", || {
            let f: Fraction<i32> = Default::default();
            assert_eq!(f.numerator(), 0);
            assert_eq!(f.denominator(), 1);
        })
        .it("reduce noop for already reduced", || {
            let f = Fraction::new(2, 3).reduce();
            assert_eq!(f.numerator(), 2);
            assert_eq!(f.denominator(), 3);
        })
        .tag("properties")
        .run();

    describe("Arithmetic")
        .it("add", || {
            let r = Fraction::new(1, 3) + Fraction::new(1, 6);
            assert_eq!(r.numerator(), 1);
            assert_eq!(r.denominator(), 2);
            let r = Fraction::new(1, 2) + 1;
            assert_eq!(r.numerator(), 3);
            assert_eq!(r.denominator(), 2);
        })
        .it("sub", || {
            let r = Fraction::new(1, 2) - Fraction::new(1, 3);
            assert_eq!(r.numerator(), 1);
            assert_eq!(r.denominator(), 6);
            let r = Fraction::new(3, 2) - 1;
            assert_eq!(r.numerator(), 1);
            assert_eq!(r.denominator(), 2);
        })
        .it("mul", || {
            let r = Fraction::new(2, 3) * Fraction::new(3, 4);
            assert_eq!(r.numerator(), 1);
            assert_eq!(r.denominator(), 2);
            let r = Fraction::new(1, 2) * 3;
            assert_eq!(r.numerator(), 3);
            assert_eq!(r.denominator(), 2);
        })
        .it("div", || {
            let r = Fraction::new(1, 2) / Fraction::new(3, 4);
            assert_eq!(r.numerator(), 2);
            assert_eq!(r.denominator(), 3);
            let r = Fraction::new(3, 4) / 2;
            assert_eq!(r.numerator(), 3);
            assert_eq!(r.denominator(), 8);
        })
        .it("neg", || {
            let f = Fraction::new(3, 4);
            assert_eq!((-f).numerator(), -3);
            assert_eq!((-f).denominator(), 4);
        })
        .it("add_assign / sub_assign / mul_assign / div_assign / rem_assign", || {
            let mut a = Fraction::new(1, 4);
            a += Fraction::new(1, 4);
            assert_eq!(a.numerator(), 1);
            assert_eq!(a.denominator(), 2);
            let mut b = Fraction::new(3, 4);
            b -= Fraction::new(1, 4);
            assert_eq!(b.numerator(), 1);
            assert_eq!(b.denominator(), 2);
            let mut c = Fraction::new(2, 3);
            c *= Fraction::new(3, 4);
            assert_eq!(c.numerator(), 1);
            assert_eq!(c.denominator(), 2);
            let mut d = Fraction::new(1, 2);
            d /= Fraction::new(3, 4);
            assert_eq!(d.numerator(), 2);
            assert_eq!(d.denominator(), 3);
        })
        .it("rem", || {
            let r = Fraction::new(7, 3) % Fraction::new(2, 3);
            assert_eq!(r.numerator(), 1);
            assert_eq!(r.denominator(), 3);
        })
        .it("partial_ord", || {
            assert!(Fraction::new(1, 3) < Fraction::new(1, 2));
        })
        .tag("arithmetic")
        .run();

    describe("Negative Arithmetic")
        .it("add negative", || {
            let r = Fraction::new(-3, 4) + Fraction::new(1, 2);
            assert_eq!(r.numerator(), -1);
            assert_eq!(r.denominator(), 4);
        })
        .it("sub negative", || {
            let r = Fraction::new(-3, 4) - Fraction::new(1, 2);
            assert_eq!(r.numerator(), -5);
            assert_eq!(r.denominator(), 4);
        })
        .it("mul with negative denominator", || {
            let r = Fraction::new(3, -4) * Fraction::new(2, 3);
            assert!(f64::abs(r.to_f64() - (-0.5)) < 1e-10);
        })
        .it("div with negative denominator", || {
            let r = Fraction::new(3, -4) / Fraction::new(1, 2);
            assert!(f64::abs(r.to_f64() - (-1.5)) < 1e-10);
        })
        .it("assign with negative", || {
            let mut a = Fraction::new(3, 4);
            a += Fraction::new(-1, 2);
            assert_eq!(a.numerator(), 1);
            assert_eq!(a.denominator(), 4);
        })
        .it("sub_assign with integer", || {
            let mut a = Fraction::new(3, 2);
            a -= 1;
            assert_eq!(a.numerator(), 1);
            assert_eq!(a.denominator(), 2);
        })
        .it("mul_assign with integer", || {
            let mut a = Fraction::new(1, 2);
            a *= 3;
            assert_eq!(a.numerator(), 3);
            assert_eq!(a.denominator(), 2);
        })
        .it("div_assign with integer", || {
            let mut a = Fraction::new(3, 4);
            a /= 2;
            assert_eq!(a.numerator(), 3);
            assert_eq!(a.denominator(), 8);
        })
        .tag("negative")
        .run();

    describe("Numeric Trait")
        .it("sqrt", || {
            let s = Fraction::new(9, 4).sqrt();
            assert!((s.to_f64() - 1.5).abs() < 1e-10);
        })
        .it("cbrt", || {
            let c = Fraction::new(8, 1).cbrt();
            assert!((c.to_f64() - 2.0).abs() < 1e-10);
        })
        .it("pow", || {
            let r = Fraction::new(2, 1).pow(&Fraction::new(3, 1));
            assert!((r.to_f64() - 8.0).abs() < 1e-10);
        })
        .it("round/floor/ceil", || {
            let f = Fraction::new(7, 3);
            assert!((f.round().to_f64() - 2.0).abs() < 1e-10);
            assert!((f.floor().to_f64() - 2.0).abs() < 1e-10);
            assert!((f.ceil().to_f64() - 3.0).abs() < 1e-10);
        })
        .it("sign", || {
            assert!((Fraction::new(-3, 4).sign().to_f64() - (-1.0)).abs() < 1e-10);
            assert!((Fraction::new(3, 4).sign().to_f64() - 1.0).abs() < 1e-10);
            assert!((Fraction::<i32>::zero().sign().to_f64() - 0.0).abs() < 1e-10);
        })
        .it("abs on fraction", || {
            assert!((Fraction::new(-3, 4).abs().to_f64() - 0.75).abs() < 1e-10);
        })
        .it("recip on fraction", || {
            let r = Fraction::new(2, 3).recip();
            assert_eq!(r.numerator(), 3);
            assert_eq!(r.denominator(), 2);
        })
        .tag("numeric")
        .run();

    describe("Display and Conversion")
        .it("display simple", || {
            assert_eq!(format!("{}", Fraction::new(3, 4)), "3/4");
        })
        .it("display integer", || {
            assert_eq!(format!("{}", Fraction::from_integer(5)), "5");
        })
        .it("from tuple", || {
            let f: Fraction<i32> = (3, 4).into();
            assert_eq!(f.numerator(), 3);
            assert_eq!(f.denominator(), 4);
        })
        .it("into f64", || {
            let v: f64 = Fraction::new(1.0, 4.0).into();
            assert_eq!(v, 0.25);
        })
        .tag("display")
        .run();

    describe("Percentage Interop")
        .it("percentage from fraction", || {
            let p: Percentage<f64> = Fraction::new(1.0, 4.0).into();
            assert!(f64::abs(p.to_percent() - 25.0) < 1e-10);
        })
        .it("fraction from percentage", || {
            let p = Percentage::<f64>::from_percent(50.0);
            let f: Fraction<f64> = p.into();
            assert!(f64::abs(f.to_f64() - 0.5) < 1e-10);
        })
        .it("percentage mul fraction", || {
            let p = Percentage::<f64>::from_percent(50.0);
            let f = Fraction::new(1.0, 2.0);
            let r: Percentage<f64> = p * f;
            assert!(f64::abs(r.to_percent() - 25.0) < 1e-10);
        })
        .tag("percentage_interop")
        .run()

        .assert_all_pass();
}
