use rvtest::capture::{is_capture_enabled, set_capture_enabled};
use rvtest::core::RunnerConfig;
use rvtest::spec::describe;

#[test]
fn capture_toggle_works_through_public_api() {
    set_capture_enabled(true);
    assert!(is_capture_enabled());
    set_capture_enabled(false);
    assert!(!is_capture_enabled());
}

#[test]
fn capture_with_spec() {
    set_capture_enabled(true);
    let suite = describe("Capture")
        .it("prints something", || {
            print!("hello from test");
        })
        .run_with_config(&RunnerConfig {
            output_capture: true,
            ..RunnerConfig::default()
        });
    set_capture_enabled(false);
    assert_eq!(suite.tests.len(), 1);
    assert!(suite.tests[0].status.is_passed());
}
