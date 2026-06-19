//! BDD-style test specification builder.
//!
//! The [`Spec`] type lets you organise tests in a nested, descriptive
//! hierarchy using [`describe`] / [`it`](Spec::it) blocks, attach
//! metadata ([`tag()`](Spec::tag), [`timeout()`](Spec::timeout),
//! [`retries()`](Spec::retries)), and register lifecycle hooks
//! ([`before_all()`](Spec::before_all), [`after_all()`](Spec::after_all),
//! [`before_each()`](Spec::before_each), [`after_each()`](Spec::after_each)).
//!
//! # Example
//!
//! ```ignore
//! use rvtest::spec::describe;
//!
//! #[test]
//! fn my_tests() {
//!     describe("Calculator")
//!         .it("adds", || assert_eq!(2 + 2, 4))
//!         .tag("arithmetic")
//!         .run()
//!         .assert_all_pass();
//! }
//! ```

use std::sync::Arc;
use std::time::{Duration, Instant};

use rand::seq::SliceRandom;

use crate::core::{RunnerConfig, SourceLocation, TestCase, TestStatus, TestSuite};

pub mod builder;
pub(crate) mod execute;

use self::execute::*;
pub use self::builder::SpecBuilder;

/// A BDD-style test specification builder.
///
/// `Spec` lets you organise tests in a nested, descriptive hierarchy using
/// [`describe`] / [`it`](Spec::it) blocks, attach metadata such as
/// [`tags`](Spec::tag), [`timeout`](Spec::timeout) and [`retries`](Spec::retries),
/// and register lifecycle hooks via [`before_all`](Spec::before_all) /
/// [`after_all`](Spec::after_all).
///
/// # Example inside `#[test]` (recommended)
///
/// ```ignore
/// use rvtest::spec::describe;
///
/// #[test]
/// fn my_tests() {
///     describe("Calculator")
///         .describe("addition")
///             .it("adds positive numbers", || {
///                 assert_eq!(2 + 2, 4);
///             })
///             .tag("arithmetic")
///             .timeout(std::time::Duration::from_secs(2))
///         .run()
///         .assert_all_pass();
/// }
/// ```
///
/// Call [`run`](Spec::run) to execute all leaf tests and produce a [`TestSuite`],
/// then call [`assert_all_pass`](crate::core::TestSuite::assert_all_pass) to
/// verify results inside a `#[test]`.
pub struct Spec {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) tags: Vec<String>,
    pub(crate) setup: Option<Arc<dyn Fn() + Send + Sync>>,
    pub(crate) teardown: Option<Arc<dyn Fn() + Send + Sync>>,
    pub(crate) before_each: Option<Arc<dyn Fn() + Send + Sync>>,
    pub(crate) after_each: Option<Arc<dyn Fn() + Send + Sync>>,
    pub(crate) children: Vec<Spec>,
    pub(crate) tests: Vec<TestEntry>,
    pub(crate) bench_entries: Vec<BenchEntry>,
    pub(crate) bench_iterations: u32,
    pub(crate) timeout: Option<Duration>,
    pub(crate) retries: u32,
}

pub(crate) struct TestEntry {
    pub(crate) name: String,
    pub(crate) location: Option<SourceLocation>,
    pub(crate) test_fn: Arc<dyn Fn() + Send + Sync>,
}

pub(crate) struct BenchEntry {
    pub(crate) name: String,
    pub(crate) location: Option<SourceLocation>,
    pub(crate) bench_fn: Arc<dyn Fn() + Send + Sync>,
    pub(crate) threshold: Option<Duration>,
}

/// Create a new top-level `Spec` with the given name.
///
/// This is the entry point for BDD-style test organisation. Use chained
/// method calls to describe the expected behaviour, then call [`run`](Spec::run).
pub fn describe(name: &str) -> Spec {
    Spec::new(name)
}

impl Spec {
    /// Create a new `Spec` with the given name.
    pub fn new(name: &str) -> Self {
        Spec {
            name: name.to_owned(),
            description: None,
            tags: Vec::new(),
            setup: None,
            teardown: None,
            before_each: None,
            after_each: None,
            children: Vec::new(),
            tests: Vec::new(),
            bench_entries: Vec::new(),
            bench_iterations: 100,
            timeout: None,
            retries: 0,
        }
    }

    /// Attach a description to this spec block.
    pub fn description(mut self, text: &str) -> Self {
        self.description = Some(text.to_owned());
        self
    }

    /// Add a tag to this spec and all contained tests.
    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_owned());
        self
    }

    /// Set the default timeout for tests in this block.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// Set the number of retries for flaky tests in this block.
    pub fn retries(mut self, count: u32) -> Self {
        self.retries = count;
        self
    }

    /// Register a setup hook run once before any test in this block.
    pub fn before_all(mut self, hook: impl Fn() + Send + Sync + 'static) -> Self {
        self.setup = Some(Arc::new(hook));
        self
    }

    /// Register a teardown hook run once after all tests in this block.
    pub fn after_all(mut self, hook: impl Fn() + Send + Sync + 'static) -> Self {
        self.teardown = Some(Arc::new(hook));
        self
    }

    /// Register a hook run before each test in this block (and child blocks).
    ///
    /// When nested, parent `before_each` hooks run before child `before_each`
    /// hooks. If the hook panics, the test is marked as failed but execution
    /// continues with the next test.
    pub fn before_each(mut self, hook: impl Fn() + Send + Sync + 'static) -> Self {
        self.before_each = Some(Arc::new(hook));
        self
    }

    /// Register a hook run after each test in this block (and child blocks).
    ///
    /// When nested, child `after_each` hooks run before parent `after_each`
    /// hooks. The hook runs even if the test itself panics.
    pub fn after_each(mut self, hook: impl Fn() + Send + Sync + 'static) -> Self {
        self.after_each = Some(Arc::new(hook));
        self
    }

    /// Register a leaf test case with the given name and body.
    ///
    /// The test body should use `assert!` / `assert_eq!` and will have its
    /// panics caught and reported as failures.
    #[track_caller]
    pub fn it(mut self, name: &str, test: impl Fn() + Send + Sync + 'static) -> Self {
        let loc = std::panic::Location::caller();
        self.tests.push(TestEntry {
            name: name.to_owned(),
            location: Some(SourceLocation {
                file: loc.file().to_owned(),
                line: loc.line(),
                column: None,
            }),
            test_fn: Arc::new(test),
        });
        self
    }

    /// Register a benchmark with the given name and body.
    ///
    /// The benchmark function will be run repeatedly to collect timing stats.
    #[track_caller]
    pub fn bench(mut self, name: &str, test: impl Fn() + Send + Sync + 'static) -> Self {
        let loc = std::panic::Location::caller();
        self.bench_entries.push(BenchEntry {
            name: name.to_owned(),
            location: Some(SourceLocation {
                file: loc.file().to_owned(),
                line: loc.line(),
                column: None,
            }),
            bench_fn: Arc::new(test),
            threshold: None,
        });
        self
    }

    /// Set the number of iterations for benchmarks in this spec.
    pub fn bench_iterations(mut self, n: u32) -> Self {
        self.bench_iterations = n;
        self
    }

    /// Set a default regression threshold for benchmarks in this spec.
    pub fn bench_threshold(mut self, dur: Duration) -> Self {
        // Apply threshold to all existing bench entries without their own threshold
        for entry in &mut self.bench_entries {
            if entry.threshold.is_none() {
                entry.threshold = Some(dur);
            }
        }
        self
    }

    /// Nest a child spec block inside this one.
    ///
    /// Child specs inherit the parent's tags, timeout, retry, and
    /// before_each/after_each settings unless they override them.
    pub fn describe(mut self, name: &str) -> SpecBuilder {
        let child_index = self.children.len();
        let child = Spec::new(name);
        self.children.push(child);
        SpecBuilder::new(self, vec![child_index])
    }

    /// Execute all leaf tests in this spec tree and return a `TestSuite`.
    ///
    /// Hooks (`before_all` / `after_all` / `before_each` / `after_each`) are
    /// honoured per block. Timing information is collected for each test and
    /// for the suite as a whole.
    pub fn run(self) -> TestSuite {
        let config = RunnerConfig::default();
        self.run_with_config(&config)
    }

    /// Execute tests with an explicit [`RunnerConfig`].
    pub fn run_with_config(self, config: &RunnerConfig) -> TestSuite {
        let mut suite = TestSuite::new(&self.name);
        suite.description = self.description.clone();

        // Enable output capture if configured
        if config.output_capture {
            crate::capture::set_capture_enabled(true);
        }

        let start = Instant::now();

        let test_cases = self.execute_recursive("", &[], None, 0, &[], &[], config);

        suite.duration = start.elapsed();
        suite.tests = test_cases;
        suite
    }

    /// Check whether this spec (or any descendant) produces at least one
    /// test that passes the current tag/name filters.
    fn has_matching(
        &self,
        prefix: &str,
        inherited_tags: &[String],
        config: &RunnerConfig,
    ) -> bool {
        let full_name = if prefix.is_empty() {
            self.name.clone()
        } else {
            format!("{} :: {}", prefix, self.name)
        };

        let merged_tags: Vec<String> = inherited_tags
            .iter()
            .cloned()
            .chain(self.tags.iter().cloned())
            .collect();

        for entry in &self.tests {
            let test_name = format!("{} :: {}", full_name, entry.name);
            if crate::tag::tags_match(&merged_tags, config)
                && crate::tag::name_matches(&test_name, config.filter.as_deref())
                && !crate::tag::name_skipped(&test_name, config.skip.as_deref())
            {
                return true;
            }
        }

        for entry in &self.bench_entries {
            let bench_name = format!("{} :: {}", full_name, entry.name);
            if crate::tag::tags_match(&merged_tags, config)
                && crate::tag::name_matches(&bench_name, config.filter.as_deref())
                && !crate::tag::name_skipped(&bench_name, config.skip.as_deref())
            {
                return true;
            }
        }

        for child in &self.children {
            if child.has_matching(&full_name, &merged_tags, config) {
                return true;
            }
        }

        false
    }

    /// Recursively execute this spec and all descendants, respecting nesting
    /// of hooks, tags, timeouts, and retries.
    #[allow(clippy::too_many_arguments)]
    fn execute_recursive(
        &self,
        prefix: &str,
        inherited_tags: &[String],
        inherited_timeout: Option<Duration>,
        inherited_retries: u32,
        inherited_before_each: &[Arc<dyn Fn() + Send + Sync>],
        inherited_after_each: &[Arc<dyn Fn() + Send + Sync>],
        config: &RunnerConfig,
    ) -> Vec<TestCase> {
        let full_name = if prefix.is_empty() {
            self.name.clone()
        } else {
            format!("{} :: {}", prefix, self.name)
        };

        let merged_tags: Vec<String> = inherited_tags
            .iter()
            .cloned()
            .chain(self.tags.iter().cloned())
            .collect();

        let merged_timeout = self.timeout.or(inherited_timeout).or(config.default_timeout);
        let merged_retries = if self.retries > 0 {
            self.retries
        } else {
            let base = inherited_retries.max(config.default_retries);
            if base == 0 && config.auto_retry { 1 } else { base }
        };

        // Build effective before_each / after_each lists.
        // Parent hooks are inherited; self hooks are appended so they
        // run after parent hooks on the way in and before them on the way out.
        let before_each: Vec<Arc<dyn Fn() + Send + Sync>> = inherited_before_each
            .iter()
            .cloned()
            .chain(self.before_each.iter().cloned())
            .collect();
        let after_each: Vec<Arc<dyn Fn() + Send + Sync>> = inherited_after_each
            .iter()
            .cloned()
            .chain(self.after_each.iter().cloned())
            .collect();

        // Skip this entire subtree if nothing matches the filters.
        // Hooks are NOT run when there are no matching tests.
        if !self.has_matching(prefix, inherited_tags, config) {
            return Vec::new();
        }

        let mut results = Vec::new();

        // --- before_all hook ---
        if let Some(ref setup) = self.setup {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| setup()));
        }

        // --- leaf tests ---
        let mut test_entries: Vec<&TestEntry> = self.tests.iter().collect();
        if config.shuffle {
            let mut rng = crate::core::rng_from_seed(config.seed);
            use rand::seq::SliceRandom;
            test_entries.shuffle(&mut rng);
        }

        // Filter tests first
        let mut filtered: Vec<(&TestEntry, String)> = test_entries
            .into_iter()
            .filter_map(|entry| {
                let test_name = format!("{} :: {}", full_name, entry.name);
                if !crate::tag::tags_match(&merged_tags, config)
                    || !crate::tag::name_matches(&test_name, config.filter.as_deref())
                    || crate::tag::name_skipped(&test_name, config.skip.as_deref())
                {
                    None
                } else {
                    Some((entry, test_name))
                }
            })
            .collect();

        // Pre-emptive ordering: run previously-failed tests first for faster feedback
        if !config.shuffle {
            let failed_names = crate::core::load_failed_tests();
            if !failed_names.is_empty() {
                filtered.sort_by(|a, b| {
                    let a_failed = failed_names.iter().any(|n| a.1.contains(n));
                    let b_failed = failed_names.iter().any(|n| b.1.contains(n));
                    b_failed.cmp(&a_failed)
                });
            }
        }

        if config.parallel && filtered.len() > 1 && !config.fail_fast {
            // Parallel execution
            use std::sync::Mutex;
            let results_mutex = &Mutex::new(Vec::new());
            let max_threads = config.max_threads.min(filtered.len());
            let chunk_size = std::cmp::max(1, filtered.len() / max_threads);

            std::thread::scope(|s| {
                for chunk in filtered.chunks(chunk_size) {
                    let (entries, names): (Vec<&TestEntry>, Vec<String>) = chunk.iter().map(|(e, n)| (*e, n.clone())).unzip();
                    let be = before_each.clone();
                    let ae = after_each.clone();
                    let fnm = full_name.clone();
                    let mtags = merged_tags.clone();

                    s.spawn(move || {
                        let mut local = Vec::new();
                        for (idx, entry) in entries.iter().enumerate() {
                            let test_name = &names[idx];
                            let test_start = Instant::now();

                            let mut hook_failed = false;
                            for hook in &be {
                                if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| hook())).is_err() {
                                    hook_failed = true;
                                }
                            }

                            let (status, captured_output) = if hook_failed {
                                (TestStatus::Failed {
                                    reason: "before_each hook failed".to_owned(),
                                    location: None,
                                }, None)
                            } else {
                                execute_with_capture(&entry.test_fn, merged_timeout, merged_retries)
                            };

                            let duration = test_start.elapsed();

                            for hook in ae.iter().rev() {
                                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| hook()));
                            }

                            local.push(TestCase {
                                name: test_name.clone(),
                                suite: Some(fnm.clone()),
                                tags: mtags.clone(),
                                status,
                                duration,
                                assertions: 0,
                                location: entry.location.clone(),
                                parameters: Vec::new(),
                                captured_output,
                                bench_stats: None,
                                bench_threshold: None,
                            });
                        }
                        let mut r = results_mutex.lock().unwrap();
                        r.extend(local);
                    });
                }
            });

            let mut locked = results_mutex.lock().unwrap();
            let taken = std::mem::take(&mut *locked);
            results.extend(taken);
        } else {
            // Sequential execution
            for (entry, test_name) in &filtered {
                let test_start = Instant::now();

                let mut hook_failed = false;
                for hook in &before_each {
                    if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| hook())).is_err() {
                        hook_failed = true;
                    }
                }

                let (status, captured_output) = if hook_failed {
                    (TestStatus::Failed {
                        reason: "before_each hook failed".to_owned(),
                        location: None,
                    }, None)
                } else {
                    execute_with_capture(&entry.test_fn, merged_timeout, merged_retries)
                };

                let duration = test_start.elapsed();
                let is_failed = status.is_failed();

                for hook in after_each.iter().rev() {
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| hook()));
                }

                results.push(TestCase {
                    name: test_name.clone(),
                    suite: Some(full_name.clone()),
                    tags: merged_tags.clone(),
                    status,
                    duration,
                    assertions: 0,
                    location: entry.location.clone(),
                    parameters: Vec::new(),
                    captured_output,
                    bench_stats: None,
                    bench_threshold: None,
                });

                if config.fail_fast && is_failed {
                    break;
                }
            }
        }

        // --- bench entries ---
        let iterations = if self.bench_iterations > 0 { self.bench_iterations } else { config.bench_iterations };

        let mut bench_entries: Vec<&BenchEntry> = self.bench_entries.iter().collect();
        if config.shuffle {
            let mut rng = crate::core::rng_from_seed(config.seed);
            bench_entries.shuffle(&mut rng);
        }

        for entry in &bench_entries {
            let bench_name = format!("{} :: {}", full_name, entry.name);

            if !crate::tag::tags_match(&merged_tags, config)
                || !crate::tag::name_matches(&bench_name, config.filter.as_deref())
                || crate::tag::name_skipped(&bench_name, config.skip.as_deref())
            {
                continue;
            }

            let threshold = entry.threshold.or(config.bench_threshold);
            let iters = iterations;
            let test_start = Instant::now();
            let (status, stats) = run_benchmark(&entry.bench_fn, iters, threshold);
            let duration = test_start.elapsed();
            let is_failed = status.is_failed();

            results.push(TestCase {
                name: bench_name,
                suite: Some(full_name.clone()),
                tags: merged_tags.clone(),
                status,
                duration,
                assertions: 0,
                location: entry.location.clone(),
                parameters: Vec::new(),
                captured_output: None,
                bench_stats: Some(stats),
                bench_threshold: threshold,
            });

            if config.fail_fast && is_failed {
                break;
            }
        }

        // --- children ---
        let had_failures = results.iter().any(|t| t.status.is_failed());
        if !config.fail_fast || !had_failures {
            let mut child_specs: Vec<&Spec> = self.children.iter().collect();
            if config.shuffle {
                let mut rng = crate::core::rng_from_seed(config.seed);
                child_specs.shuffle(&mut rng);
            }
            for child in &child_specs {
                let child_results = child.execute_recursive(
                    &full_name,
                    &merged_tags,
                    merged_timeout,
                    merged_retries,
                    &before_each,
                    &after_each,
                    config,
                );
                let child_failed = child_results.iter().any(|t| t.status.is_failed());
                results.extend(child_results);
                if config.fail_fast && child_failed {
                    break;
                }
            }
        }

        // --- after_all hook ---
        if let Some(ref teardown) = self.teardown {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| teardown()));
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn describe_creates_spec() {
        let s = describe("test");
        assert_eq!(s.name, "test");
        assert!(s.tests.is_empty());
        assert!(s.children.is_empty());
    }

    #[test]
    fn spec_description() {
        let s = Spec::new("math").description("arithmetic operations");
        assert_eq!(s.description, Some("arithmetic operations".to_owned()));
    }

    #[test]
    fn spec_tag_adds_tag() {
        let s = Spec::new("x").tag("smoke").tag("fast");
        assert_eq!(s.tags, vec!["smoke", "fast"]);
    }

    #[test]
    fn spec_timeout() {
        let s = Spec::new("x").timeout(Duration::from_secs(5));
        assert_eq!(s.timeout, Some(Duration::from_secs(5)));
    }

    #[test]
    fn spec_retries() {
        let s = Spec::new("x").retries(3);
        assert_eq!(s.retries, 3);
    }

    #[test]
    fn spec_before_all() {
        let s = Spec::new("x").before_all(|| {});
        assert!(s.setup.is_some());
    }

    #[test]
    fn spec_after_all() {
        let s = Spec::new("x").after_all(|| {});
        assert!(s.teardown.is_some());
    }

    #[test]
    fn spec_before_each() {
        let s = Spec::new("x").before_each(|| {});
        assert!(s.before_each.is_some());
    }

    #[test]
    fn spec_after_each() {
        let s = Spec::new("x").after_each(|| {});
        assert!(s.after_each.is_some());
    }

    #[test]
    fn spec_new_defaults() {
        let s = Spec::new("empty");
        assert_eq!(s.name, "empty");
        assert!(s.description.is_none());
        assert!(s.tags.is_empty());
        assert!(s.setup.is_none());
        assert!(s.teardown.is_none());
        assert!(s.before_each.is_none());
        assert!(s.after_each.is_none());
        assert!(s.children.is_empty());
        assert!(s.tests.is_empty());
        assert!(s.timeout.is_none());
        assert_eq!(s.retries, 0);
    }

    #[test]
    fn spec_run_passes() {
        let suite = Spec::new("pass")
            .it("works", || {})
            .run();
        assert_eq!(suite.tests.len(), 1);
        assert!(suite.tests[0].status.is_passed());
    }

    #[test]
    fn spec_run_with_config() {
        let config = RunnerConfig { default_timeout: Some(Duration::from_secs(10)), ..RunnerConfig::default() };
        let suite = Spec::new("cfg")
            .it("ok", || {})
            .run_with_config(&config);
        assert!(suite.success());
    }

    #[test]
    fn spec_run_empty() {
        let suite = Spec::new("empty").run();
        assert!(suite.tests.is_empty());
        assert!(suite.success());
    }

    #[test]
    fn spec_builder_methods() {
        let suite = describe("root")
            .describe("child")
                .description("a child spec")
                .tag("nested")
                .timeout(Duration::from_secs(3))
                .retries(1)
                .before_all(|| {})
                .after_all(|| {})
                .before_each(|| {})
                .after_each(|| {})
                .it("leaf", || {})
            .run();
        assert_eq!(suite.tests.len(), 1);
    }

    #[test]
    fn run_with_timeout_integration() {
        let suite = Spec::new("timeout")
            .it("fast", || {})
            .timeout(Duration::from_secs(5))
            .run();
        assert!(suite.success());
    }

    #[test]
    fn has_matching_with_filter() {
        let spec = describe("Parent")
            .tag("smoke")
            .it("child_test", || {});

        let yes = spec.has_matching("", &[], &RunnerConfig { filter: Some("child".into()), ..RunnerConfig::default() });
        assert!(yes);

        let no = spec.has_matching("", &[], &RunnerConfig { filter: Some("nonexistent".into()), ..RunnerConfig::default() });
        assert!(!no);
    }

    #[test]
    fn spec_collects_hooks_inherited() {
        let ran = Arc::new(std::sync::Mutex::new(Vec::new()));
        let r = Arc::clone(&ran);
        let spec = describe("root")
            .before_each(move || r.lock().unwrap().push("root"))
            .describe("child")
                .it("test", move || {
                    ran.lock().unwrap().push("test");
                });
        let suite = spec.run();
        assert_eq!(suite.tests.len(), 1);
    }

    #[test]
    fn spec_run_with_empty_children() {
        let suite = describe("root")
            .describe("empty_child")
            .run();
        assert!(suite.success());
        assert!(suite.tests.is_empty());
    }

    #[test]
    fn spec_describe_chaining() {
        let suite = describe("root")
            .describe("a")
                .tag("t1")
                .it("a1", || {})
            .describe("b")
                .tag("t2")
                .it("b1", || {})
            .run();
        assert_eq!(suite.tests.len(), 2);
    }

    #[test]
    fn spec_tag_on_child() {
        let suite = describe("root")
            .describe("child")
                .tag("exclude_me")
                .it("test", || {})
            .run_with_config(&RunnerConfig {
                exclude_tags: vec!["exclude_me".into()],
                ..RunnerConfig::default()
            });
        assert_eq!(suite.tests.len(), 0);
    }

    #[test]
    fn extract_panic_message_called() {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            panic!("test panic");
        }));
        let e = result.unwrap_err();
        let msg = extract_panic_message(&e);
        assert_eq!(msg, "test panic");
    }

    #[test]
    fn spec_builder_description() {
        let suite = describe("root")
            .describe("child")
                .description("a child spec")
            .run();
        let _ = suite;
    }

    #[test]
    fn spec_builder_all_methods() {
        let suite = describe("root")
            .describe("child")
                .tag("smoke")
                .timeout(Duration::from_secs(3))
                .retries(2)
                .before_all(|| {})
                .after_all(|| {})
                .before_each(|| {})
                .after_each(|| {})
                .it("test", || {})
            .run();
        assert_eq!(suite.tests.len(), 1);
        assert!(suite.success());
    }

    #[test]
    fn spec_builder_nested_describe() {
        let suite = describe("root")
            .describe("level1")
                .describe("level2")
                    .it("deep", || {})
            .run();
        assert_eq!(suite.tests.len(), 1);
        assert_eq!(suite.tests[0].name, "root :: level1 :: level2 :: deep");
    }
}
