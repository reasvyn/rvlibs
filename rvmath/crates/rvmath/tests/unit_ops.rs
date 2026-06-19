use rvtest::spec::describe;
use rvmath::{declare_family, declare_units, Unit};

declare_family!(pub Length, Meter);
declare_family!(pub Dimensionless, Ratio);

declare_units!(
    Length {
        pub Kilometer("km", 1e3),
        pub Meter("m", 1.0)
    }
);

declare_units!(
    Dimensionless {
        pub Ratio("", 1.0)
    }
);

#[test]
fn unit_ops_tests() {
    describe("Power and Roots")
        .it("pow", || {
            let m = Meter::new(2.0).pow(2.0);
            assert_eq!(m.value, 4.0);
            assert_eq!(m.power, 2.0);
        })
        .it("sqrt", || {
            let m = Meter::new(4.0).pow(2.0);
            let sqrt_m = m.sqrt();
            assert_eq!(sqrt_m.value, 4.0);
            assert_eq!(sqrt_m.power, 1.0);
        })
        .it("cbrt", || {
            let m = Meter::new(2.0).pow(3.0);
            let cbrt_m = m.cbrt();
            assert_eq!(cbrt_m.value, 2.0);
            assert_eq!(cbrt_m.power, 1.0);
        })
        .it("root", || {
            let m = Meter::new(2.0).pow(3.0);
            let root_m = m.root(3.0);
            assert_eq!(root_m.value, 2.0);
            assert_eq!(root_m.power, 1.0);
        })
        .it("inv flips power sign", || {
            let m = Meter::new(2.0).pow(2.0);
            let inv_m = m.inv();
            assert_eq!(inv_m.value, 0.25);
            assert_eq!(inv_m.power, -2.0);
        })
        .tag("power_roots")
        .run();

    describe("Rounding")
        .it("round", || {
            assert_eq!(Meter::new(2.5).round().value, 3.0);
        })
        .it("floor", || {
            assert_eq!(Meter::new(2.5).floor().value, 2.0);
        })
        .it("ceil", || {
            assert_eq!(Meter::new(2.5).ceil().value, 3.0);
        })
        .it("fract", || {
            assert_eq!(Meter::new(2.5).fract().value, 0.5);
        })
        .tag("rounding")
        .run();

    describe("Unary Ops")
        .it("neg", || {
            let m = Meter::new(2.0).pow(2.0);
            let neg_m = -m;
            assert_eq!(neg_m.value, -4.0);
            assert_eq!(neg_m.power, 2.0);
        })
        .it("abs", || {
            let m = Meter::new(-2.5).abs();
            assert_eq!(m.value, 2.5);
            assert_eq!(m.power, 1.0);
        })
        .it("signum", || {
            assert_eq!(Meter::new(3.0).signum(), 1.0);
            assert_eq!(Meter::new(-3.0).signum(), -1.0);
            assert_eq!(Meter::new(0.0).signum(), 0.0);
        })
        .tag("unary")
        .run();

    describe("Add / Sub")
        .it("add same power", || {
            let m1 = Meter::new(2.0).pow(2.0);
            let m2 = Meter::new(4.0).pow(2.0);
            let sum = m1 + m2;
            assert_eq!(sum.value, 20.0);
            assert_eq!(sum.power, 2.0);
        })
        .it("add_assign", || {
            let mut m = Meter::new(2.0).pow(2.0);
            m += Meter::new(4.0).pow(2.0);
            assert_eq!(m.value, 20.0);
        })
        .it("sub same power", || {
            let m1 = Meter::new(5.0).pow(3.0);
            let m2 = Meter::new(2.0).pow(3.0);
            assert_eq!((m1 - m2).value, 117.0);
        })
        .it("sub_assign", || {
            let mut m = Meter::new(5.0).pow(3.0);
            m -= Meter::new(2.0).pow(3.0);
            assert_eq!(m.value, 117.0);
        })
        .tag("add_sub")
        .run();

    describe("Mismatched Power Panics")
        .it("add with different powers panics", || {
            let result = std::panic::catch_unwind(|| {
                let _ = Meter::new(2.0) + Meter::new(4.0).pow(2.0);
            });
            assert!(result.is_err());
        })
        .it("sub with different powers panics", || {
            let result = std::panic::catch_unwind(|| {
                let _ = Meter::new(5.0).pow(2.0) - Meter::new(2.0);
            });
            assert!(result.is_err());
        })
        .tag("panic")
        .run();

    describe("Mul / Div")
        .it("mul adds powers", || {
            let m1 = Meter::new(2.0);
            let m2 = Meter::new(3.0);
            let res = m1 * m2;
            assert_eq!(res.value, 6.0);
            assert_eq!(res.power, 2.0);
            assert_eq!(format!("{}", res), "6m^2");
        })
        .it("mul nested powers", || {
            let area = Meter::new(2.0).pow(2.0);
            let volume = Meter::new(2.0).pow(3.0);
            let res = area * volume;
            assert_eq!(res.value, 32.0);
            assert_eq!(res.power, 5.0);
        })
        .it("mul by scalar (f64)", || {
            let m = Meter::new(2.0).pow(2.0);
            assert_eq!((m * 3.0_f64).value, 12.0);
            let mut m_mut = Meter::new(2.0).pow(2.0);
            m_mut *= 5.0;
            assert_eq!(m_mut.value, 20.0);
        })
        .it("div subtracts powers (same dim → dimensionless)", || {
            let m1 = Meter::new(10.0);
            let m2 = Meter::new(2.0);
            let res = m1 / m2;
            assert_eq!(res, 5.0);
        })
        .it("div nested powers", || {
            let area1 = Meter::new(20.0).pow(2.0);
            let area2 = Meter::new(4.0).pow(2.0);
            assert_eq!(area1 / area2, 25.0);
        })
        .it("div by scalar preserves power", || {
            let mut m = Meter::new(10.0).pow(2.0);
            m /= 2.0;
            assert_eq!(m.value, 50.0);
            assert_eq!(m.power, 2.0);
        })
        .tag("mul_div")
        .run();

    describe("Rem")
        .it("rem same power", || {
            let m1 = Meter::new(10.0).pow(2.0);
            let m2 = Meter::new(3.0).pow(2.0);
            let res = m1 % m2;
            assert_eq!(res.value, 1.0);
            assert_eq!(res.power, 2.0);
        })
        .it("rem_assign", || {
            let mut m = Meter::new(10.0).pow(2.0);
            m %= Meter::new(3.0).pow(2.0);
            assert_eq!(m.value, 1.0);
        })
        .tag("rem")
        .run();

    describe("Comparison")
        .it("equality same unit and power", || {
            let m1 = Meter::new(2.0).pow(2.0);
            let m2 = Meter::new(2.0).pow(2.0);
            assert_eq!(m1, m2);
        })
        .it("inequality different power", || {
            assert!(Meter::new(2.0).pow(2.0) != Meter::new(2.0));
        })
        .it("not equal to raw f64", || {
            let m = Meter::new(4.0);
            assert!(m != 4.0_f64);
            assert_eq!(m.value, 4.0_f64);
        })
        .tag("comparison")
        .run();

    describe("Factor")
        .it("factor_to_base", || {
            assert_eq!(Meter::new(2.0).factor_to_base(), 1.0);
            assert_eq!(Kilometer::new(2.0).factor_to_base(), 1e3);
        })
        .tag("factor")
        .run();

    describe("Clamp / Min / Max")
        .it("clamp", || {
            let v = Meter::new(5.0);
            let lo = Meter::new(2.0);
            let hi = Meter::new(8.0);
            assert_eq!(v.clamp(lo, hi).value, 5.0);
            assert_eq!(Meter::new(-1.0).clamp(lo, hi).value, 2.0);
            assert_eq!(Meter::new(10.0).clamp(lo, hi).value, 8.0);
        })
        .it("min / max", || {
            let a = Meter::new(3.0);
            let b = Meter::new(5.0);
            assert_eq!(a.min(b).value, 3.0);
            assert_eq!(a.max(b).value, 5.0);
        })
        .tag("clamp")
        .run();

    describe("Mismatched Power Panics (clamp/min/max)")
        .it("clamp mismatched power", || {
            let result = std::panic::catch_unwind(|| {
                let _ = Meter::new(5.0).clamp(Meter::new(2.0).pow(2.0), Meter::new(8.0));
            });
            assert!(result.is_err());
        })
        .it("min mismatched power", || {
            let result = std::panic::catch_unwind(|| {
                let _ = Meter::new(3.0).min(Meter::new(5.0).pow(2.0));
            });
            assert!(result.is_err());
        })
        .it("max mismatched power", || {
            let result = std::panic::catch_unwind(|| {
                let _ = Meter::new(3.0).max(Meter::new(5.0).pow(2.0));
            });
            assert!(result.is_err());
        })
        .tag("panic")
        .run();

    describe("Lerp")
        .it("lerp dimensionless", || {
            let a = Unit::<f64, Ratio>::with_power(0.0, 0.0);
            let b = Unit::<f64, Ratio>::with_power(10.0, 0.0);
            let r = a.lerp(b, 0.5);
            assert!((r.value - 5.0).abs() < 1e-12);
            assert_eq!(r.power, 0.0);
        })
        .it("lerp mismatched power panics", || {
            let result = std::panic::catch_unwind(|| {
                let a = Unit::<f64, Ratio>::with_power(0.0, 0.0);
                let b = Unit::<f64, Ratio>::with_power(10.0, 1.0);
                let _ = a.lerp(b, 0.5);
            });
            assert!(result.is_err());
        })
        .tag("lerp")
        .run();

    describe("Log/Exp (dimensionless only)")
        .it("ln", || {
            let v = Unit::<f64, Ratio>::with_power(std::f64::consts::E, 0.0);
            assert!((v.ln() - 1.0).abs() < 1e-12);
        })
        .it("log10", || {
            let v = Unit::<f64, Ratio>::with_power(100.0, 0.0);
            assert!((v.log10() - 2.0).abs() < 1e-12);
        })
        .it("log", || {
            let v = Unit::<f64, Ratio>::with_power(100.0, 0.0);
            assert!((v.log(10.0) - 2.0).abs() < 1e-12);
        })
        .it("exp", || {
            let v = Unit::<f64, Ratio>::with_power(1.0, 0.0);
            assert!((v.exp() - std::f64::consts::E).abs() < 1e-12);
        })
        .it("ln on dimensioned unit panics", || {
            let result = std::panic::catch_unwind(|| {
                let _ = Meter::new(2.0).ln();
            });
            assert!(result.is_err());
        })
        .tag("log_exp")
        .run();

    describe("Trig (dimensionless only)")
        .it("sin / cos / tan", || {
            let v = Unit::<f64, Ratio>::with_power(0.0, 0.0);
            assert!((v.sin() - 0.0).abs() < 1e-12);
            assert!((v.cos() - 1.0).abs() < 1e-12);
            assert!((v.tan() - 0.0).abs() < 1e-12);
        })
        .it("atan2", || {
            let y = Unit::<f64, Ratio>::with_power(1.0, 0.0);
            let x = Unit::<f64, Ratio>::with_power(1.0, 0.0);
            assert!((y.atan2(x) - std::f64::consts::FRAC_PI_4).abs() < 1e-12);
        })
        .tag("trig")
        .run();

    describe("dB Conversion")
        .it("to_db_power", || {
            let v = Unit::<f64, Ratio>::with_power(100.0, 0.0);
            assert!((v.to_db_power() - 20.0).abs() < 1e-12);
        })
        .it("to_db_amplitude", || {
            let v = Unit::<f64, Ratio>::with_power(100.0, 0.0);
            assert!((v.to_db_amplitude() - 40.0).abs() < 1e-12);
        })
        .tag("db")
        .run()
        .assert_all_pass();
}
