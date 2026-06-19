//! Test gap analysis — identify untested or under-described code paths.
//!
//! Combines coverage data with test descriptions to find code that is
//! covered by execution but lacks meaningful test descriptions.

use std::collections::BTreeMap;

use crate::core::TestRun;

/// A gap identified in the test suite.
#[derive(Debug)]
pub struct TestGap {
    /// The module or function with insufficient test coverage.
    pub location: String,
    /// What kind of gap this is.
    pub kind: GapKind,
    /// Suggested improvement.
    pub suggestion: String,
}

/// The type of test gap detected.
#[derive(Debug)]
pub enum GapKind {
    /// A module has test coverage but no describe/it descriptions.
    Undescribed,
    /// A public function has no corresponding test.
    UntestedFunction,
}

/// Run gap analysis on a test run.
///
/// Compares the test suite structure against the source code to find
/// modules and functions that are exercised but lack descriptive tests.
pub fn analyze_gaps(run: &TestRun, src_dir: Option<&std::path::Path>) -> Vec<TestGap> {
    let mut gaps = Vec::new();

    // Analyze describe blocks: collect all test names and their descriptions
    let test_descriptions: BTreeMap<String, bool> = run.suites
        .iter()
        .flat_map(|s| s.tests.iter())
        .map(|t| (t.name.clone(), t.suite.is_some()))
        .collect();

    // Check for undescribed modules
    for suite in &run.suites {
        if suite.description.is_none() && !suite.tests.is_empty() {
            // Check if the suite name looks like a module path
            if suite.name.contains("::") || suite.name.chars().any(|c| c.is_uppercase()) {
                gaps.push(TestGap {
                    location: suite.name.clone(),
                    kind: GapKind::Undescribed,
                    suggestion: format!("Add .description() to describe(\"{}\") block", suite.name),
                });
            }
        }
    }

    // If we have source directory, look for public functions without tests
    if let Some(dir) = src_dir
        && let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "rs")
                    && let Ok(content) = std::fs::read_to_string(&path) {
                        for line in content.lines() {
                            let trimmed = line.trim();
                            // Look for `pub fn` definitions
                            if let Some(name) = trimmed.strip_prefix("pub fn ") {
                                let fn_name = name.split('(').next().unwrap_or(name).trim();
                                // Check if any test mentions this function
                                let has_test = test_descriptions.keys().any(|t| t.contains(fn_name));
                                if !has_test && fn_name != "main" && !fn_name.starts_with('_') {
                                    gaps.push(TestGap {
                                        location: format!("{}::{}", path.file_stem().unwrap_or_default().to_string_lossy(), fn_name),
                                        kind: GapKind::UntestedFunction,
                                        suggestion: format!("Add a test for `pub fn {}`", fn_name),
                                    });
                                }
                            }
                        }
                    }
            }
        }

    gaps
}
