//! Filesystem utilities for tests — temporary directories with automatic cleanup.
//!
//! # Example
//!
//! ```ignore
//! use rvtest::fs::temp_dir;
//!
//! #[test]
//! fn test_with_temp() {
//!     let dir = temp_dir();
//!     let path = dir.path().join("output.txt");
//!     std::fs::write(&path, b"data").unwrap();
//!     assert!(path.exists());
//!     // dir dropped → entire temp directory deleted
//! }
//! ```

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Create a temporary directory with a random name.
///
/// The directory is created inside `std::env::temp_dir()` / `rvtest-`.
/// It is automatically deleted (including all contents) when the returned
/// [`TempDir`] is dropped — even if a panic occurred.
///
/// # Panics
///
/// Panics if the directory cannot be created.
pub fn temp_dir() -> TempDir {
    let base = std::env::temp_dir().join("rvtest");
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
    let path = base.join(format!("{:06x}", counter));
    ensure_empty(&path);
    TempDir { path }
}

/// Create a temporary directory with a custom prefix.
///
/// The full path will be `{temp}/rvtest-{prefix}-{random}`.
///
/// # Panics
///
/// Panics if the directory cannot be created.
pub fn temp_dir_with_prefix(prefix: &str) -> TempDir {
    let base = std::env::temp_dir().join("rvtest");
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
    let path = base.join(format!("{}-{:06x}", prefix, counter));
    ensure_empty(&path);
    TempDir { path }
}

fn ensure_empty(path: &Path) {
    let _ = std::fs::remove_dir_all(path);
    std::fs::create_dir_all(path).expect("failed to create temp directory");
}

/// A temporary directory that is automatically cleaned up on drop.
///
/// Created by [`temp_dir()`] or [`temp_dir_with_prefix()`].
pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    /// Returns the path to the temporary directory.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Consume the guard without deleting the directory.
    ///
    /// Useful when you want to persist the directory for debugging.
    pub fn leak(self) -> PathBuf {
        let path = self.path.clone();
        std::mem::forget(self);
        path
    }
}

impl AsRef<Path> for TempDir {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::{temp_dir, temp_dir_with_prefix};
    use std::path::Path;

    #[test]
    fn creates_directory() {
        let dir = temp_dir();
        assert!(dir.path().exists());
        assert!(dir.path().is_dir());
    }

    #[test]
    fn is_writable() {
        let dir = temp_dir();
        let file = dir.path().join("test.txt");
        std::fs::write(&file, b"hello").unwrap();
        assert!(file.exists());
    }

    #[test]
    fn cleaned_on_drop() {
        let path;
        {
            let dir = temp_dir();
            path = dir.path().to_owned();
            assert!(path.exists());
        }
        assert!(!path.exists(), "directory should be deleted after drop");
    }

    #[test]
    fn with_prefix_contains_prefix() {
        let dir = temp_dir_with_prefix("myapp");
        assert!(dir.path().exists());
        assert!(dir.path().to_string_lossy().contains("myapp"));
    }

    #[test]
    fn cleaned_after_panic() {
        let path;
        {
            let dir = temp_dir();
            path = dir.path().to_owned();
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                panic!("intentional");
            }));
            assert!(r.is_err());
        }
        assert!(!path.exists(), "directory should be deleted after panic + drop");
    }

    #[test]
    fn as_ref_path() {
        let dir = temp_dir();
        let p: &Path = dir.as_ref();
        assert!(p.exists());
    }

    #[test]
    fn leak_keeps_directory() {
        let path;
        {
            let dir = temp_dir();
            path = dir.leak();
            assert!(path.exists());
        }
        assert!(path.exists());
        let _ = std::fs::remove_dir_all(&path);
    }

    #[test]
    fn multiple_dirs_have_unique_paths() {
        let a = temp_dir();
        let b = temp_dir();
        assert_ne!(a.path(), b.path());
    }

    #[test]
    fn with_prefix_unique() {
        let a = temp_dir_with_prefix("test");
        let b = temp_dir_with_prefix("test");
        assert_ne!(a.path(), b.path());
    }
}
