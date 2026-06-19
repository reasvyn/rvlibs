//! CLI subcommand implementations for `cargo rvtest`.
//!
//! # Submodules
//!
//! - [`args`] — CLI argument struct ([`Cli`])
//! - [`watch`] — File-watch mode (`--watch`)
//! - [`flaky`] — Flaky test detection (`--detect-flaky`)

pub mod args;
pub mod flaky;
pub mod watch;

use std::io::{self, IsTerminal, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use sha2::Digest;

use rvtest::core::{ColorChoice, CoverageFormat, ReportFormat, TestCase, TestRun, TestStatus, TestSuite};
use rvtest::coverage::{CoverageCollector, CoverageConfig};
use rvtest::report::{self, TestReporter};
use rvtest::runner::parse_cargo_test_output;
use rvtest::tag;

use args::Cli;

/// Resolve `--profile` from CLI flag or `RVTEST_PROFILE` env var.
pub fn resolve_profile(args: &mut Cli) {
    let name = args.profile.clone()
        .or_else(|| std::env::var("RVTEST_PROFILE").ok().filter(|s| !s.is_empty()));

    let Some(name) = name else { return };

    match name.as_str() {
        "ci" => {
            if args.format == "pretty" {
                args.format = "junit".into();
            }
            args.fail_fast = true;
            args.verbose = false;
            args.show_output = false;
        }
        "dev" => {
            if args.format == "pretty" {
                args.format = "pretty".into();
            }
            args.verbose = true;
        }
        other => {
            eprintln!("warning: unknown profile '{other}', ignoring");
        }
    }
}

/// Resolve colour choice from CLI flag, `CARGO_TERM_COLOR`, or terminal detection.
pub fn resolve_color(cli_color: Option<&str>) -> ColorChoice {
    if let Some(c) = cli_color {
        return c.parse().unwrap_or(ColorChoice::Auto);
    }
    if let Ok(val) = std::env::var("CARGO_TERM_COLOR") {
        match val.as_str() {
            "always" => return ColorChoice::Always,
            "never" => return ColorChoice::Never,
            _ => {}
        }
    }
    ColorChoice::Auto
}

/// Returns `true` if ANSI colour should be used based on the resolved choice.
pub fn use_color(color: ColorChoice) -> bool {
    match color {
        ColorChoice::Always => true,
        ColorChoice::Never => false,
        ColorChoice::Auto => io::stdout().is_terminal(),
    }
}

pub fn coloured_str(s: &str, code: &str, enabled: bool) -> String {
    if enabled {
        format!("\x1b[{code}m{s}\x1b[0m")
    } else {
        s.to_owned()
    }
}

pub fn dim(s: &str) -> String {
    format!("\x1b[2m{s}\x1b[0m")
}

/// Parse `use` statements from a Rust file to extract module paths.
fn parse_use_statements(content: &str) -> Vec<String> {
    let mut modules = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("use ") {
            // Extract the module path: `use crate::foo::bar;` → `foo`
            let use_path = trimmed
                .strip_prefix("use ")
                .and_then(|s| s.strip_suffix(';'))
                .unwrap_or(trimmed);
            // Split on `::` and take the first meaningful segment
            let parts: Vec<&str> = use_path.split("::").collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == "crate" || *part == "self" || *part == "super" {
                    if let Some(next) = parts.get(i + 1)
                        && !next.starts_with('{') && *next != "self" {
                            modules.push(next.to_string());
                        }
                } else if i == 0 && !part.starts_with('{') && !part.starts_with('#') {
                    // Direct dependency path
                    modules.push((*part).to_string());
                }
            }
        }
    }
    modules
}

/// Perform impact analysis: given changed files, determine affected test modules.
///
/// This goes beyond simple name matching by parsing `use` statements in
/// changed files to find which modules/crates they depend on, then
/// mapping those to test names.
fn impact_analysis(filter: Option<&str>, skip: Option<&str>) -> Option<String> {
    let output = Command::new("git")
        .args(["diff", "--name-only", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let files: Vec<&str> = stdout.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    if files.is_empty() {
        return None;
    }

    let mut terms: Vec<String> = Vec::new();

    for file in &files {
        // Direct name-based terms (existing behavior)
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
            && let Some(crate_name) = stripped.split('/').next() {
                terms.push(crate_name.to_owned());
            }

        // Import analysis: parse `use` statements to find affected modules
        if file.ends_with(".rs")
            && let Ok(content) = std::fs::read_to_string(file) {
                let use_modules = parse_use_statements(&content);
                let new_modules: Vec<String> = use_modules.into_iter()
                    .filter(|m| !terms.contains(m))
                    .collect();
                terms.extend(new_modules);
            }
    }

    // Apply user's filter if provided
    if let Some(f) = filter {
        terms.retain(|t| t.to_lowercase().contains(&f.to_lowercase()));
    }

    // Apply skip pattern
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

/// Run `git diff --name-only HEAD` and derive a test filter from changed files.
/// Falls back to the old simple matching if impact analysis is not requested.
pub fn git_changed_filter() -> Option<String> {
    impact_analysis(None, None)
}

/// Run impact analysis with explicit filter/skip patterns.
pub fn git_impact_filter(filter: Option<&str>, skip: Option<&str>) -> Option<String> {
    impact_analysis(filter, skip)
}

/// Returns `true` if the current toolchain is nightly Rust.
pub fn is_nightly() -> bool {
    let output = Command::new("rustc").arg("--version").output().ok();
    match output {
        Some(o) if o.status.success() => {
            let s = String::from_utf8_lossy(&o.stdout);
            s.contains("nightly")
        }
        _ => false,
    }
}

/// Returns `true` if the Cranelift codegen component is installed.
pub fn has_cranelift_component() -> bool {
    let mut cmd = Command::new("rustc");
    cmd.args(["-Zcodegen-backend=cranelift", "--version"]);
    cmd.stdout(Stdio::null()).stderr(Stdio::null());
    cmd.status().map(|s| s.success()).unwrap_or(false)
}

/// Auto-detect hardware and print optimal settings.
pub fn auto_tune() {
    let cpus = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let ram_mb = detect_ram_mb();

    println!("  System:");
    println!("    CPUs:      {cpus}");
    println!("    RAM:       {} MB", ram_mb.map(|m| m.to_string()).unwrap_or("unknown".into()));

    println!("  Recommendations:");
    if let Some(linker) = detect_fast_linker() {
        println!("    Use:       cargo rvtest --fast");
        println!("    (fast linker: {linker})");
    }
    if ram_mb.unwrap_or(0) >= 4096 {
        println!("    Parallel:  cargo rvtest --max-threads {} --cache", cpus.saturating_sub(1));
    } else {
        println!("    Sequential: cargo rvtest --no-parallel");
    }
    if ram_mb.is_none() || ram_mb.unwrap_or(0) < 4096 {
        println!("    Incremental: set CARGO_INCREMENTAL=1");
    }
    if ram_mb.unwrap_or(0) >= 16000 {
        println!("    Ramdisk:   consider TARGET_DIR=/dev/shm/rust-target (16+ GB RAM detected)");
    }
}

fn detect_ram_mb() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        let content = std::fs::read_to_string("/proc/meminfo").ok()?;
        for line in content.lines() {
            if let Some(rest) = line.strip_prefix("MemTotal:") {
                let parts: Vec<&str> = rest.split_whitespace().collect();
                let kb: u64 = parts.first()?.parse().ok()?;
                return Some(kb / 1024);
            }
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = 0;
    }
    None
}

/// Detect the best available fast linker.
pub fn detect_fast_linker() -> Option<&'static str> {
    if Command::new("mold").arg("--version").stdout(Stdio::null()).stderr(Stdio::null()).status().is_ok() {
        return Some("mold");
    }
    if Command::new("ld.lld").arg("--version").stdout(Stdio::null()).stderr(Stdio::null()).status().is_ok() {
        return Some("lld");
    }
    None
}

/// Run test binaries directly (bypass `cargo test`) and parse output.
#[allow(dead_code)]
fn run_binaries_directly(binaries: &[std::path::PathBuf], filter: Option<&str>, skip: Option<&str>) -> TestRun {
    let start = SystemTime::now();
    let wall_start = Instant::now();
    let mut all_suites = Vec::new();

    for binary in binaries {
        if !binary.exists() { continue; }
        let mut cmd = Command::new(binary);
        if let Some(f) = filter {
            cmd.arg(f);
        }
        let output = match cmd.output() {
            Ok(o) => o,
            Err(_) => continue,
        };
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut suites = parse_cargo_test_output(&stderr, &stdout);

        if let Some(sp) = skip {
            for suite in &mut suites {
                suite.tests.retain(|t| !tag::name_skipped(&t.name, Some(sp)));
            }
        }
        all_suites.extend(suites);
    }

    TestRun {
        suites: all_suites,
        start_time: start,
        end_time: SystemTime::now(),
        duration: wall_start.elapsed(),
    }
}

/// Run `cargo test` and parse the output into a structured [`TestRun`].
pub fn run_cargo_test(filter: Option<&str>, fast: bool, cranelift: bool, parallel_frontend: Option<usize>, workspace: bool, skip: Option<&str>) -> TestRun {
    // Check for warm binaries first — run them directly without invoking cargo test
    if let Some(binaries) = try_warm_binaries() {
        return run_binaries_directly(&binaries, filter, skip);
    }

    let start = SystemTime::now();
    let wall_start = Instant::now();

    let mut cmd = Command::new("cargo");
    cmd.arg("test").arg("--color=never");
    if workspace {
        cmd.arg("--workspace");
    }

    let mut extra_rustflags: Vec<String> = Vec::new();

    if fast {
        cmd.env("CARGO_PROFILE_DEV_DEBUG", "0");
        if let Some(linker) = detect_fast_linker() {
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

    // Track timing phases if --why-slow is active
    let why_slow = std::env::var("RVTEST_WHY_SLOW").is_ok();
    let build_start = if why_slow { Some(Instant::now()) } else { None };

    if let Some(f) = filter {
        cmd.arg("--").arg(f);
    }

    let is_tty = io::stdout().is_terminal();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let spinner_handle = std::thread::spawn(move || {
        if !is_tty {
            r.store(false, Ordering::SeqCst);
            return;
        }
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let mut i = 0;
        while r.load(Ordering::SeqCst) {
            print!("\r  {} {}  {} running...", frames[i], dim("cargo test"), dim("tests"));
            io::stdout().flush().ok();
            i = (i + 1) % frames.len();
            std::thread::sleep(Duration::from_millis(80));
        }
    });

    let output = match cmd.output() {
        Ok(o) => {
            running.store(false, Ordering::SeqCst);
            let _ = spinner_handle.join();
            if is_tty {
                print!("\r");
                io::stdout().flush().ok();
            }
            o
        }
        Err(e) => {
            running.store(false, Ordering::SeqCst);
            let _ = spinner_handle.join();
            if is_tty {
                print!("\r");
                io::stdout().flush().ok();
            }
            eprintln!("Error: failed to run `cargo test`: {e}");
            std::process::exit(1);
        }
    };

    let duration = wall_start.elapsed();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut suites = parse_cargo_test_output(&stderr, &stdout);

    if let Some(skip_pattern) = skip
        && !skip_pattern.is_empty() {
            for suite in &mut suites {
                suite.tests.retain(|t| !tag::name_skipped(&t.name, Some(skip_pattern)));
            }
        }

    if why_slow {
        let build_dur = build_start.map(|s| s.elapsed());
        let total: std::time::Duration = suites.iter().flat_map(|s| s.tests.iter()).map(|t| t.duration).sum();
        let exec_dur = duration;

        eprintln!("\n  ⏱  Why Slow — Time Breakdown:");
        if let Some(b) = build_dur {
            eprintln!("     Cargo test (build + exec):  {:.2}s", b.as_secs_f64());
        }
        eprintln!("     Execution (all tests):       {:.2}s", exec_dur.as_secs_f64());
        eprintln!("     Sum of individual tests:      {:.2}s", total.as_secs_f64());
        eprintln!("     Overhead:                     {:.2}s", exec_dur.as_secs_f64() - total.as_secs_f64());

        // Show slowest tests
        let mut all: Vec<&rvtest::core::TestCase> = suites.iter().flat_map(|s| s.tests.iter()).collect();
        all.sort_by_key(|t| std::cmp::Reverse(t.duration));
        eprintln!("\n     Slowest 5 tests:");
        for (i, test) in all.iter().take(5).enumerate() {
            let dur_s = test.duration.as_secs_f64();
            eprintln!("       {}. {:.2}s  {}", i + 1, dur_s, test.name);
        }
        eprintln!();
    }

    TestRun {
        suites,
        start_time: start,
        end_time: SystemTime::now(),
        duration,
    }
}

/// Open a file or URL in the system browser.
pub fn open_in_browser(path: &str) {
    let _ = std::process::Command::new("xdg-open").arg(path).spawn();
    let _ = std::process::Command::new("open").arg(path).spawn(); // macOS fallback
    let _ = std::process::Command::new("cmd").args(["/c", "start", path]).spawn(); // Windows fallback
}

pub fn run_coverage(args: &Cli) -> ! {
    let cov_format: CoverageFormat = args.coverage_format.parse().unwrap_or_else(|e| {
        eprintln!("{e}, falling back to 'summary'");
        CoverageFormat::Summary
    });

    let cov_config = CoverageConfig {
        enabled: true,
        format: cov_format,
        output_dir: args.coverage_dir.clone(),
        min_threshold: args.coverage_min,
        open_report: args.coverage_open,
        ..Default::default()
    };

    let collector = CoverageCollector::new(cov_config);
    match collector.collect() {
        Ok(report) => {
            println!(
                "Coverage: {:.1}% lines, {:.1}% functions, {:.1}% regions",
                report.line_coverage,
                report.function_coverage,
                report.region_coverage,
            );
            if let Some(path) = &report.report_path {
                println!("Report: {}", path.display());
            }
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Coverage collection failed:\n{e}");
            std::process::exit(1);
        }
    }
}

pub fn render(format: &ReportFormat, run: &TestRun, slow_count: usize, use_colour: bool) -> String {
    let reporter: Box<dyn TestReporter> = match format {
        ReportFormat::Pretty => Box::new(report::PrettyReporter::new().colour(use_colour)),
        ReportFormat::Tap => Box::new(report::TapReporter),
        ReportFormat::Junit => Box::new(report::JunitReporter::new()),
        ReportFormat::Json => Box::new(report::JsonReporter),
        ReportFormat::Compact => Box::new(report::CompactReporter),
        ReportFormat::Github => Box::new(report::GithubReporter),
        ReportFormat::Agent => Box::new(report::AgentReporter),
        ReportFormat::Html => Box::new(report::HtmlReporter),
        ReportFormat::Nextest => Box::new(report::NextestReporter),
    };
    let mut out = reporter.report(run);

    if slow_count > 0 {
        let slow = run.slowest(slow_count);
        if !slow.is_empty() {
            use std::fmt::Write;
            let _ = writeln!(out);
            let _ = writeln!(out, "  {} Slowest tests", dim("⏱"));
            for (i, test) in slow.iter().enumerate() {
                let dur = report::format_duration(test.duration);
                let name = test.name.replace(" :: ", " > ");
                let _ = writeln!(out, "    {}.  {:>8}  {}", i + 1, dur, name);
            }
        }
    }

    out
}

#[allow(clippy::too_many_arguments)]
fn run_and_print(filter: &Option<String>, format: &ReportFormat, fast: bool, slow_count: usize, cranelift: bool, parallel_frontend: Option<usize>, skip: Option<String>, use_colour: bool) {
    let run = run_cargo_test(filter.as_deref(), fast, cranelift, parallel_frontend, false, skip.as_deref());
    let report = render(format, &run, slow_count, use_colour);
    println!("{report}");
}

fn build_cache_dir() -> std::path::PathBuf {
    std::path::PathBuf::from("target/.rvtest-build-cache")
}

fn build_cache_path() -> std::path::PathBuf {
    build_cache_dir().join("manifest.json")
}

/// Compute a hash of source files to use as the build cache key.
fn source_hash_for_build() -> Result<String, String> {
    let mut hasher = sha2::Sha256::new();
    let src_dirs = ["src", "tests"];
    for dir in &src_dirs {
        let dir_path = std::path::Path::new(dir);
        if dir_path.exists() {
            rvtest::core::hash_dir_recursive(&mut hasher, dir_path)?;
        }
    }
    Ok(format!("{:x}", Digest::finalize(hasher)))
}

/// Load the build cache manifest.
fn load_build_cache() -> Option<(String, Vec<std::path::PathBuf>)> {
    let path = build_cache_path();
    let content = std::fs::read_to_string(path).ok()?;
    let manifest: serde_json::Value = serde_json::from_str(&content).ok()?;
    let hash = manifest.get("hash")?.as_str()?.to_owned();
    let bins: Vec<std::path::PathBuf> = manifest
        .get("binaries")?
        .as_array()?
        .iter()
        .filter_map(|v| v.as_str().map(std::path::PathBuf::from))
        .filter(|p| p.exists())
        .collect();
    if bins.is_empty() { return None; }
    Some((hash, bins))
}

/// Save the build cache manifest.
fn save_build_cache(hash: &str, binaries: &[std::path::PathBuf]) {
    let manifest = serde_json::json!({
        "hash": hash,
        "binaries": binaries.iter().map(|b| b.to_string_lossy()).collect::<Vec<_>>(),
    });
    let dir = build_cache_dir();
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(build_cache_path(), serde_json::to_string_pretty(&manifest).unwrap_or_default());
}

/// Enable or disable build caching via a static flag.
static BUILD_CACHE_ENABLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Enable or disable build caching globally.
pub fn set_build_cache_enabled(enabled: bool) {
    BUILD_CACHE_ENABLED.store(enabled, std::sync::atomic::Ordering::SeqCst);
}

/// Returns `true` if build caching is enabled.
pub fn is_build_cache_enabled() -> bool {
    BUILD_CACHE_ENABLED.load(std::sync::atomic::Ordering::SeqCst)
}

fn warm_state_path() -> std::path::PathBuf {
    std::path::PathBuf::from("target/.rvtest-warm/state.json")
}

/// Check if a warm daemon has left behind cached binaries.
pub fn try_warm_binaries() -> Option<Vec<std::path::PathBuf>> {
    let path = warm_state_path();
    let content = std::fs::read_to_string(path).ok()?;
    let state: serde_json::Value = serde_json::from_str(&content).ok()?;
    let binaries: Vec<std::path::PathBuf> = state["binaries"]
        .as_array()?
        .iter()
        .filter_map(|v| v.as_str().map(std::path::PathBuf::from))
        .filter(|p| p.exists())
        .collect();
    if binaries.is_empty() { return None; }

    // Verify source hash hasn't changed
    if let Ok(hash) = source_hash_for_build()
        && state.get("hash")?.as_str()? == hash {
            return Some(binaries);
        }
    None
}

/// Save warm daemon state so future invocations can reuse binaries.
pub fn save_warm_state(binaries: &[std::path::PathBuf]) {
    if let Ok(hash) = source_hash_for_build() {
        let state = serde_json::json!({
            "hash": hash,
            "binaries": binaries.iter().map(|b| b.to_string_lossy()).collect::<Vec<_>>(),
        });
        let path = warm_state_path();
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = std::fs::write(&path, serde_json::to_string_pretty(&state).unwrap_or_default());
    }
}

/// Build test binaries and return their paths by parsing `cargo build --message-format=json`.
/// Uses a build cache when enabled to skip recompilation if sources haven't changed.
pub fn build_test_binaries(fast: bool, cranelift: bool, parallel_frontend: Option<usize>, workspace: bool) -> Vec<std::path::PathBuf> {
    // Check build cache first
    if is_build_cache_enabled()
        && let Ok(hash) = source_hash_for_build()
            && let Some((cached_hash, cached_bins)) = load_build_cache()
                && cached_hash == hash && !cached_bins.is_empty() {
                    eprintln!("  Using cached test binaries (source hash unchanged).");
                    return cached_bins;
                }

    use std::process::{Command, Stdio};

    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--tests", "--message-format=json"]);
    if workspace {
        cmd.arg("--workspace");
    }

    let mut extra_rustflags: Vec<String> = Vec::new();
    if fast {
        cmd.env("CARGO_PROFILE_DEV_DEBUG", "0");
        if let Some(linker) = detect_fast_linker() {
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

    // Cache the build result
    if is_build_cache_enabled() && !binaries.is_empty()
        && let Ok(hash) = source_hash_for_build() {
            save_build_cache(&hash, &binaries);
        }

    binaries
}

/// Parse test binary paths from `cargo build --message-format=json` output.
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
        let is_test = msg.target.as_ref().is_some_and(|t| {
            t.kind.iter().any(|k| k == "test" || k == "bench")
        });
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

/// List all test names from a test binary by running it with `--list`.
fn list_tests_in_binary(binary: &std::path::Path) -> Vec<String> {
    use std::process::Command;

    let output = match Command::new(binary).arg("--list").output() {
        Ok(o) => o,
        Err(_) => return Vec::new(),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut tests = Vec::new();
    for line in stdout.lines() {
        // Lines look like: "test_name: test" or "test_name: bench"
        let trimmed = line.trim();
        if let Some((name, _kind)) = trimmed.rsplit_once(": ")
            && !name.is_empty() {
                tests.push(name.to_owned());
            }
    }
    tests
}

/// Run tests with process-per-test isolation.
///
/// Builds all test binaries, discovers test names via `--list`,
/// then runs each test in its own child process.
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
            // Apply filter/skip
            if let Some(f) = filter
                && !test_name.to_lowercase().contains(&f.to_lowercase()) {
                    let skipped_case = TestCase {
                        name: test_name.clone(),
                        suite: Some(suite_name.clone()),
                        tags: Vec::new(),
                        status: TestStatus::Skipped { reason: Some("filtered out".into()) },
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
                && test_name.to_lowercase().contains(&s.to_lowercase()) {
                    let skipped_case = TestCase {
                        name: test_name.clone(),
                        suite: Some(suite_name.clone()),
                        tags: Vec::new(),
                        status: TestStatus::Skipped { reason: Some("skipped by pattern".into()) },
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
            let output = match std::process::Command::new(binary)
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
                        status: TestStatus::Failed { reason: format!("process error: {e}"), location: None },
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

            // Parse individual test result from output
            let status = if output.status.success() {
                TestStatus::Passed
            } else {
                let reason = extract_failure_reason(&stdout, &stderr, test_name);
                TestStatus::Failed { reason, location: None }
            };

            let captured = {
                let mut parts: Vec<String> = Vec::new();
                if !stdout.is_empty() {
                    parts.push(format!("stdout:\n{}", stdout));
                }
                if !stderr.is_empty() {
                    parts.push(format!("stderr:\n{}", stderr));
                }
                if parts.is_empty() { None } else { Some(parts.join("\n")) }
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

/// Extract failure reason from test output by finding the relevant section.
fn extract_failure_reason(stdout: &str, stderr: &str, test_name: &str) -> String {
    // Try to find the failure section in stdout
    let combined = format!("{stdout}\n{stderr}");
    for line in combined.lines() {
        let trimmed = line.trim();
        if trimmed.contains("FAILED") && trimmed.contains(test_name) {
            // Return surrounding context — the lines following this one up to the next test
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

    // Fallback: look for panic messages
    for line in combined.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("panicked at") || trimmed.starts_with("error:") || trimmed.starts_with("thread '") {
            return trimmed.to_owned();
        }
    }

    "test failed (see output above)".to_owned()
}
