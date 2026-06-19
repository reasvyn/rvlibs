//! Reporter that emits cargo-nextest compatible output.
//!
//! Uses a JSON-lines format compatible with nextest's machine-readable output,
//! enabling integration with nextest tooling and CI systems that understand
//! the nextest format.

use std::fmt::Write;

use crate::core::{TestRun, TestStatus};
use crate::report::TestReporter;

/// Reporter that emits cargo-nextest compatible JSON-lines output.
pub struct NextestReporter;

impl TestReporter for NextestReporter {
    fn report(&self, run: &TestRun) -> String {
        let mut out = String::new();

        // Emit a status-event for each test
        for suite in &run.suites {
            for test in &suite.tests {
                let (status, pass_fail) = match &test.status {
                    TestStatus::Passed => ("passed", "pass"),
                    TestStatus::Failed { .. } => ("failed", "fail"),
                    TestStatus::Skipped { .. } => ("skipped", "skip"),
                    TestStatus::TimedOut { .. } => ("timed_out", "fail"),
                };

                let dur_ns = test.duration.as_nanos();

                let _ = writeln!(
                    out,
                    r#"{{"type":"test","name":"{}","status":"{}","exec_time":{},"pass_fail":"{}"}}"#,
                    escape_json(&test.name),
                    status,
                    dur_ns,
                    pass_fail,
                );
            }
        }

        // Emit a summary event
        let success = if run.success() { "true" } else { "false" };
        let _ = writeln!(
            out,
            r#"{{"type":"summary","success":{},"passed":{},"failed":{},"skipped":{},"total":{},"time_ns":{}}}"#,
            success,
            run.total_passed(),
            run.total_failed(),
            run.total_skipped(),
            run.total(),
            run.duration.as_nanos(),
        );

        out
    }
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
