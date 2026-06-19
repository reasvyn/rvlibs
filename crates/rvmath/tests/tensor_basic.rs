use rvtest::spec::describe;
use rvmath::Tensor;
use rvmath::{Numeric, Unit, declare_family, declare_units};

declare_family!(pub Force, Newton);
declare_family!(pub Temperature, Kelvin);
declare_units!(Force { pub Newton("N", 1.0) });
declare_units!(Temperature { pub Kelvin("K", 1.0) });

#[test]
fn tensor_tests() {
    describe("Construction")
        .it("new creates tensor with given shape", || {
            let t = Tensor::<f64>::new(vec![2, 3]);
            assert_eq!(t.shape, vec![2, 3]);
            assert_eq!(t.data.len(), 6);
        })
        .it("default is empty", || {
            let t: Tensor<f64> = Default::default();
            assert!(t.data.is_empty());
            assert!(t.shape.is_empty());
            assert!(t.strides.is_empty());
        })
        .it("zero tensor", || {
            let z = Tensor::<f64>::zero(vec![2, 3]);
            assert_eq!(z.shape, vec![2, 3]);
            assert!(z.data.iter().all(|&v| v == 0.0));
        })
        .it("ones tensor", || {
            let o = Tensor::<f64>::ones(vec![2, 2]);
            assert!(o.data.iter().all(|&v| v == 1.0));
        })
        .it("from_data with mismatched size returns error", || {
            let result = Tensor::<f64>::from_data(vec![1.0, 2.0, 3.0], vec![2]);
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("Data size"));
        })
        .tag("construction")
        .run();

    describe("Indexing")
        .it("get and get_mut", || {
            let mut t = Tensor::<f64>::new(vec![2, 2]);
            *t.get_mut(&[0, 1]) = 5.0;
            *t.get_mut(&[1, 0]) = 10.0;
            assert_eq!(*t.get(&[0, 1]), 5.0);
            assert_eq!(*t.get(&[1, 0]), 10.0);
            assert_eq!(t.data[1], 5.0);
            assert_eq!(t.data[2], 10.0);
        })
        .it("index operator", || {
            let mut t = Tensor::<f64>::new(vec![2, 2]);
            t[&[0, 1]] = 5.0;
            t[&[1, 0]] = 10.0;
            assert_eq!(t[&[0, 1]], 5.0);
            assert_eq!(t[&[1, 0]], 10.0);
        })
        .it("try_get returns none for out-of-bounds", || {
            let t = Tensor::<f64>::new(vec![2, 2]);
            assert!(t.try_get(&[2, 0]).is_none());
            assert!(t.try_get(&[0, 2]).is_none());
            assert!(t.try_get(&[0]).is_none());
        })
        .it("try_get_mut returns none for out-of-bounds", || {
            let mut t = Tensor::<f64>::new(vec![2, 2]);
            assert!(t.try_get_mut(&[2, 0]).is_none());
            assert!(t.try_get_mut(&[0, 2]).is_none());
        })
        .tag("indexing")
        .run();

    describe("Index Calculation")
        .it("get_index computes flat offset", || {
            let t = Tensor::<f64>::new(vec![2, 3]);
            assert_eq!(t.get_index(&[0, 2]), 2);
            assert_eq!(t.get_index(&[1, 1]), 4);
        })
        .it("try_get_index validates dimensions", || {
            let t = Tensor::<f64>::new(vec![2, 2]);
            assert!(t.try_get_index(&[2, 0]).is_none());
            assert!(t.try_get_index(&[0]).is_none());
            assert!(t.try_get_index(&[0, 0, 0]).is_none());
        })
        .tag("index_calc")
        .run();

    describe("Reshape")
        .it("reshape changes shape preserving data", || {
            let t = Tensor::<f64>::from_data(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]).unwrap();
            let reshaped = t.reshape(vec![4, 1]).unwrap();
            assert_eq!(reshaped.shape, vec![4, 1]);
            assert_eq!(reshaped.data[2], 3.0);
        })
        .it("reshape with wrong total size returns error", || {
            let t = Tensor::<f64>::from_data(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]).unwrap();
            let result = t.reshape(vec![5]);
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("Cannot reshape"));
        })
        .tag("reshape")
        .run();

    describe("Arithmetic")
        .it("add", || {
            let t1 = Tensor::<f64>::from_data(vec![1.0, 2.0], vec![2]).unwrap();
            let t2 = Tensor::<f64>::from_data(vec![3.0, 4.0], vec![2]).unwrap();
            assert_eq!((t1 + t2).data, vec![4.0, 6.0]);
        })
        .it("sub", || {
            let t1 = Tensor::<f64>::from_data(vec![5.0, 7.0], vec![2]).unwrap();
            let t2 = Tensor::<f64>::from_data(vec![3.0, 4.0], vec![2]).unwrap();
            assert_eq!((t1 - t2).data, vec![2.0, 3.0]);
        })
        .it("mul (element-wise)", || {
            let t1 = Tensor::<f64>::from_data(vec![2.0, 3.0], vec![2]).unwrap();
            let t2 = Tensor::<f64>::from_data(vec![4.0, 5.0], vec![2]).unwrap();
            assert_eq!((t1 * t2).data, vec![8.0, 15.0]);
        })
        .it("div", || {
            let t1 = Tensor::<f64>::from_data(vec![6.0, 9.0], vec![2]).unwrap();
            let t2 = Tensor::<f64>::from_data(vec![2.0, 3.0], vec![2]).unwrap();
            assert_eq!((t1 / t2).data, vec![3.0, 3.0]);
        })
        .it("scalar mul", || {
            let t = Tensor::<f64>::from_data(vec![1.0, 2.0], vec![2]).unwrap();
            assert_eq!((t * 2.0).data, vec![2.0, 4.0]);
        })
        .it("scalar div", || {
            let t = Tensor::<f64>::from_data(vec![6.0, 9.0], vec![2]).unwrap();
            assert_eq!((t / 3.0).data, vec![2.0, 3.0]);
        })
        .it("rem with scalar", || {
            let t = Tensor::<f64>::from_data(vec![5.0, 10.0], vec![2]).unwrap();
            assert_eq!((t % 3.0).data, vec![2.0, 1.0]);
        })
        .it("rem with tensor", || {
            let t1 = Tensor::<f64>::from_data(vec![7.0, 8.0, 9.0], vec![3]).unwrap();
            let t2 = Tensor::<f64>::from_data(vec![4.0, 5.0, 6.0], vec![3]).unwrap();
            assert_eq!((t1 % t2).data, vec![3.0, 3.0, 3.0]);
        })
        .tag("arithmetic")
        .run();

    describe("Assign Ops")
        .it("add_assign", || {
            let mut t1 = Tensor::<f64>::from_data(vec![1.0, 2.0], vec![2]).unwrap();
            let t2 = Tensor::<f64>::from_data(vec![1.0, 1.0], vec![2]).unwrap();
            t1 += t2;
            assert_eq!(t1.data, vec![2.0, 3.0]);
        })
        .it("scalar mul_assign", || {
            let mut t = Tensor::<f64>::from_data(vec![1.0, 2.0], vec![2]).unwrap();
            t *= 3.0;
            assert_eq!(t.data, vec![3.0, 6.0]);
        })
        .it("scalar div_assign", || {
            let mut t = Tensor::<f64>::from_data(vec![6.0, 9.0], vec![2]).unwrap();
            t /= 3.0;
            assert_eq!(t.data, vec![2.0, 3.0]);
        })
        .it("scalar rem_assign", || {
            let mut t = Tensor::<f64>::from_data(vec![7.0, 8.0], vec![2]).unwrap();
            t %= 3.0;
            assert_eq!(t.data, vec![1.0, 2.0]);
        })
        .it("mul_assign tensor", || {
            let mut t1 = Tensor::<f64>::from_data(vec![2.0, 3.0], vec![2]).unwrap();
            let t2 = Tensor::<f64>::from_data(vec![4.0, 5.0], vec![2]).unwrap();
            t1 *= t2;
            assert_eq!(t1.data, vec![8.0, 15.0]);
        })
        .tag("assign")
        .run();

    describe("Comparison")
        .it("partial_ord", || {
            let t1 = Tensor::<f64>::from_data(vec![1.0, 2.0], vec![2]).unwrap();
            let t2 = Tensor::<f64>::from_data(vec![1.0, 3.0], vec![2]).unwrap();
            assert!(t1 < t2);
        })
        .tag("comparison")
        .run();

    describe("Rank")
        .it("rank of 3D tensor", || {
            assert_eq!(Tensor::<f64>::new(vec![2, 3, 4]).rank(), 3);
            assert_eq!(Tensor::<f64>::new(vec![5]).rank(), 1);
        })
        .tag("rank")
        .run();

    describe("Unit-Aware")
        .it("mul_units with same dimension", || {
            let t1 = Tensor::<Unit<f64, Newton>>::from_data(
                vec![Newton::new(1.0), Newton::new(2.0)], vec![2]).unwrap();
            let t2 = Tensor::<Unit<f64, Newton>>::from_data(
                vec![Newton::new(3.0), Newton::new(4.0)], vec![2]).unwrap();
            let res = t1.mul_units(t2);
            assert_eq!(res.data[0].value, 3.0);
            assert_eq!(res.data[0].power.to_f64(), 2.0);
        })
        .it("mul_scalar_units", || {
            let t = Tensor::<Unit<f64, Newton>>::from_data(
                vec![Newton::new(10.0)], vec![1]).unwrap();
            let scalar = Newton::new(2.0);
            let res = t.mul_scalar_units(scalar);
            assert_eq!(res.data[0].value, 20.0);
            assert_eq!(res.data[0].power.to_f64(), 2.0);
        })
        .it("zero_units", || {
            let t = Tensor::<Unit<f64, Kelvin>>::zero_units(vec![2, 3]);
            assert_eq!(t.shape, vec![2, 3]);
            assert!(t.data.iter().all(|v| v.value == 0.0 && v.power == 1.0));
        })
        .tag("units")
        .run()
        .assert_all_pass();
}
