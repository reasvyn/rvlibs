use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::core::{CoverageFormat, CoverageReport};

use super::CoverageConfig;
use super::util;

pub struct CargoLlvmCovRunner {
    pub config: CoverageConfig,
}

impl CargoLlvmCovRunner {
    pub fn run(&self) -> Result<CoverageReport, String> {
        run_cargo_llvm_cov(&self.config)
    }
}

pub struct LlvmToolsRunner {
    pub config: CoverageConfig,
}

impl LlvmToolsRunner {
    pub fn run(&self) -> Result<CoverageReport, String> {
        run_llvm_tools(&self.config)
    }
}

pub enum CovMode {
    CargoLlvmCov,
    LlvmTools,
    RawProfraw,
    Sampler,
}

pub fn run_cargo_llvm_cov(config: &CoverageConfig) -> Result<CoverageReport, String> {
    let out_dir = &config.output_dir;
    let format_flag = match config.format {
        CoverageFormat::Summary => "--summary-only",
        CoverageFormat::Html => "--html",
        CoverageFormat::Lcov => "--lcov",
        CoverageFormat::Json => "--json",
        CoverageFormat::Cobertura => "--cobertura",
    };

    let mut cmd = Command::new("cargo");
    cmd.args(["llvm-cov", "--all-targets", format_flag]);
    if !matches!(config.format, CoverageFormat::Summary) {
        cmd.arg("--output-dir").arg(out_dir);
    }
    if !config.extra_test_args.is_empty() {
        cmd.arg("--").args(&config.extra_test_args);
    }

    let status = cmd.status().map_err(|e| format!("cargo-llvm-cov: {e}"))?;
    if !status.success() {
        return Err("cargo-llvm-cov returned non-zero exit".into());
    }

    let report_path = if !matches!(config.format, CoverageFormat::Summary) {
        Some(out_dir.join(crate::coverage_raw::report_filename(config.format)))
    } else {
        None
    };
    let (line, func, region) = util::parse_llvm_cov_summary()?;
    util::check_threshold(
        config,
        CoverageReport {
            line_coverage: line,
            function_coverage: func,
            region_coverage: region,
            format: config.format,
            report_path,
        },
    )
}

pub fn run_llvm_tools(config: &CoverageConfig) -> Result<CoverageReport, String> {
    let out_dir = &config.output_dir;
    let profraw_dir = out_dir.join("profraw");
    for d in [&profraw_dir, out_dir] {
        std::fs::create_dir_all(d).map_err(|e| format!("mkdir {d:?}: {e}"))?;
    }

    let llvm_profile = profraw_dir.join("default_%p_%m.profraw");
    let build = util::run_cargo_test_no_run(config, Some(&llvm_profile))?;
    let binaries = crate::coverage_raw::parse_test_binaries(&build.stdout);

    if binaries.is_empty() {
        return Err("no test binaries produced".into());
    }

    for bin in &binaries {
        let s = Command::new(bin)
            .env("LLVM_PROFILE_FILE", llvm_profile.to_str().unwrap())
            .args(&config.extra_test_args)
            .status()
            .map_err(|e| format!("run {bin:?}: {e}"))?;
        if !s.success() {
            eprintln!("warning: {bin:?} exited non-zero");
        }
    }

    let merged = out_dir.join("merged.profdata");
    let profraws = util::glob_dir(&profraw_dir, "*.profraw")?;
    if profraws.is_empty() {
        return Err("no .profraw files produced".into());
    }

    let pdata = util::find_tool("llvm-profdata").unwrap();
    let mut mc = Command::new(&pdata);
    mc.args(["merge", "-sparse"]);
    for f in &profraws {
        mc.arg(f);
    }
    mc.arg("-o").arg(&merged);
    if !mc.status().map_err(|e| format!("llvm-profdata: {e}"))?.success() {
        return Err("llvm-profdata merge failed".into());
    }

    let cov = util::find_tool("llvm-cov").unwrap();
    let report = llvm_cov_report(config, &cov, &merged, &binaries)?;
    util::check_threshold(config, report)
}

fn llvm_cov_report(
    config: &CoverageConfig,
    cov: &Path,
    profdata: &Path,
    bins: &[PathBuf],
) -> Result<CoverageReport, String> {
    match config.format {
        CoverageFormat::Summary => {
            let (l, f, r) = llvm_summary(cov, profdata, bins)?;
            Ok(CoverageReport {
                line_coverage: l,
                function_coverage: f,
                region_coverage: r,
                format: CoverageFormat::Summary,
                report_path: None,
            })
        }
        _ => {
            let (l, f, r) = llvm_summary(cov, profdata, bins)?;
            let filename = crate::coverage_raw::report_filename(config.format);
            let path = config.output_dir.join(&filename);

            let fmt = match config.format {
                CoverageFormat::Html => "html",
                CoverageFormat::Lcov => "lcov",
                CoverageFormat::Json => "text",
                _ => return Err("format requires cargo-llvm-cov".into()),
            };

            let mut cmd = Command::new(cov);
            cmd.args(["show", "--format", fmt])
                .arg("--instr-profile")
                .arg(profdata);
            for b in bins {
                cmd.arg("--object").arg(b);
            }

            let out = cmd
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .output()
                .map_err(|e| format!("llvm-cov: {e}"))?;

            std::fs::write(&path, &out.stdout)
                .map_err(|e| format!("write {path:?}: {e}"))?;

            Ok(CoverageReport {
                line_coverage: l,
                function_coverage: f,
                region_coverage: r,
                format: config.format,
                report_path: Some(path),
            })
        }
    }
}

fn llvm_summary(cov: &Path, profdata: &Path, bins: &[PathBuf]) -> Result<(f64, f64, f64), String> {
    let mut cmd = Command::new(cov);
    cmd.args(["report", "--summary-only", "--use-color=false"])
        .arg("--instr-profile")
        .arg(profdata);
    for b in bins {
        cmd.arg("--object").arg(b);
    }
    let out = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("llvm-cov report: {e}"))?;
    Ok(util::parse_coverage_percentages(
        &String::from_utf8_lossy(&out.stdout),
    ))
}

pub fn run_raw_parser(config: &CoverageConfig) -> Result<CoverageReport, String> {
    let runner = crate::coverage_raw::RawCoverageRunner {
        output_dir: config.output_dir.clone(),
        extra_test_args: config.extra_test_args.clone(),
    };
    runner.run(config.format)
}

#[cfg(target_os = "linux")]
pub fn run_sampler(config: &CoverageConfig) -> Result<CoverageReport, String> {
    if !util::has_addr2line() {
        return Err(
            "built-in sampler requires `addr2line` (install binutils).\n\
             Or install one of:\n  \
             cargo install cargo-llvm-cov\n  \
             rustup component add llvm-tools-preview"
                .into(),
        );
    }

    let out_dir = &config.output_dir;
    std::fs::create_dir_all(out_dir).map_err(|e| format!("mkdir {out_dir:?}: {e}"))?;

    let build = util::run_cargo_test_no_run(config, None)?;
    let binaries = crate::coverage_raw::parse_test_binaries(&build.stdout);
    let binary = binaries.first().ok_or("no test binary produced")?;
    if !binary.exists() {
        return Err(format!("test binary not found: {binary:?}"));
    }

    let samples = sample_ips(binary, config.sample_interval_ms, &config.extra_test_args)?;
    if samples.is_empty() {
        return Err("no instruction pointer samples collected".into());
    }

    let locations = resolve_with_addr2line(binary, &samples)?;

    let total_source = count_source_lines("src")?;
    let unique_hit: HashSet<(String, u64)> = locations.into_iter().collect();
    let hit_count = unique_hit.len();

    let line_cov = if total_source > 0 {
        (hit_count as f64 / total_source as f64 * 100.0).min(100.0)
    } else {
        0.0
    };

    let function_cov = line_cov;

    let report = CoverageReport {
        line_coverage: line_cov,
        function_coverage: function_cov,
        region_coverage: line_cov,
        format: config.format,
        report_path: None,
    };

    println!(
        "\n📊  Built-in sampler coverage (statistical):\n   \
         Lines hit: {hit_count} / {total_source} ({line_cov:.1}%)\n   \
         Samples: {} (interval: {}ms)\n",
        samples.len(),
        config.sample_interval_ms,
    );

    util::check_threshold(config, report)
}

#[cfg(not(target_os = "linux"))]
pub fn run_sampler(_config: &CoverageConfig) -> Result<CoverageReport, String> {
    Err(
        "built-in sampler is only available on Linux.\n\
             Install one of:\n  \
             cargo install cargo-llvm-cov\n  \
             rustup component add llvm-tools-preview"
            .into(),
    )
}

#[cfg(target_os = "linux")]
fn sample_ips(binary: &Path, interval_ms: u64, _extra_args: &[String]) -> Result<Vec<u64>, String> {
    use std::ffi::CString;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    let bin_cstr = CString::new(binary.to_str().ok_or("invalid binary path")?)
        .map_err(|_| "binary path contains null byte")?;

    let child_pid = unsafe { libc::fork() };
    if child_pid == -1 {
        return Err("fork failed".into());
    }

    if child_pid == 0 {
        unsafe {
            libc::ptrace(
                libc::PTRACE_TRACEME,
                0,
                std::ptr::null_mut::<libc::c_void>(),
                std::ptr::null_mut::<libc::c_void>(),
            );
            libc::raise(libc::SIGSTOP);
        }
        let args: Vec<CString> = std::iter::once(bin_cstr.clone())
            .chain(
                _extra_args
                    .iter()
                    .map(|a| CString::new(a.as_bytes()).unwrap()),
            )
            .collect();
        let mut argv: Vec<*const libc::c_char> = args.iter().map(|a| a.as_ptr()).collect();
        argv.push(std::ptr::null());
        unsafe {
            libc::execvp(bin_cstr.as_ptr(), argv.as_ptr());
            libc::_exit(1);
        }
    }

    let mut samples: Vec<u64> = Vec::new();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let timer = std::thread::spawn(move || {
        while r.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(interval_ms));
            if !r.load(Ordering::SeqCst) {
                break;
            }
            unsafe {
                libc::kill(child_pid, libc::SIGSTOP);
            }
        }
    });

    let mut status: libc::c_int = 0;

    unsafe {
        libc::waitpid(child_pid, &mut status as *mut libc::c_int, 0);
    }

    let max_samples = 200_000u64;
    let mut sample_count = 0u64;

    loop {
        if sample_count >= max_samples {
            break;
        }

        unsafe {
            libc::waitpid(child_pid, &mut status as *mut libc::c_int, 0);
        }

        if libc::WIFEXITED(status) || libc::WIFSIGNALED(status) {
            break;
        }

        let stop_signal = if libc::WIFSTOPPED(status) {
            libc::WSTOPSIG(status)
        } else {
            0
        };

        if stop_signal == libc::SIGSTOP {
            let mut regs: libc::user_regs_struct = unsafe { std::mem::zeroed() };
            let mut iov = libc::iovec {
                iov_base: &mut regs as *mut _ as *mut libc::c_void,
                iov_len: std::mem::size_of::<libc::user_regs_struct>(),
            };

            let res = unsafe {
                libc::ptrace(
                    libc::PTRACE_GETREGSET as libc::c_uint,
                    child_pid,
                    libc::NT_PRSTATUS as *mut libc::c_void,
                    &mut iov as *mut libc::iovec as *mut libc::c_void,
                )
            };

            if res == 0 {
                #[cfg(target_arch = "x86_64")]
                let ip = regs.rip;
                #[cfg(target_arch = "aarch64")]
                let ip = regs.pc;
                #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
                let ip = 0u64;

                if ip > 0 {
                    samples.push(ip);
                    sample_count += 1;
                }
            }

            unsafe {
                libc::ptrace(
                    libc::PTRACE_CONT as libc::c_uint,
                    child_pid,
                    std::ptr::null_mut::<libc::c_void>(),
                    libc::SIGCONT as *mut libc::c_void,
                );
            }
        } else if stop_signal > 0 {
            unsafe {
                libc::ptrace(
                    libc::PTRACE_CONT as libc::c_uint,
                    child_pid,
                    std::ptr::null_mut::<libc::c_void>(),
                    stop_signal as *mut libc::c_void,
                );
            }
        } else {
            unsafe {
                libc::ptrace(
                    libc::PTRACE_CONT as libc::c_uint,
                    child_pid,
                    std::ptr::null_mut::<libc::c_void>(),
                    std::ptr::null_mut::<libc::c_void>(),
                );
            }
        }
    }

    running.store(false, Ordering::SeqCst);
    let _ = timer.join();

    unsafe {
        libc::kill(child_pid, libc::SIGKILL);
        libc::waitpid(child_pid, &mut status as *mut libc::c_int, 0);
    }

    Ok(samples)
}

#[cfg(target_os = "linux")]
fn resolve_with_addr2line(binary: &Path, ips: &[u64]) -> Result<Vec<(String, u64)>, String> {
    let unique: Vec<u64> = {
        let mut v: Vec<u64> = ips.to_vec();
        v.sort();
        v.dedup();
        v
    };

    if unique.is_empty() {
        return Ok(Vec::new());
    }

    let mut cmd = Command::new("addr2line");
    cmd.arg("-e")
        .arg(binary);
    cmd.arg("-f")
        .arg("-a");
    for ip in &unique {
        cmd.arg(format!("0x{ip:x}"));
    }

    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("addr2line: {e}"))?;

    if !output.status.success() {
        return Err("addr2line returned non-zero exit".into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut locations = Vec::new();

    let mut lines = stdout.lines();
    while let Some(_addr_line) = lines.next() {
        let _func = lines.next().unwrap_or("??");
        let loc = lines.next().unwrap_or("??:0");
        if loc != "??:0" && !loc.contains('?')
            && let Some((file, line_str)) = loc.rsplit_once(':')
                && let Ok(line_num) = line_str.parse::<u64>() {
                    locations.push((file.to_owned(), line_num));
                }
    }

    Ok(locations)
}

#[cfg(target_os = "linux")]
fn count_source_lines(dir: &str) -> Result<usize, String> {
    let mut total = 0usize;
    let mut dirs = vec![dir.to_owned()];
    while let Some(current) = dirs.pop() {
        let entries = match std::fs::read_dir(&current) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(OsStr::to_str).unwrap_or("");
                if !name.starts_with('.') && name != "target" {
                    dirs.push(path.to_str().unwrap_or("").to_owned());
                }
            } else if path.extension().is_some_and(|e| e == "rs")
                && let Ok(content) = std::fs::read_to_string(&path) {
                    for line in content.lines() {
                        let t = line.trim();
                        if !t.is_empty() && !t.starts_with("//") {
                            total += 1;
                        }
                    }
                }
        }
    }

    Ok(total)
}
