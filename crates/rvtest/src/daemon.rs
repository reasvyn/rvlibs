use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime};

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;

use crate::core::{ReportFormat, TestRun, TestSuite};
use crate::report::{self, TestReporter};
use crate::runner::parse_cargo_test_output;

/// Persistent compile daemon for sub-second test iteration.
///
/// Builds all test binaries once via `cargo build --tests`, watches `src/`
/// and `tests/` for file changes via `notify`, rebuilds incrementally on
/// change, and executes test binaries directly — bypassing `cargo test`
/// overhead for faster feedback.
///
/// Run with `cargo rvtest --daemon`.
pub struct CompileDaemon {
    filter: Option<String>,
    format: ReportFormat,
}

impl CompileDaemon {
    /// Create a new daemon with an optional name filter and output format.
    pub fn new(filter: Option<String>, format: ReportFormat) -> Self {
        CompileDaemon { filter, format }
    }

    /// Run the daemon: build, watch, rebuild, and report in a loop.
    pub fn run(&self) {
        eprintln!("  Building test binaries...");
        let mut binaries = self.build();

        if binaries.is_empty() {
            eprintln!("  No test binaries found.");
            return;
        }

        eprintln!("  {} test binary(ies) ready.", binaries.len());
        self.run_binaries(&binaries);

        let (tx, rx) = std::sync::mpsc::channel::<Result<Event, notify::Error>>();
        let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Error: cannot start file watcher: {e}");
                return;
            }
        };

        for dir in &["src", "tests"] {
            if Path::new(dir).exists() {
                let _ = watcher.watch(Path::new(dir), RecursiveMode::Recursive);
            }
        }

        eprint!("  Watching src/, tests/ for changes...\n\n");

        let debounce = Duration::from_millis(300);
        loop {
            let deadline = Instant::now() + debounce;
            let mut pending = false;

            while Instant::now() < deadline {
                match rx.recv_timeout(Duration::from_millis(50)) {
                    Ok(Ok(_)) => pending = true,
                    Ok(Err(_)) => {}
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }

            if !pending {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }

            eprintln!("  Change detected — rebuilding...\n");

            binaries = self.build();
            if !binaries.is_empty() {
                self.run_binaries(&binaries);
            }

            eprint!("\n  Watching...\n\n");
        }
    }

    fn build(&self) -> Vec<PathBuf> {
        let mut cmd = Command::new("cargo");
        cmd.args(["build", "--tests", "--message-format=json"]);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::inherit());

        let output = match cmd.output() {
            Ok(o) => o,
            Err(e) => {
                eprintln!("  Build failed: {e}");
                return Vec::new();
            }
        };

        if !output.status.success() {
            return Vec::new();
        }

        parse_test_binaries_from_json(&String::from_utf8_lossy(&output.stdout))
    }

    fn run_binaries(&self, binaries: &[PathBuf]) {
        let mut all_suites: Vec<TestSuite> = Vec::new();
        let start = SystemTime::now();
        let wall_start = Instant::now();

        for binary in binaries {
            if !binary.exists() {
                continue;
            }

            let mut cmd = Command::new(binary);
            if let Some(ref filter) = self.filter {
                cmd.arg(filter);
            }

            let output = match cmd.output() {
                Ok(o) => o,
                Err(e) => {
                    eprintln!("  Failed to run {:?}: {e}", binary);
                    continue;
                }
            };

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            let suites = parse_cargo_test_output(&stderr, &stdout);
            all_suites.extend(suites);
        }

        let duration = wall_start.elapsed();

        let run = TestRun {
            suites: all_suites,
            start_time: start,
            end_time: SystemTime::now(),
            duration,
        };

        let reporter: Box<dyn TestReporter> = match self.format {
            ReportFormat::Pretty => Box::new(report::PrettyReporter::new()),
            ReportFormat::Tap => Box::new(report::TapReporter),
            ReportFormat::Junit => Box::new(report::JunitReporter::new()),
            ReportFormat::Json => Box::new(report::JsonReporter),
            ReportFormat::Compact => Box::new(report::CompactReporter),
            ReportFormat::Github => Box::new(report::GithubReporter),
            ReportFormat::Agent => Box::new(report::AgentReporter),
            ReportFormat::Html => Box::new(report::HtmlReporter),
            ReportFormat::Nextest => Box::new(report::NextestReporter),
        };

        let report = reporter.report(&run);
        print!("{report}");
        let _ = std::io::stdout().flush();
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ArtifactMessage {
    reason: String,
    package_id: Option<String>,
    target: Option<TargetInfo>,
    filenames: Option<Vec<String>>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct TargetInfo {
    kind: Vec<String>,
    name: String,
}

fn parse_test_binaries_from_json(output: &str) -> Vec<PathBuf> {
    let mut binaries: Vec<PathBuf> = Vec::new();

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

        let is_test = msg.target.as_ref().is_some_and(|t| {
            t.kind.iter().any(|k| k == "test" || k == "bench")
        });
        if !is_test {
            continue;
        }

        if let Some(filenames) = msg.filenames {
            for f in filenames {
                let path = PathBuf::from(f);
                if binaries.iter().all(|b| b != &path) {
                    binaries.push(path);
                }
            }
        }
    }

    binaries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test_binaries_from_json_empty() {
        let bins = parse_test_binaries_from_json("");
        assert!(bins.is_empty());
    }

    #[test]
    fn parse_test_binaries_from_json_non_json() {
        let bins = parse_test_binaries_from_json("not json\nstill not json");
        assert!(bins.is_empty());
    }

    #[test]
    fn parse_test_binaries_from_json_ignores_compiler_message() {
        let json = r#"{"reason":"compiler-message","package_id":"pkg","target":{"kind":["test"],"name":"foo"},"message":{"rendered":"hello"}}"#;
        let bins = parse_test_binaries_from_json(json);
        assert!(bins.is_empty(), "compiler-message should be ignored");
    }

    #[test]
    fn parse_test_binaries_from_json_ignores_library() {
        let json = r#"{"reason":"compiler-artifact","package_id":"pkg","target":{"kind":["lib"],"name":"foo"},"filenames":["/tmp/libfoo.rlib"]}"#;
        let bins = parse_test_binaries_from_json(json);
        assert!(bins.is_empty(), "library artifacts should be ignored");
    }

    #[test]
    fn parse_test_binaries_from_json_test_binary() {
        let json = r#"{"reason":"compiler-artifact","package_id":"pkg","target":{"kind":["test"],"name":"foo"},"filenames":["/tmp/foo-abc123"]}"#;
        let bins = parse_test_binaries_from_json(json);
        assert_eq!(bins.len(), 1);
        assert_eq!(bins[0], PathBuf::from("/tmp/foo-abc123"));
    }

    #[test]
    fn parse_test_binaries_from_json_dedup() {
        let json = r#"
{"reason":"compiler-artifact","package_id":"pkg","target":{"kind":["test"],"name":"foo"},"filenames":["/tmp/foo-abc"]}
{"reason":"compiler-artifact","package_id":"pkg","target":{"kind":["test"],"name":"foo"},"filenames":["/tmp/foo-abc"]}
"#;
        let bins = parse_test_binaries_from_json(json);
        assert_eq!(bins.len(), 1, "duplicate paths should be deduplicated");
    }

    #[test]
    fn parse_test_binaries_from_json_bench_binary() {
        let json = r#"{"reason":"compiler-artifact","package_id":"pkg","target":{"kind":["bench"],"name":"my_bench"},"filenames":["/tmp/bench-abc"]}"#;
        let bins = parse_test_binaries_from_json(json);
        assert_eq!(bins.len(), 1, "bench targets should be included");
    }
}
