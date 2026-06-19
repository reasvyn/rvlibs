use std::fmt::Write;

use crate::core::{TestRun, TestStatus};
use super::TestReporter;

// ---------------------------------------------------------------------------
// JunitReporter — JUnit XML for CI integration
// ---------------------------------------------------------------------------

/// Reporter that emits JUnit-compatible XML.
///
/// This format is understood by Jenkins, GitLab CI, GitHub Actions, and
/// most other CI systems.
pub struct JunitReporter {
    suite_name: String,
}

impl JunitReporter {
    /// Create a new `JunitReporter`.
    pub fn new() -> Self {
        JunitReporter { suite_name: "rvtest".to_owned() }
    }

    /// Override the top-level suite name in the XML output.
    pub fn suite_name(mut self, name: &str) -> Self {
        self.suite_name = name.to_owned();
        self
    }
}

impl Default for JunitReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl TestReporter for JunitReporter {
    fn report(&self, run: &TestRun) -> String {
        let mut out = String::new();
        let _ = writeln!(out, r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        let _ = writeln!(
            out,
            r#"<testsuites name="{}" tests="{}" failures="{}" skipped="{}" time="{:.3}">"#,
            self.suite_name,
            run.total(),
            run.total_failed(),
            run.total_skipped(),
            run.duration.as_secs_f64(),
        );

        for suite in &run.suites {
            let _ = writeln!(
                out,
                r#"  <testsuite name="{}" tests="{}" failures="{}" skipped="{}" time="{:.3}">"#,
                escape_xml(&suite.name),
                suite.len(),
                suite.failed().count(),
                suite.skipped().count(),
                suite.duration.as_secs_f64(),
            );

            for test in &suite.tests {
                let classname = test.suite.as_deref().unwrap_or("root");
                let dur_s = test.duration.as_secs_f64();

                let location_attr = test.location.as_ref().map(|loc| {
                    format!(r#" file="{}" line="{}""#, escape_xml(&loc.file), loc.line)
                }).unwrap_or_default();

                match &test.status {
                    TestStatus::Passed => {
                        let _ = writeln!(
                            out,
                            r#"    <testcase classname="{}" name="{}" time="{:.3}"{}/>"#,
                            escape_xml(classname),
                            escape_xml(&test.name),
                            dur_s,
                            location_attr,
                        );
                    }
                    TestStatus::Failed { reason, .. } => {
                        let _ = writeln!(
                            out,
                            r#"    <testcase classname="{}" name="{}" time="{:.3}"{}>"#,
                            escape_xml(classname),
                            escape_xml(&test.name),
                            dur_s,
                            location_attr,
                        );
                        let _ = writeln!(
                            out,
                            r#"      <failure message="{}" type="AssertionError"><![CDATA[{}]]></failure>"#,
                            escape_xml(reason),
                            reason,
                        );
                        if let Some(ref output) = test.captured_output {
                            let _ = writeln!(
                                out,
                                r#"      <system-out><![CDATA[{}]]></system-out>"#,
                                output,
                            );
                        }
                        let _ = writeln!(out, "    </testcase>");
                    }
                    TestStatus::Skipped { reason } => {
                        let msg = reason.as_deref().unwrap_or("skipped");
                        let _ = writeln!(
                            out,
                            r#"    <testcase classname="{}" name="{}" time="{:.3}"{}>"#,
                            escape_xml(classname),
                            escape_xml(&test.name),
                            dur_s,
                            location_attr,
                        );
                        let _ = writeln!(
                            out,
                            r#"      <skipped message="{}" />"#,
                            escape_xml(msg),
                        );
                        let _ = writeln!(out, "    </testcase>");
                    }
                    TestStatus::TimedOut { duration: to, .. } => {
                        let _ = writeln!(
                            out,
                            r#"    <testcase classname="{}" name="{}" time="{:.3}"{}>"#,
                            escape_xml(classname),
                            escape_xml(&test.name),
                            dur_s,
                            location_attr,
                        );
                        let _ = writeln!(
                            out,
                            r#"      <failure message="timed out after {:?}" type="TimeoutError" />"#,
                            to,
                        );
                        let _ = writeln!(out, "    </testcase>");
                    }
                }
            }

            let _ = writeln!(out, "  </testsuite>");
        }

        let _ = writeln!(out, "</testsuites>");
        out
    }
}

pub(super) fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
