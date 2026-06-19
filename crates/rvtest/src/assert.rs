//! Comprehensive assertion macros with structured diff output.
//!
//! These macros extend the standard `assert_eq!` / `assert!` family with
//! better failure messages, diff output for complex types, and convenience
//! assertions for `Result` / pattern matching / floating-point comparison.
//!
//! # Usage
//!
//! ```ignore
//! use rvtest::assert::*;
//!
//! assert_eq!(42, 42);
//! assert_ok!(Ok::<_, ()>(42));
//! assert_err!(Err::<(), _>("error"));
//! assert_matches!(Some(42), Some(_));
//! assert_delta!(1.0, 1.001, 0.01);
//! ```

use std::fmt;

/// Assert that two values are equal, showing a structured diff on failure.
///
/// On mismatch, the output shows a side-by-style diff of the debug
/// representations, making it easy to spot differences in complex types.
///
/// # Example
///
/// ```ignore
/// use rvtest::assert::assert_eq;
///
/// assert_eq!(42, 42);
/// assert_eq!(vec![1, 2], vec![1, 2]);
/// ```
#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => {{
        use $crate::assert::__assert_eq_impl;
        let left = &$left;
        let right = &$right;
        if *left != *right {
            panic!("{}", __assert_eq_impl(left, right, file!(), line!(), None));
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        use $crate::assert::__assert_eq_impl;
        let left = &$left;
        let right = &$right;
        if *left != *right {
            panic!("{}", __assert_eq_impl(left, right, file!(), line!(), Some(format_args!($($arg)+))));
        }
    }};
}

/// Internal helper that builds the failure message for [`assert_eq!`].
#[doc(hidden)]
pub fn __assert_eq_impl<T: fmt::Debug + PartialEq>(
    left: &T,
    right: &T,
    file: &str,
    line: u32,
    msg: Option<fmt::Arguments<'_>>,
) -> String {
    let left_fmt = format!("{:#?}", left);
    let right_fmt = format!("{:#?}", right);

    let mut out = String::new();

    out.push_str(&format!("\n\x1b[2m{}:{}\x1b[0m\n", file, line));

    if let Some(args) = msg {
        out.push_str(&format!("  \x1b[31;1massertion failed\x1b[0m: `(left == right)`: {}\n", args));
    } else {
        out.push_str("  \x1b[31;1massertion failed\x1b[0m: `(left == right)`\n");
    }

    // Generate diff only when multiline debug output differs.
    if left_fmt != right_fmt && (left_fmt.contains('\n') || right_fmt.contains('\n')) {
        let diff = similar::TextDiff::from_lines(&left_fmt, &right_fmt);
        out.push_str("  \x1b[2mdiff\x1b[0m ↓\n");
        for change in diff.iter_all_changes() {
            let (mark, style) = match change.tag() {
                similar::ChangeTag::Delete => ("-", "\x1b[31m"),
                similar::ChangeTag::Insert => ("+", "\x1b[32m"),
                similar::ChangeTag::Equal => (" ", ""),
            };
            out.push_str(&format!("  {style}{mark} {change}{}\x1b[0m", change.value().trim_end_matches('\n')));
            out.push('\n');
        }
    } else {
        // Single-line diff: show compact side-by-side.
        out.push_str(&format!("  \x1b[31mleft\x1b[0m:  {:?}\n", left));
        out.push_str(&format!("  \x1b[32mright\x1b[0m: {:?}\n", right));
    }

    out
}

/// Assert that a `Result` is `Ok`.
///
/// Returns the inner value on success, panics with a formatted message on failure.
///
/// # Example
///
/// ```ignore
/// use rvtest::assert::assert_ok;
///
/// let val: Result<i32, &str> = Ok(42);
/// let v = assert_ok!(val);
/// assert_eq!(v, 42);
/// ```
#[macro_export]
macro_rules! assert_ok {
    ($result:expr $(,)?) => {{
        use $crate::assert::__assert_ok_impl;
        let result = $result;
        if let Err(e) = result {
            panic!("{}", __assert_ok_impl(&e, file!(), line!(), None));
        }
        result.unwrap()
    }};
    ($result:expr, $($arg:tt)+) => {{
        use $crate::assert::__assert_ok_impl;
        let result = $result;
        if let Err(e) = result {
            panic!("{}", __assert_ok_impl(&e, file!(), line!(), Some(format_args!($($arg)+))));
        }
        result.unwrap()
    }};
}

#[doc(hidden)]
pub fn __assert_ok_impl<E: fmt::Debug>(
    err: &E,
    file: &str,
    line: u32,
    msg: Option<fmt::Arguments<'_>>,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n\x1b[2m{}:{}\x1b[0m\n", file, line));
    if let Some(args) = msg {
        out.push_str(&format!("  \x1b[31;1massertion failed\x1b[0m: expected Ok, got Err: {}\n", args));
    } else {
        out.push_str("  \x1b[31;1massertion failed\x1b[0m: expected Ok, got Err\n");
    }
    out.push_str(&format!("  \x1b[31merror\x1b[0m: {:?}\n", err));
    out
}

/// Assert that a `Result` is `Err`.
///
/// Returns the inner error on success, panics with a formatted message on failure.
///
/// # Example
///
/// ```ignore
/// use rvtest::assert::assert_err;
///
/// let val: Result<i32, &str> = Err("something broke");
/// let e = assert_err!(val);
/// assert_eq!(e, "something broke");
/// ```
#[macro_export]
macro_rules! assert_err {
    ($result:expr $(,)?) => {{
        use $crate::assert::__assert_err_impl;
        let result = $result;
        if let Ok(v) = result {
            panic!("{}", __assert_err_impl(&v, file!(), line!(), None));
        }
        result.unwrap_err()
    }};
    ($result:expr, $($arg:tt)+) => {{
        use $crate::assert::__assert_err_impl;
        let result = $result;
        if let Ok(v) = result {
            panic!("{}", __assert_err_impl(&v, file!(), line!(), Some(format_args!($($arg)+))));
        }
        result.unwrap_err()
    }};
}

#[doc(hidden)]
pub fn __assert_err_impl<T: fmt::Debug>(
    val: &T,
    file: &str,
    line: u32,
    msg: Option<fmt::Arguments<'_>>,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n\x1b[2m{}:{}\x1b[0m\n", file, line));
    if let Some(args) = msg {
        out.push_str(&format!("  \x1b[31;1massertion failed\x1b[0m: expected Err, got Ok: {}\n", args));
    } else {
        out.push_str("  \x1b[31;1massertion failed\x1b[0m: expected Err, got Ok\n");
    }
    out.push_str(&format!("  \x1b[32mvalue\x1b[0m: {:?}\n", val));
    out
}

/// Assert that a value matches a pattern.
///
/// Uses `matches!` internally. Panics with a formatted message on failure.
///
/// # Example
///
/// ```ignore
/// use rvtest::assert::assert_matches;
///
/// let val = Some(42);
/// assert_matches!(val, Some(_));
/// ```
#[macro_export]
macro_rules! assert_matches {
    ($value:expr, $pattern:pat $(,)?) => {{
        #[allow(clippy::redundant_pattern_matching)]
        let value = $value;
        #[allow(clippy::redundant_pattern_matching)]
        if !matches!(value, $pattern) {
            panic!(
                "\n\x1b[2m{}:{}\x1b[0m\n  \x1b[31;1massertion failed\x1b[0m: `(value matches pattern)`\n  \x1b[31mvalue\x1b[0m:   {:?}\n  \x1b[32mexpected\x1b[0m: {}",
                file!(),
                line!(),
                value,
                stringify!($pattern),
            );
        }
    }};
    ($value:expr, $pattern:pat, $($arg:tt)+) => {{
        #[allow(clippy::redundant_pattern_matching)]
        let value = $value;
        #[allow(clippy::redundant_pattern_matching)]
        if !matches!(value, $pattern) {
            panic!(
                "\n\x1b[2m{}:{}\x1b[0m\n  \x1b[31;1massertion failed\x1b[0m: `(value matches pattern)`: {}\n  \x1b[31mvalue\x1b[0m:   {:?}\n  \x1b[32mexpected\x1b[0m: {}",
                file!(),
                line!(),
                format_args!($($arg)+),
                value,
                stringify!($pattern),
            );
        }
    }};
}

/// Helper: absolute difference for any type supporting `PartialOrd + Sub`.
#[doc(hidden)]
pub fn __abs_diff<T: PartialOrd + std::ops::Sub<Output = T> + Copy>(a: T, b: T) -> T {
    if a >= b { a - b } else { b - a }
}

/// Assert that two floats are equal within a delta (epsilon).
///
/// # Example
///
/// ```ignore
/// use rvtest::assert::assert_delta;
///
/// assert_delta!(1.0_f64, 1.001_f64, 0.01_f64);
/// ```
#[macro_export]
macro_rules! assert_delta {
    ($left:expr, $right:expr, $eps:expr $(,)?) => {{
        let left = &$left;
        let right = &$right;
        let eps = &$eps;
        let diff = $crate::assert::__abs_diff(*left, *right);
        if diff > *eps {
            panic!(
                "\n\x1b[2m{}:{}\x1b[0m\n  \x1b[31;1massertion failed\x1b[0m: `(left ≈ right)`\n  \x1b[31mleft\x1b[0m:     {:?}\n  \x1b[32mright\x1b[0m:    {:?}\n  \x1b[33mdiff\x1b[0m:     {:?}\n  \x1b[33mepsilon\x1b[0m:  {:?}",
                file!(),
                line!(),
                left,
                right,
                diff,
                eps,
            );
        }
    }};
    ($left:expr, $right:expr, $eps:expr, $($arg:tt)+) => {{
        let left = &$left;
        let right = &$right;
        let eps = &$eps;
        let diff = $crate::assert::__abs_diff(*left, *right);
        if diff > *eps {
            panic!(
                "\n\x1b[2m{}:{}\x1b[0m\n  \x1b[31;1massertion failed\x1b[0m: `(left ≈ right)`: {}\n  \x1b[31mleft\x1b[0m:     {:?}\n  \x1b[32mright\x1b[0m:    {:?}\n  \x1b[33mdiff\x1b[0m:     {:?}\n  \x1b[33mepsilon\x1b[0m:  {:?}",
                file!(),
                line!(),
                format_args!($($arg)+),
                left,
                right,
                diff,
                eps,
            );
        }
    }};
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn assert_eq_passes() {
        assert_eq!(42, 42);
        assert_eq!("hello", "hello");
        assert_eq!(vec![1, 2, 3], vec![1, 2, 3]);
    }

    #[test]
    fn assert_eq_panics_on_mismatch() {
        let r = catch_unwind(AssertUnwindSafe(|| {
            assert_eq!(1, 2);
        }));
        assert!(r.is_err());
    }

    #[test]
    fn assert_eq_with_message() {
        let r = catch_unwind(AssertUnwindSafe(|| {
            assert_eq!(1, 2, "custom message");
        }));
        assert!(r.is_err());
        let msg = r.unwrap_err().downcast_ref::<String>().unwrap().to_string();
        assert!(msg.contains("custom message"));
    }

    #[test]
    fn assert_eq_singleline_diff() {
        let r = catch_unwind(AssertUnwindSafe(|| {
            assert_eq!("hello", "world");
        }));
        assert!(r.is_err());
        let msg = r.unwrap_err().downcast_ref::<String>().unwrap().to_string();
        assert!(msg.contains("left"));
        assert!(msg.contains("hello"));
    }

    #[test]
    fn assert_eq_multiline_diff() {
        let r = catch_unwind(AssertUnwindSafe(|| {
            assert_eq!(
                vec![vec![1, 2], vec![3, 4]],
                vec![vec![1, 2], vec![3, 5]]
            );
        }));
        assert!(r.is_err());
        let msg = r.unwrap_err().downcast_ref::<String>().unwrap().to_string();
        assert!(msg.contains("diff") || msg.contains("left"));
    }

    #[test]
    fn assert_eq_with_nested_struct() {
        #[derive(Debug, PartialEq)]
        struct Point { x: i32, y: i32 }
        let r = catch_unwind(AssertUnwindSafe(|| {
            assert_eq!(Point { x: 1, y: 2 }, Point { x: 1, y: 99 });
        }));
        assert!(r.is_err());
    }

    #[test]
    fn assert_ok_passes() {
        let val: Result<i32, &str> = Ok(42);
        let v = assert_ok!(val);
        assert_eq!(v, 42);
    }

    #[test]
    fn assert_ok_panics_on_err() {
        let val: Result<i32, &str> = Err("fail");
        let r = catch_unwind(AssertUnwindSafe(|| {
            let _ = assert_ok!(val);
        }));
        assert!(r.is_err());
    }

    #[test]
    fn assert_ok_returns_inner() {
        let v = assert_ok!(Ok::<_, ()>(99));
        assert_eq!(v, 99);
    }

    #[test]
    fn assert_err_passes() {
        let val: Result<i32, &str> = Err("error");
        let e = assert_err!(val);
        assert_eq!(e, "error");
    }

    #[test]
    fn assert_err_panics_on_ok() {
        let val: Result<i32, &str> = Ok(42);
        let r = catch_unwind(AssertUnwindSafe(|| {
            let _ = assert_err!(val);
        }));
        assert!(r.is_err());
    }

    #[test]
    fn assert_matches_passes() {
        assert_matches!(Some(42), Some(_));
        assert_matches!(Ok::<_, ()>(1), Ok(_));
        assert_matches!(&vec![1, 2, 3][..], [1, ..]);
    }

    #[test]
    fn assert_matches_panics_on_mismatch() {
        let r = catch_unwind(AssertUnwindSafe(|| {
            assert_matches!(None::<i32>, Some(_));
        }));
        assert!(r.is_err());
    }

    #[test]
    fn assert_delta_passes() {
        assert_delta!(1.0_f64, 1.001_f64, 0.01_f64);
        assert_delta!(100.0_f64, 100.0001_f64, 0.001_f64);
    }

    #[test]
    fn assert_delta_panics_on_excess() {
        let r = catch_unwind(AssertUnwindSafe(|| {
            assert_delta!(1.0_f64, 2.0_f64, 0.1_f64);
        }));
        assert!(r.is_err());
    }

    #[test]
    fn assert_delta_with_message() {
        let r = catch_unwind(AssertUnwindSafe(|| {
            assert_delta!(1.0_f64, 2.0_f64, 0.1_f64, "floats differ");
        }));
        assert!(r.is_err());
        let msg = r.unwrap_err().downcast_ref::<String>().unwrap().to_string();
        assert!(msg.contains("floats differ"));
    }

    #[test]
    fn assert_eq_diff_on_multiline() {
        let complex = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(complex.clone(), complex);
    }

    #[test]
    fn assert_ok_with_message() {
        let val: Result<i32, &str> = Err("fail");
        let r = catch_unwind(AssertUnwindSafe(|| {
            let _ = assert_ok!(val, "expected success");
        }));
        assert!(r.is_err());
        let msg = r.unwrap_err().downcast_ref::<String>().unwrap().to_string();
        assert!(msg.contains("expected success"));
    }

    #[test]
    fn assert_err_with_message() {
        let val: Result<i32, &str> = Ok(42);
        let r = catch_unwind(AssertUnwindSafe(|| {
            let _ = assert_err!(val, "expected failure");
        }));
        assert!(r.is_err());
        let msg = r.unwrap_err().downcast_ref::<String>().unwrap().to_string();
        assert!(msg.contains("expected failure"));
    }

    #[test]
    fn assert_matches_with_message() {
        let r = catch_unwind(AssertUnwindSafe(|| {
            assert_matches!(None::<i32>, Some(_), "should be Some");
        }));
        assert!(r.is_err());
        let msg = r.unwrap_err().downcast_ref::<String>().unwrap().to_string();
        assert!(msg.contains("should be Some"));
    }

    #[test]
    fn assert_delta_exact_match() {
        assert_delta!(1.0_f64, 1.0_f64, 0.0_f64);
        assert_delta!(-1.0_f64, -1.0_f64, 0.0_f64);
    }

    #[test]
    fn assert_delta_large_values() {
        assert_delta!(1_000_000.0_f64, 1_000_001.0_f64, 2.0_f64);
    }
}
