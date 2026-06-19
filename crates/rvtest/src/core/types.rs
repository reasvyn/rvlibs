//! Core types shared across all rvtest modules.
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
//! - [`CoverageReport`] — aggregated coverage metrics
//! - [`SourceLocation`] — file:line:column tracking

use std::fmt;
use std::time::{Duration, SystemTime};

use crate::sandbox::SandboxConfig;

/// A location in source code where a test is defined or an assertion failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    /// The file path.
    pub file: String,
    /// The line number (1-indexed).
    pub line: u32,
    /// The optional column number.
    pub column: Option<u32>,
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.column {
            Some(col) => write!(f, "{}:{}:{}", self.file, self.line, col),
            None => write!(f, "{}:{}", self.file, self.line),
        }
    }
}

/// The outcome of a single test case execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestStatus {
    /// The test completed successfully without panicking.
    Passed,
    /// The test panicked or returned an error.
    Failed {
        /// A human-readable description of the failure.
        reason: String,
        /// Where the failure originated, if known.
        location: Option<SourceLocation>,
    },
    /// The test was skipped, optionally with a reason.
    Skipped {
        /// Why the test was skipped.
        reason: Option<String>,
    },
    /// The test exceeded its allotted time budget.
    TimedOut {
        /// The maximum duration allowed.
        duration: Duration,
        /// Where the test is defined, if known.
        location: Option<SourceLocation>,
    },
}

impl TestStatus {
    /// Returns `true` if the status represents a passing outcome.
    pub fn is_passed(&self) -> bool {
        matches!(self, TestStatus::Passed)
    }

    /// Returns `true` if the status represents any kind of failure (including timeout).
    pub fn is_failed(&self) -> bool {
        matches!(self, TestStatus::Failed { .. } | TestStatus::TimedOut { .. })
    }

    /// Returns `true` if the test was skipped.
    pub fn is_skipped(&self) -> bool {
        matches!(self, TestStatus::Skipped { .. })
    }
}

/// Statistics collected from running a benchmark.
#[derive(Debug, Clone, Copy)]
pub struct BenchStats {
    /// Number of iterations run.
    pub iterations: u32,
    /// Total wall-clock time for all iterations.
    pub total: Duration,
    /// Minimum single-iteration time.
    pub min: Duration,
    /// Maximum single-iteration time.
    pub max: Duration,
    /// Average (mean) single-iteration time.
    pub mean: Duration,
}

impl BenchStats {
    pub fn from_iterations(iterations: u32, total: Duration) -> Self {
        let mean = Duration::from_nanos((total.as_nanos() / iterations as u128) as u64);
        BenchStats {
            iterations,
            total,
            min: Duration::ZERO,
            max: Duration::ZERO,
            mean,
        }
    }
}

impl fmt::Display for TestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestStatus::Passed => write!(f, "PASSED"),
            TestStatus::Failed { reason, .. } => write!(f, "FAILED: {reason}"),
            TestStatus::Skipped { reason: Some(r) } => write!(f, "SKIPPED: {r}"),
            TestStatus::Skipped { reason: None } => write!(f, "SKIPPED"),
            TestStatus::TimedOut { duration, .. } => {
                write!(f, "TIMED OUT after {duration:?}")
            }
        }
    }
}

/// A single test case with its metadata and execution result.
#[derive(Debug, Clone)]
pub struct TestCase {
    /// The human-readable name of the test.
    pub name: String,
    /// The name of the parent suite, if any.
    pub suite: Option<String>,
    /// Tags attached to this test for filtering and organisation.
    pub tags: Vec<String>,
    /// The outcome of executing the test.
    pub status: TestStatus,
    /// How long the test took to execute.
    pub duration: Duration,
    /// How many assertions were performed (best-effort count).
    pub assertions: u64,
    /// Where the test was defined in source code.
    pub location: Option<SourceLocation>,
    /// Named parameters supplied to a parametrized test.
    pub parameters: Vec<(String, String)>,
    /// Captured stdout/stderr output during test execution, if any.
    pub captured_output: Option<String>,
    /// Benchmark statistics, if this was a benchmark test.
    pub bench_stats: Option<BenchStats>,
    /// Regression threshold for benchmark mean duration.
    pub bench_threshold: Option<Duration>,
}

impl TestCase {
    /// Create a new test case with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        TestCase {
            name: name.into(),
            suite: None,
            tags: Vec::new(),
            status: TestStatus::Passed,
            duration: Duration::ZERO,
            assertions: 0,
            location: None,
            parameters: Vec::new(),
            captured_output: None,
            bench_stats: None,
            bench_threshold: None,
        }
    }
}

/// The kind of test suite, used by the PrettyReporter for section headers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestKind {
    /// Unit tests (typically from `src/lib.rs` or `src/main.rs`).
    Unit,
    /// Integration tests (from `tests/` directory).
    Integration,
    /// Documentation tests (code examples in doc comments).
    Doc,
}

/// A collection of related test cases that share a common context.
#[derive(Debug, Clone)]
pub struct TestSuite {
    /// The name of this suite (e.g. a module or `describe` block name).
    pub name: String,
    /// An optional description of what this suite covers.
    pub description: Option<String>,
    /// The test cases belonging to this suite.
    pub tests: Vec<TestCase>,
    /// Total wall-clock duration for all tests in this suite.
    pub duration: Duration,
    /// The kind of tests in this suite.
    pub kind: TestKind,
    /// The source path or crate name (e.g. `src/lib.rs`, `tests/integration.rs`, `rvtest`).
    pub source_path: String,
}

impl TestSuite {
    /// Create a new empty suite with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        TestSuite {
            name: name.into(),
            description: None,
            tests: Vec::new(),
            duration: Duration::ZERO,
            kind: TestKind::Unit,
            source_path: String::new(),
        }
    }

    /// Returns `true` if this suite is a doc-test section.
    pub fn is_doc(&self) -> bool {
        self.kind == TestKind::Doc
    }

    /// Returns the number of tests in this suite.
    pub fn len(&self) -> usize {
        self.tests.len()
    }

    /// Returns `true` if this suite contains no tests.
    pub fn is_empty(&self) -> bool {
        self.tests.is_empty()
    }

    /// Returns an iterator over tests that passed.
    pub fn passed(&self) -> impl Iterator<Item = &TestCase> {
        self.tests.iter().filter(|t| t.status.is_passed())
    }

    /// Returns an iterator over tests that failed.
    pub fn failed(&self) -> impl Iterator<Item = &TestCase> {
        self.tests.iter().filter(|t| t.status.is_failed())
    }

    /// Returns an iterator over tests that were skipped.
    pub fn skipped(&self) -> impl Iterator<Item = &TestCase> {
        self.tests.iter().filter(|t| t.status.is_skipped())
    }

    /// Returns `true` if every test in this suite passed.
    pub fn success(&self) -> bool {
        self.failed().count() == 0
    }

    /// Panics with a detailed failure report if any test in this suite
    /// did not pass. Designed for use inside `#[test]` functions.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[test]
    /// fn my_tests() {
    ///     describe("Calculator")
    ///         .it("adds", || assert_eq!(2 + 2, 4))
    ///         .run()
    ///         .assert_all_pass();
    /// }
    /// ```
    pub fn assert_all_pass(&self) {
        let failed: Vec<&TestCase> = self.failed().collect();
        if !failed.is_empty() {
            let mut msg = format!(
                "{} test(s) failed in suite '{}':\n",
                failed.len(),
                self.name,
            );
            for t in &failed {
                let dur_ms = t.duration.as_secs_f64() * 1000.0;
                let reason = match &t.status {
                    TestStatus::Failed { reason, .. } => reason.as_str(),
                    TestStatus::TimedOut { .. } => "timed out",
                    _ => "unknown",
                };
                msg.push_str(&format!("  ✗ {} [{dur_ms:.1}ms] — {reason}\n", t.name));
            }
            panic!("{msg}");
        }
    }
}

/// Aggregated results from an entire test run consisting of one or more suites.
#[derive(Debug, Clone)]
pub struct TestRun {
    /// The suites that were executed.
    pub suites: Vec<TestSuite>,
    /// Wall-clock time the run started.
    pub start_time: SystemTime,
    /// Wall-clock time the run finished.
    pub end_time: SystemTime,
    /// Total wall-clock duration of the run.
    pub duration: Duration,
}

impl TestRun {
    /// Create a new `TestRun` starting now.
    pub fn new() -> Self {
        TestRun {
            suites: Vec::new(),
            start_time: SystemTime::now(),
            end_time: SystemTime::now(),
            duration: Duration::ZERO,
        }
    }

    /// Returns the total number of test cases across all suites.
    pub fn total(&self) -> usize {
        self.suites.iter().map(|s| s.tests.len()).sum()
    }

    /// Returns the number of passed test cases.
    pub fn total_passed(&self) -> usize {
        self.suites.iter().flat_map(|s| s.tests.iter()).filter(|t| t.status.is_passed()).count()
    }

    /// Returns the number of failed test cases (including timeouts).
    pub fn total_failed(&self) -> usize {
        self.suites.iter().flat_map(|s| s.tests.iter()).filter(|t| t.status.is_failed()).count()
    }

    /// Returns the number of skipped test cases.
    pub fn total_skipped(&self) -> usize {
        self.suites.iter().flat_map(|s| s.tests.iter()).filter(|t| t.status.is_skipped()).count()
    }

    /// Returns `true` if every test passed.
    pub fn success(&self) -> bool {
        self.total_failed() == 0
    }

    /// Returns an iterator over all test cases that failed.
    pub fn all_failed(&self) -> impl Iterator<Item = &TestCase> {
        self.suites.iter().flat_map(|s| s.failed())
    }

    /// Returns the `n` slowest test cases across all suites, sorted by
    /// duration (longest first).
    pub fn slowest(&self, n: usize) -> Vec<&TestCase> {
        let mut all: Vec<&TestCase> = self.suites.iter().flat_map(|s| s.tests.iter()).collect();
        all.sort_by_key(|t| std::cmp::Reverse(t.duration));
        all.truncate(n);
        all
    }
}

impl Default for TestRun {
    fn default() -> Self {
        Self::new()
    }
}

/// The output format used when rendering test results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReportFormat {
    /// Human-readable, colourised output (default).
    #[default]
    Pretty,
    /// Test Anything Protocol — machine-parseable line-based format.
    Tap,
    /// JUnit XML — widely supported by CI systems.
    Junit,
    /// JSON output — suitable for programmatic consumption.
    Json,
    /// Compact single-line-per-test output.
    Compact,
    /// GitHub Actions annotations.
    Github,
    /// Agent-native JSON — structured results optimised for LLM consumption.
    Agent,
    /// Standalone HTML report.
    Html,
    /// Cargo-nextest compatible JSON-lines output.
    Nextest,
}

impl std::str::FromStr for ReportFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pretty" | "human" => Ok(Self::Pretty),
            "tap" => Ok(Self::Tap),
            "junit" | "xml" => Ok(Self::Junit),
            "json" => Ok(Self::Json),
            "compact" => Ok(Self::Compact),
            "github" | "gh" => Ok(Self::Github),
            "agent" | "ai" | "llm" => Ok(Self::Agent),
            "html" => Ok(Self::Html),
            "nextest" | "next" => Ok(Self::Nextest),
            _ => Err(format!("unknown report format: {s}")),
        }
    }
}

/// Colour output preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorChoice {
    /// Let the terminal decide (default).
    #[default]
    Auto,
    /// Always emit ANSI colour codes.
    Always,
    /// Never emit ANSI colour codes.
    Never,
}

impl std::str::FromStr for ColorChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err(format!("unknown color choice: {s} (expected auto/always/never)")),
        }
    }
}

/// Global configuration for a test run.
#[derive(Debug, Clone)]
pub struct RunnerConfig {
    /// Only run tests whose name contains this string.
    pub filter: Option<String>,
    /// Only run tests carrying *all* of these tags.
    pub include_tags: Vec<String>,
    /// Skip tests carrying *any* of these tags.
    pub exclude_tags: Vec<String>,
    /// Skip tests whose name matches this pattern (complement to `filter`).
    pub skip: Option<String>,
    /// Default number of retries for flaky tests.
    pub default_retries: u32,
    /// Automatically retry failed tests once (no explicit `--retries` needed).
    pub auto_retry: bool,
    /// Default per-test timeout.
    pub default_timeout: Option<Duration>,
    /// Whether to run tests in parallel.
    pub parallel: bool,
    /// Maximum number of threads for parallel execution.
    pub max_threads: usize,
    /// Output format for results.
    pub format: ReportFormat,
    /// Stop after the first failure.
    pub fail_fast: bool,
    /// Seed for randomised features (property testing, shuffle).
    pub seed: Option<u64>,
    /// Show detailed output for each test.
    pub verbose: bool,
    /// Capture stdout/stderr during test execution and show on failure.
    pub output_capture: bool,
    /// Randomise test execution order.
    pub shuffle: bool,
    /// Colour output preference.
    pub color: ColorChoice,
    /// Run benchmarks instead of tests.
    pub bench: bool,
    /// Default number of benchmark iterations.
    pub bench_iterations: u32,
    /// Default benchmark regression threshold.
    pub bench_threshold: Option<Duration>,
    /// Run each test in a separate OS process for full isolation.
    pub process_isolation: bool,
    /// Mask secrets (API keys, tokens, passwords) in captured test output.
    pub mask_secrets: bool,
    /// Sandbox configuration for test execution.
    pub sandbox: SandboxConfig,
}

impl Default for RunnerConfig {
    fn default() -> Self {
        RunnerConfig {
            filter: None,
            include_tags: Vec::new(),
            exclude_tags: Vec::new(),
            skip: None,
            default_retries: 0,
            auto_retry: false,
            default_timeout: None,
            parallel: true,
            max_threads: num_cpus(),
            format: ReportFormat::Pretty,
            fail_fast: false,
            seed: None,
            verbose: false,
            output_capture: false,
            shuffle: false,
            color: ColorChoice::Auto,
            bench: false,
            bench_iterations: 100,
            bench_threshold: None,
            process_isolation: false,
            mask_secrets: false,
            sandbox: SandboxConfig::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// Builder methods
// ---------------------------------------------------------------------------

impl RunnerConfig {
    /// Create a preset for CI: JUnit output, fail-fast, no capture.
    pub fn ci() -> Self {
        RunnerConfig {
            format: ReportFormat::Junit,
            fail_fast: true,
            verbose: false,
            output_capture: false,
            ..Default::default()
        }
    }

    /// Create a preset for local development: pretty output, verbose.
    pub fn dev() -> Self {
        RunnerConfig {
            format: ReportFormat::Pretty,
            verbose: true,
            ..Default::default()
        }
    }

    /// Merge the non-`None` / non-default fields from `other` into `self`.
    pub fn with_config(mut self, other: &RunnerConfig) -> Self {
        if let Some(ref f) = other.filter {
            self.filter = Some(f.clone());
        }
        if !other.include_tags.is_empty() {
            self.include_tags = other.include_tags.clone();
        }
        if !other.exclude_tags.is_empty() {
            self.exclude_tags = other.exclude_tags.clone();
        }
        if let Some(ref s) = other.skip {
            self.skip = Some(s.clone());
        }
        if other.default_retries != 0 {
            self.default_retries = other.default_retries;
        }
        if other.auto_retry {
            self.auto_retry = true;
        }
        if other.default_timeout.is_some() {
            self.default_timeout = other.default_timeout;
        }
        if !other.parallel {
            self.parallel = other.parallel;
        }
        if other.max_threads != num_cpus() {
            self.max_threads = other.max_threads;
        }
        if other.format != ReportFormat::Pretty {
            self.format = other.format;
        }
        if other.fail_fast {
            self.fail_fast = true;
        }
        if other.seed.is_some() {
            self.seed = other.seed;
        }
        if other.verbose {
            self.verbose = true;
        }
        if other.output_capture {
            self.output_capture = true;
        }
        if other.shuffle {
            self.shuffle = true;
        }
        if other.color != ColorChoice::Auto {
            self.color = other.color;
        }
        if other.bench {
            self.bench = true;
        }
        if other.bench_iterations != 100 {
            self.bench_iterations = other.bench_iterations;
        }
        if other.bench_threshold.is_some() {
            self.bench_threshold = other.bench_threshold;
        }
        if other.process_isolation {
            self.process_isolation = true;
        }
        if other.mask_secrets {
            self.mask_secrets = true;
        }
        self
    }

    /// Load settings from a TOML file and merge them into this config.
    ///
    /// This is opt-in — no auto-discovery.  Returns `Err` if the file cannot
    /// be read or parsed.
    pub fn with_config_file(mut self, path: impl AsRef<std::path::Path>) -> Result<Self, String> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| format!("cannot read config: {e}"))?;
        let file_cfg: crate::config::FileConfig = toml::from_str(&content)
            .map_err(|e| format!("cannot parse config: {e}"))?;
        file_cfg.apply_to(&mut self);
        Ok(self)
    }

    /// Set the name filter.
    pub fn with_filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// Add include tags.
    pub fn with_include_tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.include_tags = tags.into_iter().map(|t| t.into()).collect();
        self
    }

    /// Add exclude tags.
    pub fn with_exclude_tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclude_tags = tags.into_iter().map(|t| t.into()).collect();
        self
    }

    /// Set default retries.
    pub fn with_retries(mut self, retries: u32) -> Self {
        self.default_retries = retries;
        self
    }

    /// Set default timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = Some(timeout);
        self
    }

    /// Set parallel execution.
    pub fn with_parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }

    /// Set max threads.
    pub fn with_max_threads(mut self, n: usize) -> Self {
        self.max_threads = n;
        self
    }

    /// Set output format.
    pub fn with_format(mut self, format: ReportFormat) -> Self {
        self.format = format;
        self
    }

    /// Set fail-fast.
    pub fn with_fail_fast(mut self, yes: bool) -> Self {
        self.fail_fast = yes;
        self
    }

    /// Set random seed.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set verbose mode.
    pub fn with_verbose(mut self, yes: bool) -> Self {
        self.verbose = yes;
        self
    }

    /// Set output capture.
    pub fn with_output_capture(mut self, yes: bool) -> Self {
        self.output_capture = yes;
        self
    }

    /// Enable or disable automatic retry of failed tests.
    pub fn with_auto_retry(mut self, yes: bool) -> Self {
        self.auto_retry = yes;
        self
    }

    /// Skip tests whose name contains this string (case-insensitive).
    pub fn with_skip(mut self, pattern: impl Into<String>) -> Self {
        self.skip = Some(pattern.into());
        self
    }

    /// Enable or disable test execution order shuffling.
    pub fn with_shuffle(mut self, yes: bool) -> Self {
        self.shuffle = yes;
        self
    }

    /// Set colour output preference.
    pub fn with_color(mut self, color: ColorChoice) -> Self {
        self.color = color;
        self
    }

    /// Enable or disable process-per-test isolation.
    pub fn with_process_isolation(mut self, yes: bool) -> Self {
        self.process_isolation = yes;
        self
    }

    /// Enable or disable secrets masking in captured test output.
    pub fn with_mask_secrets(mut self, yes: bool) -> Self {
        self.mask_secrets = yes;
        self
    }
}

/// Heuristic for the number of available CPUs.
fn num_cpus() -> usize {
    std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4)
}

/// Create a seeded or unseeded RNG for randomised operations.
pub fn rng_from_seed(seed: Option<u64>) -> rand::rngs::StdRng {
    use rand::SeedableRng;
    match seed {
        Some(s) => rand::rngs::StdRng::seed_from_u64(s),
        None => rand::rngs::StdRng::from_rng(&mut rand::rng()),
    }
}

// ---------------------------------------------------------------------------
// Coverage types
// ---------------------------------------------------------------------------

/// Output format for coverage reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CoverageFormat {
    /// Plain-text summary printed to stdout.
    #[default]
    Summary,
    /// HTML report with line-level detail.
    Html,
    /// LCOV tracefile (for IDE integration, Coveralls, etc.).
    Lcov,
    /// Machine-readable JSON.
    Json,
    /// Cobertura XML (for Jenkins, GitLab, etc.).
    Cobertura,
}

impl std::str::FromStr for CoverageFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "summary" | "text" => Ok(Self::Summary),
            "html" => Ok(Self::Html),
            "lcov" | "tracefile" => Ok(Self::Lcov),
            "json" => Ok(Self::Json),
            "cobertura" | "xml" => Ok(Self::Cobertura),
            _ => Err(format!("unknown coverage format: {s}")),
        }
    }
}

/// Aggregated coverage metrics for a codebase.
#[derive(Debug, Clone)]
pub struct CoverageReport {
    /// Percentage of lines covered (0.0 – 100.0).
    pub line_coverage: f64,
    /// Percentage of functions covered.
    pub function_coverage: f64,
    /// Percentage of regions (basic blocks) covered.
    pub region_coverage: f64,
    /// The format the full report was generated in.
    pub format: CoverageFormat,
    /// Path to the generated report file, if applicable.
    pub report_path: Option<std::path::PathBuf>,
}

// ---------------------------------------------------------------------------
