use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime};

use rvtest::core::{TestCase, TestRun, TestStatus, TestSuite};

use super::cache;
use super::tune;

pub fn build_test_binaries(
    fast: bool,
    cranelift: bool,
    parallel_frontend: Option<usize>,
    workspace: bool,
) -> Vec<std::path::PathBuf> {
    if cache::is_build_cache_enabled()
        && let Ok(hash) = cache::source_hash_for_build()
        && let Some((cached_hash, cached_bins)) = cache::load_build_cache()
        && cached_hash == hash
        && !cached_bins.is_empty()
    {
        eprintln!("  Using cached test binaries (source hash unchanged).");
        return cached_bins;
    }

    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--tests", "--message-format=json"]);
    if workspace {
        cmd.arg("--workspace");
    }

    let mut extra_rustflags: Vec<String> = Vec::new();
    if fast {
        cmd.env("CARGO_PROFILE_DEV_DEBUG", "0");
        if let Some(linker) = tune::detect_fast_linker() {
            extra_rustflags.push(format!("-C link-arg=-fuse-ld={linker}"));
        }
    }
    if cranelift {
        extra_rustflags.push("-Zcodegen-backend=cranelift".to_owned());
    }
    if let Some(n) = parallel_frontend {
        extra_rustflags.push(format!("-Zthreads={n}"));
    }
    if !extra_rustflags.is_empty() {
        let extra = extra_rustflags.join(" ");
        let existing = std::env::var_os("RUSTFLAGS");
        let merged = match existing {
            Some(ref val) if !val.is_empty() => format!("{} {}", val.to_str().unwrap_or(""), extra),
            None | Some(_) => extra,
        };
        cmd.env("RUSTFLAGS", merged);
    }

    let output = match cmd.stdout(Stdio::piped()).stderr(Stdio::inherit()).output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Build failed: {e}");
            return Vec::new();
        }
    };

    if !output.status.success() {
        return Vec::new();
    }

    let binaries = parse_test_binaries_from_json(&String::from_utf8_lossy(&output.stdout));

    if cache::is_build_cache_enabled()
        && !binaries.is_empty()
        && let Ok(hash) = cache::source_hash_for_build()
    {
        cache::save_build_cache(&hash, &binaries);
    }

    binaries
}

fn parse_test_binaries_from_json(output: &str) -> Vec<std::path::PathBuf> {
    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    struct ArtifactMessage {
        reason: String,
        package_id: Option<String>,
        target: Option<TargetInfo>,
        filenames: Option<Vec<String>>,
    }

    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    struct TargetInfo {
        kind: Vec<String>,
        name: String,
    }

    let mut binaries: Vec<std::path::PathBuf> = Vec::new();
    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let msg: ArtifactMessage = match serde_json::from_str(line) {
            Ok(m) => m,
            Err(_) => continue,
        };
        if msg.reason != "compiler-artifact" {
            continue;
        }
        let is_test = msg
            .target
            .as_ref()
            .is_some_and(|t| t.kind.iter().any(|k| k == "test" || k == "bench"));
        if !is_test {
            continue;
        }
        if let Some(filenames) = msg.filenames {
            for f in filenames {
                let path = std::path::PathBuf::from(f);
                if binaries.iter().all(|b| b != &path) {
                    binaries.push(path);
                }
            }
        }
    }
    binaries
}

fn list_tests_in_binary(binary: &std::path::Path) -> Vec<String> {
    let output = match Command::new(binary).arg("--list").output() {
        Ok(o) => o,
        Err(_) => return Vec::new(),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut tests = Vec::new();
    for line in stdout.lines() {
        let trimmed = line.trim();
        if let Some((name, _kind)) = trimmed.rsplit_once(": ")
            && !name.is_empty()
        {
            tests.push(name.to_owned());
        }
    }
    tests
}

pub fn run_tests_isolated(
    filter: Option<&str>,
    skip: Option<&str>,
    fast: bool,
    cranelift: bool,
    parallel_frontend: Option<usize>,
    workspace: bool,
) -> TestRun {
    let start = SystemTime::now();
    let wall_start = Instant::now();

    eprintln!("  Building test binaries...");
    let binaries = build_test_binaries(fast, cranelift, parallel_frontend, workspace);
    if binaries.is_empty() {
        eprintln!("  No test binaries found.");
        return TestRun::new();
    }
    eprintln!("  {} test binary(ies) built.", binaries.len());

    let mut all_suites: Vec<TestSuite> = Vec::new();

    for binary in &binaries {
        if !binary.exists() {
            continue;
        }

        let all_tests = list_tests_in_binary(binary);
        if all_tests.is_empty() {
            continue;
        }

        let suite_name = format!("isolated ({})", binary.display());
        let mut suite = TestSuite::new(&suite_name);

        for test_name in &all_tests {
            if let Some(f) = filter
                && !test_name.to_lowercase().contains(&f.to_lowercase())
            {
                let skipped_case = TestCase {
                    name: test_name.clone(),
                    suite: Some(suite_name.clone()),
                    tags: Vec::new(),
                    status: TestStatus::Skipped {
                        reason: Some("filtered out".into()),
                    },
                    duration: Duration::ZERO,
                    assertions: 0,
                    location: None,
                    parameters: Vec::new(),
                    captured_output: None,
                    bench_stats: None,
                    bench_threshold: None,
                };
                suite.tests.push(skipped_case);
                continue;
            }
            if let Some(s) = skip
                && test_name.to_lowercase().contains(&s.to_lowercase())
            {
                let skipped_case = TestCase {
                    name: test_name.clone(),
                    suite: Some(suite_name.clone()),
                    tags: Vec::new(),
                    status: TestStatus::Skipped {
                        reason: Some("skipped by pattern".into()),
                    },
                    duration: Duration::ZERO,
                    assertions: 0,
                    location: None,
                    parameters: Vec::new(),
                    captured_output: None,
                    bench_stats: None,
                    bench_threshold: None,
                };
                suite.tests.push(skipped_case);
                continue;
            }

            let test_start = Instant::now();
            let output = match Command::new(binary)
                .arg(test_name)
                .arg("--nocapture")
                .output()
            {
                Ok(o) => o,
                Err(e) => {
                    suite.tests.push(TestCase {
                        name: test_name.clone(),
                        suite: Some(suite_name.clone()),
                        tags: Vec::new(),
                        status: TestStatus::Failed {
                            reason: format!("process error: {e}"),
                            location: None,
                        },
                        duration: test_start.elapsed(),
                        assertions: 0,
                        location: None,
                        parameters: Vec::new(),
                        captured_output: None,
                        bench_stats: None,
                        bench_threshold: None,
                    });
                    continue;
                }
            };
            let duration = test_start.elapsed();

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            let status = if output.status.success() {
                TestStatus::Passed
            } else {
                let reason = extract_failure_reason(&stdout, &stderr, test_name);
                TestStatus::Failed {
                    reason,
                    location: None,
                }
            };

            let captured = {
                let mut parts: Vec<String> = Vec::new();
                if !stdout.is_empty() {
                    parts.push(format!("stdout:\n{}", stdout));
                }
                if !stderr.is_empty() {
                    parts.push(format!("stderr:\n{}", stderr));
                }
                if parts.is_empty() {
                    None
                } else {
                    Some(parts.join("\n"))
                }
            };

            suite.tests.push(TestCase {
                name: test_name.clone(),
                suite: Some(suite_name.clone()),
                tags: Vec::new(),
                status,
                duration,
                assertions: 0,
                location: None,
                parameters: Vec::new(),
                captured_output: captured,
                bench_stats: None,
                bench_threshold: None,
            });
        }

        all_suites.push(suite);
    }

    TestRun {
        suites: all_suites,
        start_time: start,
        end_time: SystemTime::now(),
        duration: wall_start.elapsed(),
    }
}

fn extract_failure_reason(stdout: &str, stderr: &str, test_name: &str) -> String {
    let combined = format!("{stdout}\n{stderr}");
    for line in combined.lines() {
        let trimmed = line.trim();
        if trimmed.contains("FAILED") && trimmed.contains(test_name) {
            let mut context = String::new();
            let mut capture = false;
            let mut lines_found = 0;
            for l in combined.lines() {
                if l.trim().contains("FAILED") && l.contains(test_name) {
                    capture = true;
                    continue;
                }
                if capture {
                    if l.trim().starts_with("test ") && l.contains("...") {
                        break;
                    }
                    if l.trim() == "failures:" {
                        break;
                    }
                    if lines_found > 0 {
                        context.push_str(l.trim());
                        context.push('\n');
                    }
                    lines_found += 1;
                    if lines_found > 20 {
                        break;
                    }
                }
            }
            if !context.is_empty() {
                return context.trim().to_owned();
            }
        }
    }

    for line in combined.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("panicked at")
            || trimmed.starts_with("error:")
            || trimmed.starts_with("thread '")
        {
            return trimmed.to_owned();
        }
    }

    "test failed (see output above)".to_owned()
}
