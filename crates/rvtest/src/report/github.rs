use std::fmt::Write;

use crate::core::{TestRun, TestStatus};
use super::TestReporter;

// ---------------------------------------------------------------------------
// GithubReporter — GitHub Actions annotations
// ---------------------------------------------------------------------------

/// Reporter that emits GitHub Actions-compatible `::error` / `::warning`
/// annotations for test failures.
///
/// Each failed or timed-out test produces one `::error` line with the
/// source file, line number, and failure message.  Passing tests are
/// silently ignored.
///
/// # Example output
///
/// ```text
/// ::error file=tests/demo.rs,line=42,title=Calculator :: adds — assertion failed
/// ```
pub struct GithubReporter;

impl TestReporter for GithubReporter {
    fn report(&self, run: &TestRun) -> String {
        let mut out = String::new();
        let mut passed = 0usize;
        let mut failed = 0usize;
        let mut skipped = 0usize;

        for suite in &run.suites {
            for test in &suite.tests {
                match &test.status {
                    TestStatus::Passed => passed += 1,
                    TestStatus::Skipped { .. } => skipped += 1,
                    TestStatus::Failed { reason, location } => {
                        failed += 1;
                        let file = location
                            .as_ref()
                            .map(|l| escape_github(l.file.as_str()))
                            .unwrap_or_else(|| "unknown".to_string());
                        let line = location
                            .as_ref()
                            .map(|l| l.line.to_string())
                            .unwrap_or_else(|| "1".to_string());
                        let title = escape_github(&test.name);
                        let msg = escape_github(reason);
                        let _ = writeln!(
                            out,
                            "::error file={file},line={line},title={title}::{msg}"
                        );
                    }
                    TestStatus::TimedOut { duration, location } => {
                        failed += 1;
                        let file = location
                            .as_ref()
                            .map(|l| escape_github(l.file.as_str()))
                            .unwrap_or_else(|| "unknown".to_string());
                        let line = location
                            .as_ref()
                            .map(|l| l.line.to_string())
                            .unwrap_or_else(|| "1".to_string());
                        let title = escape_github(&test.name);
                        let msg = format!("timed out after {duration:?}");
                        let _ = writeln!(
                            out,
                            "::error file={file},line={line},title={title}::{msg}"
                        );
                    }
                }
            }
        }

        let total = run.total();
        let _ = writeln!(
            out,
            "rvtest: {passed}/{total} passed, {failed} failed, {skipped} skipped  ({:.2}s)",
            run.duration.as_secs_f64(),
        );

        out
    }
}

pub(super) fn escape_github(s: &str) -> String {
    s.replace('%', "%25")
        .replace('\n', "%0A")
        .replace('\r', "%0D")
}
