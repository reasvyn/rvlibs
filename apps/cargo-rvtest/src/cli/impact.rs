use std::process::Command;

pub fn parse_use_statements(content: &str) -> Vec<String> {
    let mut modules = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("use ") {
            let use_path = trimmed
                .strip_prefix("use ")
                .and_then(|s| s.strip_suffix(';'))
                .unwrap_or(trimmed);
            let parts: Vec<&str> = use_path.split("::").collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == "crate" || *part == "self" || *part == "super" {
                    if let Some(next) = parts.get(i + 1)
                        && !next.starts_with('{')
                        && *next != "self"
                    {
                        modules.push(next.to_string());
                    }
                } else if i == 0 && !part.starts_with('{') && !part.starts_with('#') {
                    modules.push((*part).to_string());
                }
            }
        }
    }
    modules
}

fn impact_analysis(filter: Option<&str>, skip: Option<&str>) -> Option<String> {
    let output = Command::new("git")
        .args(["diff", "--name-only", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let files: Vec<&str> = stdout
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    if files.is_empty() {
        return None;
    }

    let mut terms: Vec<String> = Vec::new();

    for file in &files {
        if let Some(stripped) = file.strip_prefix("src/") {
            if let Some(module) = stripped.strip_suffix(".rs") {
                let term = module.replace('/', "::");
                terms.push(term);
            }
        } else if let Some(stripped) = file.strip_prefix("tests/") {
            if let Some(module) = stripped.strip_suffix(".rs") {
                terms.push(module.to_owned());
            }
        } else if let Some(stripped) = file.strip_prefix("crates/")
            && let Some(crate_name) = stripped.split('/').next()
        {
            terms.push(crate_name.to_owned());
        }

        if file.ends_with(".rs")
            && let Ok(content) = std::fs::read_to_string(file)
        {
            let use_modules = parse_use_statements(&content);
            let new_modules: Vec<String> = use_modules
                .into_iter()
                .filter(|m| !terms.contains(m))
                .collect();
            terms.extend(new_modules);
        }
    }

    if let Some(f) = filter {
        terms.retain(|t| t.to_lowercase().contains(&f.to_lowercase()));
    }

    if let Some(s) = skip {
        terms.retain(|t| !t.to_lowercase().contains(&s.to_lowercase()));
    }

    if terms.is_empty() {
        return None;
    }

    terms.sort();
    terms.dedup();
    Some(terms.join("|"))
}

pub fn git_changed_filter() -> Option<String> {
    impact_analysis(None, None)
}

pub fn git_impact_filter(filter: Option<&str>, skip: Option<&str>) -> Option<String> {
    impact_analysis(filter, skip)
}
