use rvtest::spec::describe;
use rvmath::{Natural, Num, NumberKind, NumberSet, Signed};

#[test]
fn num_basic_tests() {
    describe("Num Factory")
        .it("creates from f64 and i32 with cross-type equality", || {
            let num_f64 = Num::new(2.0);
            let num_i32 = Num::new(2);
            assert!(num_f64 == num_i32);
        })
        .it("default is zero", || {
            let n: Num<f64> = Default::default();
            assert_eq!(n.value, 0.0);
            let m: Num<i32> = Default::default();
            assert_eq!(m.value, 0);
        })
        .it("zero and one constructors", || {
            let z: Num<f64> = Num::zero();
            assert_eq!(z.value, 0.0);
            let o: Num<f64> = Num::one();
            assert_eq!(o.value, 1.0);
        })
        .tag("num_factory")
        .run();

    describe("Num Operations")
        .it("arithmetic on f64", || {
            let a = Num::new(10.0_f64);
            let b = Num::new(3.0_f64);
            assert!(((a + b).value - 13.0).abs() < 1e-10);
            assert!(((a - b).value - 7.0).abs() < 1e-10);
            assert!(((a * b).value - 30.0).abs() < 1e-10);
            assert!(((a / b).value - 10.0 / 3.0).abs() < 1e-10);
            assert!(((a % b).value - 1.0).abs() < 1e-10);
        })
        .it("arithmetic on i32", || {
            let a = Num::new(10i32);
            let b = Num::new(3i32);
            assert_eq!((a + b).value, 13);
            assert_eq!((a - b).value, 7);
            assert_eq!((a * b).value, 30);
            assert_eq!((a / b).value, 3);
            assert_eq!((a % b).value, 1);
        })
        .it("cross-type equality", || {
            let a = Num::new(5.0_f64);
            let b = Num::new(5i32);
            assert!(a == b);
            let c = Num::new(6.0_f64);
            assert!(a != c);
        })
        .it("cross-type ordering", || {
            let a = Num::new(10.0_f64);
            let b = Num::new(3i32);
            assert!(a > b);
            assert!(b < a);
        })
        .tag("num_ops")
        .run();

    describe("Natural")
        .it("positive natural is ok", || {
            let n = Natural::new(5u32).unwrap();
            assert_eq!(*n, 5u32);
        })
        .it("zero is rejected", || {
            assert!(Natural::new(0u32).is_none());
        })
        .it("negative is rejected", || {
            assert!(Natural::new(-3i32).is_none());
        })
        .it("add", || {
            let a = Natural::new(3u32).unwrap();
            let b = Natural::new(4u32).unwrap();
            assert_eq!(*(a + b), 7u32);
        })
        .it("mul", || {
            let a = Natural::new(3u32).unwrap();
            let b = Natural::new(4u32).unwrap();
            assert_eq!(*(a * b), 12u32);
        })
        .it("sub success", || {
            let a = Natural::new(5u32).unwrap();
            let b = Natural::new(3u32).unwrap();
            assert_eq!((a - b).unwrap(), Natural::new(2u32).unwrap());
        })
        .it("sub fail when a <= b", || {
            let a = Natural::new(3u32).unwrap();
            assert!((a - Natural::new(5u32).unwrap()).is_none());
            assert!((a - Natural::new(3u32).unwrap()).is_none());
        })
        .it("number_kind is Natural", || {
            assert_eq!(<Natural<u32> as NumberKind>::number_set(), NumberSet::Natural);
            assert!(!<Natural<u32> as NumberKind>::is_signed());
            assert!(<Natural<u32> as NumberKind>::is_integer_valued());
        })
        .it("succ", || {
            let n = Natural::new(1u32).unwrap();
            assert_eq!(*n.succ(), 2u32);
            assert_eq!(*Natural::new(100u32).unwrap().succ(), 101u32);
        })
        .it("pred", || {
            assert_eq!(*Natural::new(5u32).unwrap().pred().unwrap(), 4u32);
            assert!(Natural::new(1u32).unwrap().pred().is_none());
        })
        .it("checked_div", || {
            let a = Natural::new(6u32).unwrap();
            let b = Natural::new(2u32).unwrap();
            assert_eq!(*a.checked_div(b).unwrap(), 3u32);
            assert!(Natural::new(4u32).unwrap().checked_div(Natural::new(5u32).unwrap()).is_none());
        })
        .it("div operator", || {
            let a = Natural::new(6u32).unwrap();
            let b = Natural::new(2u32).unwrap();
            assert_eq!(*(a / b).unwrap(), 3u32);
            assert!((Natural::new(4u32).unwrap() / Natural::new(5u32).unwrap()).is_none());
        })
        .it("rem operator", || {
            let a = Natural::new(10u32).unwrap();
            let b = Natural::new(3u32).unwrap();
            assert_eq!(*(a % b), 1u32);
        })
        .it("display", || {
            let n = Natural::new(42u32).unwrap();
            assert_eq!(format!("{}", n), "42");
        })
        .it("into_inner", || {
            let n = Natural::new(7u32).unwrap();
            assert_eq!(n.into_inner(), 7u32);
        })
        .it("checked_add/checked_mul/checked_sub", || {
            let a = Natural::new(3u32).unwrap();
            let b = Natural::new(4u32).unwrap();
            assert_eq!(*a.checked_add(b).unwrap(), 7u32);
            assert_eq!(*a.checked_mul(b).unwrap(), 12u32);
            assert_eq!(*Natural::new(5u32).unwrap().checked_sub(Natural::new(3u32).unwrap()).unwrap(), 2u32);
            assert!(Natural::new(3u32).unwrap().checked_sub(Natural::new(3u32).unwrap()).is_none());
        })
        .tag("natural")
        .run();

    describe("NumberSet Hierarchy")
        .it("self is subset", || {
            for set in &[NumberSet::Natural, NumberSet::Whole, NumberSet::Integer,
                         NumberSet::Rational, NumberSet::Real, NumberSet::Complex] {
                assert!(set.is_subset_of(*set));
            }
        })
        .it("natural subset chain", || {
            assert!(NumberSet::Natural.is_subset_of(NumberSet::Whole));
            assert!(NumberSet::Natural.is_subset_of(NumberSet::Integer));
            assert!(NumberSet::Natural.is_subset_of(NumberSet::Rational));
            assert!(NumberSet::Natural.is_subset_of(NumberSet::Real));
            assert!(NumberSet::Natural.is_subset_of(NumberSet::Complex));
        })
        .it("whole subset chain", || {
            assert!(!NumberSet::Whole.is_subset_of(NumberSet::Natural));
            assert!(NumberSet::Whole.is_subset_of(NumberSet::Integer));
            assert!(NumberSet::Whole.is_subset_of(NumberSet::Complex));
        })
        .it("integer subset chain", || {
            assert!(!NumberSet::Integer.is_subset_of(NumberSet::Natural));
            assert!(!NumberSet::Integer.is_subset_of(NumberSet::Whole));
            assert!(NumberSet::Integer.is_subset_of(NumberSet::Rational));
            assert!(NumberSet::Integer.is_subset_of(NumberSet::Complex));
        })
        .it("rational subset chain", || {
            assert!(!NumberSet::Rational.is_subset_of(NumberSet::Integer));
            assert!(NumberSet::Rational.is_subset_of(NumberSet::Real));
            assert!(NumberSet::Rational.is_subset_of(NumberSet::Complex));
        })
        .it("real subset chain", || {
            assert!(!NumberSet::Real.is_subset_of(NumberSet::Rational));
            assert!(NumberSet::Real.is_subset_of(NumberSet::Complex));
        })
        .it("complex is top", || {
            assert!(!NumberSet::Complex.is_subset_of(NumberSet::Real));
        })
        .tag("subset")
        .run();

    describe("NumberKind for Built-in Types")
        .it("u32 is Whole", || {
            assert_eq!(u32::number_set(), NumberSet::Whole);
            assert!(!u32::is_signed());
            assert!(u32::is_integer_valued());
        })
        .it("i32 is Integer", || {
            assert_eq!(i32::number_set(), NumberSet::Integer);
            assert!(i32::is_signed());
            assert!(i32::is_integer_valued());
        })
        .it("f64 is Real", || {
            assert_eq!(f64::number_set(), NumberSet::Real);
            assert!(f64::is_signed());
            assert!(!f64::is_integer_valued());
        })
        .it("Num delegates to inner type", || {
            assert_eq!(Num::<u32>::number_set(), NumberSet::Whole);
            assert_eq!(Num::<i32>::number_set(), NumberSet::Integer);
            assert_eq!(Num::<f64>::number_set(), NumberSet::Real);
        })
        .tag("numberkind")
        .run();

    describe("NumberSet::contains")
        .it("contains checks", || {
            assert!(NumberSet::Natural.contains::<Natural<u32>>());
            assert!(!NumberSet::Natural.contains::<u32>());
            assert!(!NumberSet::Natural.contains::<i32>());
            assert!(NumberSet::Whole.contains::<u32>());
            assert!(NumberSet::Whole.contains::<Natural<u32>>());
            assert!(!NumberSet::Whole.contains::<i32>());
            assert!(NumberSet::Integer.contains::<i32>());
            assert!(NumberSet::Integer.contains::<u32>());
            assert!(!NumberSet::Integer.contains::<f64>());
            assert!(NumberSet::Real.contains::<f64>());
            assert!(NumberSet::Real.contains::<f32>());
            assert!(NumberSet::Real.contains::<i32>());
            assert!(NumberSet::Complex.contains::<f64>());
            assert!(NumberSet::Complex.contains::<i32>());
        })
        .tag("contains")
        .run();

    describe("NaN/Inf/Finite")
        .it("f64 classification", || {
            assert!(f64::NAN.is_nan());
            assert!(f64::INFINITY.is_infinite());
            assert!(f64::NEG_INFINITY.is_infinite());
            assert!(!f64::INFINITY.is_finite());
            assert!(!f64::NAN.is_finite());
            assert!(f64::is_finite(42.0_f64));
        })
        .it("f32 classification", || {
            assert!(f32::NAN.is_nan());
            assert!(f32::INFINITY.is_infinite());
            assert!(!f32::INFINITY.is_finite());
            assert!(1.0_f32.is_finite());
        })
        .tag("nan_inf")
        .run();

    describe("Signed")
        .it("is_positive on f64", || {
            assert!(<f64 as Signed>::is_positive(&1.0));
            assert!(!<f64 as Signed>::is_positive(&-1.0));
            assert!(!<f64 as Signed>::is_positive(&0.0));
        })
        .it("is_negative on f64", || {
            assert!(<f64 as Signed>::is_negative(&-1.0));
            assert!(!<f64 as Signed>::is_negative(&1.0));
            assert!(!<f64 as Signed>::is_negative(&0.0));
        })
        .it("is_positive on i32", || {
            assert!(<i32 as Signed>::is_positive(&1));
            assert!(!<i32 as Signed>::is_positive(&-1));
            assert!(!<i32 as Signed>::is_positive(&0));
        })
        .it("is_negative on i32", || {
            assert!(<i32 as Signed>::is_negative(&-1));
            assert!(!<i32 as Signed>::is_negative(&1));
            assert!(!<i32 as Signed>::is_negative(&0));
        })
        .it("is_positive on f32", || {
            assert!(<f32 as Signed>::is_positive(&1.0));
            assert!(!<f32 as Signed>::is_positive(&-1.0));
            assert!(!<f32 as Signed>::is_positive(&0.0));
        })
        .it("is_negative on f32", || {
            assert!(<f32 as Signed>::is_negative(&-1.0));
            assert!(!<f32 as Signed>::is_negative(&1.0));
            assert!(!<f32 as Signed>::is_negative(&0.0));
        })
        .tag("signed")
        .run()
        .assert_all_pass();
}
