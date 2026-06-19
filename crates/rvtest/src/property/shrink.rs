use std::fmt::Debug;

use crate::property::strategy::Strategy;

/// Attempt to shrink a counterexample to its minimal form.
pub(crate) fn shrink_counterexample<T, S>(
    value: &T,
    strategy: &S,
    property: &impl Fn(&T) -> bool,
    max_shrinks: u64,
) -> String
where
    T: Debug,
    S: Strategy<T>,
{
    let mut best_repr = format!("{:?}", value);
    let mut candidates = strategy.shrink(value);
    let mut iterations = 0u64;

    while !candidates.is_empty() && iterations < max_shrinks {
        match candidates.into_iter().find(|c| !property(c)) {
            Some(candidate) => {
                best_repr = format!("{:?}", candidate);
                candidates = strategy.shrink(&candidate);
            }
            None => break,
        }
        iterations += 1;
    }

    best_repr
}
