use std::fmt::Write;

use crate::core::{TestKind, TestRun, TestStatus};
use super::json::escape_json;
use super::TestReporter;

// ---------------------------------------------------------------------------
// AgentReporter — LLM-optimised structured JSON output
// ---------------------------------------------------------------------------

/// Reporter that emits structured JSON optimised for LLM consumption.
///
/// Includes full test metadata, source code snippets at failure locations,
/// structured failure analysis, and machine-parseable metadata.
pub struct AgentReporter;

impl TestReporter for AgentReporter {
    fn report(&self, run: &TestRun) -> String {
        let mut out = String::new();

        let _ = writeln!(out, "{{");
        let _ = writeln!(out, r#"  "format": "rvtest-agent-v1","#);
        let _ = writeln!(out, r#"  "success": {},"#, if run.success() { "true" } else { "false" });
        let _ = writeln!(out, r#"  "total": {},"#, run.total());
        let _ = writeln!(out, r#"  "passed": {},"#, run.total_passed());
        let _ = writeln!(out, r#"  "failed": {},"#, run.total_failed());
        let _ = writeln!(out, r#"  "skipped": {},"#, run.total_skipped());
        let _ = writeln!(out, r#"  "duration_secs": {:.3},"#, run.duration.as_secs_f64());
        let _ = writeln!(out, r#"  "suites": ["#);

        for (si, suite) in run.suites.iter().enumerate() {
            if si > 0 {
                out.push(',');
            }
            let _ = writeln!(out);
            let _ = writeln!(out, r#"    {{"#);
            let _ = writeln!(out, r#"      "name": "{}", "#, escape_json(&suite.name));
            let _ = writeln!(out, r#"      "kind": "{}", "#, match suite.kind {
                TestKind::Unit => "unit",
                TestKind::Integration => "integration",
                TestKind::Doc => "doc",
            });
            let _ = writeln!(out, r#"      "source_path": "{}", "#, escape_json(&suite.source_path));
            let _ = writeln!(out, r#"      "duration_secs": {:.3},"#, suite.duration.as_secs_f64());
            let _ = writeln!(out, r#"      "tests": ["#);

            for (ti, test) in suite.tests.iter().enumerate() {
                if ti > 0 {
                    out.push(',');
                }
                let _ = writeln!(out);
                let _ = writeln!(out, r#"        {{"#);
                let _ = writeln!(out, r#"          "name": "{}", "#, escape_json(&test.name));
                let (status_str, reason) = match &test.status {
                    TestStatus::Passed => ("passed", None),
                    TestStatus::Failed { reason: r, .. } => ("failed", Some(r.as_str())),
                    TestStatus::Skipped { reason: r } => ("skipped", r.as_deref()),
                    TestStatus::TimedOut { .. } => ("timed_out", None),
                };
                let _ = writeln!(out, r#"          "status": "{}", "#, status_str);
                let _ = writeln!(out, r#"          "duration_secs": {:.3},"#, test.duration.as_secs_f64());

                if let Some(r) = reason {
                    let _ = writeln!(out, r#"          "failure": "{}", "#, escape_json(r));
                }

                // Source location
                if let Some(ref loc) = test.location {
                    let _ = writeln!(out, r#"          "location": {{"#);
                    let _ = writeln!(out, r#"            "file": "{}", "#, escape_json(&loc.file));
                    let _ = writeln!(out, r#"            "line": {}"#, loc.line);
                    if let Some(col) = loc.column {
                        let _ = write!(out, r#","column":{col}"#);
                    }
                    let _ = writeln!(out);
                    let _ = writeln!(out, r#"          }}, "#);

                    // Source code snippet around the failure
                    if let Some(snippet) = read_source_snippet(&loc.file, loc.line, 3) {
                        let _ = writeln!(out, r#"          "source_snippet": "{}", "#, escape_json(&snippet));
                    }
                }

                if let Some(ref output) = test.captured_output {
                    let _ = writeln!(out, r#"          "captured_output": "{}", "#, escape_json(output));
                }

                if !test.tags.is_empty() {
                    let tags: String = test.tags.iter().map(|t| format!(r#""{}""#, escape_json(t))).collect::<Vec<_>>().join(",");
                    let _ = writeln!(out, r#"          "tags": [{}], "#, tags);
                }

                if !test.parameters.is_empty() {
                    let _ = writeln!(out, r#"          "parameters": {{"#);
                    for (pi, (k, v)) in test.parameters.iter().enumerate() {
                        if pi > 0 { out.push(','); }
                        let _ = writeln!(out, r#"            "{}": "{}""#, escape_json(k), escape_json(v));
                    }
                    let _ = writeln!(out);
                    let _ = writeln!(out, r#"          }}, "#);
                }

                let _ = writeln!(out, r#"          "analysis": {{"#);
                let _ = writeln!(out, r#"            "is_failure": {},"#, if status_str == "failed" || status_str == "timed_out" { "true" } else { "false" });
                let _ = writeln!(out, r#"            "is_flaky_candidate": false"#);
                let _ = writeln!(out, r#"          }}"#);
                let _ = write!(out, r#"        }}"#);
            }

            let _ = writeln!(out);
            let _ = writeln!(out, r#"      ]"#);
            let _ = write!(out, r#"    }}"#);
        }

        let _ = writeln!(out);
        let _ = writeln!(out, r#"  ]"#);
        let _ = writeln!(out, r#"}}"#);
        out
    }
}

/// Read a snippet of source code around a given line.
fn read_source_snippet(file: &str, line: u32, context: usize) -> Option<String> {
    let content = std::fs::read_to_string(file).ok()?;
    let lines: Vec<&str> = content.lines().collect();
    let line_idx = (line as usize).saturating_sub(1);
    let start = line_idx.saturating_sub(context);
    let end = (line_idx + context + 1).min(lines.len());
    if start >= end {
        return None;
    }
    let mut snippet = String::new();
    for (i, line) in lines.iter().enumerate().skip(start).take(end - start) {
        let prefix = if i == line_idx { "→" } else { " " };
        snippet.push_str(&format!("{prefix}{i:>4}: {line}\n"));
    }
    Some(snippet)
}
