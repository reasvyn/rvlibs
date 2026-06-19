//! Test result reporters and output formatting.
//!
//! All reporters implement the [`TestReporter`] trait:
//!
//! - [`PrettyReporter`] — colourised human-readable output (default)
//! - [`TapReporter`] — Test Anything Protocol
//! - [`JunitReporter`] — JUnit XML
//! - [`JsonReporter`] — structured JSON
//! - [`CompactReporter`] — single-line-per-test
//! - [`GithubReporter`] — GitHub Actions `::error` annotations

use crate::core::TestRun;

mod agent;
mod compact;
mod github;
mod html;
mod json;
mod junit;
mod nextest;
mod pretty;
mod tap;

pub use agent::AgentReporter;
pub use compact::CompactReporter;
pub use github::GithubReporter;
pub use html::HtmlReporter;
pub use json::JsonReporter;
pub use junit::JunitReporter;
pub use nextest::NextestReporter;
pub use pretty::PrettyReporter;
pub use tap::TapReporter;

// ---------------------------------------------------------------------------
// Reporter trait
// ---------------------------------------------------------------------------

/// Renders a [`TestRun`] into a human- or machine-readable string.
pub trait TestReporter {
    /// Format the entire test run into a string.
    fn report(&self, run: &TestRun) -> String;
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Format a `Duration` as a human-readable string (e.g. `1.23s` or `456.7ms`).
pub fn format_duration(d: std::time::Duration) -> String {
    let secs = d.as_secs_f64();
    if secs >= 1.0 {
        format!("{secs:.2}s")
    } else {
        format!("{:.1}ms", secs * 1000.0)
    }
}

fn coloured(s: &str, code: &str, enabled: bool) -> String {
    if enabled {
        format!("\x1b[{code}m{s}\x1b[0m")
    } else {
        s.to_owned()
    }
}

fn dim(s: &str, enabled: bool) -> String {
    coloured(s, "2", enabled)
}

fn coloured_count(n: usize, label: &str, colour_code: &str, enabled: bool) -> String {
    format!("{} {}", coloured(&n.to_string(), colour_code, enabled), label)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use super::pretty::*;
    use super::junit::escape_xml;
    use super::json::escape_json;
    use super::github::escape_github;
    use crate::core::{SourceLocation, TestCase, TestKind, TestRun, TestStatus, TestSuite};
    use std::time::{Duration, SystemTime};

    fn mixed_run() -> TestRun {
        let mut suite = TestSuite::new("Math");
        suite.kind = TestKind::Unit;
        suite.source_path = "src/lib.rs".into();
        suite.duration = Duration::from_millis(50);
        suite.tests = vec![
            TestCase {
                name: "Math :: add".into(), suite: Some("Math".into()), tags: vec![],
                status: TestStatus::Passed, duration: Duration::from_millis(5),
                assertions: 0, location: None,             parameters: vec![], captured_output: None,
                bench_stats: None, bench_threshold: None,
            },
            TestCase {
                name: "Math :: sub".into(), suite: Some("Math".into()), tags: vec![],
                status: TestStatus::Failed { reason: "assertion failed: 1 + 1 != 3".into(), location: None },
                duration: Duration::from_millis(3), assertions: 0, location: None,             parameters: vec![], captured_output: None,
                bench_stats: None, bench_threshold: None,
            },
            TestCase {
                name: "Math :: slow".into(), suite: Some("Math".into()), tags: vec![],
                status: TestStatus::Skipped { reason: Some("not implemented".into()) },
                duration: Duration::ZERO, assertions: 0, location: None,             parameters: vec![], captured_output: None,
                bench_stats: None, bench_threshold: None,
            },
        ];
        let mut doc_suite = TestSuite::new("doc-tests (rvtest)");
        doc_suite.kind = TestKind::Doc;
        doc_suite.source_path = "rvtest".into();
        doc_suite.tests = vec![
            TestCase {
                name: "rvtest - foo".into(), suite: None, tags: vec![],
                status: TestStatus::Passed, duration: Duration::from_millis(1),
                assertions: 0, location: None,             parameters: vec![], captured_output: None,
                bench_stats: None, bench_threshold: None,
            },
        ];
        TestRun {
            suites: vec![suite, doc_suite],
            start_time: SystemTime::now(),
            end_time: SystemTime::now(),
            duration: Duration::from_millis(55),
        }
    }

    fn empty_run() -> TestRun {
        TestRun::new()
    }

    // --- PrettyReporter ---
    #[test]
    fn test_pretty_shows_summary() {
        let r = PrettyReporter::new().colour(false).report(&mixed_run());
        assert!(r.contains("Tests"));
        assert!(r.contains("2 passed"));
        assert!(r.contains("1 failed"));
    }

    #[test]
    fn test_pretty_shows_names() {
        let r = PrettyReporter::new().colour(false).report(&mixed_run());
        assert!(r.contains("Math > add"));
        assert!(r.contains("Math > sub"));
        assert!(r.contains("Math > slow"));
    }

    #[test]
    fn test_pretty_shows_failure_reason() {
        let r = PrettyReporter::new().colour(false).report(&mixed_run());
        assert!(r.contains("assertion failed"));
    }

    #[test]
    fn test_pretty_shows_time() {
        let r = PrettyReporter::new().colour(false).report(&mixed_run());
        assert!(r.contains("Time"));
    }

    #[test]
    fn test_pretty_empty_shows_zeros() {
        let r = PrettyReporter::new().colour(false).report(&empty_run());
        assert!(r.contains("0 passed"));
    }

    #[test]
    fn test_pretty_shows_skipped_reason() {
        let r = PrettyReporter::new().colour(false).report(&mixed_run());
        assert!(r.contains("not implemented"));
    }

    // --- TapReporter ---
    #[test]
    fn test_tap_header() {
        let r = TapReporter.report(&mixed_run());
        assert!(r.starts_with("1..4"));
    }

    #[test]
    fn test_tap_ok_for_passed() {
        let r = TapReporter.report(&mixed_run());
        assert!(r.contains("ok 1"));
    }

    #[test]
    fn test_tap_not_ok_for_failed() {
        let r = TapReporter.report(&mixed_run());
        assert!(r.contains("not ok 2"));
    }

    #[test]
    fn test_tap_empty() {
        let r = TapReporter.report(&empty_run());
        assert_eq!(r.trim(), "1..0");
    }

    // --- JunitReporter ---
    #[test]
    fn test_junit_xml_declaration() {
        let r = JunitReporter::new().report(&mixed_run());
        assert!(r.starts_with("<?xml"));
    }

    #[test]
    fn test_junit_counts() {
        let r = JunitReporter::new().report(&mixed_run());
        assert!(r.contains("tests=\"4\""));
        assert!(r.contains("failures=\"1\""));
        assert!(r.contains("skipped=\"1\""));
    }

    #[test]
    fn test_junit_failure_message() {
        let r = JunitReporter::new().report(&mixed_run());
        assert!(r.contains("failure"));
        assert!(r.contains("assertion failed"));
    }

    #[test]
    fn test_junit_empty() {
        let r = JunitReporter::new().report(&empty_run());
        assert!(r.contains("tests=\"0\""));
    }

    #[test]
    fn test_junit_custom_suite_name() {
        let r = JunitReporter::new().suite_name("custom").report(&empty_run());
        assert!(r.contains("name=\"custom\""));
    }

    // --- JsonReporter ---
    #[test]
    fn test_json_structure() {
        let r = JsonReporter.report(&mixed_run());
        assert!(r.starts_with("{"));
        assert!(r.ends_with("}"));
        assert!(r.contains(r#""success":false"#));
        assert!(r.contains(r#""total":4"#));
    }

    #[test]
    fn test_json_contains_names() {
        let r = JsonReporter.report(&mixed_run());
        assert!(r.contains("Math :: add"));
    }

    #[test]
    fn test_json_empty() {
        let r = JsonReporter.report(&empty_run());
        assert!(r.contains(r#""success":true"#));
        assert!(r.contains(r#""total":0"#));
    }

    // --- CompactReporter ---
    #[test]
    fn test_compact_status_and_name() {
        let r = CompactReporter.report(&mixed_run());
        assert!(r.contains("PASS"));
        assert!(r.contains("FAIL"));
        assert!(r.contains("SKIP"));
        assert!(r.contains("Math :: add"));
    }

    #[test]
    fn test_compact_results() {
        let r = CompactReporter.report(&mixed_run());
        assert!(r.contains("2/4 passed"));
    }

    #[test]
    fn test_compact_empty() {
        let r = CompactReporter.report(&empty_run());
        assert!(r.contains("0/0 passed"));
    }

    // --- GithubReporter ---
    #[test]
    fn test_github_error_annotation() {
        let r = GithubReporter.report(&mixed_run());
        assert!(r.contains("::error"));
        assert!(r.contains("assertion failed"));
    }

    #[test]
    fn test_github_summary() {
        let r = GithubReporter.report(&mixed_run());
        assert!(r.contains("rvtest:"));
        assert!(r.contains("2/4 passed"));
    }

    #[test]
    fn test_github_empty_no_errors() {
        let r = GithubReporter.report(&empty_run());
        assert!(r.contains("0/0 passed"));
        assert!(!r.contains("::error"));
    }

    // --- Helpers ---
    #[test]
    fn test_format_duration_seconds() {
        let s = format_duration(Duration::from_secs_f64(2.5));
        assert_eq!(s, "2.50s");
    }

    #[test]
    fn test_format_duration_millis() {
        let s = format_duration(Duration::from_millis(500));
        assert_eq!(s, "500.0ms");
    }

    #[test]
    fn test_format_duration_zero() {
        let s = format_duration(Duration::ZERO);
        assert_eq!(s, "0.0ms");
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("a & b < c > d \" e ' f"), "a &amp; b &lt; c &gt; d &quot; e &apos; f");
    }

    #[test]
    fn test_escape_json_basic() {
        assert_eq!(escape_json(r#"a"b\c"#), r#"a\"b\\c"#);
    }

    #[test]
    fn test_escape_json_newline() {
        assert_eq!(escape_json("a\nb"), "a\\nb");
    }

    #[test]
    fn test_escape_github_percent() {
        assert_eq!(escape_github("100%"), "100%25");
    }

    #[test]
    fn test_escape_github_newline() {
        assert_eq!(escape_github("a\nb"), "a%0Ab");
    }

    fn timed_out_run() -> TestRun {
        let mut suite = TestSuite::new("Timeout");
        suite.kind = TestKind::Unit;
        suite.source_path = "src/lib.rs".into();
        suite.duration = Duration::from_secs(5);
        suite.tests = vec![
            TestCase {
                name: "Timeout :: slow".into(), suite: Some("Timeout".into()), tags: vec![],
                status: TestStatus::TimedOut { duration: Duration::from_secs(5), location: Some(SourceLocation { file: "src/lib.rs".into(), line: 42, column: Some(7) }) },
                duration: Duration::from_secs(5), assertions: 0, location: Some(SourceLocation { file: "src/lib.rs".into(), line: 42, column: Some(7) }),             parameters: vec![], captured_output: None,
                bench_stats: None, bench_threshold: None,
            },
        ];
        TestRun { suites: vec![suite], start_time: SystemTime::now(), end_time: SystemTime::now(), duration: Duration::from_secs(5) }
    }

    #[test]
    fn test_pretty_timed_out_shows_location() {
        let r = PrettyReporter::new().colour(false).report(&timed_out_run());
        assert!(r.contains("timed out"));
        assert!(r.contains("src/lib.rs"));
    }

    #[test]
    fn test_junit_timed_out() {
        let r = JunitReporter::new().report(&timed_out_run());
        assert!(r.contains("TimeoutError"));
        assert!(r.contains("timed out"));
    }

    #[test]
    fn test_json_timed_out() {
        let r = JsonReporter.report(&timed_out_run());
        assert!(r.contains("timed_out"));
    }

    #[test]
    fn test_github_timed_out() {
        let r = GithubReporter.report(&timed_out_run());
        assert!(r.contains("::error"));
        assert!(r.contains("timed out"));
    }

    #[test]
    fn test_tap_timed_out() {
        let r = TapReporter.report(&timed_out_run());
        assert!(r.contains("TIMEOUT"));
    }

    #[test]
    fn test_compact_timed_out() {
        let r = CompactReporter.report(&timed_out_run());
        assert!(r.contains("TIMEOUT"));
    }

    // --- Helper function tests ---

    #[test]
    fn test_section_label_unit() {
        let mut suite = TestSuite::new("test");
        suite.kind = TestKind::Unit;
        suite.source_path = "src/lib.rs".into();
        let label = section_label(&suite, false);
        assert!(label.contains("unit tests"));
        assert!(label.contains("src/lib.rs"));
    }

    #[test]
    fn test_section_label_integration() {
        let mut suite = TestSuite::new("test");
        suite.kind = TestKind::Integration;
        suite.source_path = "tests/integration.rs".into();
        let label = section_label(&suite, false);
        assert!(label.contains("integration"));
        assert!(label.contains("tests/integration.rs"));
    }

    #[test]
    fn test_section_label_doc() {
        let mut suite = TestSuite::new("test");
        suite.kind = TestKind::Doc;
        suite.source_path = "rvtest".into();
        let label = section_label(&suite, false);
        assert!(label.contains("doc-tests"));
    }

    #[test]
    fn test_coloured_on() {
        let s = coloured("hello", "31", true);
        assert_eq!(s, "\x1b[31mhello\x1b[0m");
    }

    #[test]
    fn test_coloured_off() {
        let s = coloured("hello", "31", false);
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_dim_colour() {
        let s = dim("test", true);
        assert_eq!(s, "\x1b[2mtest\x1b[0m");
    }

    #[test]
    fn test_dim_no_colour() {
        let s = dim("test", false);
        assert_eq!(s, "test");
    }

    #[test]
    fn test_coloured_count() {
        let s = coloured_count(42, "passed", "32", true);
        assert_eq!(s, "\x1b[32m42\x1b[0m passed");
    }

    #[test]
    fn test_coloured_count_no_colour() {
        let s = coloured_count(7, "failed", "31", false);
        assert_eq!(s, "7 failed");
    }

    #[test]
    fn test_doc_icon() {
        let icon = doc_icon(false);
        assert_eq!(icon, "?");
    }

    #[test]
    fn test_build_summary_no_failures() {
        let s = build_summary(5, 0, 0, false);
        assert_eq!(s, "5 passed");
    }

    #[test]
    fn test_build_summary_with_failures() {
        let s = build_summary(3, 1, 0, false);
        assert!(s.contains("3 passed"));
        assert!(s.contains("1 failed"));
    }

    #[test]
    fn test_build_summary_with_docs() {
        let s = build_summary(5, 0, 2, false);
        assert!(s.contains("5 passed"));
        assert!(s.contains("2 doc-tests"));
    }

    #[test]
    fn test_build_summary_colour() {
        let s = build_summary(1, 1, 0, true);
        assert!(s.contains("\x1b[32m1\x1b[0m passed"));
        assert!(s.contains("\x1b[31m1\x1b[0m failed"));
    }

    #[test]
    fn test_status_icon_all_variants() {
        assert_eq!(status_icon(&TestStatus::Passed, false), "✓");
        assert_eq!(status_icon(&TestStatus::Failed { reason: "".into(), location: None }, false), "✗");
        assert_eq!(status_icon(&TestStatus::Skipped { reason: None }, false), "–");
        assert_eq!(status_icon(&TestStatus::TimedOut { duration: Duration::ZERO, location: None }, false), "⊗");
    }

    #[test]
    fn test_status_icon_colour() {
        let passed = status_icon(&TestStatus::Passed, true);
        assert!(passed.contains("\x1b[32m"));
        let failed = status_icon(&TestStatus::Failed { reason: "".into(), location: None }, true);
        assert!(failed.contains("\x1b[31m"));
    }

    #[test]
    fn test_format_location_with_column() {
        let loc = SourceLocation { file: "src/lib.rs".into(), line: 42, column: Some(7) };
        let s = format_location(&loc, false);
        assert_eq!(s, "src/lib.rs:42:7");
    }

    #[test]
    fn test_format_location_without_column() {
        let loc = SourceLocation { file: "src/main.rs".into(), line: 10, column: None };
        let s = format_location(&loc, false);
        assert_eq!(s, "src/main.rs:10");
    }

    #[test]
    fn test_format_location_with_colour() {
        let loc = SourceLocation { file: "src/lib.rs".into(), line: 1, column: Some(1) };
        let s = format_location(&loc, true);
        assert!(s.contains("\x1b[36m"));
    }
}
