use std::fmt::Write;

use crate::core::{TestRun, TestStatus};
use super::TestReporter;

// ---------------------------------------------------------------------------
// TapReporter — Test Anything Protocol
// ---------------------------------------------------------------------------

/// Reporter that emits TAP (Test Anything Protocol) output.
///
/// TAP is a simple line-based protocol widely used in the Perl and
/// JavaScript ecosystems and supported by many CI tools.
pub struct TapReporter;

impl TestReporter for TapReporter {
    fn report(&self, run: &TestRun) -> String {
        let mut out = String::new();

        let total = run.total();
        let _ = writeln!(out, "1..{total}");

        let mut index = 0;
        for suite in &run.suites {
            for test in &suite.tests {
                index += 1;
                let ok = if test.status.is_passed() { "ok" } else { "not ok" };
                let duration_ms = test.duration.as_secs_f64() * 1000.0;

                let _ = writeln!(out, "{ok} {index} - {} [{duration_ms:.1}ms]", test.name);

                if let TestStatus::Failed { ref reason, .. } = test.status {
                    for line in reason.lines() {
                        let _ = writeln!(out, "  {line}");
                    }
                }

                if let TestStatus::TimedOut { duration, .. } = test.status {
                    let _ = writeln!(out, "  # TIMEOUT after {duration:?}");
                }

                if let TestStatus::Skipped { ref reason } = test.status {
                    let reason = reason.as_deref().unwrap_or("no reason given");
                    let _ = writeln!(out, "  # SKIP {reason}");
                }
            }
        }

        out
    }
}
