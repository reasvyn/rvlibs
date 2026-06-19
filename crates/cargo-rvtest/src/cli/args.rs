use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "cargo-rvtest",
    about = "A Next Level Testing Library for Rust",
    version,
    long_about = "rvtest is A Next Level Testing Library for Rust.\n\n\
                   rvtest extends Rust's built-in testing with BDD specs, \
                   property-based testing, parametrized tests, and rich reporting. \
                   Use `cargo rvtest` to run tests or `cargo rvtest --coverage` \
                   for code coverage analysis."
)]
pub struct Cli {
    // === Profile ===
    #[arg(long = "profile")]
    pub profile: Option<String>,

    // === Test options ===
    #[arg(short = 'f', long = "filter")]
    pub filter: Option<String>,

    #[arg(short = 't', long = "tag")]
    pub include_tags: Vec<String>,

    #[arg(short = 'E', long = "exclude-tag")]
    pub exclude_tags: Vec<String>,

    #[arg(short = 'r', long = "retries", default_value = "0")]
    pub retries: u32,

    #[arg(long = "auto-retry")]
    pub auto_retry: bool,

    #[arg(long = "timeout")]
    pub timeout_secs: Option<f64>,

    #[arg(long = "no-parallel")]
    pub no_parallel: bool,

    #[arg(long = "max-threads", default_value = "0")]
    pub max_threads: usize,

    #[arg(short = 'F', long = "format", default_value = "pretty")]
    pub format: String,

    #[arg(long = "fail-fast")]
    pub fail_fast: bool,

    #[arg(long = "seed")]
    pub seed: Option<u64>,

    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    #[arg(long = "show-output")]
    pub show_output: bool,

    #[arg(long = "skip")]
    pub skip: Option<String>,

    #[arg(long = "shuffle")]
    pub shuffle: bool,

    #[arg(long = "color")]
    pub color: Option<String>,

    // === Watch / Daemon mode ===
    #[arg(long = "watch")]
    pub watch: bool,

    #[arg(long = "daemon")]
    pub daemon: bool,

    // === Flaky detection ===
    #[arg(long = "detect-flaky", default_missing_value = "10", num_args = 0..=1, require_equals = false, default_value = "0")]
    pub detect_flaky: u32,

    // === Fast mode ===
    #[arg(long = "fast")]
    pub fast: bool,

    #[arg(long = "cranelift")]
    pub cranelift: bool,

    #[arg(long = "parallel-frontend")]
    pub parallel_frontend: Option<usize>,

    // === Profiling ===
    #[arg(long = "profile-slow", default_missing_value = "5", num_args = 0..=1, require_equals = false)]
    pub profile_slow: Option<u32>,

    // === Snapshot options ===
    #[arg(long = "update-all")]
    pub update_all: bool,

    #[arg(long = "review")]
    pub review: bool,

    // === Coverage options ===
    #[arg(long = "coverage")]
    pub coverage: bool,

    #[arg(long = "coverage-format", default_value = "summary")]
    pub coverage_format: String,

    #[arg(long = "coverage-dir", default_value = "target/coverage")]
    pub coverage_dir: PathBuf,

    #[arg(long = "coverage-min")]
    pub coverage_min: Option<f64>,

    #[arg(long = "coverage-open")]
    pub coverage_open: bool,

    // === Diff / Changed ===
    #[arg(long = "diff")]
    pub diff: bool,

    #[arg(long = "changed")]
    pub changed: bool,

    // === New features ===
    #[arg(long = "list")]
    pub list: bool,

    #[arg(long = "retest")]
    pub retest: bool,

    #[arg(long = "failed", hide = true)]
    pub failed: bool,

    #[arg(long = "workspace")]
    pub workspace: bool,

    // === Flaky quarantine ===
    #[arg(long = "quarantine")]
    pub quarantine: bool,

    #[arg(long = "flaky-report")]
    pub flaky_report: bool,

    #[arg(long = "include-flaky")]
    pub include_flaky: bool,

    #[arg(long = "unquarantine")]
    pub unquarantine: bool,

    // === Benchmark options ===
    #[arg(long = "bench")]
    pub bench: bool,

    #[arg(long = "bench-iterations", default_value = "100")]
    pub bench_iterations: u32,

    #[arg(long = "bench-threshold")]
    pub bench_threshold_ms: Option<f64>,

    /// Save benchmark results as baseline for future comparison.
    #[arg(long = "save-baseline")]
    pub save_baseline: bool,

    /// Compare benchmark results against saved baseline.
    #[arg(long = "compare-baseline")]
    pub compare_baseline: bool,

    /// Generate an HTML report file.
    #[arg(long = "report-html")]
    pub report_html: Option<String>,

    /// Analyze test gaps (coverage vs descriptions).
    #[arg(long = "gap-analysis")]
    pub gap_analysis: bool,

    /// Process isolation — run each test in a separate OS process.
    #[arg(long = "isolate")]
    pub isolate: bool,

    /// Mask secrets (API keys, tokens, passwords) in captured test output.
    #[arg(long = "mask-secrets")]
    pub mask_secrets: bool,

    /// Enable test sandboxing (restrict filesystem, network, env).
    #[arg(long = "sandbox")]
    pub sandbox: bool,

    /// Filesystem whitelist for sandbox mode (comma-separated paths).
    #[arg(long = "sandbox-fs", default_value = "src/,tests/")]
    pub sandbox_fs: String,

    /// Disable network access in sandbox mode.
    #[arg(long = "sandbox-no-net")]
    pub sandbox_no_net: bool,

    /// Environment variable allowlist for sandbox mode (comma-separated).
    #[arg(long = "sandbox-env", default_value = "PATH,HOME,TMPDIR")]
    pub sandbox_env: String,

    /// Max open file descriptors in sandbox mode.
    #[arg(long = "sandbox-max-fds")]
    pub sandbox_max_fds: Option<u64>,

    /// Max child processes / threads in sandbox mode.
    #[arg(long = "sandbox-max-procs")]
    pub sandbox_max_procs: Option<u64>,

    /// Max stack size in bytes in sandbox mode.
    #[arg(long = "sandbox-max-stack")]
    pub sandbox_max_stack: Option<u64>,

    /// Disable core dumps in sandbox mode.
    #[arg(long = "sandbox-no-core")]
    pub sandbox_no_core: bool,

    /// Max virtual address space in bytes in sandbox mode.
    #[arg(long = "sandbox-max-as")]
    pub sandbox_max_as: Option<u64>,

    /// Auto-detect and apply optimal performance settings.
    #[arg(long = "tune")]
    pub tune: bool,

    /// Profile test execution — show time breakdown by phase.
    #[arg(long = "why-slow")]
    pub why_slow: bool,

    /// Warm daemon mode — build once, cache results, reuse on next run.
    #[arg(long = "warm")]
    pub warm: bool,

    /// Verify integrity checksums of artifacts (snapshots, cache, etc.).
    #[arg(long = "verify-checksums")]
    pub verify_checksums: bool,

    /// Enforce sandbox permissions strictly (fail on violation).
    #[arg(long = "sandbox-enforce")]
    pub sandbox_enforce: bool,

    /// Impact analysis — smarter test selection via import/module graph.
    #[arg(long = "impact")]
    pub impact: bool,

    /// Enable test result caching (skip passing tests with unchanged sources).
    #[arg(long = "cache")]
    pub cache: bool,

    /// Clear the test result cache.
    #[arg(long = "clear-cache")]
    pub clear_cache: bool,

    /// Enable test binary build caching (skip rebuild when sources unchanged).
    #[arg(long = "build-cache")]
    pub build_cache: bool,

    /// Open the HTML report in the system browser.
    #[arg(long = "open-report")]
    pub open_report: bool,
}
