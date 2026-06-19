use std::sync::Arc;
use std::time::Duration;

use crate::core::{RunnerConfig, SourceLocation, TestSuite};
use crate::spec::{BenchEntry, Spec, TestEntry};

/// A builder that lets you chain `.describe()` calls on a parent spec.
///
/// Created by [`Spec::describe`], this wrapper holds a reference to the
/// parent and allows you to chain configuration on the child before
/// returning to the parent.
pub struct SpecBuilder {
    parent: Spec,
    /// Path of indices from `parent` to the current child.
    path: Vec<usize>,
}

impl SpecBuilder {
    fn child_mut(&mut self) -> &mut Spec {
        let mut current = &mut self.parent;
        for &idx in &self.path {
            current = &mut current.children[idx];
        }
        current
    }

    pub(crate) fn new(parent: Spec, path: Vec<usize>) -> Self {
        SpecBuilder { parent, path }
    }

    /// Attach a description to the child spec.
    pub fn description(mut self, text: &str) -> Self {
        self.child_mut().description = Some(text.to_owned());
        self
    }

    /// Add a tag to the child spec.
    pub fn tag(mut self, tag: &str) -> Self {
        self.child_mut().tags.push(tag.to_owned());
        self
    }

    /// Set a timeout on the child spec.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.child_mut().timeout = Some(duration);
        self
    }

    /// Set retries on the child spec.
    pub fn retries(mut self, count: u32) -> Self {
        self.child_mut().retries = count;
        self
    }

    /// Register a setup hook on the child spec.
    pub fn before_all(mut self, hook: impl Fn() + Send + Sync + 'static) -> Self {
        self.child_mut().setup = Some(Arc::new(hook));
        self
    }

    /// Register a teardown hook on the child spec.
    pub fn after_all(mut self, hook: impl Fn() + Send + Sync + 'static) -> Self {
        self.child_mut().teardown = Some(Arc::new(hook));
        self
    }

    /// Register a hook run before each test in the child spec (and its children).
    pub fn before_each(mut self, hook: impl Fn() + Send + Sync + 'static) -> Self {
        self.child_mut().before_each = Some(Arc::new(hook));
        self
    }

    /// Register a hook run after each test in the child spec (and its children).
    pub fn after_each(mut self, hook: impl Fn() + Send + Sync + 'static) -> Self {
        self.child_mut().after_each = Some(Arc::new(hook));
        self
    }

    /// Add a leaf test to the child spec.
    #[track_caller]
    pub fn it(mut self, name: &str, test: impl Fn() + Send + Sync + 'static) -> Self {
        let loc = std::panic::Location::caller();
        self.child_mut().tests.push(TestEntry {
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

    /// Register a benchmark on the child spec.
    #[track_caller]
    pub fn bench(mut self, name: &str, test: impl Fn() + Send + Sync + 'static) -> Self {
        let loc = std::panic::Location::caller();
        self.child_mut().bench_entries.push(BenchEntry {
            name: name.to_owned(),
            location: Some(SourceLocation {
                file: loc.file().to_owned(),
                line: loc.line(),
                column: None,
            }),
            bench_fn: std::sync::Arc::new(test),
            threshold: None,
        });
        self
    }

    /// Set the number of benchmark iterations on the child spec.
    pub fn bench_iterations(mut self, n: u32) -> Self {
        self.child_mut().bench_iterations = n;
        self
    }

    /// Set a default benchmark regression threshold on the child spec.
    pub fn bench_threshold(mut self, dur: std::time::Duration) -> Self {
        let child = self.child_mut();
        for entry in &mut child.bench_entries {
            if entry.threshold.is_none() {
                entry.threshold = Some(dur);
            }
        }
        self
    }

    /// Nest a deeper spec inside the child.
    pub fn describe(mut self, name: &str) -> SpecBuilder {
        let child = Spec::new(name);
        self.child_mut().children.push(child);
        let child_index = self.child_mut().children.len() - 1;
        let mut path = self.path;
        path.push(child_index);
        SpecBuilder { parent: self.parent, path }
    }

    /// Run all tests starting from the parent spec.
    pub fn run(self) -> TestSuite {
        self.parent.run()
    }

    /// Run all tests with an explicit config.
    pub fn run_with_config(self, config: &RunnerConfig) -> TestSuite {
        self.parent.run_with_config(config)
    }
}
