use rvtest::snapshot::{assert_snapshot, set_snapshot_dir, set_update_all};

#[test]
fn snapshot_create_and_match() {
    set_update_all(false);

    let tmp = std::env::temp_dir().join("rvtest_snap_test");
    let _ = std::fs::remove_dir_all(&tmp);
    set_snapshot_dir(&tmp);
    set_update_all(true);

    assert_snapshot("hello", &"Hello, world!");

    set_update_all(false);

    assert_snapshot("hello", &"Hello, world!");

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn snapshot_mismatch_detected() {
    set_update_all(false);

    let tmp = std::env::temp_dir().join("rvtest_snap_mismatch");
    let _ = std::fs::remove_dir_all(&tmp);
    set_snapshot_dir(&tmp);
    set_update_all(true);

    assert_snapshot("mismatch_test", &"original value");

    set_update_all(false);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_snapshot("mismatch_test", &"different value");
    }));
    assert!(result.is_err(), "snapshot mismatch should panic");

    let _ = std::fs::remove_dir_all(&tmp);
}
