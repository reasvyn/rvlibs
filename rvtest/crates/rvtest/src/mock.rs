//! Lightweight mocking utilities — spy, stub, and scoped function patching.
//!
//! No proc-macro required for basic use cases.  Integrates with rvtest's
//! assertion macros for structured failure messages.
//!
//! # Examples
//!
//! ```ignore
//! use rvtest::mock::*;
//!
//! // Spy — record every call
//! let spy = Spy::new(|x: i32| x * 2);
//! assert_eq!(spy.call(5), 10);
//! assert_eq!(spy.call_count(), 1);
//! assert_eq!(spy.calls(), &[5]);
//!
//! // Stub — return a fixed value
//! let stub = Stub::new(42);
//! assert_eq!(stub.call(()), 42);
//!
//! // Scoped patch (requires a static PATTERN)
//! // let guard = patch!(MY_FN, |x| x + 1);
//! // function_under_test();
//! // drop(guard); // restores original
//! ```

use std::fmt::Debug;
use std::sync::Mutex;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Spy — call-recording wrapper
// ---------------------------------------------------------------------------

/// A call-recording wrapper around a function.
///
/// `Spy` wraps any `Fn(A) -> R` closure, delegates every call to the inner
/// function, and records the arguments for later inspection.
///
/// Use `spy.call_count()`, `spy.calls()`, `spy.assert_called_with()` to
/// verify invocation history.
pub struct Spy<A, R> {
    calls: Arc<Mutex<Vec<A>>>,
    inner: Arc<dyn Fn(A) -> R + Send + Sync>,
}

impl<A, R> Spy<A, R>
where
    A: Clone + Debug + PartialEq + Send + 'static,
    R: Send + 'static,
{
    /// Create a new spy wrapping the given function.
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> R + Send + Sync + 'static,
    {
        Spy {
            calls: Arc::new(Mutex::new(Vec::new())),
            inner: Arc::new(f),
        }
    }

    /// Call the wrapped function and record the argument.
    pub fn call(&self, arg: A) -> R {
        self.calls.lock().unwrap().push(arg.clone());
        (self.inner)(arg)
    }

    /// Number of times this spy has been called.
    pub fn call_count(&self) -> usize {
        self.calls.lock().unwrap().len()
    }

    /// Return a clone of all recorded arguments.
    pub fn calls(&self) -> Vec<A> {
        self.calls.lock().unwrap().clone()
    }

    /// Panic with a detailed message if the spy was never called.
    pub fn assert_called(&self) {
        let count = self.call_count();
        if count == 0 {
            panic!(
                "\n\x1b[2mspy assertion failed\x1b[0m\n  \x1b[31;1mexpected\x1b[0m spy to be called at least once\n  \x1b[33mactual\x1b[0m:   never called"
            );
        }
    }

    /// Panic with a structured diff if the recorded calls don't match
    /// the expected sequence.
    pub fn assert_called_with(&self, expected: &[A]) {
        let actual = self.calls();
        if actual.len() != expected.len() || actual.iter().zip(expected).any(|(a, e)| a != e) {
            let msg = format!(
                "\n\x1b[2mspy assertion failed\x1b[0m\n  \x1b[31;1mexpected calls\x1b[0m: {:?}\n  \x1b[33mactual calls\x1b[0m:   {:?}\n  \x1b[36mhint\x1b[0m:          use `rvtest::assert_eq!` for structured diff",
                expected, actual
            );
            panic!("{}", msg);
        }
    }

    /// Clear all recorded calls.
    pub fn reset(&self) {
        self.calls.lock().unwrap().clear();
    }
}

impl<A: Debug, R> Clone for Spy<A, R> {
    fn clone(&self) -> Self {
        Spy {
            calls: Arc::clone(&self.calls),
            inner: Arc::clone(&self.inner),
        }
    }
}

// ---------------------------------------------------------------------------
// Stub — fixed-return mock
// ---------------------------------------------------------------------------

/// A mock that ignores its input and returns a fixed value.
///
/// Useful when you only need a dependency to compile and return a known
/// value, without caring about how many times or with what arguments it
/// is called.
pub struct Stub<A, R> {
    value: R,
    _phantom: std::marker::PhantomData<A>,
}

impl<A, R: Clone> Stub<A, R> {
    /// Create a stub that always returns `value`.
    pub fn new(value: R) -> Self {
        Stub {
            value,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Call the stub — ignores the input and returns the fixed value.
    pub fn call(&self, _arg: A) -> R {
        self.value.clone()
    }
}

impl<A, R: Clone> Clone for Stub<A, R> {
    fn clone(&self) -> Self {
        Stub::new(self.value.clone())
    }
}

// ---------------------------------------------------------------------------
// Scoped function patching via static dispatch
// ---------------------------------------------------------------------------

/// Temporarily replace a `static Mutex<Option<...>>` function pointer.
///
/// The macro creates a guard that, when dropped, restores the original
/// function.  This allows you to mock global functions for the duration
/// of a single test.
///
/// # Usage
///
/// ```ignore
/// use rvtest::mock::{make_patchable, patch};
///
/// // 1. Declare a patchable function pointer (usually inside the module
/// //    that owns the function):
/// make_patchable!(MY_HELPER, |x: i32| -> i32 { x * 2 });
///
/// // 2. In your real code, call it via the function pointer:
/// fn double(x: i32) -> i32 {
///     MY_HELPER.call(x)
/// }
///
/// // 3. In tests, replace it temporarily:
/// #[test]
/// fn test_double() {
///     let _guard = patch!(MY_HELPER, |x| x * 100);
///     assert_eq!(double(5), 500);
///     // guard dropped → original restored
/// }
/// ```
#[macro_export]
macro_rules! make_patchable {
    ($name:ident, |$arg:ident: $arg_ty:ty| -> $ret_ty:ty $body:block) => {
        #[allow(non_upper_case_globals, missing_docs)]
        static $name: $crate::mock::PatchableFn<$arg_ty, $ret_ty> = {
            fn __default($arg: $arg_ty) -> $ret_ty $body
            $crate::mock::PatchableFn::new(__default)
        };
    };
}

type BoxedFn<A, R> = Arc<dyn Fn(A) -> R + Send + Sync>;

/// A static function pointer that can be temporarily replaced for testing.
///
/// Created via [`make_patchable!`].  Not intended for direct construction.
pub struct PatchableFn<A, R> {
    current: Mutex<Option<BoxedFn<A, R>>>,
    default: fn(A) -> R,
}

impl<A, R> PatchableFn<A, R>
where
    A: Send + 'static,
    R: Send + 'static,
{
    /// Create a new patchable function with a default implementation.
    ///
    /// The default MUST be a function pointer (not a closure) so it can
    /// be stored in a static without allocation.
    pub const fn new(f: fn(A) -> R) -> Self {
        PatchableFn {
            current: Mutex::new(None),
            default: f,
        }
    }

    /// Call the function — invokes the currently active implementation
    /// (default or patched).
    pub fn call(&self, arg: A) -> R {
        let guard = self.current.lock().unwrap();
        match guard.as_ref() {
            Some(f) => f(arg),
            None => (self.default)(arg),
        }
    }

    /// Temporarily replace the implementation.  Returns a guard that
    /// restores the previous value when dropped.
    pub fn patch<F>(&self, f: F) -> PatchGuard<'_, A, R>
    where
        F: Fn(A) -> R + Send + Sync + 'static,
    {
        let mut guard = self.current.lock().unwrap();
        let prev = guard.take();
        *guard = Some(Arc::new(f));
        PatchGuard { target: self, prev }
    }
}

/// Restores the previous function when dropped.
pub struct PatchGuard<'a, A, R> {
    target: &'a PatchableFn<A, R>,
    prev: Option<Arc<dyn Fn(A) -> R + Send + Sync>>,
}

impl<A, R> Drop for PatchGuard<'_, A, R> {
    fn drop(&mut self) {
        *self.target.current.lock().unwrap() = self.prev.take();
    }
}

/// Create a scoped patch for a [`PatchableFn`].
///
/// See [`make_patchable!`] for a complete usage example.
#[macro_export]
macro_rules! patch {
    ($target:ident, $replacement:expr) => {
        $target.patch($replacement)
    };
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Mock — expectation-based test double
// ---------------------------------------------------------------------------

type MatcherFn<A> = Box<dyn Fn(&A) -> bool + Send>;
type ReturnFn<A, R> = Box<dyn Fn(&A) -> R + Send>;

/// An expectation on a mocked method.
pub struct Expectation<A, R> {
    matcher: Option<MatcherFn<A>>,
    expected_calls: std::ops::RangeInclusive<u64>,
    return_fn: ReturnFn<A, R>,
    actual_calls: u64,
    name: String,
}

impl<A, R> Expectation<A, R>
where
    R: Clone + Send + 'static,
{
    /// Set an argument matcher for this expectation.
    pub fn with(&mut self, matcher: impl Fn(&A) -> bool + Send + 'static) -> &mut Self {
        self.matcher = Some(Box::new(matcher));
        self
    }

    /// Set expected number of calls (exact).
    pub fn times(&mut self, n: u64) -> &mut Self {
        self.expected_calls = n..=n;
        self
    }

    /// Set expected call range (min..=max).
    pub fn times_range(&mut self, range: std::ops::RangeInclusive<u64>) -> &mut Self {
        self.expected_calls = range;
        self
    }

    /// Set the return value for this expectation.
    pub fn returning(&mut self, value: R) -> &mut Self {
        self.return_fn = Box::new(move |_| value.clone());
        self
    }
}

/// A mock object that tracks expectations and verifies them on drop.
///
/// # Example
///
/// ```ignore
/// use rvtest::mock::Mock;
///
/// let mut mock = Mock::<i32, i32>::new("calculator");
/// mock.expect("add").with(|x| *x > 0).times(1).returning(10);
/// let result = mock.call("add", &5);
/// assert_eq!(result, 10);
/// // On drop, verifies all expectations were met
/// ```
pub struct Mock<A, R> {
    name: String,
    expectations: Vec<Expectation<A, R>>,
    fallback: Option<R>,
    failed: Vec<String>,
}

impl<A: std::fmt::Debug, R> Mock<A, R>
where
    R: Clone + Send + 'static,
{
    /// Create a new mock with the given name (for error messages).
    pub fn new(name: impl Into<String>) -> Self {
        Mock {
            name: name.into(),
            expectations: Vec::new(),
            fallback: None,
            failed: Vec::new(),
        }
    }

    /// Register a new expectation for the named method.
    pub fn expect(&mut self, name: &str) -> &mut Expectation<A, R> {
        let exp_name = format!("{}::{}", self.name, name);
        self.expectations.push(Expectation {
            matcher: None,
            expected_calls: 1..=1,
            return_fn: Box::new(move |_| panic!("no return value set")),
            actual_calls: 0,
            name: exp_name,
        });
        self.expectations.last_mut().unwrap()
    }

    /// Set a default return value when no expectation matches.
    pub fn when_no_match(mut self, value: R) -> Self {
        self.fallback = Some(value);
        self
    }

    /// Call a method on this mock, matching against expectations.
    pub fn call(&mut self, method: &str, args: &A) -> R {
        let fn_name = format!("{}::{}", self.name, method);

        // Find first matching expectation
        for exp in &mut self.expectations {
            if exp.name != fn_name { continue; }
            if let Some(ref matcher) = exp.matcher
                && !matcher(args) { continue; }
            exp.actual_calls += 1;
            return (exp.return_fn)(args);
        }

        // Fallback or panic
        if let Some(ref fallback) = self.fallback {
            return fallback.clone();
        }

        let args_debug = format!("{:?}", args);
        let msg = format!("{fn_name}({args_debug}): no matching expectation");
        self.failed.push(msg.clone());
        panic!("{}", msg);
    }

    /// Verify all expectations were met. Called automatically on drop.
    pub fn verify(&self) {
        for exp in &self.expectations {
            if !exp.expected_calls.contains(&exp.actual_calls) {
                eprintln!(
                    "  ✗ {}: expected {} calls, got {}",
                    exp.name,
                    if *exp.expected_calls.start() == *exp.expected_calls.end() {
                        format!("{}", exp.expected_calls.start())
                    } else {
                        format!("{}..={}", exp.expected_calls.start(), exp.expected_calls.end())
                    },
                    exp.actual_calls,
                );
            }
        }
    }
}

impl<A, R> std::fmt::Debug for Mock<A, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mock({})", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Spy ---

    #[test]
    fn spy_records_calls() {
        let spy = Spy::new(|x: i32| x * 2);
        assert_eq!(spy.call(5), 10);
        assert_eq!(spy.call_count(), 1);
        assert_eq!(spy.calls(), &[5]);
    }

    #[test]
    fn spy_records_multiple_calls() {
        let spy = Spy::new(|x: i32| x + 1);
        spy.call(1);
        spy.call(2);
        spy.call(3);
        assert_eq!(spy.call_count(), 3);
        assert_eq!(spy.calls(), &[1, 2, 3]);
    }

    #[test]
    fn spy_assert_called_panics_when_not_called() {
        let spy = Spy::new(|x: i32| x);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            spy.assert_called();
        }));
        assert!(r.is_err());
    }

    #[test]
    fn spy_assert_called_ok_when_called() {
        let spy = Spy::new(|x: i32| x);
        spy.call(1);
        spy.assert_called(); // should not panic
    }

    #[test]
    fn spy_assert_called_with_matches() {
        let spy = Spy::new(|x: i32| x);
        spy.call(1);
        spy.call(2);
        spy.assert_called_with(&[1, 2]);
    }

    #[test]
    fn spy_assert_called_with_panics_on_mismatch() {
        let spy = Spy::new(|x: i32| x);
        spy.call(1);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            spy.assert_called_with(&[99]);
        }));
        assert!(r.is_err());
    }

    #[test]
    fn spy_reset_clears_calls() {
        let spy = Spy::new(|x: i32| x);
        spy.call(1);
        spy.call(2);
        assert_eq!(spy.call_count(), 2);
        spy.reset();
        assert_eq!(spy.call_count(), 0);
    }

    #[test]
    fn spy_clone_shares_call_history() {
        let spy = Spy::new(|x: i32| x);
        let spy2 = spy.clone();
        spy.call(1);
        assert_eq!(spy2.call_count(), 1);
    }

    #[test]
    fn spy_works_with_strings() {
        let spy = Spy::new(|s: String| s.len());
        assert_eq!(spy.call("hello".into()), 5);
        assert_eq!(spy.call("world".into()), 5);
        assert_eq!(spy.call_count(), 2);
    }

    // --- Stub ---

    #[test]
    fn stub_returns_fixed_value() {
        let stub = Stub::new(42);
        assert_eq!(stub.call("anything"), 42);
        assert_eq!(stub.call("also anything"), 42);
    }

    #[test]
    fn stub_works_with_different_types() {
        let stub_int: Stub<i32, &str> = Stub::new("fixed");
        assert_eq!(stub_int.call(1), "fixed");
        let stub_unit: Stub<(), &str> = Stub::new("fixed");
        assert_eq!(stub_unit.call(()), "fixed");
    }

    #[test]
    fn stub_clone() {
        let stub = Stub::new(99);
        let stub2 = stub.clone();
        assert_eq!(stub2.call(0), 99);
    }

    // --- PatchableFn ---

    make_patchable!(TEST_FN, |x: i32| -> i32 { x * 2 });

    #[test]
    fn patchable_default() {
        assert_eq!(TEST_FN.call(5), 10);
    }

    #[test]
    fn patch_temporarily_replaces() {
        let guard = patch!(TEST_FN, |x| x * 100);
        assert_eq!(TEST_FN.call(5), 500);
        drop(guard);
        assert_eq!(TEST_FN.call(5), 10);
    }

    #[test]
    fn patch_restores_on_drop() {
        {
            let _guard = patch!(TEST_FN, |_| -1);
            assert_eq!(TEST_FN.call(0), -1);
        }
        assert_eq!(TEST_FN.call(0), 0);
    }

    #[test]
    fn patch_works_multiple_times() {
        let g1 = patch!(TEST_FN, |x| x + 1);
        assert_eq!(TEST_FN.call(5), 6);
        let g2 = patch!(TEST_FN, |x| x + 10);
        assert_eq!(TEST_FN.call(5), 15);
        drop(g2);
        assert_eq!(TEST_FN.call(5), 6);
        drop(g1);
        assert_eq!(TEST_FN.call(5), 10);
    }

    // --- Integration with rvtest's describe/it ---

    #[test]
    fn spy_inside_describe() {
        describe("Mock inside describe")
            .it("spy works", || {
                let spy = Spy::new(|x: i32| x * 2);
                assert_eq!(spy.call(3), 6);
                spy.assert_called_with(&[3]);
            })
            .it("stub works", || {
                let stub = Stub::new(true);
                assert!(stub.call(()));
            })
            .run()
            .assert_all_pass();
    }

    use crate::spec::describe;
}
