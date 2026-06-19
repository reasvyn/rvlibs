use crate::core::{TestRun, TestStatus};
use super::TestReporter;

// ---------------------------------------------------------------------------
// JsonReporter — machine-readable JSON
// ---------------------------------------------------------------------------

/// Reporter that emits results as a JSON object.
pub struct JsonReporter;

impl TestReporter for JsonReporter {
    fn report(&self, run: &TestRun) -> String {
        let mut out = String::new();

        let success = if run.success() { "true" } else { "false" };
        out.push_str(&format!(
            r#"{{"success":{},"total":{},"passed":{},"failed":{},"skipped":{},"duration_secs":{:.3},"suites":["#,
            success,
            run.total(),
            run.total_passed(),
            run.total_failed(),
            run.total_skipped(),
            run.duration.as_secs_f64(),
        ));

        for (si, suite) in run.suites.iter().enumerate() {
            if si > 0 {
                out.push(',');
            }
            out.push_str(&format!(
                r#"{{"name":"{}","duration_secs":{:.3},"tests":["#,
                escape_json(&suite.name),
                suite.duration.as_secs_f64(),
            ));

            for (ti, test) in suite.tests.iter().enumerate() {
                if ti > 0 {
                    out.push(',');
                }
                let (status_str, reason) = match &test.status {
                    TestStatus::Passed => ("passed", None),
                    TestStatus::Failed { reason, .. } => ("failed", Some(reason.as_str())),
                    TestStatus::Skipped { reason } => ("skipped", reason.as_deref()),
                    TestStatus::TimedOut { .. } => ("timed_out", None),
                };

                out.push_str(&format!(
                    r#"{{"name":"{}","status":"{}","duration_secs":{:.3}"#,
                    escape_json(&test.name),
                    status_str,
                    test.duration.as_secs_f64(),
                ));

                if let Some(r) = reason {
                    out.push_str(&format!(r#","reason":"{}""#, escape_json(r)));
                }

                if let Some(ref loc) = test.location {
                    out.push_str(&format!(
                        r#","location":{{"file":"{}","line":{}"#,
                        escape_json(&loc.file),
                        loc.line,
                    ));
                    if let Some(col) = loc.column {
                        out.push_str(&format!(r#","column":{col}"#));
                    }
                    out.push('}');
                }

                if let Some(ref output) = test.captured_output {
                    out.push_str(&format!(r#","captured_output":"{}""#, escape_json(output)));
                }

                if !test.parameters.is_empty() {
                    out.push_str(r#","parameters":{"#);
                    for (pi, (k, v)) in test.parameters.iter().enumerate() {
                        if pi > 0 {
                            out.push(',');
                        }
                        out.push_str(&format!(r#""{}":"{}""#, escape_json(k), escape_json(v)));
                    }
                    out.push('}');
                }

                out.push_str("]}");
            }

            out.push('}');
        }

        out.push(']');
        out.push('}');
        out
    }
}

pub(super) fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
