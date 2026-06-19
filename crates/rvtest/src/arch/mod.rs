//! Architecture-enforcement tests.
//!
//! Declare module dependency rules with [`arch_check`](crate::arch::arch_check)
//! to enforce layering and prevent circular dependencies.
//!
//! # Submodules
//!
//! - [`graph`] — Dependency graph parsing and cycle detection
//! - [`checker`] — Rule checking and undocumented-item discovery

pub mod checker;
pub mod graph;

pub use checker::{arch_check, ArchCheck, AllModulesRuleBuilder, ModuleRuleBuilder};

// Re-exports for tests (only active when testing)
#[cfg(test)]
pub(crate) use checker::{
    collect_rs_files, find_undocumented_pub_items, has_preceding_doc_comment,
};
#[cfg(test)]
pub(crate) use graph::{parse_deps, path_to_module, DependencyGraph};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn path_to_module_simple() {
        assert_eq!(path_to_module(Path::new("core.rs")), "core");
    }

    #[test]
    fn path_to_module_mod_rs() {
        assert_eq!(path_to_module(Path::new("mod.rs")), "crate_root");
    }

    #[test]
    fn path_to_module_subdir_mod() {
        assert_eq!(path_to_module(Path::new("sub/mod.rs")), "sub");
    }

    #[test]
    fn parse_deps_use_crate() {
        let deps = parse_deps("use crate::core::TestSuite;\n");
        assert!(deps.contains("core"));
        assert_eq!(deps.len(), 1);
    }

    #[test]
    fn parse_deps_pub_mod() {
        let deps = parse_deps("pub mod spec;\n");
        assert!(deps.contains("spec"));
    }

    #[test]
    fn parse_deps_private_mod() {
        let deps = parse_deps("mod internal;\n");
        assert!(deps.contains("internal"));
    }

    #[test]
    fn parse_deps_comment_ignored() {
        let deps = parse_deps("// use crate::something;\nmod real;\n");
        assert!(!deps.contains("something"));
        assert!(deps.contains("real"));
    }

    #[test]
    fn parse_deps_empty() {
        let deps = parse_deps("");
        assert!(deps.is_empty());
    }

    #[test]
    fn parse_deps_multiple() {
        let code = "use crate::core;\nuse crate::report;\npub mod runner;\n";
        let deps = parse_deps(code);
        assert!(deps.contains("core"));
        assert!(deps.contains("report"));
        assert!(deps.contains("runner"));
        assert_eq!(deps.len(), 3);
    }

    #[test]
    fn collect_rs_files_finds_rs_files() {
        let dir = std::env::temp_dir().join("rvtest_arch_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("foo.rs"), "").unwrap();
        std::fs::write(dir.join("bar.txt"), "").unwrap();
        std::fs::create_dir_all(dir.join("sub")).unwrap();
        std::fs::write(dir.join("sub").join("baz.rs"), "").unwrap();

        let mut files = Vec::new();
        collect_rs_files(&dir, &mut files);

        let names: Vec<String> = files.iter().map(|f| f.file_name().unwrap().to_str().unwrap().to_owned()).collect();
        assert!(names.contains(&"foo.rs".into()));
        assert!(names.contains(&"baz.rs".into()));
        assert!(!names.contains(&"bar.txt".into()));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn collect_rs_files_skips_hidden_dir() {
        let dir = std::env::temp_dir().join("rvtest_arch_hidden");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::create_dir_all(dir.join(".hidden")).unwrap();
        std::fs::write(dir.join(".hidden").join("lib.rs"), "").unwrap();

        let mut files = Vec::new();
        collect_rs_files(&dir, &mut files);
        let has_hidden = files.iter().any(|f| f.to_string_lossy().contains(".hidden"));
        assert!(!has_hidden, ".hidden directory contents should be skipped");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn collect_rs_files_nonexistent_dir() {
        let mut files = Vec::new();
        collect_rs_files(Path::new("/nonexistent_dir_xyz"), &mut files);
        assert!(files.is_empty());
    }

    #[test]
    fn dependency_graph_empty_edges() {
        let graph = DependencyGraph { edges: std::collections::HashMap::new() };
        let deps = graph.dependencies_of("nonexistent");
        assert!(deps.is_empty());
    }

    #[test]
    fn dependency_graph_dependencies_of() {
        let mut edges = std::collections::HashMap::new();
        let mut deps = std::collections::HashSet::new();
        deps.insert("core".to_string());
        deps.insert("report".to_string());
        edges.insert("spec".to_string(), deps);

        let graph = DependencyGraph { edges };
        let deps = graph.dependencies_of("spec");
        assert_eq!(deps.len(), 2);
        assert!(deps.contains("core"));
        assert!(deps.contains("report"));
    }

    #[test]
    fn dependency_graph_no_cycles() {
        let mut edges = std::collections::HashMap::new();
        edges.insert("a".to_string(), {
            let mut s = std::collections::HashSet::new();
            s.insert("b".to_string());
            s
        });
        edges.insert("b".to_string(), {
            let mut s = std::collections::HashSet::new();
            s.insert("c".to_string());
            s
        });
        edges.insert("c".to_string(), std::collections::HashSet::new());
        let graph = DependencyGraph { edges };
        let cycles = graph.find_cycles();
        assert!(cycles.is_empty(), "should have no cycles: {:?}", cycles);
    }

    #[test]
    fn dependency_graph_detects_cycles() {
        let mut edges = std::collections::HashMap::new();
        edges.insert("a".to_string(), {
            let mut s = std::collections::HashSet::new();
            s.insert("b".to_string());
            s
        });
        edges.insert("b".to_string(), {
            let mut s = std::collections::HashSet::new();
            s.insert("a".to_string());
            s
        });
        let graph = DependencyGraph { edges };
        let cycles = graph.find_cycles();
        assert!(!cycles.is_empty(), "should detect a-b-a cycle");
    }

    #[test]
    fn dependency_graph_self_loop() {
        let mut edges = std::collections::HashMap::new();
        edges.insert("a".to_string(), {
            let mut s = std::collections::HashSet::new();
            s.insert("a".to_string());
            s
        });
        let graph = DependencyGraph { edges };
        let cycles = graph.find_cycles();
        assert!(!cycles.is_empty(), "self-loop should be a cycle");
    }

    #[test]
    fn has_preceding_doc_comment_with_doc_comment() {
        let lines = vec!["/// Does stuff", "pub fn foo() {}"];
        assert!(has_preceding_doc_comment(&lines, 1));
    }

    #[test]
    fn has_preceding_doc_comment_without_doc_comment() {
        let lines = vec!["", "pub fn foo() {}"];
        assert!(!has_preceding_doc_comment(&lines, 1));
    }

    #[test]
    fn has_preceding_doc_comment_first_line() {
        let lines = vec!["pub fn foo() {}"];
        assert!(!has_preceding_doc_comment(&lines, 0));
    }

    #[test]
    fn has_preceding_doc_comment_inner_doc() {
        let lines = vec!["//! Module docs", "pub fn foo() {}"];
        assert!(has_preceding_doc_comment(&lines, 1));
    }

    #[test]
    fn find_undocumented_pub_items_empty() {
        let dir = std::env::temp_dir().join("rvtest_arch_pub_empty");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let items = find_undocumented_pub_items(&dir);
        assert!(items.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn find_undocumented_pub_items_no_pub() {
        let dir = std::env::temp_dir().join("rvtest_arch_pub_nopub");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("lib.rs"), "fn private() {}").unwrap();
        let items = find_undocumented_pub_items(&dir);
        assert!(items.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn find_undocumented_pub_items_documented() {
        let dir = std::env::temp_dir().join("rvtest_arch_pub_docd");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("lib.rs"), "/// documented\npub fn foo() {}").unwrap();
        let items = find_undocumented_pub_items(&dir);
        assert!(items.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn find_undocumented_pub_items_undocumented() {
        let dir = std::env::temp_dir().join("rvtest_arch_pub_undoc");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("lib.rs"), "pub fn foo() {}").unwrap();
        let items = find_undocumented_pub_items(&dir);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].1.name, "foo");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
