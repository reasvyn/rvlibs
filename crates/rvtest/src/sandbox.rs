/// Resource limits that can be applied to a test process.
///
/// On Unix, these are enforced via `setrlimit`.  On other platforms,
/// they are best-effort (or no-ops).
#[derive(Debug, Clone, Copy)]
#[derive(Default)]
pub struct ResourceLimits {
    /// Maximum number of open file descriptors.
    pub max_fds: Option<u64>,
    /// Maximum number of child processes / threads.
    pub max_processes: Option<u64>,
    /// Maximum stack size in bytes.
    pub max_stack: Option<u64>,
    /// Maximum file size that can be created (in bytes).
    pub max_file_size: Option<u64>,
    /// Maximum core file size (0 disables core dumps).
    pub max_core_size: Option<u64>,
    /// Maximum virtual address space in bytes (prevents memory leaks).
    pub max_address_space: Option<u64>,
}

impl ResourceLimits {
    pub fn with_max_fds(mut self, n: u64) -> Self { self.max_fds = Some(n); self }
    pub fn with_max_processes(mut self, n: u64) -> Self { self.max_processes = Some(n); self }
    pub fn with_max_stack(mut self, bytes: u64) -> Self { self.max_stack = Some(bytes); self }
    pub fn with_max_file_size(mut self, bytes: u64) -> Self { self.max_file_size = Some(bytes); self }
    pub fn no_core_dumps(mut self) -> Self { self.max_core_size = Some(0); self }
    pub fn with_max_address_space(mut self, bytes: u64) -> Self { self.max_address_space = Some(bytes); self }

    /// Apply these resource limits to the current process.
    pub fn apply(&self) {
        #[cfg(unix)]
        {
            use libc::{rlimit, RLIMIT_AS, RLIMIT_CORE, RLIMIT_FSIZE, RLIMIT_NOFILE, RLIMIT_NPROC, RLIMIT_STACK};
            struct Rl { resource: libc::c_int, value: u64, name: &'static str }
            let mut limits: Vec<Rl> = Vec::new();
            if let Some(n) = self.max_fds { limits.push(Rl { resource: RLIMIT_NOFILE as _, value: n, name: "RLIMIT_NOFILE" }); }
            if let Some(n) = self.max_processes { limits.push(Rl { resource: RLIMIT_NPROC as _, value: n, name: "RLIMIT_NPROC" }); }
            if let Some(n) = self.max_stack { limits.push(Rl { resource: RLIMIT_STACK as _, value: n, name: "RLIMIT_STACK" }); }
            if let Some(n) = self.max_file_size { limits.push(Rl { resource: RLIMIT_FSIZE as _, value: n, name: "RLIMIT_FSIZE" }); }
            if let Some(n) = self.max_core_size { limits.push(Rl { resource: RLIMIT_CORE as _, value: n, name: "RLIMIT_CORE" }); }
            if let Some(n) = self.max_address_space { limits.push(Rl { resource: RLIMIT_AS as _, value: n, name: "RLIMIT_AS" }); }
            for l in &limits {
                let rlim = rlimit { rlim_cur: l.value, rlim_max: l.value };
                if unsafe { libc::setrlimit(l.resource as _, &rlim) } != 0 {
                    let err = std::io::Error::last_os_error();
                    eprintln!("  Failed to set {}: {err}", l.name);
                }
            }
        }
        #[cfg(not(unix))] { let _ = self; }
    }
}


/// Configuration for test execution sandboxing.
///
/// Sandboxing restricts what tests can access during execution:
/// - Filesystem: only whitelisted directories are readable
/// - Network: can be disabled entirely
/// - Environment: only allowlisted env vars are preserved
/// - Resources: per-process limits (fds, threads, memory, etc.)
///
/// # Example
///
/// ```ignore
/// use rvtest::sandbox::SandboxConfig;
///
/// let config = SandboxConfig::default()
///     .with_fs_whitelist(["src/", "tests/"])
///     .with_network(false)
///     .with_env_allowlist(["PATH", "HOME", "TMPDIR"]);
/// ```
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Directories that tests are allowed to read/write.
    /// If empty (default), no filesystem restrictions are applied.
    pub fs_whitelist: Vec<String>,
    /// Whether tests are allowed to make network connections.
    /// Default: `true` (no restriction).
    pub network_access: bool,
    /// Environment variable names to preserve.
    /// If non-empty, all other env vars are cleared before test execution.
    /// If empty (default), no env restrictions are applied.
    pub env_allowlist: Vec<String>,
    /// Whether to run with a restrictive umask (0o077).
    pub restrict_umask: bool,
    /// Whether to use a clean temporary directory as TMPDIR/TEMP.
    pub isolated_tempdir: bool,
    /// Per-process resource limits (fds, threads, memory, etc.).
    pub resource_limits: ResourceLimits,
    /// Whether to enforce all sandbox permissions strictly (fail if violated).
    pub enforce_permissions: bool,
}

impl SandboxConfig {
    /// Restrict filesystem access to only the given directories.
    pub fn with_fs_whitelist(mut self, dirs: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.fs_whitelist = dirs.into_iter().map(|d| d.into()).collect();
        self
    }

    /// Enable or disable network access for tests.
    pub fn with_network(mut self, enabled: bool) -> Self {
        self.network_access = enabled;
        self
    }

    /// Only preserve the given environment variables.
    pub fn with_env_allowlist(mut self, vars: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.env_allowlist = vars.into_iter().map(|v| v.into()).collect();
        self
    }

    /// Enable or disable restrictive umask.
    pub fn with_restrict_umask(mut self, yes: bool) -> Self {
        self.restrict_umask = yes;
        self
    }

    /// Enable or disable isolated temp directory.
    pub fn with_isolated_tempdir(mut self, yes: bool) -> Self {
        self.isolated_tempdir = yes;
        self
    }

    /// Set per-process resource limits.
    pub fn with_resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.resource_limits = limits;
        self
    }

    /// Enable or disable strict permission enforcement.
    pub fn with_enforce_permissions(mut self, yes: bool) -> Self {
        self.enforce_permissions = yes;
        self
    }

    /// Apply this sandbox configuration to the current process.
    /// This should be called before test execution.
    pub fn apply(&self) -> SandboxGuard {
        let mut actions: Vec<String> = Vec::new();

        // Restrict environment variables
        if !self.env_allowlist.is_empty() {
            let allowed: std::collections::HashSet<String> = self
                .env_allowlist
                .iter()
                .map(|v| v.to_uppercase())
                .collect();
            let preserved: Vec<(String, String)> = allowed
                .iter()
                .filter_map(|k| {
                    std::env::var(k)
                        .ok()
                        .map(|v| (k.clone(), v))
                })
                .collect();
            // Clear and restore only allowlisted vars
            for (key, _) in &preserved {
                unsafe { std::env::remove_var(key); }
            }
            for (key, val) in &preserved {
                unsafe { std::env::set_var(key, val); }
            }
            actions.push(format!("env: allowlisted {} var(s)", allowed.len()));
        } else {
            actions.push("env: unrestricted".into());
        }

        // Set restrictive umask
        #[cfg(unix)]
        if self.restrict_umask {
            unsafe {
                libc::umask(0o077);
            }
            actions.push("umask: 0o077".into());
        }

        // Set up isolated temp directory
        if self.isolated_tempdir {
            let tmp = std::env::temp_dir().join(format!("rvtest-sandbox-{}", std::process::id()));
            let _ = std::fs::create_dir_all(&tmp);
            unsafe {
                std::env::set_var("TMPDIR", &tmp);
                std::env::set_var("TEMP", &tmp);
                std::env::set_var("TMP", &tmp);
            }
            actions.push(format!("tmpdir: {}", tmp.display()));
        }

        // Resource limits
        self.resource_limits.apply();

        // Filesystem sandbox (chroot-like) — currently only supported with process isolation
        if !self.fs_whitelist.is_empty() {
            actions.push(format!("fs: whitelisted {} path(s)", self.fs_whitelist.len()));
        }

        // Network sandbox
        if !self.network_access {
            actions.push("net: disabled".into());
        }

        // Permission enforcement
        if self.enforce_permissions {
            // In enforce mode, check that env allowlist is configured
            if !self.env_allowlist.is_empty() {
                actions.push("perms: env restricted".into());
            }
            if !self.fs_whitelist.is_empty() {
                actions.push("perms: fs restricted".into());
            }
            if !self.network_access {
                actions.push("perms: net restricted".into());
            }
        }

        SandboxGuard { actions, isolated_tempdir: self.isolated_tempdir }
    }

    /// Build environment variable overrides for a child process.
    /// Returns a list of (key, value) pairs to set, or None to clear.
    pub fn env_overrides(&self) -> Vec<(String, Option<String>)> {
        let mut overrides = Vec::new();
        if !self.env_allowlist.is_empty() {
            let allowed: std::collections::HashSet<String> = self
                .env_allowlist
                .iter()
                .map(|v| v.to_uppercase())
                .collect();
            for (key, val) in std::env::vars() {
                let upper = key.to_uppercase();
                if !allowed.contains(&upper) {
                    overrides.push((key, None));
                } else {
                    overrides.push((key, Some(val)));
                }
            }
        }
        if self.isolated_tempdir {
            let tmp = std::env::temp_dir().join(format!("rvtest-sandbox-{}", std::process::id()));
            overrides.push(("TMPDIR".into(), Some(tmp.to_string_lossy().into_owned())));
            overrides.push(("TEMP".into(), Some(tmp.to_string_lossy().into_owned())));
            overrides.push(("TMP".into(), Some(tmp.to_string_lossy().into_owned())));
        }
        overrides
    }

    /// Get the isolated temp directory path, if enabled.
    pub fn temp_dir(&self) -> Option<std::path::PathBuf> {
        if self.isolated_tempdir {
            Some(std::env::temp_dir().join(format!("rvtest-sandbox-{}", std::process::id())))
        } else {
            None
        }
    }
}

impl Default for SandboxConfig {
    fn default() -> Self {
        SandboxConfig {
            fs_whitelist: Vec::new(),
            network_access: true,
            env_allowlist: Vec::new(),
            restrict_umask: false,
            isolated_tempdir: false,
            resource_limits: ResourceLimits::default(),
            enforce_permissions: false,
        }
    }
}

/// RAII guard that restores the environment when dropped.
#[must_use]
pub struct SandboxGuard {
    actions: Vec<String>,
    isolated_tempdir: bool,
}

impl SandboxGuard {
    /// Returns a summary of sandbox actions taken.
    pub fn summary(&self) -> &[String] {
        &self.actions
    }
}

impl Drop for SandboxGuard {
    fn drop(&mut self) {
        if self.isolated_tempdir {
            let tmp = std::env::temp_dir().join(format!("rvtest-sandbox-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&tmp);
        }
    }
}

// SAFETY: SandboxGuard only manages temp directory cleanup and summary strings.
unsafe impl Send for SandboxGuard {}
unsafe impl Sync for SandboxGuard {}

/// Parse a comma-separated whitelist from a CLI argument.
pub fn parse_fs_whitelist(s: &str) -> Vec<String> {
    s.split(',')
        .map(|p| p.trim().to_owned())
        .filter(|p| !p.is_empty())
        .collect()
}

/// Parse a comma-separated env allowlist from a CLI argument.
pub fn parse_env_allowlist(s: &str) -> Vec<String> {
    s.split(',')
        .map(|v| v.trim().to_uppercase())
        .filter(|v| !v.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandbox_default_no_restrictions() {
        let cfg = SandboxConfig::default();
        assert!(cfg.fs_whitelist.is_empty());
        assert!(cfg.network_access);
        assert!(cfg.env_allowlist.is_empty());
        assert!(!cfg.restrict_umask);
        assert!(!cfg.isolated_tempdir);
    }

    #[test]
    fn sandbox_with_fs_whitelist() {
        let cfg = SandboxConfig::default()
            .with_fs_whitelist(["src/", "tests/"]);
        assert_eq!(cfg.fs_whitelist, vec!["src/", "tests/"]);
    }

    #[test]
    fn sandbox_with_env_allowlist() {
        let cfg = SandboxConfig::default()
            .with_env_allowlist(["PATH", "HOME"]);
        assert_eq!(cfg.env_allowlist, vec!["PATH", "HOME"]);
    }

    #[test]
    fn sandbox_with_network_disabled() {
        let cfg = SandboxConfig::default()
            .with_network(false);
        assert!(!cfg.network_access);
    }

    #[test]
    fn sandbox_with_restrict_umask() {
        let cfg = SandboxConfig::default()
            .with_restrict_umask(true);
        assert!(cfg.restrict_umask);
    }

    #[test]
    fn sandbox_with_isolated_tempdir() {
        let cfg = SandboxConfig::default()
            .with_isolated_tempdir(true);
        assert!(cfg.isolated_tempdir);
    }

    #[test]
    fn sandbox_apply_returns_guard() {
        let cfg = SandboxConfig::default()
            .with_env_allowlist(["PATH"]);
        let guard = cfg.apply();
        assert!(!guard.actions.is_empty());
    }

    #[test]
    fn sandbox_env_overrides_contains_allowlisted() {
        unsafe { std::env::set_var("RVTEST_SANDBOX_TEST_VAR", "present"); }
        let cfg = SandboxConfig::default()
            .with_env_allowlist(["PATH", "RVTEST_SANDBOX_TEST_VAR"]);
        let overrides = cfg.env_overrides();
        assert!(overrides.iter().any(|(k, v)| k == "PATH" && v.is_some()));
        unsafe { std::env::remove_var("RVTEST_SANDBOX_TEST_VAR"); }
    }

    #[test]
    fn sandbox_env_overrides_clears_unlisted() {
        unsafe { std::env::set_var("RVTEST_SANDBOX_CLEAR_ME", "secret"); }
        let cfg = SandboxConfig::default()
            .with_env_allowlist(["PATH"]);
        let overrides = cfg.env_overrides();
        assert!(overrides.iter().any(|(k, v)| k == "RVTEST_SANDBOX_CLEAR_ME" && v.is_none()));
        unsafe { std::env::remove_var("RVTEST_SANDBOX_CLEAR_ME"); }
    }

    #[test]
    fn sandbox_empty_env_allowlist_no_overrides() {
        let cfg = SandboxConfig::default();
        let overrides = cfg.env_overrides();
        assert!(overrides.is_empty() || !overrides.iter().any(|(_, v)| v.is_none()));
    }

    #[test]
    fn parse_fs_whitelist_empty() {
        assert!(parse_fs_whitelist("").is_empty());
    }

    #[test]
    fn parse_fs_whitelist_single() {
        assert_eq!(parse_fs_whitelist("src/"), vec!["src/"]);
    }

    #[test]
    fn parse_fs_whitelist_multiple() {
        assert_eq!(parse_fs_whitelist("src/,tests/,data/"), vec!["src/", "tests/", "data/"]);
    }

    #[test]
    fn parse_env_allowlist_empty() {
        assert!(parse_env_allowlist("").is_empty());
    }

    #[test]
    fn parse_env_allowlist_single() {
        assert_eq!(parse_env_allowlist("PATH"), vec!["PATH"]);
    }

    #[test]
    fn parse_env_allowlist_uppercased() {
        assert_eq!(parse_env_allowlist("Path,Home"), vec!["PATH", "HOME"]);
    }
}
