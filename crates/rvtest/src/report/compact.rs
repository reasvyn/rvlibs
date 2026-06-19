use std::fmt::Write;

use crate::core::{TestRun, TestStatus};
use super::{format_duration, TestReporter};

// ---------------------------------------------------------------------------
// CompactReporter — one line per test
// ---------------------------------------------------------------------------

/// Minimal, single-line-per-test reporter suitable for quick feedback.
pub struct CompactReporter;

impl TestReporter for CompactReporter {
    fn report(&self, run: &TestRun) -> String {
        let mut out = String::new();
        let total = run.total();
        let passed = run.total_passed();
        let failed = run.total_failed();
        let skipped = run.total_skipped();

        for suite in &run.suites {
            for test in &suite.tests {
                let status = match test.status {
                    TestStatus::Passed => "PASS",
                    TestStatus::Failed { .. } => "FAIL",
                    TestStatus::Skipped { .. } => "SKIP",
                    TestStatus::TimedOut { .. } => "TIMEOUT",
                };
                let dur = format_duration(test.duration);
                let _ = writeln!(out, "{status}  {dur:>7}  {}", test.name);
                if let Some(ref captured) = test.captured_output {
                    for line in captured.lines() {
                        let _ = writeln!(out, "  | {}", line);
                    }
                }
            }
        }

        let _ = writeln!(
            out,
            "\nResults: {passed}/{total} passed, {failed} failed, {skipped} skipped  ({:.2}s)",
            run.duration.as_secs_f64(),
        );

        out
    }
}
