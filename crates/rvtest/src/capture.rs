//! Lightweight stdout/stderr capture for test execution.
//!
//! Captures output written to `println!`, `eprintln!`, `write!(io::stdout())`,
//! etc. during a test closure and returns it as a `String`.
//!
//! # Implementation
//!
//! On Unix this uses `libc::pipe()` + `dup2` to redirect file descriptors 1
//! and 2. On other platforms (including Windows) capture is a no-op and
//! returns empty strings.  The pipe is read after the closure completes
//! and the original fds are restored.

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(unix)]
mod imp {
    use std::io::{Read, Write};
    use std::os::fd::{FromRawFd, RawFd};

    pub struct CaptureGuard {
        saved_stdout: RawFd,
        saved_stderr: RawFd,
        read_stdout: RawFd,
        read_stderr: RawFd,
    }

    impl CaptureGuard {
        /// Redirect stdout and stderr to pipes.  Returns a guard that MUST
        /// be held for the duration of the captured code.
        pub fn start() -> Option<Self> {
            // Flush any buffered output before redirecting fds
            let _ = std::io::stdout().flush();
            let _ = std::io::stderr().flush();

            unsafe {
                let mut stdout_fds: [RawFd; 2] = [0; 2];
                let mut stderr_fds: [RawFd; 2] = [0; 2];

                if libc::pipe(stdout_fds.as_mut_ptr()) != 0 {
                    return None;
                }
                if libc::pipe(stderr_fds.as_mut_ptr()) != 0 {
                    libc::close(stdout_fds[0]);
                    libc::close(stdout_fds[1]);
                    return None;
                }

                let saved_stdout = libc::dup(libc::STDOUT_FILENO);
                let saved_stderr = libc::dup(libc::STDERR_FILENO);
                if saved_stdout < 0 || saved_stderr < 0 {
                    libc::close(stdout_fds[0]);
                    libc::close(stdout_fds[1]);
                    libc::close(stderr_fds[0]);
                    libc::close(stderr_fds[1]);
                    return None;
                }

                libc::dup2(stdout_fds[1], libc::STDOUT_FILENO);
                libc::close(stdout_fds[1]);
                libc::dup2(stderr_fds[1], libc::STDERR_FILENO);
                libc::close(stderr_fds[1]);

                Some(CaptureGuard { saved_stdout, saved_stderr, read_stdout: stdout_fds[0], read_stderr: stderr_fds[0] })
            }
        }

        /// Read captured output and restore original fds.
        pub fn stop(self) -> (String, String) {
            unsafe {
                libc::dup2(self.saved_stdout, libc::STDOUT_FILENO);
                libc::close(self.saved_stdout);
                libc::dup2(self.saved_stderr, libc::STDERR_FILENO);
                libc::close(self.saved_stderr);
            }

            let read_pipe = |fd: RawFd| -> String {
                let mut file = unsafe { std::fs::File::from_raw_fd(fd) };
                let mut buf = Vec::new();
                let _ = file.read_to_end(&mut buf);
                String::from_utf8_lossy(&buf).into_owned()
            };

            let stdout = read_pipe(self.read_stdout);
            let stderr = read_pipe(self.read_stderr);
            (stdout, stderr)
        }
    }
}

#[cfg(not(unix))]
mod imp {
    pub struct CaptureGuard;

    impl CaptureGuard {
        pub fn start() -> Option<Self> { None }
        pub fn stop(self) -> (String, String) { (String::new(), String::new()) }
    }
}

/// Whether output capture is currently enabled for tests.
static CAPTURE_ENABLED: AtomicBool = AtomicBool::new(false);

/// Enable or disable output capture globally.
pub fn set_capture_enabled(enabled: bool) {
    CAPTURE_ENABLED.store(enabled, Ordering::SeqCst);
}

/// Returns `true` if output capture is currently enabled.
pub fn is_capture_enabled() -> bool {
    CAPTURE_ENABLED.load(Ordering::SeqCst)
}

/// Run a closure with stdout/stderr captured.
///
/// Returns `(result, captured_stdout, captured_stderr)`.
/// If capture is disabled or unavailable, stdout/stderr will be empty strings.
pub fn capture<T>(
    f: impl FnOnce() -> T,
) -> (T, String, String) {
    if !is_capture_enabled() {
        return (f(), String::new(), String::new());
    }

    match imp::CaptureGuard::start() {
        Some(guard) => {
            let result = f();
            // Flush before restoring fds so buffered output reaches the pipes
            let _ = std::io::stdout().flush();
            let _ = std::io::stderr().flush();
            let (stdout, stderr) = guard.stop();
            (result, stdout, stderr)
        }
        None => (f(), String::new(), String::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_disabled_by_default() {
        assert!(!is_capture_enabled());
    }

    #[test]
    fn capture_enable_toggle() {
        set_capture_enabled(true);
        assert!(is_capture_enabled());
        set_capture_enabled(false);
        assert!(!is_capture_enabled());
    }

    #[test]
    fn capture_returns_empty_when_disabled() {
        let (result, stdout, stderr) = capture(|| 42);
        assert_eq!(result, 42);
        assert!(stdout.is_empty());
        assert!(stderr.is_empty());
    }

    #[test]
    fn capture_captures_stdout() {
        set_capture_enabled(true);
        let (result, stdout, _) = capture(|| {
            // Use write! with flush to bypass the BufWriter
            let mut out = std::io::stdout().lock();
            let _ = write!(out, "hello ");
            let _ = write!(out, "world");
            let _ = out.flush();
        });
        set_capture_enabled(false);
        assert_eq!(result, ());
        if cfg!(unix) {
            assert_eq!(stdout, "hello world");
        }
    }

    #[test]
    fn capture_captures_stderr() {
        set_capture_enabled(true);
        let (_, _, stderr) = capture(|| {
            let mut out = std::io::stderr().lock();
            let _ = write!(out, "error msg");
            let _ = out.flush();
        });
        set_capture_enabled(false);
        if cfg!(unix) {
            assert_eq!(stderr, "error msg");
        }
    }
}
