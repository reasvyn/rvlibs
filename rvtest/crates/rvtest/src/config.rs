//! Configuration file parsing for `rvtest.toml`.
//!
//! This module is purely a parser — it does NOT auto-discover or auto-load
//! config files.  Use [`RunnerConfig::with_config_file`] to opt in.
//!
//! # Example TOML
//!
//! ```toml
//! filter = "auth"
//! format = "compact"
//! verbose = true
//! fail_fast = true
//! ```

use serde::Deserialize;

use crate::core::RunnerConfig;

/// Mirrors the fields of [`RunnerConfig`] for TOML deserialisation.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct FileConfig {
    /// Filter test names by substring.
    pub filter: Option<String>,
    /// Only run tests carrying all of these tags.
    pub include_tags: Vec<String>,
    /// Skip tests carrying any of these tags.
    pub exclude_tags: Vec<String>,
    /// Skip tests whose name matches this pattern.
    pub skip: Option<String>,
    /// Number of retries for flaky tests.
    pub retries: Option<u32>,
    /// Automatically retry failed tests once.
    pub auto_retry: Option<bool>,
    /// Default per-test timeout in seconds.
    pub timeout_secs: Option<f64>,
    /// Disable parallel execution.
    pub no_parallel: Option<bool>,
    /// Maximum number of threads for parallel execution.
    pub max_threads: Option<usize>,
    /// Output format (pretty, tap, junit, json, compact, github).
    pub format: Option<String>,
    /// Stop after the first failure.
    pub fail_fast: Option<bool>,
    /// Seed for randomised features.
    pub seed: Option<u64>,
    /// Show verbose output.
    pub verbose: Option<bool>,
    /// Show captured stdout/stderr.
    pub show_output: Option<bool>,
    /// Randomise test execution order.
    pub shuffle: Option<bool>,
    /// Colour output preference.
    pub color: Option<String>,
    /// Run each test in a separate OS process for full isolation.
    pub isolate: Option<bool>,
    /// Mask secrets (API keys, tokens, passwords) in captured test output.
    pub mask_secrets: Option<bool>,
}

impl FileConfig {
    /// Apply non-`None` settings from this file config into a [`RunnerConfig`].
    pub fn apply_to(&self, runner: &mut RunnerConfig) {
        if let Some(ref f) = self.filter {
            runner.filter = Some(f.clone());
        }
        if !self.include_tags.is_empty() {
            runner.include_tags = self.include_tags.clone();
        }
        if !self.exclude_tags.is_empty() {
            runner.exclude_tags = self.exclude_tags.clone();
        }
        if let Some(ref s) = self.skip {
            runner.skip = Some(s.clone());
        }
        if let Some(r) = self.retries {
            runner.default_retries = r;
        }
        if let Some(a) = self.auto_retry {
            runner.auto_retry = a;
        }
        if let Some(t) = self.timeout_secs {
            runner.default_timeout = Some(std::time::Duration::from_secs_f64(t));
        }
        if let Some(v) = self.no_parallel {
            runner.parallel = !v;
        }
        if let Some(m) = self.max_threads {
            runner.max_threads = m;
        }
        if let Some(ref f) = self.format
            && let Ok(fmt) = f.parse() {
                runner.format = fmt;
            }
        if let Some(v) = self.fail_fast {
            runner.fail_fast = v;
        }
        if let Some(s) = self.seed {
            runner.seed = Some(s);
        }
        if let Some(v) = self.verbose {
            runner.verbose = v;
        }
        if let Some(v) = self.show_output {
            runner.output_capture = v;
        }
        if let Some(v) = self.shuffle {
            runner.shuffle = v;
        }
        if let Some(ref c) = self.color
            && let Ok(color) = c.parse() {
                runner.color = color;
            }
        if let Some(v) = self.isolate {
            runner.process_isolation = v;
        }
        if let Some(v) = self.mask_secrets {
            runner.mask_secrets = v;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_toml() {
        let cfg: FileConfig = toml::from_str("").unwrap();
        assert!(cfg.filter.is_none());
        assert!(cfg.include_tags.is_empty());
    }

    #[test]
    fn parse_basic_config() {
        let toml = r#"
filter = "auth"
format = "compact"
verbose = true
fail_fast = true
"#;
        let cfg: FileConfig = toml::from_str(toml).unwrap();
        assert_eq!(cfg.filter.as_deref(), Some("auth"));
        assert_eq!(cfg.verbose, Some(true));
        assert_eq!(cfg.fail_fast, Some(true));
    }

    #[test]
    fn apply_to_runner() {
        let toml = r#"
filter = "api"
verbose = true
retries = 2
"#;
        let cfg: FileConfig = toml::from_str(toml).unwrap();
        let mut runner = RunnerConfig::default();
        cfg.apply_to(&mut runner);
        assert_eq!(runner.filter.as_deref(), Some("api"));
        assert!(runner.verbose);
        assert_eq!(runner.default_retries, 2);
    }

    #[test]
    fn apply_to_runner_empty_does_nothing() {
        let cfg: FileConfig = toml::from_str("").unwrap();
        let mut runner = RunnerConfig::default();
        let original = runner.clone();
        cfg.apply_to(&mut runner);
        assert_eq!(runner.verbose, original.verbose);
    }
}
