use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use rvtest::core::RunnerConfig;
use rvtest::spec::describe;

#[test]
fn timeout_test_passes_quickly() {
    describe("Quick")
        .it("fast", || {})
        .timeout(Duration::from_secs(1))
        .run()
        .assert_all_pass();
}

#[test]
fn tag_exclude_filters_correctly() {
    let suite = describe("Filtered")
        .tag("slow")
        .it("should be excluded", || {})
        .run_with_config(&RunnerConfig {
            exclude_tags: vec!["slow".into()],
            ..RunnerConfig::default()
        });
    assert_eq!(suite.tests.len(), 0, "slow test should be excluded");
}

#[test]
fn tag_include_filters_correctly() {
    let suite = describe("Filtered")
        .tag("smoke")
        .it("should be included", || {})
        .run_with_config(&RunnerConfig {
            include_tags: vec!["smoke".into()],
            ..RunnerConfig::default()
        });
    assert_eq!(suite.tests.len(), 1, "smoke test should be included");
}

#[test]
fn name_filter_excludes_correctly() {
    let suite = describe("Name filter")
        .it("keep_me", || {})
        .it("exclude_me", || {})
        .run_with_config(&RunnerConfig {
            filter: Some("keep".into()),
            ..RunnerConfig::default()
        });
    assert_eq!(suite.tests.len(), 1);
    assert_eq!(suite.tests[0].name, "Name filter :: keep_me");
}

#[test]
fn auto_retry_retries_failed_tests() {
    let counter = AtomicU32::new(0);
    let result = catch_unwind(AssertUnwindSafe(|| {
        describe("AutoRetry")
            .it("succeeds on retry", move || {
                let prev = counter.fetch_add(1, Ordering::SeqCst);
                if prev == 0 {
                    panic!("first attempt fails");
                }
            })
            .run_with_config(&RunnerConfig {
                auto_retry: true,
                ..RunnerConfig::default()
            })
            .assert_all_pass();
    }));
    assert!(result.is_ok(), "auto_retry should retry and pass");
}
