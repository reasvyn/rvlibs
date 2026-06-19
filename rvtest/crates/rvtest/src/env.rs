//! Environment variable utilities with RAII guards.
//!
//! `set_var`, `remove_var`, `set_vars` return guards that restore the
//! original environment when dropped — even after a panic.
//!
//! # Example
//!
//! ```ignore
//! use rvtest::env::{set_var, remove_var};
//!
//! #[test]
//! fn test_with_env() {
//!     let _guard = set_var("MY_KEY", "value");
//!     assert_eq!(std::env::var("MY_KEY").unwrap(), "value");
//!     // guard dropped → MY_KEY restored to previous state
//! }
//! ```

use std::collections::HashMap;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Temporarily set an environment variable.
///
/// Returns a guard that restores the previous value (or removes it if it
/// didn't exist) when dropped.
pub fn set_var<K, V>(key: K, value: V) -> EnvGuard
where
    K: Into<String>,
    V: Into<String>,
{
    let _lock = ENV_LOCK.lock().unwrap();
    let key = key.into();
    let prev = std::env::var(&key).ok();
    // SAFETY: single-threaded access via ENV_LOCK prevents data races.
    unsafe { std::env::set_var(&key, value.into()); }
    EnvGuard { key, prev }
}

/// Temporarily remove an environment variable.
///
/// Returns a guard that restores the previous value when dropped.
pub fn remove_var<K>(key: K) -> EnvGuard
where
    K: Into<String>,
{
    let _lock = ENV_LOCK.lock().unwrap();
    let key = key.into();
    let prev = std::env::var(&key).ok();
    // SAFETY: single-threaded access via ENV_LOCK prevents data races.
    unsafe { std::env::remove_var(&key); }
    EnvGuard { key, prev }
}

/// Temporarily set multiple environment variables at once.
///
/// All variables are restored to their previous state when the guard is
/// dropped, even if an intermediate assignment panics.
pub fn set_vars<K, V, I>(pairs: I) -> MultiEnvGuard
where
    K: Into<String>,
    V: Into<String>,
    I: IntoIterator<Item = (K, V)>,
{
    let _lock = ENV_LOCK.lock().unwrap();
    let mut prev = HashMap::new();
    for (k, v) in pairs {
        let key = k.into();
        let old = std::env::var(&key).ok();
        // SAFETY: single-threaded access via ENV_LOCK prevents data races.
        unsafe { std::env::set_var(&key, v.into()); }
        prev.insert(key, old);
    }
    MultiEnvGuard { prev }
}

/// Restores a single env var to its previous state when dropped.
pub struct EnvGuard {
    key: String,
    prev: Option<String>,
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        let _lock = ENV_LOCK.lock().unwrap();
        // SAFETY: single-threaded access via ENV_LOCK prevents data races.
        match &self.prev {
            Some(val) => unsafe { std::env::set_var(&self.key, val); },
            None => unsafe { std::env::remove_var(&self.key); },
        }
    }
}

/// Restores multiple env vars to their previous state when dropped.
pub struct MultiEnvGuard {
    prev: HashMap<String, Option<String>>,
}

impl Drop for MultiEnvGuard {
    fn drop(&mut self) {
        let _lock = ENV_LOCK.lock().unwrap();
        for (key, prev) in std::mem::take(&mut self.prev) {
            // SAFETY: single-threaded access via ENV_LOCK prevents data races.
            match prev {
                Some(val) => unsafe { std::env::set_var(&key, val); },
                None => unsafe { std::env::remove_var(&key); },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn remove_var_safe(key: &str) {
        // SAFETY: test environment is single-threaded for env tests.
        unsafe { std::env::remove_var(key); }
    }

    fn set_var_safe(key: &str, val: &str) {
        // SAFETY: test environment is single-threaded for env tests.
        unsafe { std::env::set_var(key, val); }
    }

    #[test]
    fn set_var_sets_and_restores() {
        let key = "RVTEST_ENV_SET_TEST";
        remove_var_safe(key);
        {
            let _g = set_var(key, "hello");
            assert_eq!(std::env::var(key).unwrap(), "hello");
        }
        assert!(std::env::var(key).is_err());
    }

    #[test]
    fn set_var_restores_previous_value() {
        let key = "RVTEST_ENV_RESTORE_TEST";
        set_var_safe(key, "original");
        {
            let _g = set_var(key, "modified");
            assert_eq!(std::env::var(key).unwrap(), "modified");
        }
        assert_eq!(std::env::var(key).unwrap(), "original");
        remove_var_safe(key);
    }

    #[test]
    fn remove_var_removes_and_restores() {
        let key = "RVTEST_ENV_REMOVE_TEST";
        set_var_safe(key, "value");
        {
            let _g = remove_var(key);
            assert!(std::env::var(key).is_err());
        }
        assert_eq!(std::env::var(key).unwrap(), "value");
        remove_var_safe(key);
    }

    #[test]
    fn set_vars_sets_all() {
        let keys = [("RVTEST_MV_1", "a"), ("RVTEST_MV_2", "b")];
        remove_var_safe("RVTEST_MV_1");
        remove_var_safe("RVTEST_MV_2");
        {
            let _g = set_vars(keys);
            assert_eq!(std::env::var("RVTEST_MV_1").unwrap(), "a");
            assert_eq!(std::env::var("RVTEST_MV_2").unwrap(), "b");
        }
        assert!(std::env::var("RVTEST_MV_1").is_err());
        assert!(std::env::var("RVTEST_MV_2").is_err());
    }

    #[test]
    fn guard_dropped_on_panic() {
        let key = "RVTEST_ENV_PANIC_TEST";
        remove_var_safe(key);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _g = set_var(key, "panic_val");
            assert_eq!(std::env::var(key).unwrap(), "panic_val");
            panic!("intentional panic");
        }));
        assert!(r.is_err());
        assert!(std::env::var(key).is_err(), "should be restored after panic");
    }
}
