//! Core types and configuration shared across all rvtest modules.
//!
//! This module defines the foundational data structures:
//!
//! - [`TestStatus`] — outcome of a single test (Passed, Failed, Skipped, TimedOut)
//! - [`TestCase`] — a single test with metadata and result
//! - [`TestSuite`] — a collection of related test cases
//! - [`TestRun`] — aggregate results from one or more suites
//! - [`TestKind`] — Unit, Integration, or Doc tests
//! - [`RunnerConfig`] — global configuration for a test run
//! - [`ReportFormat`] / [`CoverageFormat`] — output format enums
//! - [`ColorChoice`] — colour output preference
//! - [`CoverageReport`] — aggregated coverage metrics
//! - [`SourceLocation`] — file:line:column tracking
//! - [`RunDiff`] — comparison results between two test runs
//!
//! # Submodules
//!
//! - [`types`] — Core domain types (TestStatus, TestCase, TestSuite, TestRun, RunnerConfig, etc.)
//! - [`cache`] — Serialization, disk persistence, and diff computation (CachedRun, RunDiff)

mod types;
mod cache;
pub mod gap;

pub use types::*;
pub use cache::*;
pub use gap::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::time::SystemTime;

mod test_status {
    use super::*;

    #[test]
    fn passed_is_passed_true() {
        assert!(TestStatus::Passed.is_passed());
    }

    #[test]
    fn passed_is_failed_false() {
        assert!(!TestStatus::Passed.is_failed());
    }

    #[test]
    fn passed_is_skipped_false() {
        assert!(!TestStatus::Passed.is_skipped());
    }

    #[test]
    fn failed_is_failed_true() {
        let s = TestStatus::Failed { reason: "x".into(), location: None };
        assert!(s.is_failed());
    }

    #[test]
    fn failed_is_passed_false() {
        let s = TestStatus::Failed { reason: "x".into(), location: None };
        assert!(!s.is_passed());
    }

    #[test]
    fn skipped_is_skipped_true() {
        let s = TestStatus::Skipped { reason: None };
        assert!(s.is_skipped());
    }

    #[test]
    fn timed_out_is_failed() {
        let s = TestStatus::TimedOut { duration: Duration::from_secs(1), location: None };
        assert!(s.is_failed());
    }

    #[test]
    fn display_passed() {
        assert_eq!(format!("{}", TestStatus::Passed), "PASSED");
    }

    #[test]
    fn display_failed() {
        let s = TestStatus::Failed { reason: "boom".into(), location: None };
        assert_eq!(format!("{}", s), "FAILED: boom");
    }

    #[test]
    fn display_skipped_no_reason() {
        let s = TestStatus::Skipped { reason: None };
        assert_eq!(format!("{}", s), "SKIPPED");
    }

    #[test]
    fn display_skipped_with_reason() {
        let s = TestStatus::Skipped { reason: Some("slow".into()) };
        assert_eq!(format!("{}", s), "SKIPPED: slow");
    }

    #[test]
    fn display_timed_out() {
        let s = TestStatus::TimedOut { duration: Duration::from_secs(5), location: None };
        let text = format!("{}", s);
        assert!(text.contains("TIMED OUT"));
        assert!(text.contains("5s"));
    }
}

mod source_location {
    use super::*;

    #[test]
    fn display_with_column() {
        let loc = SourceLocation { file: "src/lib.rs".into(), line: 42, column: Some(7) };
        assert_eq!(format!("{}", loc), "src/lib.rs:42:7");
    }

    #[test]
    fn display_without_column() {
        let loc = SourceLocation { file: "src/lib.rs".into(), line: 42, column: None };
        assert_eq!(format!("{}", loc), "src/lib.rs:42");
    }
}

mod test_case {
    use super::*;

    #[test]
    fn new_creates_passed() {
        let tc = TestCase::new("my test");
        assert_eq!(tc.name, "my test");
        assert!(tc.status.is_passed());
        assert_eq!(tc.duration, Duration::ZERO);
    }

    #[test]
    fn new_has_no_suite() {
        let tc = TestCase::new("x");
        assert!(tc.suite.is_none());
    }

    #[test]
    fn new_empty_tags() {
        let tc = TestCase::new("x");
        assert!(tc.tags.is_empty());
    }
}

mod test_suite {
    use super::*;

    fn sample_suite() -> TestSuite {
        let mut suite = TestSuite::new("Math");
        suite.tests.push(TestCase {
            name: "add".into(), suite: Some("Math".into()), tags: vec![],
            status: TestStatus::Passed, duration: Duration::from_millis(5),
            assertions: 0, location: None, parameters: vec![], captured_output: None,
            bench_stats: None, bench_threshold: None,
        });
        suite.tests.push(TestCase {
            name: "sub".into(), suite: Some("Math".into()), tags: vec![],
            status: TestStatus::Failed { reason: "expected 2 got 3".into(), location: None },
            duration: Duration::from_millis(3), assertions: 0, location: None, parameters: vec![],
            captured_output: None,
            bench_stats: None, bench_threshold: None,
        });
        suite.tests.push(TestCase {
            name: "skip".into(), suite: Some("Math".into()), tags: vec![],
            status: TestStatus::Skipped { reason: None },
            duration: Duration::ZERO, assertions: 0, location: None, parameters: vec![],
            captured_output: None,
            bench_stats: None, bench_threshold: None,
        });
        suite
    }

    #[test]
    fn len_counts_tests() {
        assert_eq!(sample_suite().len(), 3);
    }

    #[test]
    fn passed_returns_only_passed() {
        assert_eq!(sample_suite().passed().count(), 1);
    }

    #[test]
    fn failed_returns_only_failed() {
        assert_eq!(sample_suite().failed().count(), 1);
    }

    #[test]
    fn skipped_returns_only_skipped() {
        assert_eq!(sample_suite().skipped().count(), 1);
    }

    #[test]
    fn success_false_when_failures() {
        assert!(!sample_suite().success());
    }

    #[test]
    fn success_true_when_all_pass() {
        let mut suite = TestSuite::new("AllGood");
        suite.tests.push(TestCase::new("t1"));
        assert!(suite.success());
    }

    #[test]
    fn empty_suite_is_empty() {
        let suite = TestSuite::new("Empty");
        assert!(suite.is_empty());
    }

    #[test]
    fn is_doc_false_by_default() {
        let suite = TestSuite::new("x");
        assert!(!suite.is_doc());
    }

    #[test]
    fn is_doc_true_when_kind_doc() {
        let mut suite = TestSuite::new("x");
        suite.kind = TestKind::Doc;
        assert!(suite.is_doc());
    }
}

mod test_run {
    use super::*;

    fn sample_run() -> TestRun {
        let mut suite = TestSuite::new("A");
        suite.tests.push(TestCase { name: "t1".into(), suite: None, tags: vec![],
            status: TestStatus::Passed, duration: Duration::from_millis(1),
            assertions: 0, location: None, parameters: vec![], captured_output: None,
            bench_stats: None, bench_threshold: None });
        suite.tests.push(TestCase { name: "t2".into(), suite: None, tags: vec![],
            status: TestStatus::Failed { reason: "fail".into(), location: None },
            duration: Duration::from_millis(2), assertions: 0, location: None, parameters: vec![], captured_output: None,
            bench_stats: None, bench_threshold: None });
        suite.tests.push(TestCase { name: "t3".into(), suite: None, tags: vec![],
            status: TestStatus::Skipped { reason: None },
            duration: Duration::ZERO, assertions: 0, location: None, parameters: vec![], captured_output: None,
            bench_stats: None, bench_threshold: None });
        TestRun { suites: vec![suite], start_time: SystemTime::now(),
            end_time: SystemTime::now(), duration: Duration::from_millis(10) }
    }

    #[test]
    fn total_counts_all() {
        assert_eq!(sample_run().total(), 3);
    }

    #[test]
    fn total_passed() {
        assert_eq!(sample_run().total_passed(), 1);
    }

    #[test]
    fn total_failed() {
        assert_eq!(sample_run().total_failed(), 1);
    }

    #[test]
    fn total_skipped() {
        assert_eq!(sample_run().total_skipped(), 1);
    }

    #[test]
    fn success_false_with_failures() {
        assert!(!sample_run().success());
    }

    #[test]
    fn success_true_all_pass() {
        let run = TestRun::new();
        assert!(run.success());
    }

    #[test]
    fn slowest_returns_n_longest() {
        let run = sample_run();
        let slow = run.slowest(2);
        assert_eq!(slow.len(), 2);
        assert_eq!(slow[0].name, "t2");
    }

    #[test]
    fn slowest_returns_all_when_n_larger() {
        let run = sample_run();
        let slow = run.slowest(10);
        assert_eq!(slow.len(), 3);
    }

    #[test]
    fn all_failed_iter() {
        let run = sample_run();
        let failed: Vec<_> = run.all_failed().collect();
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].name, "t2");
    }

    #[test]
    fn default_run_empty() {
        let run = TestRun::default();
        assert!(run.suites.is_empty());
        assert!(run.success());
    }
}

mod report_format {
    use super::*;

    #[test]
    fn parse_pretty() {
        assert_eq!("pretty".parse::<ReportFormat>().unwrap(), ReportFormat::Pretty);
        assert_eq!("human".parse::<ReportFormat>().unwrap(), ReportFormat::Pretty);
    }

    #[test]
    fn parse_tap() {
        assert_eq!("tap".parse::<ReportFormat>().unwrap(), ReportFormat::Tap);
    }

    #[test]
    fn parse_junit() {
        assert_eq!("junit".parse::<ReportFormat>().unwrap(), ReportFormat::Junit);
        assert_eq!("xml".parse::<ReportFormat>().unwrap(), ReportFormat::Junit);
    }

    #[test]
    fn parse_json() {
        assert_eq!("json".parse::<ReportFormat>().unwrap(), ReportFormat::Json);
    }

    #[test]
    fn parse_compact() {
        assert_eq!("compact".parse::<ReportFormat>().unwrap(), ReportFormat::Compact);
    }

    #[test]
    fn parse_github() {
        assert_eq!("github".parse::<ReportFormat>().unwrap(), ReportFormat::Github);
        assert_eq!("gh".parse::<ReportFormat>().unwrap(), ReportFormat::Github);
    }

    #[test]
    fn parse_unknown_error() {
        assert!("wtf".parse::<ReportFormat>().is_err());
    }

    #[test]
    fn parse_is_case_insensitive() {
        assert_eq!("JSON".parse::<ReportFormat>().unwrap(), ReportFormat::Json);
        assert_eq!("Pretty".parse::<ReportFormat>().unwrap(), ReportFormat::Pretty);
    }

    #[test]
    fn default_is_pretty() {
        assert_eq!(ReportFormat::default(), ReportFormat::Pretty);
    }
}

mod coverage_format {
    use super::*;

    #[test]
    fn parse_summary() {
        assert_eq!("summary".parse::<CoverageFormat>().unwrap(), CoverageFormat::Summary);
        assert_eq!("text".parse::<CoverageFormat>().unwrap(), CoverageFormat::Summary);
    }

    #[test]
    fn parse_html() {
        assert_eq!("html".parse::<CoverageFormat>().unwrap(), CoverageFormat::Html);
    }

    #[test]
    fn parse_lcov() {
        assert_eq!("lcov".parse::<CoverageFormat>().unwrap(), CoverageFormat::Lcov);
        assert_eq!("tracefile".parse::<CoverageFormat>().unwrap(), CoverageFormat::Lcov);
    }

    #[test]
    fn parse_json() {
        assert_eq!("json".parse::<CoverageFormat>().unwrap(), CoverageFormat::Json);
    }

    #[test]
    fn parse_cobertura() {
        assert_eq!("cobertura".parse::<CoverageFormat>().unwrap(), CoverageFormat::Cobertura);
        assert_eq!("xml".parse::<CoverageFormat>().unwrap(), CoverageFormat::Cobertura);
    }

    #[test]
    fn parse_unknown_error() {
        assert!("bogus".parse::<CoverageFormat>().is_err());
    }

    #[test]
    fn default_is_summary() {
        assert_eq!(CoverageFormat::default(), CoverageFormat::Summary);
    }
}

mod runner_config {
    use super::*;

    #[test]
    fn default_parallel_true() {
        assert!(RunnerConfig::default().parallel);
    }

    #[test]
    fn default_format_pretty() {
        assert_eq!(RunnerConfig::default().format, ReportFormat::Pretty);
    }

    #[test]
    fn default_no_filter() {
        assert!(RunnerConfig::default().filter.is_none());
    }

    #[test]
    fn default_no_tags() {
        let cfg = RunnerConfig::default();
        assert!(cfg.include_tags.is_empty());
        assert!(cfg.exclude_tags.is_empty());
    }

    #[test]
    fn default_zero_retries() {
        assert_eq!(RunnerConfig::default().default_retries, 0);
    }

    #[test]
    fn coverage_report_with_path() {
        let report = CoverageReport {
            line_coverage: 80.0,
            function_coverage: 90.0,
            region_coverage: 80.0,
            format: CoverageFormat::Html,
            report_path: Some(std::path::PathBuf::from("report.html")),
        };
        assert_eq!(report.format, CoverageFormat::Html);
        assert_eq!(report.report_path.unwrap().to_str().unwrap(), "report.html");
    }
}

mod runner_config_builder {
    use super::*;

    #[test]
    fn with_filter() {
        let cfg = RunnerConfig::default().with_filter("auth");
        assert_eq!(cfg.filter.as_deref(), Some("auth"));
    }

    #[test]
    fn with_verbose() {
        let cfg = RunnerConfig::default().with_verbose(true);
        assert!(cfg.verbose);
    }

    #[test]
    fn with_fail_fast() {
        let cfg = RunnerConfig::default().with_fail_fast(true);
        assert!(cfg.fail_fast);
    }

    #[test]
    fn with_format() {
        let cfg = RunnerConfig::default().with_format(ReportFormat::Compact);
        assert_eq!(cfg.format, ReportFormat::Compact);
    }

    #[test]
    fn with_retries() {
        let cfg = RunnerConfig::default().with_retries(3);
        assert_eq!(cfg.default_retries, 3);
    }

    #[test]
    fn with_timeout() {
        let cfg = RunnerConfig::default().with_timeout(Duration::from_secs(30));
        assert_eq!(cfg.default_timeout, Some(Duration::from_secs(30)));
    }

    #[test]
    fn with_parallel() {
        let cfg = RunnerConfig::default().with_parallel(false);
        assert!(!cfg.parallel);
    }

    #[test]
    fn chained_builder() {
        let cfg = RunnerConfig::default()
            .with_filter("api")
            .with_verbose(true)
            .with_fail_fast(true)
            .with_format(ReportFormat::Compact);
        assert_eq!(cfg.filter.as_deref(), Some("api"));
        assert!(cfg.verbose);
        assert!(cfg.fail_fast);
        assert_eq!(cfg.format, ReportFormat::Compact);
    }

    #[test]
    fn preset_ci() {
        let cfg = RunnerConfig::ci();
        assert_eq!(cfg.format, ReportFormat::Junit);
        assert!(cfg.fail_fast);
        assert!(!cfg.verbose);
    }

    #[test]
    fn preset_dev() {
        let cfg = RunnerConfig::dev();
        assert_eq!(cfg.format, ReportFormat::Pretty);
        assert!(cfg.verbose);
    }

    #[test]
    fn with_config_merges() {
        let base = RunnerConfig::default().with_verbose(true).with_fail_fast(true);
        let merged = RunnerConfig::default().with_config(&base);
        assert!(merged.verbose);
        assert!(merged.fail_fast);
    }

    #[test]
    fn with_config_does_not_override_defaults() {
        let base = RunnerConfig::default(); // all defaults
        let merged = RunnerConfig::default()
            .with_filter("keep")
            .with_config(&base);
        // filter from chained call should be kept
        assert_eq!(merged.filter.as_deref(), Some("keep"));
    }

    #[test]
    fn with_output_capture() {
        let cfg = RunnerConfig::default().with_output_capture(true);
        assert!(cfg.output_capture);
    }

    #[test]
    fn with_include_tags() {
        let cfg = RunnerConfig::default().with_include_tags(["smoke", "core"]);
        assert_eq!(cfg.include_tags, vec!["smoke", "core"]);
    }

    #[test]
    fn with_exclude_tags() {
        let cfg = RunnerConfig::default().with_exclude_tags(["slow"]);
        assert_eq!(cfg.exclude_tags, vec!["slow"]);
    }

    #[test]
    fn with_seed() {
        let cfg = RunnerConfig::default().with_seed(42);
        assert_eq!(cfg.seed, Some(42));
    }

    #[test]
    fn with_max_threads() {
        let cfg = RunnerConfig::default().with_max_threads(8);
        assert_eq!(cfg.max_threads, 8);
    }
}
}
