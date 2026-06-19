use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use rvtest::spec::describe;

#[test]
fn execution_passes_when_all_pass() {
    describe("Math")
        .it("adds", || assert_eq!(2 + 2, 4))
        .it("subtracts", || assert_eq!(5 - 3, 2))
        .run()
        .assert_all_pass();
}

#[test]
fn execution_reports_failures() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        describe("Failing")
            .it("fails", || panic!("intentional failure"))
            .run()
            .assert_all_pass();
    }));
    assert!(result.is_err(), "assert_all_pass should panic on failure");
}

#[test]
fn execution_supports_tags_and_timeout() {
    describe("Tagged")
        .it("passing", || {})
        .tag("smoke")
        .timeout(Duration::from_secs(1))
        .run()
        .assert_all_pass();
}

#[test]
fn execution_retries_flaky_tests() {
    let counter = AtomicU32::new(0);
    describe("Flaky")
        .it("succeeds on retry", move || {
            let prev = counter.fetch_add(1, Ordering::SeqCst);
            if prev == 0 {
                panic!("first attempt fails");
            }
        })
        .retries(2)
        .run()
        .assert_all_pass();
}

#[test]
fn execution_runs_before_all_hook() {
    let ran = Arc::new(AtomicBool::new(false));
    let setup = Arc::clone(&ran);
    describe("Setup")
        .before_all(move || {
            setup.store(true, Ordering::SeqCst);
        })
        .it("hook executed", move || {
            assert!(ran.load(Ordering::SeqCst), "before_all should have run");
        })
        .run()
        .assert_all_pass();
}

#[test]
fn child_hooks_before_all_on_child() {
    let ran = Arc::new(AtomicBool::new(false));
    let setup = Arc::clone(&ran);
    let check = Arc::clone(&ran);
    describe("Parent")
        .describe("Child")
            .before_all(move || {
                setup.store(true, Ordering::SeqCst);
            })
            .it("child test", move || {
                assert!(check.load(Ordering::SeqCst), "before_all should have run");
            })
        .run()
        .assert_all_pass();
}

#[test]
fn child_hooks_after_all_on_child() {
    let ran = Arc::new(AtomicBool::new(false));
    let cleanup = Arc::clone(&ran);
    let verify = Arc::clone(&ran);
    describe("Parent")
        .describe("Child")
            .after_all(move || {
                cleanup.store(true, Ordering::SeqCst);
            })
            .it("child test", move || {})
        .run()
        .assert_all_pass();
    assert!(verify.load(Ordering::SeqCst), "after_all should have run");
}

#[test]
fn child_hooks_multiple_nesting_levels() {
    let order = Arc::new(std::sync::Mutex::new(Vec::new()));
    let o1 = Arc::clone(&order);
    let o2 = Arc::clone(&order);
    let o3 = Arc::clone(&order);
    let o_test = Arc::clone(&order);
    describe("Outer")
        .before_all(move || o1.lock().unwrap().push("outer_before"))
        .after_all(move || o2.lock().unwrap().push("outer_after"))
        .describe("Inner")
            .before_all(move || o3.lock().unwrap().push("inner_before"))
            .it("test", move || {
                o_test.lock().unwrap().push("test");
            })
        .run()
        .assert_all_pass();
    let ord = order.lock().unwrap();
    assert_eq!(ord[0], "outer_before", "outer before_all should run first");
    assert_eq!(ord[1], "inner_before", "inner before_all should run second");
    assert_eq!(ord[2], "test", "test should run after hooks");
    assert_eq!(ord[3], "outer_after", "outer after_all should run last");
}

#[test]
fn source_location_captures_caller() {
    let suite = describe("Loc")
        .it("inner", || {})
        .run();
    assert_eq!(suite.tests.len(), 1);
    let loc = suite.tests[0].location.as_ref().expect("should have location");
    assert!(loc.file.ends_with("tests/spec.rs"), "should be spec.rs, got {}", loc.file);
    assert!(loc.line > 0, "line should be positive");
}

#[test]
fn before_each_runs_before_each_test() {
    let counter = Arc::new(AtomicU32::new(0));
    let c_before = Arc::clone(&counter);
    let c_first = Arc::clone(&counter);
    let c_second = Arc::clone(&counter);
    let c_verify = Arc::clone(&counter);
    describe("Hooks")
        .before_each(move || { c_before.fetch_add(1, Ordering::SeqCst); })
        .it("first", move || {
            assert_eq!(counter.load(Ordering::SeqCst), 1, "before_each should have run");
        })
        .it("second", move || {
            assert_eq!(c_first.load(Ordering::SeqCst), 2, "before_each should have run again");
        })
        .run()
        .assert_all_pass();
    assert_eq!(c_second.load(Ordering::SeqCst), 2, "before_each ran exactly twice");
    let _ = c_verify;
}

#[test]
fn after_each_runs_after_each_test() {
    let ran = Arc::new(AtomicBool::new(false));
    let check = Arc::clone(&ran);
    let verify = Arc::clone(&ran);
    describe("Hooks")
        .after_each(move || {
            check.store(true, Ordering::SeqCst);
        })
        .it("test", move || {
            assert!(!verify.load(Ordering::SeqCst), "after_each should NOT have run yet");
        })
        .run()
        .assert_all_pass();
    assert!(ran.load(Ordering::SeqCst), "after_each should have run");
}

#[test]
fn after_each_runs_even_if_test_panics() {
    let ran = Arc::new(AtomicBool::new(false));
    let check = Arc::clone(&ran);
    let verify = Arc::clone(&ran);
    let result = catch_unwind(AssertUnwindSafe(|| {
        describe("Flaky")
            .after_each(move || {
                check.store(true, Ordering::SeqCst);
            })
            .it("will fail", || {
                panic!("intentional failure");
            })
            .run()
            .assert_all_pass();
    }));
    assert!(result.is_err(), "should have failed");
    assert!(verify.load(Ordering::SeqCst), "after_each should run even after panic");
}

#[test]
fn before_each_inherits_from_parent() {
    let order = Arc::new(std::sync::Mutex::new(Vec::new()));
    let o1 = Arc::clone(&order);
    let o_test = Arc::clone(&order);
    describe("Parent")
        .before_each(move || o1.lock().unwrap().push("parent"))
        .describe("Child")
            .it("test", move || {
                o_test.lock().unwrap().push("test");
            })
        .run()
        .assert_all_pass();
    assert_eq!(order.lock().unwrap().len(), 2);
    assert_eq!(order.lock().unwrap()[0], "parent");
}

#[test]
fn hook_ordering_outermost_before_each_innermost_after_each() {
    let order = Arc::new(std::sync::Mutex::new(Vec::new()));
    let o1 = Arc::clone(&order);
    let o2 = Arc::clone(&order);
    let o3 = Arc::clone(&order);
    let o4 = Arc::clone(&order);
    let o_test = Arc::clone(&order);
    describe("Outer")
        .before_each(move || o1.lock().unwrap().push("outer_before"))
        .after_each(move || o3.lock().unwrap().push("outer_after"))
        .describe("Inner")
            .before_each(move || o2.lock().unwrap().push("inner_before"))
            .after_each(move || o4.lock().unwrap().push("inner_after"))
            .it("test", move || {
                o_test.lock().unwrap().push("test");
            })
        .run()
        .assert_all_pass();
    let ord = order.lock().unwrap();
    assert_eq!(ord[0], "outer_before", "outer before_each first");
    assert_eq!(ord[1], "inner_before", "inner before_each second");
    assert_eq!(ord[2], "test", "test runs after all before_each");
    assert_eq!(ord[3], "inner_after", "inner after_each first (innermost)");
    assert_eq!(ord[4], "outer_after", "outer after_each last (outermost)");
}
