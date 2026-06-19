//! Property-based testing with random generation and shrinking.
//!
//! Verify invariants over thousands of randomly generated inputs.
//! When a counter-example is found, it is automatically shrunk to
//! the minimal failing input.
//!
//! # Submodules
//!
//! - [`strategy`] — Strategy trait and built-in strategies (any, vec, map, filter)
//! - [`shrink`] — Counterexample shrinking
//! - [`check`] — Property check functions (check, check_with)

mod shrink;
mod strategy;

mod check;

pub use self::check::{check, check_with, PropertyConfig};
pub use self::strategy::{
    any, filter, map, vec, FilterStrategy, MapStrategy, RangeStrategy, Strategy, StrategyProvider,
    VecStrategy,
};

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn any_bool_generates() {
        let strategy = any::<bool>();
        let mut rng = &mut StdRng::seed_from_u64(42);
        let val = strategy.generate(&mut rng);
        // Just verify it's a bool
        let _: bool = val;
    }

    #[test]
    fn any_i32_generates() {
        let strategy = any::<i32>();
        let mut rng = &mut StdRng::seed_from_u64(42);
        for _ in 0..100 {
            let val = strategy.generate(&mut rng);
            assert!((i32::MIN..=i32::MAX).contains(&val));
        }
    }

    #[test]
    fn any_u64_generates() {
        let strategy = any::<u64>();
        let mut rng = &mut StdRng::seed_from_u64(99);
        for _ in 0..10 {
            let _val = strategy.generate(&mut rng);
        }
    }

    #[test]
    fn any_is_strategy_provider() {
        // Compile-time check: types with StrategyProvider work
        let _s: RangeStrategy<i32> = any::<i32>();
        let _s: RangeStrategy<bool> = any::<bool>();
    }

    #[test]
    fn check_passes_for_valid_property() {
        check("identity", any::<i32>(), |a: &i32| *a == *a);
    }

    #[test]
    fn check_panics_on_false_property() {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            check("false", any::<i32>(), |_: &i32| false);
        }));
        assert!(result.is_err());
    }

    #[test]
    fn check_with_custom_config() {
        let config = PropertyConfig { num_tests: 5, max_shrinks: 10, seed: Some(12345) };
        check_with("custom", any::<u32>(), |_: &u32| true, config);
    }

    #[test]
    fn bool_shrink_true_to_false() {
        let strategy = any::<bool>();
        let shrunk = strategy.shrink(&true);
        assert_eq!(shrunk, vec![false]);
    }

    #[test]
    fn bool_shrink_false_empty() {
        let strategy = any::<bool>();
        let shrunk = strategy.shrink(&false);
        assert!(shrunk.is_empty());
    }

    #[test]
    fn map_transforms_output() {
        let strategy = map(any::<i32>(), |x| x.to_string());
        let mut rng = &mut StdRng::seed_from_u64(7);
        let val = strategy.generate(&mut rng);
        // Result should be a String representation of an i32
        let _parsed: i32 = val.parse().expect("should be a valid i32 string");
    }

    #[test]
    fn filter_rejects_bad_values() {
        let strategy = filter(any::<i32>(), |x| x % 2 == 0);
        let mut rng = &mut StdRng::seed_from_u64(42);
        for _ in 0..50 {
            let val = strategy.generate(&mut rng);
            assert!(val % 2 == 0, "filter should only produce even numbers, got {val}");
        }
    }

    #[test]
    fn vec_strategy_generates() {
        let strategy = vec(any::<i32>(), 0, 5);
        let mut rng = &mut StdRng::seed_from_u64(1);
        for _ in 0..20 {
            let v = strategy.generate(&mut rng);
            assert!(v.len() <= 5, "vec len {} > max 5", v.len());
        }
    }

    #[test]
    fn vec_strategy_shrink_pops() {
        let strategy = vec(any::<i32>(), 0, 10);
        let candidates = strategy.shrink(&vec![1, 2, 3]);
        assert_eq!(candidates, vec![vec![1, 2]]);
    }

    #[test]
    fn vec_strategy_shrink_empty() {
        let strategy = vec(any::<i32>(), 0, 10);
        let candidates: Vec<Vec<i32>> = strategy.shrink(&vec![]);
        assert!(candidates.is_empty());
    }

    #[test]
    fn strategy_provider_for_all_int_types() {
        // Compile-time checks
        let _: RangeStrategy<i8> = any::<i8>();
        let _: RangeStrategy<i16> = any::<i16>();
        let _: RangeStrategy<i64> = any::<i64>();
        let _: RangeStrategy<u8> = any::<u8>();
        let _: RangeStrategy<u16> = any::<u16>();
        let _: RangeStrategy<u32> = any::<u32>();
        let _: RangeStrategy<u64> = any::<u64>();
        let _: RangeStrategy<usize> = any::<usize>();
    }

    #[test]
    fn default_property_config() {
        let cfg = PropertyConfig::default();
        assert_eq!(cfg.num_tests, 100);
        assert_eq!(cfg.max_shrinks, 1000);
        assert!(cfg.seed.is_none());
    }

    mod map_strategy {
        use super::*;

        #[test]
        fn cloned_works() {
            let s = map(any::<i32>(), |x| x * 2);
            let _ = s.clone();
        }
    }
}
