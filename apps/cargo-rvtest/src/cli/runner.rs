use std::io::{self, IsTerminal, Write};
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant, SystemTime};

use rvtest::core::{ReportFormat, TestCase, TestRun};
use rvtest::runner::parse_cargo_test_output;
use rvtest::tag;

use super::cache;
use super::profile;
use super::render;
use super::tune;

fn run_binaries_directly(
    binaries: &[std::path::PathBuf],
    filter: Option<&str>,
    skip: Option<&str>,
) -> TestRun {
    let start = SystemTime::now();
    let wall_start = Instant::now();
    let mut all_suites = Vec::new();

    for binary in binaries {
        if !binary.exists() {
            continue;
        }
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
                suite
                    .tests
                    .retain(|t| !tag::name_skipped(&t.name, Some(sp)));
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

pub fn run_cargo_test(
    filter: Option<&str>,
    fast: bool,
    cranelift: bool,
    parallel_frontend: Option<usize>,
    workspace: bool,
    skip: Option<&str>,
) -> TestRun {
    if let Some(binaries) = cache::try_warm_binaries() {
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
            print!(
                "\r  {} {}  {} running...",
                frames[i],
                profile::dim("cargo test"),
                profile::dim("tests")
            );
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
        && !skip_pattern.is_empty()
    {
        for suite in &mut suites {
            suite
                .tests
                .retain(|t| !tag::name_skipped(&t.name, Some(skip_pattern)));
        }
    }

    if why_slow {
        let build_dur = build_start.map(|s| s.elapsed());
        let total: std::time::Duration = suites
            .iter()
            .flat_map(|s| s.tests.iter())
            .map(|t| t.duration)
            .sum();
        let exec_dur = duration;

        eprintln!("\n  ⏱  Why Slow — Time Breakdown:");
        if let Some(b) = build_dur {
            eprintln!("     Cargo test (build + exec):  {:.2}s", b.as_secs_f64());
        }
        eprintln!(
            "     Execution (all tests):       {:.2}s",
            exec_dur.as_secs_f64()
        );
        eprintln!(
            "     Sum of individual tests:      {:.2}s",
            total.as_secs_f64()
        );
        eprintln!(
            "     Overhead:                     {:.2}s",
            exec_dur.as_secs_f64() - total.as_secs_f64()
        );

        let mut all: Vec<&TestCase> = suites.iter().flat_map(|s| s.tests.iter()).collect();
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

pub(crate) fn run_and_print(
    filter: &Option<String>,
    format: &ReportFormat,
    fast: bool,
    slow_count: usize,
    cranelift: bool,
    parallel_frontend: Option<usize>,
    skip: Option<String>,
    use_colour: bool,
) {
    let run = run_cargo_test(
        filter.as_deref(),
        fast,
        cranelift,
        parallel_frontend,
        false,
        skip.as_deref(),
    );
    let report = render::render(format, &run, slow_count, use_colour);
    println!("{report}");
}
