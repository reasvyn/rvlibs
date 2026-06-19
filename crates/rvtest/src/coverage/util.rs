use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use crate::core::CoverageReport;

use super::CoverageConfig;

pub fn which(name: &str) -> Option<PathBuf> {
    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        let candidate = dir.join(name);
        if candidate.exists() {
            return Some(candidate);
        }
        if cfg!(windows) {
            let candidate_exe = dir.join(format!("{name}.exe"));
            if candidate_exe.exists() {
                return Some(candidate_exe);
            }
        }
    }
    None
}

pub fn glob_dir(dir: &Path, pattern: &str) -> Result<Vec<PathBuf>, String> {
    let mut results = Vec::new();
    let entries = std::fs::read_dir(dir).map_err(|e| format!("read_dir {dir:?}: {e}"))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("entry: {e}"))?;
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(OsStr::to_str)
            && name.contains(pattern.trim_end_matches('*'))
        {
            results.push(path);
        }
    }
    Ok(results)
}

pub fn parse_coverage_percentages(summary: &str) -> (f64, f64, f64) {
    let mut line = 0.0;
    let mut func = 0.0;
    let mut region = 0.0;
    for line_text in summary.lines() {
        let t = line_text.trim();
        if t.starts_with("Lines:") || t.starts_with("  Lines:") {
            line = extract_pct(t);
        } else if t.starts_with("Functions:") || t.starts_with("  Functions:") {
            func = extract_pct(t);
        } else if t.starts_with("Regions:") || t.starts_with("  Regions:") {
            region = extract_pct(t);
        }
    }
    (line, func, region)
}

pub fn extract_pct(s: &str) -> f64 {
    if let Some(start) = s.find(|c: char| c.is_ascii_digit()) {
        let rest = &s[start..];
        if let Some(end) = rest.find('%')
            && let Ok(val) = rest[..end].parse::<f64>()
        {
            return val;
        }
    }
    0.0
}

pub fn find_tool(name: &str) -> Option<PathBuf> {
    if let Some(path) = which(name) {
        return Some(path);
    }
    if let Ok(home) = std::env::var("RUSTUP_HOME")
        && let Ok(output) = Command::new("rustup").args(["default"]).output()
            && output.status.success() {
                let tc = String::from_utf8_lossy(&output.stdout).trim().to_owned();
                for sub in ["lib/rustlib/x86_64-unknown-linux-gnu/bin", "bin"] {
                    let p = PathBuf::from(&home).join("toolchains").join(&tc).join(sub).join(name);
                    if p.exists() {
                        return Some(p);
                    }
                }
            }
    None
}

pub fn has_cargo_llvm_cov() -> bool {
    Command::new("cargo")
        .args(["llvm-cov", "--help"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn has_llvm_tools() -> bool {
    find_tool("llvm-profdata").is_some()
        && find_tool("llvm-cov").is_some()
}

pub fn has_addr2line() -> bool {
    find_tool("addr2line").is_some()
}

pub fn self_contained_profraw() -> bool {
    let rustc_version = || -> Option<(u32, u32)> {
        let output = Command::new("rustc").arg("--version").output().ok()?;
        let s = String::from_utf8_lossy(&output.stdout);
        let v = s.split_whitespace().nth(1)?;
        let parts: Vec<&str> = v.split('.').collect();
        let major: u32 = parts.first()?.parse().ok()?;
        let minor: u32 = parts.get(1)?.parse().ok()?;
        Some((major, minor))
    };

    rustc_version()
        .map(|(major, minor)| major >= 1 && minor >= 96)
        .unwrap_or(false)
}

pub fn check_threshold(config: &CoverageConfig, report: CoverageReport) -> Result<CoverageReport, String> {
    if let Some(threshold) = config.min_threshold
        && report.line_coverage < threshold
    {
        return Err(format!(
            "coverage {:.1}% is below minimum {threshold:.1}%",
            report.line_coverage,
        ));
    }

    if config.open_report
        && let Some(ref path) = report.report_path {
            open_in_browser(path);
        }

    Ok(report)
}

pub fn run_cargo_test_no_run(config: &CoverageConfig, llvm_profile: Option<&Path>) -> Result<Output, String> {
    let mut cmd = Command::new("cargo");
    cmd.args(["test", "--no-run", "--message-format=json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());

    if let Some(prof) = llvm_profile {
        cmd.env("CARGO_INCREMENTAL", "0");
        cmd.env("RUSTFLAGS", "-Cinstrument-coverage");
        cmd.env("LLVM_PROFILE_FILE", prof.to_str().unwrap());
    }

    if !config.extra_test_args.is_empty() {
        cmd.arg("--").args(&config.extra_test_args);
    }

    cmd.output().map_err(|e| format!("cargo test --no-run: {e}"))
}

pub fn parse_llvm_cov_summary() -> Result<(f64, f64, f64), String> {
    let output = Command::new("cargo")
        .args(["llvm-cov", "--summary-only", "--all-targets"])
        .output()
        .map_err(|e| format!("cargo-llvm-cov summary: {e}"))?;
    if !output.status.success() {
        return Ok((0.0, 0.0, 0.0));
    }
    Ok(parse_coverage_percentages(&String::from_utf8_lossy(&output.stdout)))
}

#[cfg(target_os = "linux")]
fn open_in_browser(path: &Path) {
    let _ = Command::new("xdg-open").arg(path).status();
}

#[cfg(target_os = "macos")]
fn open_in_browser(path: &Path) {
    let _ = Command::new("open").arg(path).status();
}

#[cfg(target_os = "windows")]
fn open_in_browser(path: &Path) {
    let _ = Command::new("cmd").args(["/c", "start", ""]).arg(path).status();
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn open_in_browser(_path: &Path) {
    eprintln!("--open not supported on this platform");
}
