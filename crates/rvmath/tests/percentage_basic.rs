use rvtest::spec::describe;
use rvmath::num::{Num, NumberKind, Percentage};
use rvmath::{Numeric, declare_family, declare_units};

declare_family!(pub Length, Meter);
declare_units!(Length { pub Meter("m", 1.0) });

const EPS: f64 = 1e-10;

#[test]
fn percentage_tests() {
    describe("Creation")
        .it("from_percent", || {
            let p: Percentage<f64> = Percentage::from_percent(50.0);
            assert!((p.to_percent() - 50.0).abs() < EPS);
            let ratio: f64 = p.to_f64();
            assert!((ratio - 0.5).abs() < EPS);
        })
        .it("from_ratio", || {
            let p = Percentage::<f64>::from_ratio(0.5);
            assert!((p.to_percent() - 50.0).abs() < EPS);
        })
        .it("default is zero", || {
            let p: Percentage<f64> = Default::default();
            assert!(p.ratio().abs() < EPS);
        })
        .it("zero percent values", || {
            let p = Percentage::<f64>::from_ratio(0.0);
            assert!((p.to_percent() - 0.0).abs() < EPS);
            let p = Percentage::<f64>::from_percent(0.0);
            assert!((p.ratio() - 0.0).abs() < EPS);
        })
        .tag("creation")
        .run();

    describe("Num Interaction")
        .it("num * percentage = scaled value", || {
            let n = Num::new(100.0);
            let p = Percentage::from_percent(50.0);
            let res = n * p;
            assert!((res.value - 50.0).abs() < EPS);
        })
        .it("num + percentage = num + n*pct", || {
            let n = Num::new(100.0);
            let p = Percentage::from_percent(10.0);
            let res_add = n + p;
            assert!((res_add.value - 110.0).abs() < EPS);
            let res_sub = n - p;
            assert!((res_sub.value - 90.0).abs() < EPS);
        })
        .tag("num")
        .run();

    describe("Unit Interaction")
        .it("unit * percentage = scaled unit", || {
            let m = Meter::new(10.0);
            let p = Percentage::from_percent(50.0);
            let res = m * p;
            assert!((res.value - 5.0).abs() < EPS);
            assert!((res.power.to_f64() - 1.0).abs() < EPS);
        })
        .it("unit + percentage = unit + unit*pct", || {
            let m = Meter::new(100.0);
            let p = Percentage::from_percent(20.0);
            let res_add = m + p;
            assert!((res_add.value - 120.0).abs() < EPS);
            let res_sub = m - p;
            assert!((res_sub.value - 80.0).abs() < EPS);
        })
        .tag("unit")
        .run();

    describe("Percentage Arithmetic")
        .it("add / sub", || {
            let a = Percentage::from_ratio(0.3);
            let b = Percentage::from_ratio(0.2);
            assert!((a + b).ratio() - 0.5 < EPS);
            let a = Percentage::from_ratio(0.5);
            assert!((a - b).ratio() - 0.3 < EPS);
        })
        .it("mul / div", || {
            let a = Percentage::from_ratio(0.5);
            let b = Percentage::from_ratio(0.5);
            assert!((a * b).ratio() - 0.25 < EPS);
            let a = Percentage::from_ratio(0.6);
            let b = Percentage::from_ratio(0.2);
            assert!((a / b).ratio() - 3.0 < EPS);
        })
        .it("rem", || {
            let a = Percentage::from_ratio(0.7);
            let b = Percentage::from_ratio(0.3);
            assert!((a % b).ratio() - 0.1 < EPS);
        })
        .it("neg", || {
            let p = Percentage::from_ratio(0.5);
            assert!((-p).ratio() - (-0.5) < EPS);
        })
        .tag("arithmetic")
        .run();

    describe("Assign Ops")
        .it("add_assign/sub_assign", || {
            let mut a = Percentage::from_ratio(0.3);
            a += Percentage::from_ratio(0.2);
            assert!((a.ratio() - 0.5).abs() < EPS);
            let mut b = Percentage::from_ratio(0.5);
            b -= Percentage::from_ratio(0.2);
            assert!((b.ratio() - 0.3).abs() < EPS);
        })
        .it("mul_assign/div_assign", || {
            let mut a = Percentage::from_ratio(0.5);
            a *= Percentage::from_ratio(0.5);
            assert!((a.ratio() - 0.25).abs() < EPS);
            let mut b = Percentage::from_ratio(0.6);
            b /= Percentage::from_ratio(0.2);
            assert!((b.ratio() - 3.0).abs() < EPS);
        })
        .it("rem_assign", || {
            let mut a = Percentage::from_ratio(0.7);
            a %= Percentage::from_ratio(0.3);
            assert!((a.ratio() - 0.1).abs() < EPS);
        })
        .tag("assign")
        .run();

    describe("Scalar Ops")
        .it("percentage * scalar = f64", || {
            let p = Percentage::from_ratio(0.5);
            let r: f64 = p * 10.0;
            assert!((r - 5.0).abs() < EPS);
        })
        .tag("scalar")
        .run();

    describe("Display and NumberKind")
        .it("display", || {
            assert_eq!(format!("{}", Percentage::<f64>::from_percent(50.0)), "50%");
            assert_eq!(format!("{}", Percentage::<f64>::from_percent(0.0)), "0%");
            assert_eq!(format!("{}", Percentage::<f64>::from_percent(100.0)), "100%");
        })
        .it("number_kind", || {
            assert_eq!(<Percentage<f64> as NumberKind>::number_set(), rvmath::num::NumberSet::Real);
            assert!(<Percentage<f64> as NumberKind>::is_signed());
            assert!(!<Percentage<f64> as NumberKind>::is_integer_valued());
        })
        .tag("meta")
        .run()

        .assert_all_pass();
}
