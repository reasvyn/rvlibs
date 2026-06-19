use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::arch::checker::collect_rs_files;

pub(crate) struct DependencyGraph {
    pub(crate) edges: HashMap<String, HashSet<String>>,
}

impl DependencyGraph {
    pub(crate) fn from_dir(dir: &Path) -> Result<Self, String> {
        if !dir.is_dir() {
            return Err(format!("directory not found: {:?}", dir));
        }

        let mut edges: HashMap<String, HashSet<String>> = HashMap::new();
        let mut files: Vec<PathBuf> = Vec::new();
        collect_rs_files(dir, &mut files);

        for file in &files {
            let rel = file.strip_prefix(dir).map_err(|e| format!("path: {e}"))?;
            let module = path_to_module(rel);
            let content = std::fs::read_to_string(file)
                .map_err(|e| format!("read {:?}: {e}", file))?;
            let deps = parse_deps(&content);

            let entry: &mut HashSet<String> = edges.entry(module).or_default();
            for dep in deps {
                if dep.starts_with("crate::") {
                    entry.insert(dep.trim_start_matches("crate::").to_owned());
                }
            }
        }

        Ok(DependencyGraph { edges })
    }

    pub(crate) fn dependencies_of(&self, module: &str) -> HashSet<String> {
        self.edges.get(module).cloned().unwrap_or_default()
    }

    pub(crate) fn find_cycles(&self) -> Vec<Vec<String>> {
        let nodes: Vec<&String> = self.edges.keys().collect();
        let mut visited: HashSet<&String> = HashSet::new();
        let mut in_stack: HashSet<&String> = HashSet::new();
        let mut stack: Vec<&String> = Vec::new();
        let mut cycles: Vec<Vec<String>> = Vec::new();

        fn dfs<'a>(
            node: &'a String,
            graph: &'a HashMap<String, HashSet<String>>,
            visited: &mut HashSet<&'a String>,
            in_stack: &mut HashSet<&'a String>,
            stack: &mut Vec<&'a String>,
            cycles: &mut Vec<Vec<String>>,
        ) {
            if !visited.insert(node) {
                return;
            }
            in_stack.insert(node);
            stack.push(node);

            if let Some(deps) = graph.get(node) {
                for dep in deps {
                    if in_stack.contains(dep) {
                        let pos = stack.iter().position(|n| *n == dep).unwrap();
                        let cycle: Vec<String> = stack[pos..]
                            .iter()
                            .map(|s| (*s).clone())
                            .collect();
                        cycles.push(cycle);
                    } else {
                        dfs(dep, graph, visited, in_stack, stack, cycles);
                    }
                }
            }

            stack.pop();
            in_stack.remove(node);
        }

        for node in &nodes {
            dfs(node, &self.edges, &mut visited, &mut in_stack, &mut stack, &mut cycles);
        }

        cycles
    }
}

pub(crate) fn path_to_module(rel: &Path) -> String {
    let s = rel.to_string_lossy();
    let stem = s.strip_suffix(".rs").unwrap_or(&s);
    if stem.ends_with("/mod") || stem == "mod" {
        let parent = rel.parent().and_then(|p| p.to_str()).unwrap_or("");
        return if parent.is_empty() { "crate_root".into() } else { parent.replace('/', "::") };
    }
    stem.replace('/', "::")
}

pub(crate) fn parse_deps(content: &str) -> HashSet<String> {
    let mut deps = HashSet::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("use crate::") {
            let path = rest.trim_end_matches(';');
            let top = path.split("::").next().unwrap_or(path);
            if !top.is_empty() {
                deps.insert(top.to_owned());
            }
        }
        if let Some(rest) = trimmed.strip_prefix("pub mod ") {
            let name = rest.split(';').next().unwrap_or(rest).trim();
            deps.insert(name.to_owned());
        } else if let Some(rest) = trimmed.strip_prefix("mod ") {
            let name = rest.split(';').next().unwrap_or(rest).trim();
            deps.insert(name.to_owned());
        }
    }
    deps
}
