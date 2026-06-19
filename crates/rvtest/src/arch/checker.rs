use std::collections::HashSet;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::arch::graph::DependencyGraph;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Start building an architecture check.
pub fn arch_check() -> ArchCheck {
    ArchCheck { rules: Vec::new(), src_dir: PathBuf::from("src") }
}

// ---------------------------------------------------------------------------
// ArchCheck builder
// ---------------------------------------------------------------------------

/// Builder for architecture-enforcement rules.
///
/// Use [`arch_check()`] to create one, then chain `.module(...).may_depend_on(...)`
/// and finish with `.assert_all_pass()`.
pub struct ArchCheck {
    rules: Vec<Rule>,
    src_dir: PathBuf,
}

impl ArchCheck {
    /// Set a custom source directory (for non-standard crate layouts).
    pub fn src_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.src_dir = path.into();
        self
    }

    /// Select a module to attach rules to.
    pub fn module(mut self, name: &str) -> ModuleRuleBuilder {
        self.rules.push(Rule::Module {
            name: name.to_owned(),
            allowed_deps: None,
            forbidden_deps: None,
        });
        ModuleRuleBuilder { check: self, module_name: name.to_owned() }
    }

    /// Select the global set of rules that apply across all modules.
    pub fn all_modules(self) -> AllModulesRuleBuilder {
        AllModulesRuleBuilder { check: self }
    }

    /// Run all rules and panic on violations.
    pub fn assert_all_pass(self) {
        if let Err(msg) = self.run() {
            panic!("Architecture violations:\n{}", msg);
        }
    }

    /// Run all rules and return `Ok(())` or `Err(violations)`.
    pub fn run(mut self) -> Result<(), String> {
        let graph = DependencyGraph::from_dir(&self.src_dir)?;

        // Finalise module rules: no explicit rules → forbid nothing.
        for rule in &mut self.rules {
            if let Rule::Module { allowed_deps, forbidden_deps, .. } = rule
                && allowed_deps.is_none() && forbidden_deps.is_none() {
                    *forbidden_deps = Some(HashSet::new());
                }
        }

        let mut violations = Vec::new();

        for rule in &self.rules {
            match rule {
                Rule::Module { name, allowed_deps, forbidden_deps } => {
                    let deps = graph.dependencies_of(name);
                    if let Some(allowed) = allowed_deps {
                        for dep in &deps {
                            if !allowed.contains(dep) {
                                violations.push(format!(
                                    "  {} must not depend on {} (allowed: {})",
                                    name,
                                    dep,
                                    allowed.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
                                ));
                            }
                        }
                    }
                    if let Some(forbidden) = forbidden_deps {
                        for dep in &deps {
                            if forbidden.contains(dep) {
                                violations.push(format!(
                                    "  {} must not depend on {}",
                                    name, dep
                                ));
                            }
                        }
                    }
                }
                Rule::NoCycles => {
                    for cycle in &graph.find_cycles() {
                        violations.push(format!(
                            "  cycle detected: {}",
                            cycle.join(" → ")
                        ));
                    }
                }
                Rule::PublicApiDocRequired => {
                    for (file, item) in find_undocumented_pub_items(&self.src_dir) {
                        violations.push(format!(
                            "  {}:{} — public item `{}` is missing a doc comment",
                            file.display(), item.line, item.name
                        ));
                    }
                }
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations.join("\n"))
        }
    }
}

// ---------------------------------------------------------------------------
// Module rule builder
// ---------------------------------------------------------------------------

/// Builder returned by [`ArchCheck::module`] to attach rules to a module.
pub struct ModuleRuleBuilder {
    check: ArchCheck,
    module_name: String,
}

impl ModuleRuleBuilder {
    fn find_mut(&mut self) -> &mut Rule {
        self.check
            .rules
            .iter_mut()
            .find(|r| matches!(r, Rule::Module { name, .. } if *name == self.module_name))
            .expect("module rule not found")
    }

    /// Declare that this module may only depend on the listed modules.
    ///
    /// Any dependency not in this list will cause a violation.
    pub fn may_depend_on(mut self, deps: &[&str]) -> ArchCheck {
        let set: HashSet<String> = deps.iter().map(|s| s.to_string()).collect();
        if let Rule::Module { allowed_deps, .. } = self.find_mut() {
            *allowed_deps = Some(set);
        }
        self.check
    }

    /// Declare that this module may NOT depend on any of the listed modules.
    pub fn may_not_depend_on(mut self, deps: &[&str]) -> ArchCheck {
        let set: HashSet<String> = deps.iter().map(|s| s.to_string()).collect();
        if let Rule::Module { forbidden_deps, .. } = self.find_mut() {
            let current = forbidden_deps.get_or_insert_with(HashSet::new);
            current.extend(set);
        }
        self.check
    }
}

impl From<ModuleRuleBuilder> for ArchCheck {
    fn from(b: ModuleRuleBuilder) -> Self {
        b.check
    }
}

// ---------------------------------------------------------------------------
// All-modules rule builder
// ---------------------------------------------------------------------------

/// Builder returned by [`ArchCheck::all_modules`] for global rules.
pub struct AllModulesRuleBuilder {
    check: ArchCheck,
}

impl AllModulesRuleBuilder {
    /// Require that the module dependency graph contains no cycles.
    pub fn must_not_have_cycles(mut self) -> ArchCheck {
        self.check.rules.push(Rule::NoCycles);
        self.check
    }

    /// Require that all public items have doc comments.
    pub fn public_api_doc_required(mut self) -> ArchCheck {
        self.check.rules.push(Rule::PublicApiDocRequired);
        self.check
    }
}

// ---------------------------------------------------------------------------
// Internal rule types
// ---------------------------------------------------------------------------

enum Rule {
    Module {
        name: String,
        allowed_deps: Option<HashSet<String>>,
        forbidden_deps: Option<HashSet<String>>,
    },
    NoCycles,
    PublicApiDocRequired,
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::Module { name, .. } => write!(f, "Module({})", name),
            Rule::NoCycles => write!(f, "NoCycles"),
            Rule::PublicApiDocRequired => write!(f, "PublicApiDocRequired"),
        }
    }
}

// ---------------------------------------------------------------------------
// File scanning helpers
// ---------------------------------------------------------------------------

pub(crate) fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if !name.starts_with('.') && name != "target" {
                collect_rs_files(&path, out);
            }
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

pub(crate) struct UndocumentedItem {
    pub(crate) name: String,
    pub(crate) line: usize,
}

pub(crate) fn find_undocumented_pub_items(src_dir: &Path) -> Vec<(PathBuf, UndocumentedItem)> {
    let mut result = Vec::new();
    let mut files = Vec::new();
    collect_rs_files(src_dir, &mut files);

    for file in &files {
        let content = match std::fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let trimmed = lines[i].trim();

            let pub_kw = if let Some(rest) = trimmed.strip_prefix("pub ") {
                rest.trim()
            } else {
                i += 1;
                continue;
            };

            // Skip re-exports and already-documented items
            if has_preceding_doc_comment(&lines, i) {
                i += 1;
                continue;
            }

            let item_name = if let Some(name) = pub_kw
                .strip_prefix("fn ")
                .or_else(|| pub_kw.strip_prefix("struct "))
                .or_else(|| pub_kw.strip_prefix("enum "))
                .or_else(|| pub_kw.strip_prefix("trait "))
                .or_else(|| pub_kw.strip_prefix("type "))
                .or_else(|| pub_kw.strip_prefix("const "))
                .or_else(|| pub_kw.strip_prefix("mod "))
                .or_else(|| pub_kw.strip_prefix("use "))
            {
                // Extract the name (up to first paren/bracket/semicolon/whitespace)
                name.split(['(', '{', ';', '=', '<', ':'])
                    .next()
                    .unwrap_or("")
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_owned()
            } else {
                i += 1;
                continue;
            };

            if !item_name.is_empty() && item_name != "_" {
                result.push((
                    file.clone(),
                    UndocumentedItem { name: item_name, line: i + 1 },
                ));
            }

            i += 1;
        }
    }

    result
}

pub(crate) fn has_preceding_doc_comment(lines: &[&str], idx: usize) -> bool {
    if idx == 0 {
        return false;
    }
    let prev = lines[idx.saturating_sub(1)].trim();
    prev.starts_with("///") || prev.starts_with("/**") || prev.starts_with("//!")
}
