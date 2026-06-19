//! Code coverage collection.
//!
//! Collects LLVM instrumentation coverage data using multiple backends:
//! self-contained pure-Rust `.profraw` parser, `cargo-llvm-cov`, or
//! `llvm-tools`.
//!
//! # Submodules
//!
//! - [`backends`] — Coverage backend runners (self-contained, cargo-llvm-cov, llvm-tools)
//! - [`util`] — Helper functions (which, glob_dir, parse_coverage)

use std::path::PathBuf;

use crate::core::{CoverageFormat, CoverageReport};

pub mod backends;
pub(crate) mod util;

pub use util::{
    check_threshold, extract_pct, find_tool, glob_dir, has_addr2line, has_cargo_llvm_cov,
    has_llvm_tools, parse_coverage_percentages, parse_llvm_cov_summary, run_cargo_test_no_run,
    self_contained_profraw, which,
};

/// Configuration for a coverage collection run.
#[derive(Debug, Clone)]
pub struct CoverageConfig {
    /// Whether coverage is enabled at all.
    pub enabled: bool,
    /// Desired output format.
    pub format: CoverageFormat,
    /// Directory to place coverage artifacts.
    pub output_dir: PathBuf,
    /// Minimum acceptable line-coverage percentage. `collect()` returns an
    /// error if coverage falls below this threshold.
    pub min_threshold: Option<f64>,
    /// Open the report in the system browser after generation.
    pub open_report: bool,
    /// Any extra CLI arguments to forward to the test runner.
    pub extra_test_args: Vec<String>,
    /// Sampling interval in milliseconds for the built-in sampler.
    pub sample_interval_ms: u64,
}

impl Default for CoverageConfig {
    fn default() -> Self {
        CoverageConfig {
            enabled: false,
            format: CoverageFormat::Summary,
            output_dir: PathBuf::from("target/coverage"),
            min_threshold: None,
            open_report: false,
            extra_test_args: Vec::new(),
            sample_interval_ms: 5,
        }
    }
}

/// Collects code coverage data.
///
/// Tries three strategies in order:
///
/// 1. **cargo-llvm-cov** — best results, requires `cargo install cargo-llvm-cov`.
/// 2. **Manual llvm-tools** — uses `-Cinstrument-coverage`, `llvm-profdata`,
///    `llvm-cov`. Requires `rustup component add llvm-tools-preview`.
/// 3. **Built-in sampler** — lightweight statistical sampling via `ptrace`
///    + `addr2line`. Works without any LLVM tools.
pub struct CoverageCollector {
    config: CoverageConfig,
}

impl CoverageCollector {
    /// Create a new collector with the given configuration.
    pub fn new(config: CoverageConfig) -> Self {
        CoverageCollector { config }
    }

    /// Run coverage collection, trying backends in order of quality.
    ///
    /// 1. `cargo-llvm-cov` (best output)
    /// 2. Manual `llvm-tools` (llvm-profdata + llvm-cov)
    /// 3. Self-contained pure-Rust `.profraw` parser
    /// 4. Built-in sampler (Linux only, ptrace + addr2line)
    pub fn collect(&self) -> Result<CoverageReport, String> {
        if util::has_cargo_llvm_cov() {
            return backends::run_cargo_llvm_cov(&self.config);
        }
        if util::has_llvm_tools() {
            return backends::run_llvm_tools(&self.config);
        }
        // Lightweight profraw parser (pure-Rust, no external tools).
        if util::self_contained_profraw() {
            return backends::run_raw_parser(&self.config);
        }
        // Fallback: built-in sampler.
        backends::run_sampler(&self.config)
    }

    #[allow(dead_code)]
    fn check_threshold_and_open(&self, report: CoverageReport) -> Result<CoverageReport, String> {
        util::check_threshold(&self.config, report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_coverage_all_fields() {
        let summary = "Lines:   45.6%\nFunctions:  60.0%\nRegions:   45.6%\n";
        let (l, f, r) = parse_coverage_percentages(summary);
        assert!((l - 45.6).abs() < 0.01);
        assert!((f - 60.0).abs() < 0.01);
        assert!((r - 45.6).abs() < 0.01);
    }

    #[test]
    fn parse_coverage_partial() {
        let summary = "Lines:   100.0%\n";
        let (l, f, r) = parse_coverage_percentages(summary);
        assert!((l - 100.0).abs() < 0.01);
        assert_eq!(f, 0.0);
        assert_eq!(r, 0.0);
    }

    #[test]
    fn parse_coverage_empty() {
        assert_eq!(parse_coverage_percentages(""), (0.0, 0.0, 0.0));
    }

    #[test]
    fn parse_coverage_indented() {
        let summary = "  Lines:   33.3%\n  Functions:  50.0%\n";
        let (l, f, _) = parse_coverage_percentages(summary);
        assert!((l - 33.3).abs() < 0.01);
        assert!((f - 50.0).abs() < 0.01);
    }

    #[test]
    fn extract_pct_normal() {
        assert!((extract_pct("Lines:   75.5%") - 75.5).abs() < 0.01);
    }

    #[test]
    fn extract_pct_no_percent() {
        assert_eq!(extract_pct("no percentage here"), 0.0);
    }

    #[test]
    fn extract_pct_only_number() {
        assert!((extract_pct("42%") - 42.0).abs() < 0.01);
    }

    #[test]
    fn extract_pct_no_digits() {
        assert_eq!(extract_pct("No digits here %"), 0.0);
    }

    #[test]
    fn which_returns_none_for_nonexistent() {
        let result = which("this_tool_definitely_does_not_exist_xyz");
        assert!(result.is_none());
    }

    #[test]
    fn coverage_config_default() {
        let cfg = CoverageConfig::default();
        assert!(!cfg.enabled);
        assert_eq!(cfg.format, CoverageFormat::Summary);
        assert_eq!(cfg.output_dir, PathBuf::from("target/coverage"));
        assert!(cfg.min_threshold.is_none());
        assert!(!cfg.open_report);
        assert!(cfg.extra_test_args.is_empty());
    }

    #[test]
    fn coverage_config_custom() {
        let cfg = CoverageConfig {
            enabled: true,
            format: CoverageFormat::Html,
            output_dir: PathBuf::from("custom"),
            min_threshold: Some(80.0),
            open_report: true,
            extra_test_args: vec!["--feature".into()],
            sample_interval_ms: 10,
        };
        assert!(cfg.enabled);
        assert_eq!(cfg.format, CoverageFormat::Html);
        assert_eq!(cfg.min_threshold, Some(80.0));
    }

    #[test]
    fn coverage_report_struct() {
        let report = CoverageReport {
            line_coverage: 50.0,
            function_coverage: 60.0,
            region_coverage: 50.0,
            format: CoverageFormat::Summary,
            report_path: None,
        };
        assert!((report.line_coverage - 50.0).abs() < 0.01);
        assert!(report.report_path.is_none());
    }

    #[test]
    fn coverage_collector_new_default() {
        let cfg = CoverageConfig::default();
        let collector = CoverageCollector::new(cfg);
        assert!(!collector.config.enabled);
    }

    #[test]
    fn coverage_config_sample_interval() {
        let cfg = CoverageConfig {
            sample_interval_ms: 100,
            ..CoverageConfig::default()
        };
        assert_eq!(cfg.sample_interval_ms, 100);
    }

    #[test]
    fn which_finds_sh() {
        let result = which("sh");
        assert!(result.is_some(), "sh should be in PATH");
        let path = result.unwrap();
        assert!(path.exists());
    }

    #[test]
    fn which_checks_absolute_paths() {
        let result = which("/bin/sh");
        if let Some(path) = result {
            assert!(path.exists());
        }
    }

    #[test]
    fn glob_dir_finds_profraw() {
        let tmp = std::env::temp_dir().join("rvtest_glob_test");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        std::fs::write(tmp.join("test.profraw"), "").unwrap();
        std::fs::write(tmp.join("other.txt"), "").unwrap();

        let files = glob_dir(&tmp, "profraw").unwrap();
        assert_eq!(files.len(), 1, "should find one .profraw file, got {files:?}");
        assert!(
            files[0].to_string_lossy().contains("test.profraw"),
            "should find test.profraw"
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn glob_dir_empty_dir() {
        let tmp = std::env::temp_dir().join("rvtest_glob_empty");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let files = glob_dir(&tmp, "*.profraw").unwrap();
        assert!(files.is_empty());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn glob_dir_nonexistent_dir() {
        let tmp = std::env::temp_dir().join("rvtest_glob_nonexistent");
        let _ = std::fs::remove_dir_all(&tmp);
        let result = glob_dir(&tmp, "*");
        assert!(result.is_err());
    }

    #[test]
    fn extract_pct_zero() {
        assert!((extract_pct("0%") - 0.0).abs() < 0.01);
    }

    #[test]
    fn extract_pct_trailing_decimal() {
        assert!((extract_pct("50.%") - 50.0).abs() < 0.01, "50.% should parse as 50.0");
    }

    #[test]
    fn extract_pct_negative() {
        assert!((extract_pct("-10.5%") - 10.5).abs() < 0.01, "should parse 10.5 from -10.5%");
    }

    #[test]
    fn parse_coverage_nonstandard_order() {
        let summary = "Regions:   30.0%\nLines:   50.0%\nFunctions:   70.0%\n";
        let (l, f, r) = parse_coverage_percentages(summary);
        assert!((l - 50.0).abs() < 0.01);
        assert!((f - 70.0).abs() < 0.01);
        assert!((r - 30.0).abs() < 0.01);
    }

    #[test]
    fn check_threshold_above_min() {
        let cfg = CoverageConfig {
            min_threshold: Some(50.0),
            ..CoverageConfig::default()
        };
        let collector = CoverageCollector::new(cfg);
        let report = CoverageReport {
            line_coverage: 80.0,
            function_coverage: 90.0,
            region_coverage: 80.0,
            format: CoverageFormat::Summary,
            report_path: None,
        };
        let result = collector.check_threshold_and_open(report);
        assert!(result.is_ok(), "above threshold should pass");
    }

    #[test]
    fn check_threshold_below_min() {
        let cfg = CoverageConfig {
            min_threshold: Some(50.0),
            ..CoverageConfig::default()
        };
        let collector = CoverageCollector::new(cfg);
        let report = CoverageReport {
            line_coverage: 30.0,
            function_coverage: 40.0,
            region_coverage: 30.0,
            format: CoverageFormat::Summary,
            report_path: None,
        };
        let result = collector.check_threshold_and_open(report);
        assert!(result.is_err(), "below threshold should fail");
    }

    #[test]
    fn check_threshold_no_min() {
        let cfg = CoverageConfig {
            min_threshold: None,
            ..CoverageConfig::default()
        };
        let collector = CoverageCollector::new(cfg);
        let report = CoverageReport {
            line_coverage: 10.0,
            function_coverage: 10.0,
            region_coverage: 10.0,
            format: CoverageFormat::Summary,
            report_path: None,
        };
        let result = collector.check_threshold_and_open(report);
        assert!(result.is_ok(), "no threshold should always pass");
    }

    #[test]
    fn which_empty_path() {
        let original = std::env::var_os("PATH");
        unsafe { std::env::set_var("PATH", ""); }
        let result = which("anything");
        assert!(result.is_none());
        if let Some(p) = original {
            unsafe { std::env::set_var("PATH", p); }
        }
    }
}
