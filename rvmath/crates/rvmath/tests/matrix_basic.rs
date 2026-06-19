use rvtest::spec::describe;
use rvmath::{Mat2x2, Mat3x3, Mat4x4, Vec2};
use rvmath::{Numeric, Unit, declare_family, declare_units};

declare_family!(pub Length, Meter);
declare_family!(pub Force, Newton);
declare_units!(Length { pub Meter("m", 1.0) });
declare_units!(Force { pub Newton("N", 1.0) });

#[test]
fn matrix_tests() {
    describe("Basic Construction")
        .it("creates 2x2 from array", || {
            let m = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            assert_eq!(m.data[0][0], 1.0);
            assert_eq!(m.data[1][1], 4.0);
        })
        .it("default is zero matrix", || {
            let m: Mat2x2<f64> = Default::default();
            assert_eq!(m.data[0][0], 0.0);
            assert_eq!(m.data[1][1], 0.0);
        })
        .it("from array via Into", || {
            let arr = [[1.0, 2.0], [3.0, 4.0]];
            let m: Mat2x2 = arr.into();
            assert_eq!(m.data[0][0], 1.0);
        })
        .it("components accessor", || {
            let m = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            let comp = m.components();
            assert_eq!(comp[0][0], 1.0);
            assert_eq!(comp[1][1], 4.0);
        })
        .tag("construction")
        .run();

    describe("Arithmetic")
        .it("add", || {
            let m1 = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            let m2 = Mat2x2::new([[5.0, 6.0], [7.0, 8.0]]);
            let m3 = m1 + m2;
            assert_eq!(m3.data[0][0], 6.0);
            assert_eq!(m3.data[1][1], 12.0);
        })
        .it("add assign", || {
            let mut m1 = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            let m2 = Mat2x2::new([[1.0, 1.0], [1.0, 1.0]]);
            m1 += m2;
            assert_eq!(m1.data[0][0], 2.0);
        })
        .it("element-wise mul", || {
            let a = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            let b = Mat2x2::new([[5.0, 6.0], [7.0, 8.0]]);
            let r = a * b;
            assert_eq!(r.data[0][0], 5.0);
            assert_eq!(r.data[1][1], 32.0);
        })
        .it("scalar mul", || {
            let m = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            let r = m * 2.0;
            assert_eq!(r.data[0][0], 2.0);
            assert_eq!(r.data[1][1], 8.0);
        })
        .it("scalar div", || {
            let m = Mat2x2::new([[2.0, 4.0], [6.0, 8.0]]);
            let r = m / 2.0;
            assert_eq!(r.data[0][0], 1.0);
            assert_eq!(r.data[1][1], 4.0);
        })
        .it("rem with scalar", || {
            let m = Mat2x2::new([[5.0, 7.0], [9.0, 11.0]]);
            let res = m % 3.0;
            assert_eq!(res.data[0][0], 2.0);
            assert_eq!(res.data[1][1], 2.0);
        })
        .it("integer matrix ops", || {
            let m1 = Mat2x2::<i32>::new([[1, 2], [3, 4]]);
            let m2 = Mat2x2::<i32>::new([[5, 6], [7, 8]]);
            let m3 = m1 + m2;
            assert_eq!(m3.data[0][0], 6);
        })
        .tag("arithmetic")
        .run();

    describe("Matrix Multiply and Transpose")
        .it("matrix multiply 2x2", || {
            let m1 = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            let m2 = Mat2x2::new([[5.0, 6.0], [7.0, 8.0]]);
            let m3 = m1.mul_mat(m2);
            assert_eq!(m3.data[0][0], 19.0);
            assert_eq!(m3.data[1][1], 50.0);
        })
        .it("matrix-vector multiply", || {
            let m = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            let v = Vec2::new([5.0, 6.0]);
            let v2 = m * v;
            assert_eq!(v2.data[0], 17.0);
            assert_eq!(v2.data[1], 39.0);
        })
        .it("transpose 2x2", || {
            let m = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            let mt = m.transpose();
            assert_eq!(mt.data[0][1], 3.0);
            assert_eq!(mt.data[1][0], 2.0);
        })
        .it("transpose 4x4", || {
            let m = Mat4x4::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]);
            let mt = m.transpose();
            assert_eq!(mt.data[0][1], 5.0);
            assert_eq!(mt.data[1][0], 2.0);
            assert_eq!(mt.data[2][3], 15.0);
        })
        .tag("mul_transpose")
        .run();

    describe("Determinant and Inverse")
        .it("det2 of 2x2", || {
            let m = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            assert_eq!(m.det2(), -2.0);
        })
        .it("det3 of singular matrix is 0", || {
            let m = Mat3x3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
            assert_eq!(m.det3(), 0.0);
        })
        .it("det3 of non-singular matrix", || {
            let m = Mat3x3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 10.0]]);
            assert!((m.det3() - (-3.0)).abs() < 1e-10);
        })
        .it("inv2 of 2x2", || {
            let m = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            let inv = m.inv2().unwrap();
            assert_eq!(inv.data[0][0], -2.0);
            assert_eq!(inv.data[0][1], 1.0);
            assert_eq!(inv.data[1][0], 1.5);
            assert_eq!(inv.data[1][1], -0.5);
        })
        .it("inv2 of singular returns none", || {
            let m = Mat2x2::new([[1.0, 2.0], [2.0, 4.0]]);
            assert!(m.inv2().is_none());
        })
        .tag("det_inv")
        .run();

    describe("Row/Col Accessors")
        .it("row and column access", || {
            let m = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            assert_eq!(m.row(0), [1.0, 2.0]);
            assert_eq!(m.col(1), [2.0, 4.0]);
        })
        .it("row_vec and col_vec", || {
            let m = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            let rv = m.row_vec(0);
            assert_eq!(rv.data, [1.0, 2.0]);
            let cv = m.col_vec(1);
            assert_eq!(cv.data, [2.0, 4.0]);
        })
        .tag("accessors")
        .run();

    describe("Identity and Zero")
        .it("identity 2x2", || {
            let id = Mat2x2::<f64>::identity();
            assert_eq!(id.data[0][0], 1.0);
            assert_eq!(id.data[0][1], 0.0);
            assert_eq!(id.data[1][0], 0.0);
            assert_eq!(id.data[1][1], 1.0);
        })
        .it("identity 3x3", || {
            let id = Mat3x3::<f64>::identity();
            assert_eq!(id.data[0][0], 1.0);
            assert_eq!(id.data[0][1], 0.0);
            assert_eq!(id.data[1][1], 1.0);
            assert_eq!(id.data[2][2], 1.0);
        })
        .it("zero 2x2", || {
            let z = Mat2x2::<f64>::zero();
            assert_eq!(z.data[0][1], 0.0);
            assert_eq!(z.data[1][0], 0.0);
        })
        .it("zero 3x3", || {
            let z = Mat3x3::<f64>::zero();
            assert_eq!(z.data[0][0], 0.0);
            assert_eq!(z.data[2][2], 0.0);
        })
        .it("zero 4x4", || {
            let z = Mat4x4::<f64>::zero();
            for i in 0..4 {
                for j in 0..4 {
                    assert_eq!(z.data[i][j], 0.0);
                }
            }
        })
        .tag("identity_zero")
        .run();

    describe("4x4 Operations")
        .it("add 4x4", || {
            let a = Mat4x4::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]);
            let b = Mat4x4::new([
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
            ]);
            let c = a + b;
            assert_eq!(c.data[0][0], 2.0);
            assert_eq!(c.data[3][3], 17.0);
        })
        .tag("4x4")
        .run();

    describe("Comparison")
        .it("partial ord", || {
            let m1 = Mat2x2::new([[1.0, 2.0], [3.0, 4.0]]);
            let m2 = Mat2x2::new([[1.0, 2.0], [3.0, 5.0]]);
            assert!(m1 < m2);
            assert!(m2 > m1);
        })
        .tag("comparison")
        .run();

    describe("Unit-Aware")
        .it("matrix-vector with units", || {
            let m = Mat2x2::<Unit<f64, Meter>>::new([
                [Meter::new(1.0), Meter::new(2.0)],
                [Meter::new(3.0), Meter::new(4.0)],
            ]);
            let v = Vec2::<Unit<f64, Meter>>::new([Meter::new(5.0), Meter::new(6.0)]);
            let res = m * v;
            assert_eq!(res.data[0].value, 17.0);
            assert_eq!(res.data[0].power.to_f64(), 2.0);
        })
        .it("matrix-multiply with units", || {
            let m1 = Mat2x2::<Unit<f64, Meter>>::new([
                [Meter::new(1.0), Meter::new(0.0)],
                [Meter::new(0.0), Meter::new(1.0)],
            ]);
            let m2 = Mat2x2::<Unit<f64, Meter>>::new([
                [Meter::new(2.0), Meter::new(0.0)],
                [Meter::new(0.0), Meter::new(2.0)],
            ]);
            let res = m1.mul_mat_units(m2);
            assert_eq!(res.data[0][0].value, 2.0);
            assert_eq!(res.data[0][0].power.to_f64(), 2.0);
        })
        .it("zero_units 2x2", || {
            let z = Mat2x2::<Unit<f64, Newton>>::zero_units();
            for i in 0..2 {
                for j in 0..2 {
                    assert_eq!(z.data[i][j].value, 0.0);
                    assert_eq!(z.data[i][j].power, 1.0);
                }
            }
        })
        .it("zero_units 3x3", || {
            let z = Mat3x3::<Unit<f64, Newton>>::zero_units();
            assert_eq!(z.data[0][0].value, 0.0);
            assert_eq!(z.data[2][2].value, 0.0);
            assert_eq!(z.data[0][0].power, 1.0);
        })
        .tag("units")
        .run()
        .assert_all_pass();
}
