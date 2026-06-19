use std::fmt::Write;

use crate::core::{SourceLocation, TestCase, TestKind, TestRun, TestStatus, TestSuite};
use super::{coloured, dim, coloured_count, format_duration, TestReporter};

// ---------------------------------------------------------------------------
// PrettyReporter — colourful, human-friendly output
// ---------------------------------------------------------------------------

/// Human-readable reporter with optional colour.
///
/// Output style is inspired by modern test frameworks (PestPHP, Vitest, Jest):
///
/// ```text
///   ✓ Calculator > addition > adds (0.5ms)
///   ✗ Calculator > subtracts (0.3ms)
///     → assertion failed: expected 2, got 3
///
///   Tests  3 passed, 1 failed
///   Time   0.52s
/// ```
pub struct PrettyReporter {
    use_colour: bool,
    collapse_passing: bool,
}

impl PrettyReporter {
    /// Create a new `PrettyReporter`.
    pub fn new() -> Self {
        PrettyReporter { use_colour: true, collapse_passing: true }
    }

    /// Enable or disable ANSI colour codes.
    pub fn colour(mut self, enabled: bool) -> Self {
        self.use_colour = enabled;
        self
    }

    /// Enable or disable collapsing consecutive passing tests into a group.
    pub fn collapse_passing(mut self, yes: bool) -> Self {
        self.collapse_passing = yes;
        self
    }
}

impl Default for PrettyReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl TestReporter for PrettyReporter {
    fn report(&self, run: &TestRun) -> String {
        let mut out = String::new();

        for suite in &run.suites {
            if suite.tests.is_empty() {
                continue;
            }

            // Section header (only if there are multiple suite kinds or doc-tests)
            let show_header = run.suites.len() > 1;
            if show_header {
                let label = section_label(suite, self.use_colour);
                let _ = writeln!(out, "\n  {}", label);
            }

            if self.collapse_passing {
                // Collapse consecutive passing tests (only groups of 2+)
                let _pass_count: usize = 0;
                let mut pass_group: Vec<&TestCase> = Vec::new();

                for test in &suite.tests {
                    if test.status.is_passed() {
                        pass_group.push(test);
                        continue;
                    }

                    // Flush pending passing group
                    if pass_group.len() == 1 {
                        render_test(&mut out, pass_group[0], suite.is_doc(), self.use_colour);
                    } else if pass_group.len() > 1 {
                        let icon = coloured("✓", "32", self.use_colour);
                        let _ = writeln!(out, "  {icon} {} tests passed", pass_group.len());
                    }
                    pass_group.clear();

                    // Render non-passing test
                    render_test(&mut out, test, suite.is_doc(), self.use_colour);
                }

                // Flush trailing passing group
                if pass_group.len() == 1 {
                    render_test(&mut out, pass_group[0], suite.is_doc(), self.use_colour);
                } else if pass_group.len() > 1 {
                    let icon = coloured("✓", "32", self.use_colour);
                    let _ = writeln!(out, "  {icon} {} tests passed", pass_group.len());
                }
            } else {
                for test in &suite.tests {
                    render_test(&mut out, test, suite.is_doc(), self.use_colour);
                }
            }
        }

        // Summary
        let passed = run.total_passed();
        let failed = run.total_failed();
        let dur_s = run.duration.as_secs_f64();
        let doc_count: usize = run.suites.iter().filter(|s| s.is_doc()).map(|s| s.tests.len()).sum();

        let _ = writeln!(out);
        let summary = build_summary(passed, failed, doc_count, self.use_colour);
        let _ = writeln!(out, "  Tests  {}", summary);
        let _ = writeln!(out, "  Time   {dur_s:.2}s");

        out
    }
}

pub(super) fn render_test(out: &mut String, test: &TestCase, is_doc: bool, colour: bool) {
    let icon = if is_doc {
        doc_icon(colour)
    } else {
        status_icon(&test.status, colour)
    };
    let name = test.name.replace(" :: ", " > ");
    let dur = format_duration(test.duration);

    let _ = writeln!(out, "  {icon} {name} ({dur})");

    if let TestStatus::Failed { ref reason, ref location } = test.status {
        for line in reason.lines() {
            let _ = writeln!(out, "    {} {}", dim("→", colour), line);
        }
        if let Some(loc) = location {
            let _ = writeln!(out, "      {} {}", dim("at", colour), format_location(loc, colour));
        }
    }

    if let TestStatus::TimedOut { duration, ref location } = test.status {
        let _ = writeln!(out, "    {} timed out after {duration:?}", dim("→", colour));
        if let Some(loc) = location {
            let _ = writeln!(out, "      {} {}", dim("at", colour), format_location(loc, colour));
        }
    }

    if let Some(ref captured) = test.captured_output {
        for line in captured.lines() {
            let _ = writeln!(out, "    {} {}", dim("│", colour), line);
        }
    }

    if !is_doc
        && let TestStatus::Skipped { ref reason } = test.status
            && let Some(r) = reason {
                let _ = writeln!(out, "    {} skipped: {}", dim("→", colour), r);
            }

    if let Some(ref stats) = test.bench_stats {
        let _ = writeln!(out, "    {} {} iterations, mean {:.3}ms, min {:.3}ms, max {:.3}ms",
            dim("⏱", colour),
            stats.iterations,
            stats.mean.as_secs_f64() * 1000.0,
            stats.min.as_secs_f64() * 1000.0,
            stats.max.as_secs_f64() * 1000.0,
        );
    }
}

pub(super) fn section_label(suite: &TestSuite, colour: bool) -> String {
    let label = match suite.kind {
        TestKind::Unit => format!("unit tests ({})", suite.source_path),
        TestKind::Integration => format!("integration ({})", suite.source_path),
        TestKind::Doc => format!("doc-tests ({})", suite.source_path),
    };
    let padded = format!("  {}  ", label);
    let width = 54usize.saturating_sub(padded.chars().count());
    let left = width / 2;
    let right = width - left;
    let mut line = String::new();
    for _ in 0..left { line.push('─'); }
    line.push_str(&padded);
    for _ in 0..right { line.push('─'); }
    dim(&line, colour)
}

pub(super) fn doc_icon(colour: bool) -> String {
    coloured("?", "33", colour)
}

pub(super) fn status_icon(status: &TestStatus, colour: bool) -> String {
    match status {
        TestStatus::Passed => coloured("✓", "32", colour),
        TestStatus::Failed { .. } => coloured("✗", "31", colour),
        TestStatus::Skipped { .. } => coloured("–", "33", colour),
        TestStatus::TimedOut { .. } => coloured("⊗", "31", colour),
    }
}

pub(super) fn build_summary(passed: usize, failed: usize, doc_count: usize, colour: bool) -> String {
    let mut parts: Vec<String> = Vec::new();
    parts.push(coloured_count(passed, "passed", "32", colour));
    if failed > 0 {
        parts.push(coloured_count(failed, "failed", "31", colour));
    }
    if doc_count > 0 {
        parts.push(coloured_count(doc_count, "doc-tests", "33", colour));
    }
    parts.join(", ")
}

pub(super) fn format_location(loc: &SourceLocation, colour: bool) -> String {
    let s = match loc.column {
        Some(col) => format!("{}:{}:{}", loc.file, loc.line, col),
        None => format!("{}:{}", loc.file, loc.line),
    };
    coloured(&s, "36", colour)
}
