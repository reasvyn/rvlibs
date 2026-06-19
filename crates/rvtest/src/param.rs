use std::fmt::Debug;
use std::time::Instant;

use crate::core::{TestCase, TestStatus};

/// Run a test against multiple input values, producing one [`TestCase`] per
/// input. Each case is automatically named `"{name}[{index}]"`.
///
/// The `test` closure receives a reference to each input and should use
/// `assert!` / `assert_eq!` to validate behaviour. Panics are caught and
/// reported as failures on the individual case.
///
/// # Example
///
/// ```ignore
/// use rvtest::param::parametrize;
///
/// let cases = parametrize("addition", vec![
///     (1, 2, 3),
///     (0, 0, 0),
///     (-1, 1, 0),
///     (-1, -2, -3),
/// ], |(a, b, expected)| {
///     assert_eq!(a + b, *expected);
/// });
///
/// assert_eq!(cases.len(), 4);
/// assert!(cases.iter().all(|c| c.status.is_passed()));
/// ```
pub fn parametrize<Input, Test>(
    name: &str,
    cases: impl IntoIterator<Item = Input>,
    test: Test,
) -> Vec<TestCase>
where
    Input: Debug,
    Test: Fn(&Input),
{
    let mut results = Vec::new();

    for (index, input) in cases.into_iter().enumerate() {
        let test_name = format!("{}[{}]", name, index);
        let start = Instant::now();

        let status = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            test(&input);
        }));

        let duration = start.elapsed();
        let status = match status {
            Ok(_) => TestStatus::Passed,
            Err(panic_info) => {
                let reason = extract_panic_message(&panic_info);
                TestStatus::Failed { reason, location: None }
            }
        };

        results.push(TestCase {
            name: test_name,
            suite: Some(name.to_owned()),
            tags: Vec::new(),
            status,
            duration,
            assertions: 0,
            location: None,
            parameters: vec![("input".to_owned(), format!("{input:?}"))], captured_output: None,
            bench_stats: None,
            bench_threshold: None,
        });
    }

    results
}

/// Run a test against named input values, producing one [`TestCase`] per
/// input with the given label.
///
/// # Example
///
/// ```ignore
/// use rvtest::param::parametrize_named;
///
/// let cases = parametrize_named("parse", vec![
///     ("empty", ""),
///     ("valid", "42"),
///     ("negative", "-1"),
/// ], |input| {
///     assert!(input.parse::<i32>().is_ok() || input.is_empty());
/// });
/// ```
pub fn parametrize_named<'a, Input, Test>(
    suite_name: &str,
    cases: impl IntoIterator<Item = (&'a str, Input)>,
    test: Test,
) -> Vec<TestCase>
where
    Input: Debug,
    Test: Fn(&Input),
{
    let mut results = Vec::new();

    for (label, input) in cases.into_iter() {
        let test_name = format!("{} :: {}", suite_name, label);
        let start = Instant::now();

        let status = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            test(&input);
        }));

        let duration = start.elapsed();
        let status = match status {
            Ok(_) => TestStatus::Passed,
            Err(panic_info) => {
                let reason = extract_panic_message(&panic_info);
                TestStatus::Failed { reason, location: None }
            }
        };

        results.push(TestCase {
            name: test_name,
            suite: Some(suite_name.to_owned()),
            tags: Vec::new(),
            status,
            duration,
            assertions: 0,
            location: None,
            parameters: vec![("input".to_owned(), format!("{input:?}"))], captured_output: None,
            bench_stats: None,
            bench_threshold: None,
        });
    }

    results
}

fn extract_panic_message(panic_info: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = panic_info.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = panic_info.downcast_ref::<String>() {
        s.clone()
    } else {
        "test panicked".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parametrize_all_pass() {
        let results = parametrize("add", [(1, 2), (3, 4)], |(a, b)| {
            assert_eq!(*a + *b, a + b);
        });
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|c| c.status.is_passed()));
    }

    #[test]
    fn parametrize_names_are_indexed() {
        let results = parametrize("test", [10, 20], |_| {});
        assert_eq!(results[0].name, "test[0]");
        assert_eq!(results[1].name, "test[1]");
    }

    #[test]
    fn parametrize_captures_failure() {
        let results = parametrize("div", [(4, 2), (1, 0)], |(a, b)| {
            assert_ne!(*b, 0, "division by zero");
            let _ = a / b;
        });
        assert_eq!(results.len(), 2);
        assert!(results[0].status.is_passed());
        assert!(results[1].status.is_failed());
    }

    #[test]
    fn parametrize_empty_input() {
        let results = parametrize::<i32, _>("empty", [], |_| {});
        assert!(results.is_empty());
    }

    #[test]
    fn parametrize_named_all_pass() {
        let results = parametrize_named("parse", [("ok", "42"), ("neg", "-1")], |s| {
            assert!(s.parse::<i32>().is_ok());
        });
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|c| c.status.is_passed()));
    }

    #[test]
    fn parametrize_named_uses_labels() {
        let results = parametrize_named("suite", [("a", 1), ("b", 2)], |_| {});
        assert_eq!(results[0].name, "suite :: a");
        assert_eq!(results[1].name, "suite :: b");
    }

    #[test]
    fn parametrize_named_failure() {
        let results = parametrize_named("test", [("zero", 0), ("positive", 1)], |n| {
            assert!(*n > 0, "must be positive");
        });
        assert!(results[0].status.is_failed());
        assert!(results[1].status.is_passed());
    }

    #[test]
    fn parametrize_records_parameters() {
        let results = parametrize("add", [(1, 2)], |_| {});
        assert!(!results[0].parameters.is_empty());
        assert_eq!(results[0].parameters[0].0, "input");
    }

    #[test]
    fn extract_panic_message_str() {
        let msg = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            panic!("custom message");
        }));
        let e = msg.unwrap_err();
        let s = extract_panic_message(&e);
        assert_eq!(s, "custom message");
    }

    #[test]
    fn extract_panic_message_string() {
        let msg = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            panic!("{}", "owned string");
        }));
        let e = msg.unwrap_err();
        let s = extract_panic_message(&e);
        assert_eq!(s, "owned string");
    }

    #[test]
    fn extract_panic_message_fallback() {
        let msg = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            panic!("{}", 42);
        }));
        let e = msg.unwrap_err();
        let s = extract_panic_message(&e);
        assert_eq!(s, "42");
    }
}
