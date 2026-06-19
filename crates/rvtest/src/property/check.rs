use std::fmt::Debug;

use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::property::shrink::shrink_counterexample;
use crate::property::strategy::Strategy;

const DEFAULT_NUM_TESTS: u64 = 100;
const DEFAULT_SHRINKS: u64 = 1000;

/// Configuration for property-based test execution.
#[derive(Debug, Clone)]
pub struct PropertyConfig {
    /// How many random test cases to generate.
    pub num_tests: u64,
    /// Maximum number of shrink steps per failing case.
    pub max_shrinks: u64,
    /// Seed for deterministic replay.
    pub seed: Option<u64>,
}

impl Default for PropertyConfig {
    fn default() -> Self {
        PropertyConfig { num_tests: DEFAULT_NUM_TESTS, max_shrinks: DEFAULT_SHRINKS, seed: None }
    }
}

/// Run a property-based test.
///
/// Generates random inputs using the given `strategy`, passing each to
/// `property`. If the property returns `false` for any input, the function
/// attempts to shrink the counterexample and then panics with a descriptive
/// message.
///
/// # Example
///
/// ```ignore
/// use rvtest::property::{check, any};
///
/// check("reversal is involutive", any::<Vec<i32>>(), |v: &Vec<i32>| {
///     let rev: Vec<_> = v.iter().rev().copied().collect();
///     let revrev: Vec<_> = rev.iter().rev().copied().collect();
///     revrev == **v
/// });
/// ```
pub fn check<T, S>(
    _name: &str,
    strategy: S,
    property: impl Fn(&T) -> bool,
) where
    T: Debug,
    S: Strategy<T>,
{
    check_with(_name, strategy, property, PropertyConfig::default());
}

/// Run a property-based test with a custom configuration.
///
/// Same as [`check`] but accepts a [`PropertyConfig`] for fine-grained
/// control over the number of tests, shrinking, and seeding.
pub fn check_with<T, S>(
    _name: &str,
    strategy: S,
    property: impl Fn(&T) -> bool,
    config: PropertyConfig,
) where
    T: Debug,
    S: Strategy<T>,
{
    let seed = config.seed.unwrap_or_else(rand::random);
    let mut rng = StdRng::seed_from_u64(seed);

    for _ in 0..config.num_tests {
        let value = strategy.generate(&mut rng);
        if !property(&value) {
            let shrunk = shrink_counterexample(&value, &strategy, &property, config.max_shrinks);
            panic!(
                "property falsified after {} test(s)\n\
                 seed: {seed}\n\
                 counterexample: {value:?}\n\
                 shrunk to: {shrunk:?}",
                config.num_tests,
            );
        }
    }
}
