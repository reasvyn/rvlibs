use rvtest::spec::describe;
use rvmath::Vec3;
use rvmath::{Unit, declare_family, declare_units};

declare_family!(pub Length, Meter);
declare_units!(
    Length {
        pub Meter("m", 1.0)
    }
);

#[test]
fn vector_units_tests() {
    describe("Vector with Units")
        .it("add preserves unit power", || {
            let v1 = Vec3::<Unit<f64, Meter>>::new([
                Meter::new(1.0), Meter::new(2.0), Meter::new(3.0),
            ]);
            let v2 = Vec3::<Unit<f64, Meter>>::new([
                Meter::new(4.0), Meter::new(5.0), Meter::new(6.0),
            ]);
            let sum = v1 + v2;
            assert_eq!(sum.data[0].value, 5.0);
            assert_eq!(sum.data[0].power, 1.0);
        })
        .it("dot_units returns unit with power 2", || {
            let v1 = Vec3::<Unit<f64, Meter>>::new([
                Meter::new(1.0), Meter::new(2.0), Meter::new(3.0),
            ]);
            let v2 = Vec3::<Unit<f64, Meter>>::new([
                Meter::new(4.0), Meter::new(5.0), Meter::new(6.0),
            ]);
            let dot = v1.dot_units(v2);
            assert_eq!(dot.value, 32.0);
            assert_eq!(dot.power, 2.0);
        })
        .it("length_units returns unit with power 1", || {
            let v = Vec3::<Unit<f64, Meter>>::new([
                Meter::new(1.0), Meter::new(2.0), Meter::new(3.0),
            ]);
            let len = v.length_units();
            assert_eq!(len.value, (14.0_f64).sqrt());
            assert_eq!(len.power, 1.0);
        })
        .tag("vector_units")
        .run()
        .assert_all_pass();
}
