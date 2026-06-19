#![allow(clippy::assertions_on_constants)]

// Integration tests for `rvtest-macros`.
//
// These tests verify that the `#[describe]` and `#[it]` proc macros
// generate correct code by compiling and running the expanded tests.

#[allow(unused_imports)]
use rvtest_macros::{after_all, before_all, describe, it, retries, tag, timeout};
use std::sync::atomic::{AtomicU32, Ordering};

// ---------------------------------------------------------------------------
// Basic spec with tags, timeout, retries
// ---------------------------------------------------------------------------

#[describe("Macros :: Basic")]
mod basic {
    #[it("basic test passes")]
    fn basic() {
        assert_eq!(2 + 2, 4);
    }

    #[it("test with tag")]
    #[tag("smoke")]
    fn tagged() {
        assert_eq!(5 - 3, 2);
    }

    #[it("test with timeout")]
    #[timeout(std::time::Duration::from_secs(5))]
    fn timed() {
        assert_eq!(3 * 3, 9);
    }

    #[it("test with retries")]
    #[retries(2)]
    fn flaky() {
        assert_eq!(10 / 2, 5);
    }
}

// ---------------------------------------------------------------------------
// Nested describe blocks
// ---------------------------------------------------------------------------

#[describe("Macros :: Nested")]
mod nested {
    #[allow(clippy::assertions_on_constants)]
    #[it("outer test")]
    fn outer() {
        assert!(true);
    }

    #[describe("inner")]
    #[tag("nested")]
    mod inner_mod {
        #[it("inner test passes")]
        fn inner() {
            assert_eq!(1 + 1, 2);
        }
    }
}

// ---------------------------------------------------------------------------
// before_all / after_all hooks (no-op in this test)
// ---------------------------------------------------------------------------

#[describe("Macros :: Hooks")]
mod hooks {
    #[before_all(|| {})]
    #[after_all(|| {})]
    #[allow(clippy::assertions_on_constants)]
    #[it("hook test")]
    fn hook_test() {
        assert!(true);
    }
}

// ---------------------------------------------------------------------------
// Retry flaky tests (uses a static counter)
// ---------------------------------------------------------------------------

static FLAKY_COUNTER: AtomicU32 = AtomicU32::new(0);

#[describe("Macros :: Retry")]
mod retry_mod {
    #[it("succeeds on retry")]
    #[retries(3)]
    fn flaky() {
        let prev = FLAKY_COUNTER.fetch_add(1, Ordering::SeqCst);
        if prev < 2 {
            panic!("transient failure {}", prev);
        }
    }

    #[it("verifies retries ran")]
    fn verify_ran() {
        let val = FLAKY_COUNTER.load(Ordering::SeqCst);
        assert!(val >= 3, "expected >= 3, got {val}");
    }
}
