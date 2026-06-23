use std::collections::HashMap;

use crate::cli::run_cargo_test;

/// Run `cargo test` N times and report which tests are flaky.
pub fn detect_flaky(
    filter: Option<String>,
    num_runs: u32,
    verbose: bool,
    fast: bool,
    cranelift: bool,
    parallel_frontend: Option<usize>,
) {
    eprintln!("\n  🔍 Running test suite {num_runs} times to detect flaky tests...\n");

    let mut results: HashMap<String, (u32, u32)> = HashMap::new();

    for run in 1..=num_runs {
        if verbose {
            eprint!("  Run {run}/{num_runs}... ");
        }

        let test_run = run_cargo_test(
            filter.as_deref(),
            fast,
            cranelift,
            parallel_frontend,
            false,
            None,
        );

        for suite in &test_run.suites {
            for test in &suite.tests {
                if test.status.is_skipped() {
                    continue;
                }
                let entry = results.entry(test.name.clone()).or_insert((0, 0));
                entry.1 += 1;
                if test.status.is_passed() {
                    entry.0 += 1;
                }
            }
        }

        if verbose {
            let passed = test_run.total_passed();
            let failed = test_run.total_failed();
            eprintln!("{passed} passed, {failed} failed");
        }
    }

    eprintln!();
    let mut flaky_found = false;
    let mut flaky_names: Vec<String> = Vec::new();

    let mut sorted: Vec<_> = results.into_iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    for (name, (passes, total)) in &sorted {
        let rate = *passes as f64 / *total as f64 * 100.0;
        if rate < 100.0 {
            flaky_found = true;
            flaky_names.push(name.clone());
            eprintln!("  ⚠  {name:<60} {passes}/{total} passes ({rate:.0}%)");
        }
    }

    if flaky_found {
        rvtest::core::save_flaky_tests(&flaky_names);
        eprintln!();
        eprintln!("  💾 Flaky test list saved to target/.rvtest-cache/flaky.json");
        eprintln!("  ▶  Use `cargo rvtest --quarantine` to skip these tests.");
    } else {
        rvtest::core::save_flaky_tests(&[]);
        eprintln!("  ✅ No flaky tests detected — every test passed on all {num_runs} runs.");
    }
    eprintln!();
}
