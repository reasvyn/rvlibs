//! Test runner and cargo-test output parsing.
//!
//! [`TestRunner`] collects [`Spec`]s and runs them with a [`RunnerConfig`],
//! supporting filtering, parallel execution, retries, and timeouts.
//!
//! [`parse_cargo_test_output()`] converts `cargo test` (or direct test
//! binary) stdout/stderr into structured [`TestSuite`]s.
//!
//! Convenience functions [`run_tests()`] and [`run_and_exit()`] provide
//! quick entry points for simple use cases.

use std::time::{Duration, Instant, SystemTime};

use crate::core::{ReportFormat, RunnerConfig, TestCase, TestKind, TestRun, TestStatus, TestSuite};
use crate::report::{self, TestReporter};
use crate::spec::Spec;

/// A configurable test runner that collects specs and executes them.
///
/// `TestRunner` provides a unified entry point for running tests defined with
/// [`Spec`], applying filtering, parallel execution, retries, and timeouts
/// according to a [`RunnerConfig`].
///
/// # Example
///
/// ```ignore
/// use rvtest::runner::TestRunner;
/// use rvtest::spec::describe;
/// use rvtest::core::RunnerConfig;
///
/// let runner = TestRunner::new(RunnerConfig::default())
///     .add_spec(describe("Math")
///         .it("adds", || assert_eq!(2 + 2, 4))
///     );
///
/// let run = runner.run();
/// assert!(run.success());
/// ```
pub struct TestRunner {
    config: RunnerConfig,
    specs: Vec<Spec>,
}

impl TestRunner {
    /// Create a new runner with the given configuration.
    pub fn new(config: RunnerConfig) -> Self {
        TestRunner { config, specs: Vec::new() }
    }

    /// Add a test spec to the runner.
    pub fn add_spec(mut self, spec: Spec) -> Self {
        self.specs.push(spec);
        self
    }

    /// Add multiple test specs at once.
    pub fn add_specs(mut self, specs: impl IntoIterator<Item = Spec>) -> Self {
        self.specs.extend(specs);
        self
    }

    /// Execute all registered specs and return a [`TestRun`] with aggregated
    /// results.
    pub fn run(mut self) -> TestRun {
        let start_time = SystemTime::now();
        let wall_start = Instant::now();

        let mut suites = Vec::new();

        let specs = std::mem::take(&mut self.specs);
        for spec in specs {
            let suite = self.run_spec(spec);
            suites.push(suite);
        }

        let duration = wall_start.elapsed();

        TestRun {
            suites,
            start_time,
            end_time: SystemTime::now(),
            duration,
        }
    }

    fn run_spec(&self, spec: Spec) -> TestSuite {
        spec.run_with_config(&self.config)
    }

    /// Render the test run results using the configured report format.
    pub fn report(&self, run: &TestRun) -> String {
        let reporter: Box<dyn TestReporter> = match self.config.format {
            ReportFormat::Pretty => Box::new(report::PrettyReporter::new()),
            ReportFormat::Tap => Box::new(report::TapReporter),
            ReportFormat::Junit => Box::new(report::JunitReporter::new()),
            ReportFormat::Json => Box::new(report::JsonReporter),
        ReportFormat::Compact => Box::new(report::CompactReporter),
        ReportFormat::Github => Box::new(report::GithubReporter),
        ReportFormat::Agent => Box::new(report::AgentReporter),
        ReportFormat::Html => Box::new(report::HtmlReporter),
        ReportFormat::Nextest => Box::new(report::NextestReporter),
        };
        reporter.report(run)
    }
}

/// Convenience function to run specs with default configuration.
///
/// Equivalent to `TestRunner::new(RunnerConfig::default()).add_specs(specs).run()`.
pub fn run_tests(specs: impl IntoIterator<Item = Spec>) -> TestRun {
    TestRunner::new(RunnerConfig::default())
        .add_specs(specs)
        .run()
}

/// Run specs and print the report to stdout.
///
/// Exits the process with code `0` on success or `1` on failure.
pub fn run_and_exit(specs: impl IntoIterator<Item = Spec>) -> ! {
    let config = RunnerConfig::default();
    let run = run_tests(specs);
    let report = render_report_with_config(&config, &run);
    println!("{report}");
    std::process::exit(if run.success() { 0 } else { 1 });
}

fn render_report_with_config(config: &RunnerConfig, run: &TestRun) -> String {
    let reporter: Box<dyn TestReporter> = match config.format {
        ReportFormat::Pretty => Box::new(report::PrettyReporter::new()),
        ReportFormat::Tap => Box::new(report::TapReporter),
        ReportFormat::Junit => Box::new(report::JunitReporter::new()),
        ReportFormat::Json => Box::new(report::JsonReporter),
        ReportFormat::Compact => Box::new(report::CompactReporter),
        ReportFormat::Github => Box::new(report::GithubReporter),
        ReportFormat::Agent => Box::new(report::AgentReporter),
        ReportFormat::Html => Box::new(report::HtmlReporter),
        ReportFormat::Nextest => Box::new(report::NextestReporter),
    };
    reporter.report(run)
}

// ---------------------------------------------------------------------------
// Cargo test output parsing
// ---------------------------------------------------------------------------

/// Parse `cargo test` output (or direct test binary output) into [`TestSuite`]s.
///
/// Section headers (`Running …`, `Doc-tests …`) are on stderr,
/// test-result lines and failure details are on stdout.
///
/// When stderr contains no section headers (e.g. running a test binary
/// directly), a fallback single section is created.
pub fn parse_cargo_test_output(stderr: &str, stdout: &str) -> Vec<TestSuite> {
    struct Section {
        kind: TestKind,
        source_path: String,
        tests: Vec<(String, TestStatus)>,
        failure_details: Vec<String>,
    }

    let mut sections: Vec<Section> = Vec::new();

    // ── Phase 1: parse stderr for section headers ──
    for line in stderr.lines() {
        let trimmed = line.trim();

        if let Some(rest) = trimmed.strip_prefix("Running ") {
            let before_parens = rest.split_once(" (").map_or(rest, |(p, _)| p);
            let (kind, source_path) = if let Some(path) = before_parens.strip_prefix("unittests ") {
                (TestKind::Unit, path.to_owned())
            } else {
                (TestKind::Integration, before_parens.to_owned())
            };
            sections.push(Section { kind, source_path, tests: Vec::new(), failure_details: Vec::new() });
        }

        if let Some(crate_name) = trimmed.strip_prefix("Doc-tests ") {
            sections.push(Section { kind: TestKind::Doc, source_path: crate_name.to_owned(), tests: Vec::new(), failure_details: Vec::new() });
        }
    }

    // If no sections were found, create a fallback single section.
    if sections.is_empty() {
        sections.push(Section { kind: TestKind::Unit, source_path: "tests".to_owned(), tests: Vec::new(), failure_details: Vec::new() });
    }

    // ── Phase 2: parse stdout for test lines and failure details ──
    let mut si = 0;
    let mut in_failure = false;

    for line in stdout.lines() {
        let trimmed = line.trim();

        // ---- <test> stdout ----
        if trimmed.starts_with("---- ") && trimmed.ends_with(" stdout ----") {
            in_failure = true;
            continue;
        }

        // failures:
        if trimmed == "failures:" {
            in_failure = false;
            continue;
        }

        // test result: … summary line – move to next section
        if trimmed.starts_with("test result: ") {
            if si < sections.len() {
                let sec = &mut sections[si];
                if !sec.failure_details.is_empty() {
                    let details = std::mem::take(&mut sec.failure_details);
                    let mut iter = details.into_iter();
                    for (_, status) in &mut sec.tests {
                        if matches!(status, TestStatus::Failed { .. }) {
                            let detail: String = iter.by_ref()
                                .take_while(|l| !l.starts_with("test ") && !l.starts_with("----"))
                                .collect::<Vec<_>>()
                                .join("\n");
                            if !detail.is_empty() {
                                let old = std::mem::replace(status, TestStatus::Passed);
                                if let TestStatus::Failed { reason: _, location } = old {
                                    *status = TestStatus::Failed { reason: detail, location };
                                }
                            }
                        }
                    }
                }
            }
            si += 1;
            continue;
        }

        // Collect failure detail lines
        if in_failure && !trimmed.is_empty() && !trimmed.starts_with("----")
            && si < sections.len() {
                sections[si].failure_details.push(trimmed.to_owned());
            }

        // Test result lines: `test <name> ... <status>`
        if let Some(rest) = trimmed.strip_prefix("test ")
            && let Some((name, rest)) = rest.split_once(" ... ") {
                let status = if rest.starts_with("ok") {
                    TestStatus::Passed
                } else if rest.starts_with("FAILED") {
                    TestStatus::Failed { reason: String::new(), location: None }
                } else if rest.starts_with("ignored") {
                    TestStatus::Skipped { reason: None }
                } else {
                    continue;
                };

                if si < sections.len() {
                    let sec = &mut sections[si];
                    let test_name = if sec.kind == TestKind::Doc && !sec.source_path.is_empty() {
                        format!("{} - {}", sec.source_path, name)
                    } else {
                        name.to_owned()
                    };
                    sec.tests.push((test_name, status));
                }
            }
    }

    // Build TestSuites
    let mut suites: Vec<TestSuite> = Vec::new();
    for sec in sections {
        let suite_name = match sec.kind {
            TestKind::Unit => format!("unit tests ({})", sec.source_path),
            TestKind::Integration => format!("integration ({})", sec.source_path),
            TestKind::Doc => format!("doc-tests ({})", sec.source_path),
        };
        let mut suite = TestSuite::new(&suite_name);
        for (name, status) in sec.tests {
            suite.tests.push(TestCase {
                name,
                suite: Some(suite_name.clone()),
                tags: Vec::new(),
                status,
                duration: Duration::ZERO,
                assertions: 0,
                location: None,
                parameters: Vec::new(), captured_output: None,
                bench_stats: None,
                bench_threshold: None,
            });
        }
        suite.kind = sec.kind;
        suite.source_path = sec.source_path;
        suites.push(suite);
    }

    suites
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::describe;

    #[test]
    fn test_runner_new_with_config() {
        let config = RunnerConfig::default();
        let runner = TestRunner::new(config);
        assert!(runner.specs.is_empty());
    }

    #[test]
    fn test_runner_add_spec() {
        let config = RunnerConfig::default();
        let runner = TestRunner::new(config)
            .add_spec(describe("test").it("pass", || {}));
        assert_eq!(runner.specs.len(), 1);
    }

    #[test]
    fn test_runner_add_specs() {
        let config = RunnerConfig::default();
        let runner = TestRunner::new(config)
            .add_specs(vec![
                describe("a").it("a1", || {}),
                describe("b").it("b1", || {}),
            ]);
        assert_eq!(runner.specs.len(), 2);
    }

    #[test]
    fn test_runner_executes_specs() {
        let config = RunnerConfig::default();
        let run = TestRunner::new(config)
            .add_spec(describe("test").it("pass", || {}))
            .run();
        assert_eq!(run.total(), 1);
        assert!(run.success());
    }

    #[test]
    fn test_runner_reports() {
        let config = RunnerConfig::default();
        let run = TestRunner::new(config)
            .add_spec(describe("test").it("pass", || {}))
            .run();
        let report = render_report_with_config(&RunnerConfig::default(), &run);
        assert!(report.contains("Tests"));
    }

    #[test]
    fn test_run_tests_fn() {
        let run = run_tests(vec![describe("a").it("t", || {})]);
        assert_eq!(run.total(), 1);
        assert!(run.success());
    }

    #[test]
    fn test_runner_filters_tests() {
        let config = RunnerConfig {
            filter: Some("nonexistent".into()),
            ..RunnerConfig::default()
        };
        let run = TestRunner::new(config)
            .add_spec(describe("test").it("pass", || {}))
            .run();
        assert_eq!(run.total(), 0);
        assert!(run.success());
    }
}
