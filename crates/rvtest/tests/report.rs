use rvtest::core::TestRun;
use rvtest::report::{CompactReporter, GithubReporter, JsonReporter, PrettyReporter, TapReporter, TestReporter};

#[test]
fn pretty_reporter_shows_summary() {
    let report = PrettyReporter::new().colour(false).report(&TestRun::new());
    assert!(report.contains("Tests"), "should have Tests line");
    assert!(report.contains("0 passed"), "should show pass count");
    assert!(report.contains("Time"), "should have Time line");
}

#[test]
fn tap_reporter_outputs_correct_header() {
    let report = TapReporter.report(&TestRun::new());
    assert!(report.starts_with("1..0"));
}

#[test]
fn compact_reporter_shows_counts() {
    let report = CompactReporter.report(&TestRun::new());
    assert!(report.contains("Results:"), "should have results line: {report:?}");
    assert!(report.contains("0/0"), "should show zero counts");
}

#[test]
fn json_reporter_is_valid() {
    let report = JsonReporter.report(&TestRun::new());
    assert!(report.contains(r#""success":true"#));
    assert!(report.contains(r#""suites":["#));
}

#[test]
fn github_reporter_shows_zero_failures() {
    let report = GithubReporter.report(&TestRun::new());
    assert!(report.contains("0/0 passed"));
}
