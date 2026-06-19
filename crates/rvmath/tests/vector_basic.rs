use rvtest::spec::describe;
use rvmath::{declare_family, declare_units, Unit, Vec2, Vec3, Vec4};
use std::convert::TryFrom;

declare_family!(pub TestLength, TestMeter);
declare_units!(TestLength { pub TestMeter("m", 1.0) });

#[test]
fn vector_tests() {
    describe("Construction")
        .it("from array", || {
            let v = Vec2::from([1.0, 2.0]);
            assert_eq!(v.data, [1.0, 2.0]);
        })
        .it("splat", || {
            assert_eq!(Vec2::splat(10.0).data, [10.0, 10.0]);
            assert_eq!(Vec3::splat(5.0).data, [5.0, 5.0, 5.0]);
            assert_eq!(Vec4::splat(7.0).data, [7.0, 7.0, 7.0, 7.0]);
        })
        .it("new_coords", || {
            assert_eq!(Vec2::new_coords(1.0, 2.0).data, [1.0, 2.0]);
            assert_eq!(Vec3::new_coords(1.0, 2.0, 3.0).data, [1.0, 2.0, 3.0]);
            assert_eq!(Vec4::new_coords(1.0, 2.0, 3.0, 4.0).data, [1.0, 2.0, 3.0, 4.0]);
        })
        .it("default is zero", || {
            assert_eq!(Vec2::<f64>::default().data, [0.0, 0.0]);
            assert_eq!(Vec4::<f64>::default().data, [0.0, 0.0, 0.0, 0.0]);
        })
        .it("zero vector", || {
            assert_eq!(Vec2::<f64>::zero().data, [0.0, 0.0]);
        })
        .tag("construction")
        .run();

    describe("Accessors")
        .it("Vec2 x/y", || {
            let v = Vec2::new_coords(1.0, 2.0);
            assert_eq!(v.x(), 1.0);
            assert_eq!(v.y(), 2.0);
        })
        .it("Vec3 x/y/z", || {
            let v = Vec3::new_coords(1.0, 2.0, 3.0);
            assert_eq!(v.x(), 1.0);
            assert_eq!(v.y(), 2.0);
            assert_eq!(v.z(), 3.0);
        })
        .it("Vec4 x/y/z/w", || {
            let v = Vec4::new_coords(1.0, 2.0, 3.0, 4.0);
            assert_eq!(v.x(), 1.0);
            assert_eq!(v.y(), 2.0);
            assert_eq!(v.z(), 3.0);
            assert_eq!(v.w(), 4.0);
        })
        .it("index operator", || {
            let v = Vec2::new([10.0, 20.0]);
            assert_eq!(v[0], 10.0);
            assert_eq!(v[1], 20.0);
        })
        .it("components", || {
            let v = Vec2::new([1.0, 2.0]);
            assert_eq!(v.components(), [1.0, 2.0]);
        })
        .tag("accessors")
        .run();

    describe("Arithmetic")
        .it("add", || {
            let v1 = Vec2::from([1.0, 2.0]);
            let v2 = Vec2::from([3.0, 4.0]);
            assert_eq!((v1 + v2).data, [4.0, 6.0]);
        })
        .it("sub", || {
            let v1 = Vec2::new([5.0, 7.0]);
            let v2 = Vec2::new([3.0, 4.0]);
            assert_eq!((v1 - v2).data, [2.0, 3.0]);
        })
        .it("scalar mul", || {
            let v = Vec2::new([1.0, 2.0]);
            assert_eq!((v * 3.0).data, [3.0, 6.0]);
        })
        .it("scalar div", || {
            let v = Vec2::new([6.0, 9.0]);
            assert_eq!((v / 3.0).data, [2.0, 3.0]);
        })
        .tag("arithmetic")
        .run();

    describe("Dot and Length")
        .it("dot product", || {
            let v1 = Vec2::from([1.0, 2.0]);
            let v2 = Vec2::from([3.0, 4.0]);
            assert_eq!(v1.dot(v2), 11.0);
        })
        .it("length", || {
            let v = Vec2::from([1.0, 2.0]);
            assert_eq!(v.length(), (5.0_f64).sqrt());
        })
        .it("distance", || {
            let v1 = Vec2::<f64>::new([0.0, 0.0]);
            let v2 = Vec2::<f64>::new([3.0, 4.0]);
            assert!((v1.distance(v2) - 5.0).abs() < 1e-10);
        })
        .it("distance identical is 0", || {
            let v1 = Vec2::<f64>::new([1.0, 2.0]);
            let v2 = Vec2::<f64>::new([1.0, 2.0]);
            assert!((v1.distance(v2) - 0.0).abs() < 1e-10);
        })
        .tag("dot_length")
        .run();

    describe("Normalize")
        .it("normalize unit length", || {
            let v = Vec2::<f64>::new([3.0, 4.0]);
            let n = v.normalize();
            assert!((n.length() - 1.0).abs() < 1e-10);
        })
        .it("normalize zero vector returns zero", || {
            let v = Vec2::<f64>::new([0.0, 0.0]);
            let n = v.normalize();
            assert_eq!(n.data, [0.0, 0.0]);
        })
        .tag("normalize")
        .run();

    describe("Conversions")
        .it("into Vec", || {
            let v = Vec2::new([1.0, 2.0]);
            let vec: Vec<f64> = v.into();
            assert_eq!(vec, vec![1.0, 2.0]);
        })
        .it("try_from Vec success", || {
            let vec = vec![1.0, 2.0];
            let v = Vec2::try_from(vec).unwrap();
            assert_eq!(v.data, [1.0, 2.0]);
        })
        .it("try_from Vec wrong length", || {
            let vec = vec![1.0, 2.0, 3.0];
            let result = Vec2::<f64>::try_from(vec);
            assert!(result.is_err());
        })
        .tag("conversions")
        .run();

    describe("Unit-Aware")
        .it("zero_units", || {
            let v = Vec2::<Unit<f64, TestMeter>>::zero_units();
            assert_eq!(v.data[0].value, 0.0);
            assert_eq!(v.data[0].power, 1.0);
        })
        .it("normalize_units", || {
            let v = Vec2::<Unit<f64, TestMeter>>::new([
                TestMeter::new(3.0), TestMeter::new(4.0),
            ]);
            let n = v.normalize_units();
            assert!((n.data[0].value - 0.6).abs() < 1e-10);
            assert!((n.data[1].value - 0.8).abs() < 1e-10);
        })
        .it("normalize_units zero", || {
            let v = Vec2::<Unit<f64, TestMeter>>::new([
                TestMeter::new(0.0), TestMeter::new(0.0),
            ]);
            let n = v.normalize_units();
            assert_eq!(n.data[0].value, 0.0);
        })
        .it("values extracts raw values", || {
            let v = Vec2::<Unit<f64, TestMeter>>::new([
                TestMeter::new(3.0), TestMeter::new(4.0),
            ]);
            assert_eq!(v.values(), [3.0, 4.0]);
        })
        .tag("units")
        .run()
        .assert_all_pass();
}
