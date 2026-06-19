use std::panic::{catch_unwind, AssertUnwindSafe};

use rvtest::property::{any, check};

#[test]
fn property_passes_for_valid_properties() {
    check(
        "identity with zero",
        any::<i32>(),
        |a: &i32| *a == *a,
    );
}

#[test]
fn property_detects_falsified_properties() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        check(
            "intentionally false",
            any::<i32>(),
            |_: &i32| false,
        );
    }));
    assert!(result.is_err(), "check should panic on falsified property");
}
