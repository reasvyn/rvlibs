use crate::core::RunnerConfig;

/// Returns `true` if a test with the given name matches the skip pattern.
///
/// When `skip` is `None`, no tests are skipped via name. Matching is
/// case-insensitive substring comparison.
pub fn name_skipped(name: &str, skip: Option<&str>) -> bool {
    match skip {
        None => false,
        Some(s) => {
            if s.is_empty() {
                return false;
            }
            name.to_lowercase().contains(&s.to_lowercase())
        }
    }
}

/// Returns `true` if a test with the given `tags` should be included in the
/// run according to the filtering rules in `config`.
///
/// A test is included when:
///
/// - Every tag in `config.include_tags` is present in `tags` (AND semantics).
/// - No tag in `config.exclude_tags` is present in `tags`.
/// - The test's name matches `config.filter` (substring match), if set.
pub fn tags_match(tags: &[String], config: &RunnerConfig) -> bool {
    // Include tags: ALL must be present.
    if !config.include_tags.is_empty() {
        for required in &config.include_tags {
            if !tags.iter().any(|t| t == required) {
                return false;
            }
        }
    }

    // Exclude tags: NONE may be present.
    for excluded in &config.exclude_tags {
        if tags.iter().any(|t| t == excluded) {
            return false;
        }
    }

    true
}

/// Returns `true` if a test with the given name passes the filter string.
///
/// When `filter` is `None`, all names match. Matching is case-insensitive
/// substring comparison.
pub fn name_matches(name: &str, filter: Option<&str>) -> bool {
    match filter {
        None => true,
        Some(f) => {
            if f.is_empty() {
                return true;
            }
            name.to_lowercase().contains(&f.to_lowercase())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::RunnerConfig;

    fn config(include: Vec<&str>, exclude: Vec<&str>) -> RunnerConfig {
        RunnerConfig {
            include_tags: include.into_iter().map(String::from).collect(),
            exclude_tags: exclude.into_iter().map(String::from).collect(),
            ..RunnerConfig::default()
        }
    }

    mod tags_match {
        use super::*;

        #[test]
        fn no_tags_no_filters() {
            assert!(tags_match(&[], &config(vec![], vec![])));
        }

        #[test]
        fn matching_include_tag() {
            let tags = vec!["smoke".to_string()];
            assert!(tags_match(&tags, &config(vec!["smoke"], vec![])));
        }

        #[test]
        fn missing_include_tag() {
            let tags = vec!["slow".to_string()];
            assert!(!tags_match(&tags, &config(vec!["smoke"], vec![])));
        }

        #[test]
        fn all_include_tags_must_match() {
            let tags = vec!["smoke".to_string(), "fast".to_string()];
            assert!(tags_match(&tags, &config(vec!["smoke", "fast"], vec![])));
        }

        #[test]
        fn not_all_include_tags_match() {
            let tags = vec!["smoke".to_string()];
            assert!(!tags_match(&tags, &config(vec!["smoke", "fast"], vec![])));
        }

        #[test]
        fn exclude_tag_rejected() {
            let tags = vec!["slow".to_string()];
            assert!(!tags_match(&tags, &config(vec![], vec!["slow"])));
        }

        #[test]
        fn exclude_tag_not_present() {
            let tags = vec!["fast".to_string()];
            assert!(tags_match(&tags, &config(vec![], vec!["slow"])));
        }

        #[test]
        fn include_wins_over_exclude() {
            // include is checked first — if include fails, we never check exclude
            let tags = vec!["slow".to_string()];
            assert!(!tags_match(&tags, &config(vec!["smoke"], vec!["slow"])));
        }

        #[test]
        fn exclude_rejected_even_if_include_matches() {
            let tags = vec!["smoke".to_string(), "slow".to_string()];
            assert!(!tags_match(&tags, &config(vec!["smoke"], vec!["slow"])));
        }

        #[test]
        fn multiple_exclude_tags() {
            let tags = vec!["slow".to_string()];
            assert!(!tags_match(&tags, &config(vec![], vec!["slow", "flaky"])));
        }

        #[test]
        fn empty_tags_with_include() {
            assert!(!tags_match(&[], &config(vec!["smoke"], vec![])));
        }

        #[test]
        fn empty_tags_no_include_no_exclude() {
            assert!(tags_match(&[], &config(vec![], vec![])));
        }
    }

    mod name_skipped_tests {
        use super::*;

        #[test]
        fn no_skip_pattern() {
            assert!(!name_skipped("anything", None));
        }

        #[test]
        fn empty_skip_pattern() {
            assert!(!name_skipped("anything", Some("")));
        }

        #[test]
        fn exact_match_skipped() {
            assert!(name_skipped("slow_test", Some("slow_test")));
        }

        #[test]
        fn substring_match_skipped() {
            assert!(name_skipped("database :: slow :: query", Some("slow")));
        }

        #[test]
        fn case_insensitive_skip() {
            assert!(name_skipped("SLOW_TEST", Some("slow")));
            assert!(name_skipped("slow_test", Some("SLOW")));
        }

        #[test]
        fn no_skip_match() {
            assert!(!name_skipped("fast_test", Some("slow")));
        }
    }

    mod name_matches {
        use super::*;

        #[test]
        fn no_filter() {
            assert!(name_matches("anything", None));
        }

        #[test]
        fn empty_filter_matches_all() {
            assert!(name_matches("anything", Some("")));
        }

        #[test]
        fn exact_match() {
            assert!(name_matches("hello", Some("hello")));
        }

        #[test]
        fn case_insensitive() {
            assert!(name_matches("Hello World", Some("hello")));
            assert!(name_matches("hello world", Some("WORLD")));
        }

        #[test]
        fn substring_match() {
            assert!(name_matches("Calculator :: adds", Some("adds")));
        }

        #[test]
        fn no_match() {
            assert!(!name_matches("Calculator", Some("Database")));
        }

        #[test]
        fn partial_word_no_match() {
            assert!(!name_matches("addition", Some("subtract")));
        }

        #[test]
        fn filter_matches_start() {
            assert!(name_matches("testing framework", Some("test")));
        }

        #[test]
        fn filter_matches_end() {
            assert!(name_matches("rust testing", Some("testing")));
        }
    }
}
