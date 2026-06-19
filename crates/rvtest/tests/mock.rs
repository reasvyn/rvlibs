use std::panic::{catch_unwind, AssertUnwindSafe};

#[test]
fn spy_records_calls() {
    let spy = rvtest::mock::Spy::new(|x: i32| x * 2);
    assert_eq!(spy.call(5), 10);
    assert_eq!(spy.call_count(), 1);
    spy.assert_called_with(&[5]);
}

#[test]
fn spy_records_multiple_calls_in_order() {
    let spy = rvtest::mock::Spy::new(|x: i32| x + 1);
    spy.call(1);
    spy.call(2);
    spy.call(3);
    spy.assert_called_with(&[1, 2, 3]);
}

#[test]
fn spy_can_be_reset() {
    let spy = rvtest::mock::Spy::new(|x: i32| x);
    spy.call(1);
    spy.call(2);
    assert_eq!(spy.call_count(), 2);
    spy.reset();
    assert_eq!(spy.call_count(), 0);
}

#[test]
fn spy_assert_called_works_when_called() {
    let spy = rvtest::mock::Spy::new(|x: i32| x);
    spy.call(1);
    spy.assert_called();
}

#[test]
fn spy_assert_called_panics_when_never_called() {
    let spy = rvtest::mock::Spy::new(|x: i32| x);
    let r = catch_unwind(AssertUnwindSafe(|| {
        spy.assert_called();
    }));
    assert!(r.is_err());
}

#[test]
fn spy_assert_called_with_panics_on_mismatch() {
    let spy = rvtest::mock::Spy::new(|x: i32| x);
    spy.call(1);
    let r = catch_unwind(AssertUnwindSafe(|| {
        spy.assert_called_with(&[99]);
    }));
    assert!(r.is_err());
}

#[test]
fn stub_returns_fixed_value() {
    let stub = rvtest::mock::Stub::new(42);
    assert_eq!(stub.call("anything"), 42);
}
