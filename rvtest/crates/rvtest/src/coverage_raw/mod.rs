//! Pure-Rust `.profraw` parser for self-contained code coverage.
//!
//! Parses LLVM coverage instrumentation output (`*.profraw` files)
//! entirely in Rust — no `llvm-profdata` or `llvm-cov` required.
//!
//! # Submodules
//!
//! - [`parser`] — Binary `.profraw` format parser
//! - [`types`] — Coverage data structures (CoverageTotals, FunctionCoverage)
//! - [`report`] — Report generation and test-binary discovery

mod parser;
mod report;
mod types;

pub use report::{
    compute_coverage_from_profraw, parse_test_binaries, report_filename,
    RawCoverageRunner, write_report,
};

#[cfg(test)]
mod tests {
    use super::parser::parse_raw_profile;
    use super::types::CoverageTotals;
    use super::*;
    use crate::core::CoverageFormat;

    #[test]
    fn test_coverage_totals_empty() {
        let t = CoverageTotals::new();
        assert_eq!(t.line_pct(), 0.0);
        assert_eq!(t.func_pct(), 0.0);
    }

    #[test]
    fn test_coverage_totals_aggregation() {
        let mut t = CoverageTotals::new();
        t.add(&CoverageTotals { total_counters: 100, covered_counters: 80, total_functions: 10, covered_functions: 8 });
        t.add(&CoverageTotals { total_counters: 200, covered_counters: 150, total_functions: 20, covered_functions: 15 });
        assert!((t.line_pct() - 76.666).abs() < 0.01);
        assert!((t.func_pct() - 76.666).abs() < 0.01);
    }

    #[test]
    fn test_coverage_totals_always_within_100() {
        let mut t = CoverageTotals::new();
        t.add(&CoverageTotals { total_counters: 10, covered_counters: 20, total_functions: 10, covered_functions: 10 });
        assert_eq!(t.line_pct(), 100.0);
        assert_eq!(t.func_pct(), 100.0);
    }

    #[test]
    fn test_report_filename() {
        assert_eq!(report_filename(CoverageFormat::Summary), "summary.txt");
        assert_eq!(report_filename(CoverageFormat::Html), "index.html");
        assert_eq!(report_filename(CoverageFormat::Lcov), "lcov.info");
        assert_eq!(report_filename(CoverageFormat::Json), "coverage.json");
        assert_eq!(report_filename(CoverageFormat::Cobertura), "cobertura.xml");
    }

    #[test]
    fn test_parse_test_binaries_empty() {
        let bins = parse_test_binaries(b"");
        assert!(bins.is_empty());
    }

    #[test]
    fn test_parse_test_binaries_non_json() {
        let bins = parse_test_binaries(b"not json\nat all");
        assert!(bins.is_empty());
    }

    #[test]
    fn test_parse_test_binaries_ignores_non_artifact() {
        let input = br#"{"reason":"compiler-artifact","filenames":["/tmp/test_bin"],"target_kind":["bin"],"profile":{"test":true}}"#;
        let bins = parse_test_binaries(input);
        assert!(bins.is_empty());
    }

    #[test]
    fn test_parse_test_binaries_filters_library_artifacts() {
        let input = br#"{"reason":"compiler-artifact","filenames":["/tmp/lib.rlib"],"target_kind":["lib"],"profile":{"test":false}}"#;
        let bins = parse_test_binaries(input);
        assert!(bins.is_empty(), "should filter non-test artifacts");
    }

    #[test]
    fn test_write_report_summary_does_nothing() {
        let dir = std::env::temp_dir().join("rvtest_cov_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("summary.txt");

        let result = write_report(CoverageFormat::Summary, 50.0, 60.0, 50.0, &path);
        assert!(result.is_ok());
        assert!(!path.exists(), "summary should not write a file");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_write_report_json_writes_file() {
        let dir = std::env::temp_dir().join("rvtest_cov_test_json");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("coverage.json");

        let result = write_report(CoverageFormat::Json, 75.0, 80.0, 75.0, &path);
        assert!(result.is_ok());
        assert!(path.exists());

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("75.0"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_parse_raw_profile_bad_magic() {
        let data = vec![0u8; 256];
        let result = parse_raw_profile(&data);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.contains("magic"), "error should mention magic, got: {e}");
        }
    }

    #[test]
    fn test_parse_raw_profile_too_small() {
        let result = parse_raw_profile(&[0u8; 10]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_raw_profile_valid_empty() {
        let magic = 0xff6c70726f667281u64.to_le_bytes();
        let version = 10u64.to_le_bytes();
        let zeros: [u8; 112] = [0; 112];

        let mut data = Vec::new();
        data.extend_from_slice(&magic);
        data.extend_from_slice(&version);
        data.extend_from_slice(&zeros);

        let result = parse_raw_profile(&data);
        assert!(result.is_ok(), "valid empty profraw should parse: {:?}", result.err());
        let profile = result.unwrap();
        assert_eq!(profile.num_data, 0);
        assert_eq!(profile.num_counters, 0);
        assert!(profile.functions.is_empty());
    }

    #[test]
    fn test_write_report_html() {
        let dir = std::env::temp_dir().join("rvtest_cov_html");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("index.html");

        let result = write_report(CoverageFormat::Html, 50.0, 60.0, 50.0, &path);
        assert!(result.is_ok());
        assert!(path.exists());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_write_report_lcov() {
        let dir = std::env::temp_dir().join("rvtest_cov_lcov");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("lcov.info");

        let result = write_report(CoverageFormat::Lcov, 70.0, 80.0, 70.0, &path);
        assert!(result.is_ok());
        assert!(path.exists());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_coverage_format_display() {
        assert_eq!(format!("{}", CoverageFormat::Summary), "summary");
        assert_eq!(format!("{}", CoverageFormat::Html), "html");
        assert_eq!(format!("{}", CoverageFormat::Lcov), "lcov");
        assert_eq!(format!("{}", CoverageFormat::Json), "json");
        assert_eq!(format!("{}", CoverageFormat::Cobertura), "cobertura");
    }

    #[test]
    fn test_parse_test_binaries_ignores_wrong_reason() {
        let input = br#"{"reason":"build-script-executed","filenames":[],"target_kind":[]}"#;
        let bins = parse_test_binaries(input);
        assert!(bins.is_empty());
    }

    #[test]
    fn test_parse_test_binaries_doc_test_filtered() {
        let input = br#"{"reason":"compiler-artifact","filenames":["/tmp/rvtest-abc123"],"target_kind":["test"],"profile":{"test":true}}"#;
        let bins = parse_test_binaries(input);
        assert!(bins.is_empty(), "doc-test only binaries should be filtered");
    }

    #[test]
    fn test_parse_test_binaries_integration_not_filtered() {
        let input = br#"{"reason":"compiler-artifact","filenames":["/tmp/integration-abc123"],"target_kind":["test"],"profile":{"test":true}}"#;
        let bins = parse_test_binaries(input);
        assert!(bins.is_empty(), "should be empty (file doesn't exist), not filtered as doc-test");
    }

    #[test]
    fn test_parse_test_binaries_no_profile_falls_back_to_target_kind() {
        let input = br#"{"reason":"compiler-artifact","filenames":["/tmp/nonexistent_bin"],"target_kind":["bin"]}"#;
        let bins = parse_test_binaries(input);
        assert!(bins.is_empty(), "file doesn't exist but should pass the test binary check");
    }

    #[test]
    fn test_parse_test_binaries_no_profile_not_bin_or_test() {
        let input = br#"{"reason":"compiler-artifact","filenames":["/tmp/lib.rlib"],"target_kind":["lib"]}"#;
        let bins = parse_test_binaries(input);
        assert!(bins.is_empty(), "lib without test profile should be filtered");
    }

    #[test]
    fn test_parse_test_binaries_empty_line_skipped() {
        let input = b"\n\n";
        let bins = parse_test_binaries(input);
        assert!(bins.is_empty());
    }

    fn build_profraw(num_data: u64, num_counters: u64, counter_values: &[u64]) -> Vec<u8> {
        let mut data = Vec::new();

        data.extend_from_slice(&0xff6c70726f667281u64.to_le_bytes());
        data.extend_from_slice(&10u64.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes());
        data.extend_from_slice(&num_data.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes());
        data.extend_from_slice(&num_counters.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes());

        for _ in 0..num_data {
            let n_counters = (num_counters / num_data.max(1)) as u32;
            data.extend_from_slice(&0u64.to_le_bytes());
            data.extend_from_slice(&0u64.to_le_bytes());
            data.extend_from_slice(&0u64.to_le_bytes());
            data.extend_from_slice(&0u64.to_le_bytes());
            data.extend_from_slice(&0u64.to_le_bytes());
            data.extend_from_slice(&0u64.to_le_bytes());
            data.extend_from_slice(&n_counters.to_le_bytes());
            data.extend_from_slice(&0u16.to_le_bytes());
            data.extend_from_slice(&0u16.to_le_bytes());
            data.extend_from_slice(&0u16.to_le_bytes());
            data.extend_from_slice(&0u16.to_le_bytes());
            data.extend_from_slice(&0u32.to_le_bytes());
        }

        for val in counter_values {
            data.extend_from_slice(&val.to_le_bytes());
        }

        data
    }

    #[test]
    fn test_parse_raw_profile_version_error() {
        let mut data = build_profraw(0, 0, &[]);
        data[8..16].copy_from_slice(&99u64.to_le_bytes());
        let result = parse_raw_profile(&data);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.contains("version"), "error should mention version, got: {e}");
        }
    }

    #[test]
    fn test_parse_raw_profile_data_overflow() {
        let counters: [u64; 0] = [];
        let mut data = build_profraw(5, 0, &counters);
        data.truncate(128);
        let result = parse_raw_profile(&data);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.contains("data records"), "error should mention data records, got: {e}");
        }
    }

    #[test]
    fn test_parse_raw_profile_counters_overflow() {
        let counters: [u64; 2] = [1, 2];
        let mut data = build_profraw(1, 100, &counters);
        data.truncate(128 + 64);
        let result = parse_raw_profile(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_raw_profile_with_one_function() {
        let counters = [0u64, 5, 10];
        let data = build_profraw(1, 3, &counters);

        let result = parse_raw_profile(&data);
        assert!(result.is_ok(), "should parse valid profraw: {:?}", result.err());
        let profile = result.unwrap();
        assert_eq!(profile.num_data, 1);
        assert_eq!(profile.num_counters, 3);
        assert_eq!(profile.functions.len(), 1);
        assert_eq!(profile.functions[0].num_counters, 3);
        assert_eq!(profile.functions[0].counters, vec![0, 5, 10]);
        assert_eq!(profile.functions[0].covered, 2);
    }

    #[test]
    fn test_parse_raw_profile_with_two_functions() {
        let counters = [1u64, 2, 3, 0, 0, 5];
        let data = build_profraw(2, 6, &counters);

        let result = parse_raw_profile(&data);
        assert!(result.is_ok(), "should parse multi-function profraw: {:?}", result.err());
        let profile = result.unwrap();
        assert_eq!(profile.functions.len(), 2);
        assert_eq!(profile.functions[0].counters, vec![1, 2, 3]);
        assert_eq!(profile.functions[0].covered, 3);
        assert_eq!(profile.functions[1].counters, vec![0, 0, 5]);
        assert_eq!(profile.functions[1].covered, 1);
    }

    #[test]
    fn test_compute_coverage_from_profraw() {
        let counters = [0u64, 10, 20, 0, 0];
        let data = build_profraw(1, 5, &counters);

        let dir = std::env::temp_dir().join("rvtest_cov_compute");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test.profraw");
        std::fs::write(&path, &data).unwrap();

        let result = compute_coverage_from_profraw(&path);
        assert!(result.is_ok(), "should compute coverage: {:?}", result.err());
        let totals = result.unwrap();
        assert!((totals.line_pct() - 40.0).abs() < 0.01);
        assert!((totals.func_pct() - 100.0).abs() < 0.01);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_compute_coverage_from_profraw_not_found() {
        let path = std::env::temp_dir().join("nonexistent_xyz123.profraw");
        let result = compute_coverage_from_profraw(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_coverage_totals_region_pct() {
        let t = CoverageTotals { total_counters: 100, covered_counters: 50, total_functions: 10, covered_functions: 5 };
        assert_eq!(t.region_pct(), t.line_pct());
    }

    #[test]
    fn test_write_report_cobertura() {
        let dir = std::env::temp_dir().join("rvtest_cov_cobertura");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("cobertura.xml");

        let result = write_report(CoverageFormat::Cobertura, 60.0, 70.0, 60.0, &path);
        assert!(result.is_ok());
        assert!(path.exists());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
