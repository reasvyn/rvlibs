use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

use rvtest::core::ReportFormat;

use crate::cli::run_and_print;

#[allow(clippy::too_many_arguments)]
pub fn watch_loop(mut filter: Option<String>, format_str: String, fast: bool, slow_count: usize, cranelift: bool, parallel_frontend: Option<usize>, skip: Option<String>, use_colour: bool) {
    let done = Arc::new(AtomicBool::new(false));
    let format: ReportFormat = format_str.parse().unwrap_or(ReportFormat::Pretty);

    let (tx, rx) = std::sync::mpsc::channel::<Result<Event, notify::Error>>();
    let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Error: cannot start file watcher: {e}");
            std::process::exit(1);
        }
    };
    for dir in &["src", "tests"] {
        if Path::new(dir).exists() {
            let _ = watcher.watch(Path::new(dir), RecursiveMode::Recursive);
        }
    }

    run_and_print(&filter, &format, fast, slow_count, cranelift, parallel_frontend, skip.clone(), use_colour);
    eprint!("  Watching src/, tests/ for changes... [q] quit [r] re-run [f] filter\n\n");

    #[cfg(unix)]
    {
        unsafe {
            libc::signal(libc::SIGINT, sigint_handler as *const () as libc::sighandler_t);
        }
    }

    let debounce = Duration::from_millis(300);
    let mut pending = false;

    loop {
        if done.load(Ordering::SeqCst) {
            break;
        }

        let deadline = Instant::now() + debounce;
        while Instant::now() < deadline && !done.load(Ordering::SeqCst) {
            match rx.recv_timeout(Duration::from_millis(50)) {
                Ok(Ok(_)) => pending = true,
                Ok(Err(_)) => {}
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }

        #[cfg(unix)]
        if !done.load(Ordering::SeqCst) {
            match check_watch_key() {
                WatchKey::Quit => {
                    eprintln!("Quitting.");
                    break;
                }
                WatchKey::Rerun => {
                    eprintln!("  Re-running tests...\n");
                    run_and_print(&filter, &format, fast, slow_count, cranelift, parallel_frontend, skip.clone(), use_colour);
                    eprint!("\n  Watching... [q] quit [r] re-run [f] filter\n\n");
                    continue;
                }
                WatchKey::Filter => {
                    eprint!("  Enter filter: ");
                    let _ = std::io::stdout().flush();
                    let mut input = String::new();
                    if std::io::stdin().read_line(&mut input).is_ok() {
                        let trimmed = input.trim().to_owned();
                        if trimmed.is_empty() {
                            filter = None;
                            eprintln!("  Filter cleared.");
                        } else {
                            filter = Some(trimmed);
                            eprintln!("  Filter set to: {}", filter.as_deref().unwrap_or(""));
                        }
                    }
                    eprintln!("  Re-running tests...\n");
                    run_and_print(&filter, &format, fast, slow_count, cranelift, parallel_frontend, skip.clone(), use_colour);
                    eprint!("\n  Watching... [q] quit [r] re-run [f] filter\n\n");
                    continue;
                }
                WatchKey::None => {}
            }
        }

        if !pending && !done.load(Ordering::SeqCst) {
            std::thread::sleep(Duration::from_millis(100));
            continue;
        }

        pending = false;

        if done.load(Ordering::SeqCst) {
            break;
        }

        eprintln!("  Change detected — re-running tests...\n");
        run_and_print(&filter, &format, fast, slow_count, cranelift, parallel_frontend, skip.clone(), use_colour);
        eprint!("\n  Watching... [q] quit [r] re-run [f] filter\n\n");
    }
}

#[cfg(unix)]
unsafe extern "C" fn sigint_handler(_: libc::c_int) {}

enum WatchKey { Quit, Rerun, Filter, None }

#[cfg(unix)]
fn check_watch_key() -> WatchKey {
    use std::os::fd::AsRawFd;
    let fd = io::stdin().as_raw_fd();
    let mut fds: libc::fd_set = unsafe { std::mem::zeroed() };
    unsafe { libc::FD_SET(fd, &mut fds) };
    let mut tv = libc::timeval { tv_sec: 0, tv_usec: 0 };
    let ret = unsafe { libc::select(fd + 1, &mut fds, std::ptr::null_mut(), std::ptr::null_mut(), &mut tv) };
    if ret > 0 {
        let mut buf = [0u8; 1];
        if io::stdin().read_exact(&mut buf).is_ok() {
            match buf[0] {
                b'q' | b'Q' => return WatchKey::Quit,
                b'r' | b'R' => return WatchKey::Rerun,
                b'f' | b'F' => return WatchKey::Filter,
                _ => {}
            }
        }
    }
    WatchKey::None
}

#[cfg(not(unix))]
fn check_watch_key() -> WatchKey {
    WatchKey::None
}
