use std::panic::{catch_unwind, AssertUnwindSafe};

#[test]
fn assert_eq_passes_on_equal_values() {
    rvtest::assert_eq!(42, 42);
    rvtest::assert_eq!("hello", "hello");
    rvtest::assert_eq!(vec![1, 2, 3], vec![1, 2, 3]);
}

#[test]
fn assert_eq_panics_on_mismatch() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        rvtest::assert_eq!(1, 2);
    }));
    assert!(result.is_err(), "assert_eq should panic on mismatch");
}

#[test]
fn assert_eq_with_custom_message() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        rvtest::assert_eq!(1, 2, "custom: expected 1 == 2");
    }));
    assert!(result.is_err());
    let msg = result.unwrap_err().downcast_ref::<String>().unwrap().to_string();
    assert!(msg.contains("custom:"), "message should contain custom text: {msg}");
}

#[test]
fn assert_eq_multiline_diff_message() {
    let r = catch_unwind(AssertUnwindSafe(|| {
        rvtest::assert_eq!(
            vec![vec![1, 2], vec![3, 4]],
            vec![vec![1, 2], vec![9, 9]]
        );
    }));
    assert!(r.is_err(), "should fail on multiline mismatch");
}

#[test]
fn assert_eq_with_custom_message_on_multiline_types() {
    let r = catch_unwind(AssertUnwindSafe(|| {
        rvtest::assert_eq!(
            vec![1, 2],
            vec![1, 99],
            "lists don't match"
        );
    }));
    assert!(r.is_err());
    let msg = r.unwrap_err().downcast_ref::<String>().unwrap().to_string();
    assert!(msg.contains("lists don't match"));
}

#[test]
fn assert_ok_returns_inner_value() {
    let v = rvtest::assert_ok!(Ok::<_, &str>(42));
    assert_eq!(v, 42);
}

#[test]
fn assert_ok_panics_on_err() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let val: Result<i32, &str> = Err("fail");
        let _v = rvtest::assert_ok!(val);
    }));
    assert!(result.is_err(), "assert_ok should panic on Err");
}

#[test]
fn assert_ok_with_custom_message() {
    let val: Result<i32, &str> = Err("fail");
    let result = catch_unwind(AssertUnwindSafe(|| {
        let _v = rvtest::assert_ok!(val, "expected Ok");
    }));
    assert!(result.is_err());
}

#[test]
fn assert_ok_with_complex_error_type() {
    let v = rvtest::assert_ok!(Ok::<_, Box<dyn std::error::Error>>(42));
    assert_eq!(v, 42);
}

#[test]
fn assert_err_returns_error_value() {
    let e = rvtest::assert_err!(Err::<i32, _>("error msg"));
    assert_eq!(e, "error msg");
}

#[test]
fn assert_err_panics_on_ok() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let val: Result<i32, &str> = Ok(42);
        let _e = rvtest::assert_err!(val);
    }));
    assert!(result.is_err(), "assert_err should panic on Ok");
}

#[test]
fn assert_err_with_complex_ok_type() {
    let r = catch_unwind(AssertUnwindSafe(|| {
        let _e = rvtest::assert_err!(Ok::<i32, &str>(42));
    }));
    assert!(r.is_err());
}

#[test]
fn assert_matches_passes() {
    let a = Some(42);
    let b: Result<i32, ()> = Ok(1);
    rvtest::assert_matches!(a, Some(_));
    rvtest::assert_matches!(b, Ok(_));
}

#[test]
fn assert_matches_on_option_none() {
    let n: Option<i32> = None;
    rvtest::assert_matches!(n, None);
}

#[test]
fn assert_matches_panics_on_mismatch() {
    let n: Option<i32> = Some(42);
    let result = catch_unwind(AssertUnwindSafe(|| {
        rvtest::assert_matches!(n, None);
    }));
    assert!(result.is_err(), "assert_matches should panic on mismatch");
}

#[test]
fn assert_delta_passes_within_epsilon() {
    rvtest::assert_delta!(1.0_f64, 1.001_f64, 0.01_f64);
    rvtest::assert_delta!(100.0_f64, 100.0001_f64, 0.001_f64);
}

#[test]
fn assert_delta_panics_outside_epsilon() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        rvtest::assert_delta!(1.0_f64, 2.0_f64, 0.1_f64);
    }));
    assert!(result.is_err(), "assert_delta should panic outside epsilon");
}

#[test]
fn assert_delta_with_negative_values() {
    rvtest::assert_delta!(-10.0_f64, -10.5_f64, 1.0_f64);
}

#[test]
fn assert_delta_with_message_on_excess() {
    let r = catch_unwind(AssertUnwindSafe(|| {
        rvtest::assert_delta!(1.0_f64, 100.0_f64, 1.0_f64, "too far apart");
    }));
    assert!(r.is_err());
}
