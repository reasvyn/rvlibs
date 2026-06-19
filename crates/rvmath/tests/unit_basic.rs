use rvtest::spec::describe;
use rvmath::Unit;
use rvmath::{declare_family, declare_units};

declare_family!(pub Length, Meter);

declare_units!(
    Length {
        pub Kilometer("km", 1e3),
        pub Meter("m", 1.0)
    }
);

#[test]
fn unit_basic_tests() {
    describe("Factory")
        .it("new creates unit with value and symbol", || {
            let m = Unit::<f64, Meter>::new(2.0);
            assert_eq!(m.value(), 2.0);
            assert_eq!(m.symbol(), "m");
        })
        .it("built-in unit constructor", || {
            let km = Kilometer::new(4.0);
            assert_eq!(km.value(), 4.0);
            assert_eq!(km.symbol(), "km");
        })
        .it("with_power applies power to display", || {
            let m2 = Meter::with_power(3.0, 2.0);
            assert_eq!(m2.value(), 3.0);
            assert_eq!(m2.symbol(), "m^2");
        })
        .it("different powers are not equal", || {
            let m = Meter::new(3.0);
            let m2 = Meter::with_power(3.0, 2.0);
            assert!(m != m2);
        })
        .tag("factory")
        .run();

    describe("Display")
        .it("formats with symbol", || {
            assert_eq!(format!("{}", Meter::new(10.0)), "10m");
        })
        .it("formats with power", || {
            let area = Meter::new(4.0).pow(2.0);
            assert_eq!(format!("{}", area), "16m^2");
        })
        .tag("display")
        .run();

    describe("Conversion")
        .it("km to m", || {
            let km = Kilometer::new(4.0);
            let m = km.convert_to::<Meter>();
            assert_eq!(m.value(), 4000.0);
            assert_eq!(m.symbol(), "m");
        })
        .it("m to km (round-trip)", || {
            let km = Kilometer::new(4.0);
            let m_back = km.convert_to::<Meter>().convert_to::<Kilometer>();
            assert_eq!(m_back.value(), 4.0);
        })
        .it("convert trait method", || {
            let km = Kilometer::new(4.0);
            let m_converted: Unit<f64, Meter> = km.convert();
            assert_eq!(m_converted.value(), 4000.0);
        })
        .tag("conversion")
        .run();

    describe("f64 Conversion")
        .it("f64 into unit", || {
            let m = Meter::new(10.0);
            let m_from_f64: Unit<f64, Meter> = 10.0.into();
            assert_eq!(m, m_from_f64);
        })
        .it("unit into f64", || {
            let m = Meter::new(10.0);
            let m_as_f64: f64 = m.into();
            assert_eq!(m_as_f64, 10.0);
        })
        .tag("f64_conv")
        .run();

    #[cfg(feature = "serde")]
    describe("Serde")
        .it("serialize and deserialize", || {
            let m = Meter::new(10.0);
            let serialized = serde_json::to_string(&m).unwrap();
            let deserialized: Unit<f64, Meter> = serde_json::from_str(&serialized).unwrap();
            assert_eq!(m, deserialized);
        })
        .tag("serde")
        .run()
        .assert_all_pass();
}
