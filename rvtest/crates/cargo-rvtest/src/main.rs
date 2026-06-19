//! `cargo-rvtest` — CLI binary for the rvtest testing library.
//!
//! This crate provides the `cargo rvtest` subcommand. Install it:
//!
//! ```bash
//! cargo install cargo-rvtest
//! ```
//!
//! Then run `cargo rvtest` to execute tests with formatted output,
//! or see `cargo rvtest --help` for all options.

mod cli;

use clap::Parser;
use cli::args::Cli;
use rvtest::core::ReportFormat;

#[cfg(test)]
use rvtest::core::{TestCase, TestKind, TestStatus, TestSuite};
#[cfg(test)]
use rvtest::report;
#[cfg(test)]
use rvtest::runner::parse_cargo_test_output;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let raw_args: Vec<String> = if args.len() > 1 && args[1] == "rvtest" {
        let mut a = vec![args[0].clone()];
        a.extend_from_slice(&args[2..]);
        a
    } else {
        args
    };

    let mut args = Cli::parse_from(raw_args);
    cli::resolve_profile(&mut args);

    // === Coverage mode ===
    if args.coverage || args.coverage_open {
        cli::run_coverage(&args);
    }

    // === Cache config ===
    if args.clear_cache {
        rvtest::core::clear_result_cache();
        eprintln!("  ✅ Test result cache cleared.");
        std::process::exit(0);
    }
    if args.cache {
        rvtest::core::set_result_cache_enabled(true);
    }
    if args.build_cache {
        cli::set_build_cache_enabled(true);
    }
    if args.why_slow {
        unsafe { std::env::set_var("RVTEST_WHY_SLOW", "1"); }
    }

    // === Snapshot config ===
    if args.update_all {
        rvtest::snapshot::set_update_all(true);
    }
    if args.review {
        rvtest::snapshot::set_review_mode(true);
    }

    // === Flaky quarantine ===
    if args.flaky_report {
        let flaky = rvtest::core::load_flaky_tests();
        if flaky.is_empty() {
            eprintln!("  ✅ No quarantined (flaky) tests.");
        } else {
            eprintln!("  ⚠  Quarantined (flaky) tests:");
            for name in &flaky {
                eprintln!("    • {name}");
            }
            eprintln!("\n  {} test(s) in quarantine.", flaky.len());
            eprintln!("  Use `cargo rvtest --unquarantine` to clear.");
        }
        std::process::exit(0);
    }

    if args.unquarantine {
        rvtest::core::save_flaky_tests(&[]);
        eprintln!("  ✅ Quarantine list cleared.");
        std::process::exit(0);
    }

    let flaky_filter = if args.quarantine && !args.include_flaky {
        let flaky = rvtest::core::load_flaky_tests();
        if !flaky.is_empty() {
            eprintln!("  ⚠  {} test(s) in quarantine (use --include-flaky to override).", flaky.len());
            Some(flaky.join("|"))
        } else {
            None
        }
    } else {
        None
    };

    let combined_skip = match (&args.skip, &flaky_filter) {
        (Some(s), Some(f)) => Some(format!("{}|{}", s, f)),
        (Some(s), None) => Some(s.clone()),
        (None, Some(f)) => Some(f.clone()),
        (None, None) => None,
    };

    // === Warm daemon mode ===
    if args.warm {
        eprintln!("  Warming test binaries...");
        let binaries = cli::build_test_binaries(args.fast, args.cranelift, args.parallel_frontend, args.workspace);
        if binaries.is_empty() {
            eprintln!("  No test binaries to warm.");
        } else {
            cli::save_warm_state(&binaries);
            eprintln!("  {} test binary(ies) warmed. Use `cargo rvtest` to run from cache.", binaries.len());
        }
        std::process::exit(0);
    }

    // === Daemon mode ===
    if args.daemon {
        let filter = args.filter.clone();
        let format: ReportFormat = args.format.parse().unwrap_or(ReportFormat::Pretty);
        let daemon = rvtest::daemon::CompileDaemon::new(filter, format);
        daemon.run();
        return;
    }

    // === Watch mode ===
    if args.watch {
        let use_colour = cli::use_color(cli::resolve_color(args.color.as_deref()));
        cli::watch::watch_loop(
            args.filter.clone(),
            args.format.clone(),
            args.fast,
            args.profile_slow.unwrap_or(0) as usize,
            args.cranelift,
            args.parallel_frontend,
            combined_skip.clone(),
            use_colour,
        );
        return;
    }

    // === Flaky detection ===
    if args.detect_flaky > 0 {
        cli::flaky::detect_flaky(
            args.filter.clone(),
            args.detect_flaky,
            args.verbose,
            args.fast,
            args.cranelift,
            args.parallel_frontend,
        );
        return;
    }

    // === Tune mode ===
    if args.tune {
        cli::auto_tune();
        std::process::exit(0);
    }

    // === List mode ===
    if args.list || args.retest || args.failed {
        let retest = args.retest || args.failed;
        let filter = if retest {
            let failed = rvtest::core::load_failed_tests();
            if failed.is_empty() {
                eprintln!("  No previously failed tests found.");
                std::process::exit(0);
            }
            Some(failed.join("|"))
        } else {
            args.filter.clone()
        };

        let run = cli::run_cargo_test(filter.as_deref(), args.fast, args.cranelift, args.parallel_frontend, args.workspace, combined_skip.as_deref());

        if args.list {
            for suite in &run.suites {
                for test in &suite.tests {
                    println!("{}", test.name);
                }
            }
            let total = run.total();
            eprintln!("\n  {} test(s) listed.", total);
        } else {
            let use_colour = cli::use_color(cli::resolve_color(args.color.as_deref()));
            let report = cli::render(&ReportFormat::Pretty, &run, args.profile_slow.unwrap_or(0) as usize, use_colour);
            println!("{report}");
            rvtest::core::save_failed_tests(&run);
        }
        std::process::exit(if run.success() { 0 } else { 1 });
    }

    // === Test mode ===
    if args.impact && args.filter.is_none() {
        if let Some(impact_filter) = cli::git_impact_filter(args.filter.as_deref(), args.skip.as_deref()) {
            eprintln!("  🔍 Impact analysis detected, auto-filtering: {impact_filter}");
            args.filter = Some(impact_filter);
        } else {
            eprintln!("  No impacted tests detected or not a git repository.");
        }
    } else if args.changed && args.filter.is_none() {
        if let Some(changed_filter) = cli::git_changed_filter() {
            eprintln!("  🔍 Changed files detected, auto-filtering: {changed_filter}");
            args.filter = Some(changed_filter);
        } else {
            eprintln!("  No changed files detected or not a git repository.");
        }
    }

    let format: ReportFormat = args.format.parse().unwrap_or_else(|e| {
        eprintln!("{e}, falling back to 'pretty'");
        ReportFormat::Pretty
    });

    if args.mask_secrets {
        rvtest::secrets::set_mask_secrets_enabled(true);
    }

    if args.verify_checksums {
        rvtest::checksum::set_checksum_enabled(true);
        // Verify snapshot checksums if the directory exists
        let snap_dir = std::path::Path::new("tests/snapshots");
        if snap_dir.exists() {
            let errors = rvtest::checksum::verify_checksums(snap_dir);
            if !errors.is_empty() {
                eprintln!("  Checksum verification errors:");
                for err in &errors {
                    eprintln!("    ✗ {err}");
                }
            } else {
                eprintln!("  ✓ Snapshot checksums verified.");
            }
        }
    }

    if args.sandbox {
        let mut rl = rvtest::sandbox::ResourceLimits::default();
        if let Some(n) = args.sandbox_max_fds { rl = rl.with_max_fds(n); }
        if let Some(n) = args.sandbox_max_procs { rl = rl.with_max_processes(n); }
        if let Some(n) = args.sandbox_max_stack { rl = rl.with_max_stack(n); }
        if args.sandbox_no_core { rl = rl.no_core_dumps(); }
        if let Some(n) = args.sandbox_max_as { rl = rl.with_max_address_space(n); }

        let sb_config = rvtest::sandbox::SandboxConfig::default()
            .with_fs_whitelist(rvtest::sandbox::parse_fs_whitelist(&args.sandbox_fs))
            .with_network(!args.sandbox_no_net)
            .with_env_allowlist(rvtest::sandbox::parse_env_allowlist(&args.sandbox_env))
            .with_restrict_umask(true)
            .with_isolated_tempdir(true)
            .with_resource_limits(rl)
            .with_enforce_permissions(args.sandbox_enforce);
        let guard = sb_config.apply();
        // Store the guard in a static or pass it through to ensure it lives
        // for the duration of the test run
        let summary = guard.summary().join("; ");
        if args.verbose {
            eprintln!("  🔒 Sandbox active: {summary}");
        }
        // Leak the guard so it lives for the process lifetime
        std::mem::forget(guard);
    }

    if args.cranelift || args.parallel_frontend.is_some() {
        if !cli::is_nightly() {
            eprintln!("warning: --cranelift and --parallel-frontend require nightly Rust.\n\
                       Switch with: `rustup default nightly` or use `cargo +nightly rvtest`.");
        }
        if args.cranelift && !cli::has_cranelift_component() {
            eprintln!("warning: Cranelift codegen backend not found.\n\
                       Install: `rustup component add rustc-codegen-cranelift-preview --toolchain nightly`");
        }
    }

    let run = if args.isolate {
        cli::run_tests_isolated(args.filter.as_deref(), combined_skip.as_deref(), args.fast, args.cranelift, args.parallel_frontend, args.workspace)
    } else {
        cli::run_cargo_test(args.filter.as_deref(), args.fast, args.cranelift, args.parallel_frontend, args.workspace, combined_skip.as_deref())
    };

    rvtest::core::save_failed_tests(&run);
    rvtest::core::save_full_run(&run);

    if args.bench || args.save_baseline {
        rvtest::core::save_bench_baseline(&run);
    }
    if args.compare_baseline {
        let baseline = rvtest::core::load_bench_baseline();
        if !baseline.is_empty() {
            for suite in &run.suites {
                for test in &suite.tests {
                    if let Some(ref stats) = test.bench_stats
                        && let Some(b) = baseline.get(&test.name) {
                            let base_mean = b["mean_secs"].as_f64().unwrap_or(0.0);
                            let current_mean = stats.mean.as_secs_f64();
                            let ratio = current_mean / base_mean;
                            if ratio > 1.2 {
                                eprintln!("  ⚠  REGRESSION: {} mean {:.3}ms vs baseline {:.3}ms ({:.1}% slower)",
                                    test.name, current_mean * 1000.0, base_mean * 1000.0, (ratio - 1.0) * 100.0);
                            } else if ratio < 0.8 {
                                eprintln!("  ✓  IMPROVEMENT: {} mean {:.3}ms vs baseline {:.3}ms ({:.1}% faster)",
                                    test.name, current_mean * 1000.0, base_mean * 1000.0, (1.0 - ratio) * 100.0);
                            }
                        }
                }
            }
        }
    }

    let use_colour = cli::use_color(cli::resolve_color(args.color.as_deref()));
    let mut report = cli::render(&format, &run, args.profile_slow.unwrap_or(0) as usize, use_colour);

    // Show --diff comparison
    if args.diff {
        use std::fmt::Write;
        let diff = rvtest::core::compute_diff(&run);
        if diff.has_changes() {
            let _ = writeln!(report);
            let _ = writeln!(report, "  {} Run diff vs previous run:", cli::dim("📊"));
            if !diff.new_failures.is_empty() {
                let _ = writeln!(report, "    {} New failures ({}):", cli::coloured_str("✗", "31", use_colour), diff.new_failures.len());
                for name in &diff.new_failures {
                    let _ = writeln!(report, "      • {name}");
                }
            }
            if !diff.recovered.is_empty() {
                let _ = writeln!(report, "    {} Recovered ({}):", cli::coloured_str("✓", "32", use_colour), diff.recovered.len());
                for name in &diff.recovered {
                    let _ = writeln!(report, "      • {name}");
                }
            }
            if !diff.slower.is_empty() {
                let _ = writeln!(report, "    {} Slower ({}):", cli::coloured_str("↓", "33", use_colour), diff.slower.len());
                for (name, prev, new) in &diff.slower {
                    let _ = writeln!(report, "      • {name}  {prev:.2}s → {new:.2}s");
                }
            }
            if !diff.faster.is_empty() {
                let _ = writeln!(report, "    {} Faster ({}):", cli::coloured_str("↑", "32", use_colour), diff.faster.len());
                for (name, prev, new) in &diff.faster {
                    let _ = writeln!(report, "      • {name}  {prev:.2}s → {new:.2}s");
                }
            }
        } else {
            let _ = writeln!(report, "\n  {} No significant changes from previous run.", cli::dim("📊"));
        }
    }

    if let Some(ref path) = args.report_html {
        let html = rvtest::report::HtmlReporter.report(&run);
        let _ = std::fs::write(path, &html);
        eprintln!("  📄 HTML report written to {path}");
    }

    if args.open_report {
        if let Some(ref path) = args.report_html {
            cli::open_in_browser(path);
        } else {
            let path = "target/rvtest-report.html";
            let html = rvtest::report::HtmlReporter.report(&run);
            let _ = std::fs::write(path, &html);
            cli::open_in_browser(path);
        }
    }

    if args.gap_analysis {
        let src_dir = std::path::Path::new("src");
        let gaps = rvtest::core::analyze_gaps(&run, if src_dir.exists() { Some(src_dir) } else { None });
        if gaps.is_empty() {
            println!("\n  ✅ No test gaps detected.");
        } else {
            println!("\n  📋 Test Gap Analysis:");
            for gap in &gaps {
                let icon = match gap.kind {
                    rvtest::core::GapKind::Undescribed => "📝",
                    rvtest::core::GapKind::UntestedFunction => "🔧",
                };
                println!("    {}  {}", icon, gap.location);
                println!("       {}", gap.suggestion);
            }
            println!("  {} gap(s) found.", gaps.len());
        }
    }

    println!("{report}");
    std::process::exit(if run.success() { 0 } else { 1 });
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use cli::args::Cli;
    use std::time::{Duration, SystemTime};

    fn dim(s: &str) -> String {
        format!("\x1b[2m{s}\x1b[0m")
    }

    #[test]
    fn dim_wraps_in_ansi() {
        let s = dim("hello");
        assert_eq!(s, "\x1b[2mhello\x1b[0m");
    }

    #[test]
    fn dim_empty_string() {
        let s = dim("");
        assert_eq!(s, "\x1b[2m\x1b[0m");
    }

    #[test]
    fn detect_fast_linker_returns_some_or_none() {
        let linker = cli::detect_fast_linker();
        match linker {
            Some("mold") | Some("lld") | None => {}
            _ => panic!("unexpected linker: {linker:?}"),
        }
    }

    // ---- parse_cargo_test_output ----

    #[test]
    fn parse_cargo_test_output_empty() {
        let suites = parse_cargo_test_output("", "");
        assert!(suites.is_empty() || suites.len() == 1);
    }

    #[test]
    fn parse_cargo_test_output_with_one_pass() {
        let stderr = "Running unittests src/lib.rs (target/debug/deps/lib-abc123)\n";
        let stdout = "test my_test ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\n";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert_eq!(suites.len(), 1);
        assert_eq!(suites[0].tests.len(), 1);
        assert!(suites[0].tests[0].status.is_passed());
    }

    #[test]
    fn parse_cargo_test_output_with_failure() {
        let stderr = "Running unittests src/lib.rs (target/debug/deps/lib-abc123)\n";
        let stdout = "test failing_test ... FAILED\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out\n\nfailures:\n\nfailures:\n    failing_test\n";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert_eq!(suites.len(), 1);
        assert!(!suites[0].success());
    }

    #[test]
    fn parse_cargo_test_output_with_ignored() {
        let stderr = "Running unittests src/lib.rs (target/debug/deps/lib-abc123)\n";
        let stdout = "test skipped_test ... ignored\ntest result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out\n";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert_eq!(suites.len(), 1);
        assert!(suites[0].tests[0].status.is_skipped());
    }

    #[test]
    fn parse_cargo_test_output_multiple_sections() {
        let stderr = "Running unittests src/lib.rs (target/debug/deps/lib-abc123)\nRunning tests/integration.rs (target/debug/deps/integration-def456)\n";
        let stdout = "test unit_test ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\ntest integration_test ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\n";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert_eq!(suites.len(), 2);
    }

    #[test]
    fn parse_cargo_test_output_doc_tests() {
        let stderr = "Doc-tests rvtest\n";
        let stdout = "test test_foo ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\n";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert_eq!(suites.len(), 1);
        assert_eq!(suites[0].kind, TestKind::Doc);
    }

    #[test]
    fn parse_cargo_test_output_fallback_section() {
        let suites = parse_cargo_test_output("", "test foo ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored\n");
        assert!(!suites.is_empty());
    }

    #[test]
    fn parse_cargo_test_output_failure_details() {
        let stderr = "Running unittests src/lib.rs (target/debug/deps/lib-abc123)\n";
        let stdout = "\
---- my_test stdout ----
some detail line
another detail
test my_test ... FAILED
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
failures:
    my_test
";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert!(!suites.is_empty());
        assert!(!suites[0].success());
    }

    #[test]
    fn parse_cargo_test_output_doc_test_name_formatting() {
        let stderr = "Doc-tests rvtest\n";
        let stdout = "test test_foo ... ok\ntest result: ok. 1 passed; 0 failed\n";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert_eq!(suites.len(), 1);
        assert_eq!(suites[0].kind, TestKind::Doc);
        assert_eq!(suites[0].source_path, "rvtest");
        assert_eq!(suites[0].tests[0].name, "rvtest - test_foo");
    }

    #[test]
    fn parse_cargo_test_output_failure_with_location_suite_name() {
        let stderr = "Running tests/integration.rs (target/debug/deps/integration-abc)\n";
        let stdout = "test my_test ... FAILED\ntest result: FAILED. 0 passed; 1 failed\n";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert_eq!(suites[0].kind, TestKind::Integration);
        assert_eq!(suites[0].source_path, "tests/integration.rs");
    }

    #[test]
    fn parse_cargo_test_output_no_parentheses() {
        let stderr = "Running unittests src/lib.rs\n";
        let stdout = "test foo ... ok\ntest result: ok. 1 passed; 0 failed\n";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert!(!suites.is_empty());
    }

    #[test]
    fn parse_cargo_test_output_malformed_test_line() {
        let stderr = "Running unittests src/lib.rs\n";
        let stdout = "test malformed_no_separator\ntest result: ok. 0 passed; 0 failed\n";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert_eq!(suites.len(), 1);
        assert_eq!(suites[0].tests.len(), 0);
    }

    #[test]
    fn parse_cargo_test_output_extra_lines_after_last_section() {
        let stderr = "Running unittests src/lib.rs\n";
        let stdout = "test t1 ... ok\ntest result: ok. 1 passed; 0 failed\ntest extra_after_result ... ok\n";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert_eq!(suites.len(), 1);
        assert_eq!(suites[0].tests.len(), 1);
    }

    #[test]
    fn parse_cargo_test_output_multiple_failures_with_details() {
        let stderr = "Running unittests src/lib.rs\n";
        let stdout = "\
---- test_a stdout ----
detail for a
---- test_b stdout ----
detail for b
test test_a ... FAILED
test test_b ... FAILED
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
failures:
    test_a
    test_b
";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert!(!suites.is_empty());
        assert!(!suites[0].success());
        assert_eq!(suites[0].tests.len(), 2);
    }

    #[test]
    fn parse_cargo_test_output_empty_failure_line() {
        let stderr = "Running unittests src/lib.rs\n";
        let stdout = "\
---- my_test stdout ----

test my_test ... FAILED
test result: FAILED. 0 passed; 1 failed; 0 ignored
failures:
    my_test
";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert!(!suites[0].success());
    }

    #[test]
    fn parse_cargo_test_output_unknown_status_skipped() {
        let stderr = "Running unittests src/lib.rs\n";
        let stdout = "\
test my_test ... ???unknown???
test result: ok. 0 passed; 0 failed; 0 ignored
";
        let suites = parse_cargo_test_output(stderr, stdout);
        assert_eq!(suites[0].tests.len(), 0);
    }

    // ---- helpers ----

    #[test]
    fn is_nightly_returns_bool() {
        let _ = cli::is_nightly();
    }

    #[test]
    fn has_cranelift_component_returns_bool() {
        let _ = cli::has_cranelift_component();
    }

    // ---- Cli arg parsing ----

    #[test]
    fn cli_defaults() {
        let args = Cli::parse_from(["cargo-rvtest"]);
        assert!(!args.fast);
        assert!(!args.cranelift);
        assert!(args.parallel_frontend.is_none());
    }

    #[test]
    fn cli_cranelift_flag() {
        let args = Cli::parse_from(["cargo-rvtest", "--cranelift"]);
        assert!(args.cranelift);
    }

    #[test]
    fn cli_parallel_frontend() {
        let args = Cli::parse_from(["cargo-rvtest", "--parallel-frontend", "4"]);
        assert_eq!(args.parallel_frontend, Some(4));
    }

    #[test]
    fn cli_cranelift_with_fast() {
        let args = Cli::parse_from(["cargo-rvtest", "--fast", "--cranelift"]);
        assert!(args.fast);
        assert!(args.cranelift);
    }

    #[test]
    fn cli_review_flag() {
        let args = Cli::parse_from(["cargo-rvtest", "--review"]);
        assert!(args.review);
    }

    #[test]
    fn cli_daemon_flag() {
        let args = Cli::parse_from(["cargo-rvtest", "--daemon"]);
        assert!(args.daemon);
    }

    #[test]
    fn cli_profile_ci_flag() {
        let args = Cli::parse_from(["cargo-rvtest", "--profile", "ci"]);
        assert_eq!(args.profile.as_deref(), Some("ci"));
    }

    #[test]
    fn cli_profile_dev_flag() {
        let args = Cli::parse_from(["cargo-rvtest", "--profile", "dev"]);
        assert_eq!(args.profile.as_deref(), Some("dev"));
    }

    #[test]
    fn resolve_profile_ci_sets_fail_fast() {
        let mut args = Cli::parse_from(["cargo-rvtest", "--profile", "ci"]);
        assert!(!args.fail_fast);
        cli::resolve_profile(&mut args);
        assert!(args.fail_fast);
        assert_eq!(args.format, "junit");
        assert!(!args.verbose);
    }

    #[test]
    fn resolve_profile_dev_sets_verbose() {
        let mut args = Cli::parse_from(["cargo-rvtest", "--profile", "dev"]);
        assert!(!args.verbose);
        cli::resolve_profile(&mut args);
        assert!(args.verbose);
    }

    #[test]
    fn resolve_profile_unknown_warns() {
        let mut args = Cli::parse_from(["cargo-rvtest", "--profile", "nonexistent"]);
        cli::resolve_profile(&mut args);
        assert!(!args.verbose);
    }

    #[test]
    fn cli_all_fast_flags() {
        let args = Cli::parse_from([
            "cargo-rvtest",
            "--fast",
            "--cranelift",
            "--parallel-frontend",
            "8",
        ]);
        assert!(args.fast);
        assert!(args.cranelift);
        assert_eq!(args.parallel_frontend, Some(8));
    }

    // ---- render ---

    #[test]
    fn render_with_slow_tests() {
        let mut suite = TestSuite::new("test");
        suite.tests.push(TestCase {
            name: "test :: slow".into(), suite: Some("test".into()), tags: vec![],
            status: TestStatus::Passed, duration: Duration::from_secs(2),
            assertions: 0, location: None, parameters: vec![], captured_output: None,
            bench_stats: None, bench_threshold: None,
        });
        let run = rvtest::core::TestRun {
            suites: vec![suite],
            start_time: SystemTime::now(),
            end_time: SystemTime::now(),
            duration: Duration::from_secs(2),
        };
        let result = cli::render(&ReportFormat::Compact, &run, 5, false);
        assert!(result.contains("Slowest"));
        assert!(result.contains("2.00s"));
    }

    #[test]
    fn render_without_slow() {
        let run = rvtest::core::TestRun::new();
        let result = cli::render(&ReportFormat::Compact, &run, 0, false);
        assert!(result.contains("0/0"));
    }

    #[test]
    fn render_all_formats() {
        let run = rvtest::core::TestRun::new();
            for fmt in [ReportFormat::Pretty, ReportFormat::Tap, ReportFormat::Junit, ReportFormat::Json, ReportFormat::Compact, ReportFormat::Github, ReportFormat::Agent, ReportFormat::Html, ReportFormat::Nextest] {
            let result = cli::render(&fmt, &run, 0, false);
            assert!(!result.is_empty(), "render output should not be empty for {fmt:?}");
        }
    }

    #[test]
    fn render_slow_zero_no_slow_section() {
        let mut suite = TestSuite::new("test");
        suite.tests.push(TestCase {
            name: "test :: fast".into(), suite: Some("test".into()), tags: vec![],
            status: TestStatus::Passed, duration: Duration::from_millis(1),
            assertions: 0, location: None, parameters: vec![], captured_output: None,
            bench_stats: None, bench_threshold: None,
        });
        let run = rvtest::core::TestRun {
            suites: vec![suite],
            start_time: SystemTime::now(),
            end_time: SystemTime::now(),
            duration: Duration::from_millis(1),
        };
        let result = cli::render(&ReportFormat::Pretty, &run, 0, false);
        assert!(!result.contains("Slowest"), "should not show slowest section when count is 0");
    }

    #[test]
    fn render_slow_nonzero_no_tests() {
        let run = rvtest::core::TestRun::new();
        let result = cli::render(&ReportFormat::Pretty, &run, 5, false);
        assert!(!result.contains("Slowest"));
    }

    #[test]
    fn render_pretty_with_multiple_suites() {
        let mut s1 = TestSuite::new("A");
        s1.tests.push(TestCase::new("A :: t1"));
        let mut s2 = TestSuite::new("B");
        s2.tests.push(TestCase::new("B :: t2"));
        let run = rvtest::core::TestRun {
            suites: vec![s1, s2],
            start_time: SystemTime::now(),
            end_time: SystemTime::now(),
            duration: Duration::from_millis(10),
        };
        let result = cli::render(&ReportFormat::Pretty, &run, 0, false);
        assert!(result.contains("A"));
        assert!(result.contains("B"));
    }

    #[test]
    fn format_duration_exact_second() {
        let s = report::format_duration(Duration::from_secs(1));
        assert_eq!(s, "1.00s");
    }

    #[test]
    fn format_duration_just_below_second() {
        let s = report::format_duration(Duration::from_millis(999));
        assert_eq!(s, "999.0ms");
    }
}
