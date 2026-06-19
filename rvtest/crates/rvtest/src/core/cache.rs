use std::time::SystemTime;

use sha2::Digest;

use crate::core::*;

// Last-run cache — persist full TestRun for --diff / last-run comparison
// ---------------------------------------------------------------------------

fn cache_dir() -> std::path::PathBuf {
    let base = std::env::var("CARGO_MANIFEST_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
    base.join("target/.rvtest-cache")
}

fn last_run_cache_path() -> std::path::PathBuf {
    cache_dir().join("failed.json")
}

fn last_run_snapshot_path() -> std::path::PathBuf {
    cache_dir().join("last-run.json")
}

fn flaky_cache_path() -> std::path::PathBuf {
    cache_dir().join("flaky.json")
}

/// Serializable representation of a [`TestRun`] for the cache.
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct CachedRun {
    suites: Vec<CachedSuite>,
    duration_secs: f64,
    start_time_secs: u64,
    end_time_secs: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CachedSuite {
    name: String,
    tests: Vec<CachedTest>,
    duration_secs: f64,
    kind: String,
    source_path: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CachedTest {
    name: String,
    suite: Option<String>,
    tags: Vec<String>,
    status: String,
    reason: Option<String>,
    duration_secs: f64,
    location_file: Option<String>,
    location_line: Option<u32>,
    location_column: Option<u32>,
    bench_threshold_secs: Option<f64>,
    bench_iterations: Option<u32>,
    bench_mean_secs: Option<f64>,
    bench_min_secs: Option<f64>,
    bench_max_secs: Option<f64>,
}

impl From<&TestRun> for CachedRun {
    fn from(run: &TestRun) -> Self {
        CachedRun {
            suites: run.suites.iter().map(CachedSuite::from).collect(),
            duration_secs: run.duration.as_secs_f64(),
            start_time_secs: run.start_time.duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0),
            end_time_secs: run.end_time.duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0),
        }
    }
}

impl From<&TestSuite> for CachedSuite {
    fn from(suite: &TestSuite) -> Self {
        CachedSuite {
            name: suite.name.clone(),
            tests: suite.tests.iter().map(CachedTest::from).collect(),
            duration_secs: suite.duration.as_secs_f64(),
            kind: match suite.kind {
                TestKind::Unit => "unit".into(),
                TestKind::Integration => "integration".into(),
                TestKind::Doc => "doc".into(),
            },
            source_path: suite.source_path.clone(),
        }
    }
}

impl From<&TestCase> for CachedTest {
    fn from(test: &TestCase) -> Self {
        let (status, reason) = match &test.status {
            TestStatus::Passed => ("passed", None),
            TestStatus::Failed { reason: r, .. } => ("failed", Some(r.clone())),
            TestStatus::Skipped { reason: r } => ("skipped", r.clone()),
            TestStatus::TimedOut { .. } => ("timed_out", None),
        };
        CachedTest {
            name: test.name.clone(),
            suite: test.suite.clone(),
            tags: test.tags.clone(),
            status: status.into(),
            reason,
            duration_secs: test.duration.as_secs_f64(),
            location_file: test.location.as_ref().map(|l| l.file.clone()),
            location_line: test.location.as_ref().map(|l| l.line),
            location_column: test.location.as_ref().and_then(|l| l.column),
            bench_threshold_secs: test.bench_threshold.map(|d| d.as_secs_f64()),
            bench_iterations: test.bench_stats.as_ref().map(|s| s.iterations),
            bench_mean_secs: test.bench_stats.as_ref().map(|s| s.mean.as_secs_f64()),
            bench_min_secs: test.bench_stats.as_ref().map(|s| s.min.as_secs_f64()),
            bench_max_secs: test.bench_stats.as_ref().map(|s| s.max.as_secs_f64()),
        }
    }
}

impl CachedRun {
    fn diff(&self, previous: &CachedRun) -> RunDiff {
        let mut new_failures = Vec::new();
        let mut recovered = Vec::new();
        let mut slower = Vec::new();
        let mut faster = Vec::new();

        let prev_map: std::collections::HashMap<&str, (&CachedSuite, &CachedTest)> = previous
            .suites.iter()
            .flat_map(|s| s.tests.iter().map(move |t| (t.name.as_str(), (s, t))))
            .collect();

        for suite in &self.suites {
            for test in &suite.tests {
                if test.status == "failed" || test.status == "timed_out" {
                    match prev_map.get(test.name.as_str()) {
                        Some((_, prev)) if prev.status == "passed" => {
                            new_failures.push(test.name.clone());
                        }
                        Some((_, prev)) if prev.status == "failed" || prev.status == "timed_out" => {
                            let change = test.duration_secs - prev.duration_secs;
                            if change > 0.5 {
                                slower.push((test.name.clone(), prev.duration_secs, test.duration_secs));
                            }
                        }
                        _ => {
                            new_failures.push(test.name.clone());
                        }
                    }
                } else if test.status == "passed" {
                    match prev_map.get(test.name.as_str()) {
                        Some((_, prev)) if prev.status == "failed" || prev.status == "timed_out" => {
                            recovered.push(test.name.clone());
                        }
                        Some((_, prev)) => {
                            let change = prev.duration_secs - test.duration_secs;
                            if change > 0.5 {
                                faster.push((test.name.clone(), prev.duration_secs, test.duration_secs));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        RunDiff { new_failures, recovered, slower, faster }
    }
}

/// Differences between two test runs.
#[derive(Debug, Default)]
pub struct RunDiff {
    /// Tests that passed before but failed now.
    pub new_failures: Vec<String>,
    /// Tests that failed before but pass now.
    pub recovered: Vec<String>,
    /// Tests that got significantly slower: (name, prev_secs, new_secs).
    pub slower: Vec<(String, f64, f64)>,
    /// Tests that got significantly faster: (name, prev_secs, new_secs).
    pub faster: Vec<(String, f64, f64)>,
}

impl RunDiff {
    /// Returns `true` if there are any differences.
    pub fn has_changes(&self) -> bool {
        !self.new_failures.is_empty() || !self.recovered.is_empty() || !self.slower.is_empty()
    }
}

/// Save the full test run to the cache for later comparison.
pub fn save_full_run(run: &TestRun) {
    let cached: CachedRun = CachedRun::from(run);
    let path = last_run_snapshot_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let json = serde_json::to_string_pretty(&cached).expect("serialize full run");
    let _ = std::fs::write(&path, &json);
}

/// Load the previous full run from the cache.
pub(crate) fn load_previous_run() -> Option<CachedRun> {
    let path = last_run_snapshot_path();
    let content = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&content).ok()
}

/// Compare the current run against the previous cached run.
pub fn compute_diff(run: &TestRun) -> RunDiff {
    match load_previous_run() {
        Some(prev) => {
            let current = CachedRun::from(run);
            current.diff(&prev)
        }
        None => RunDiff::default(),
    }
}

/// Persist the list of failed test names from this run.
pub fn save_failed_tests(run: &TestRun) {
    let path = last_run_cache_path();
    let failed: Vec<String> = run
        .suites
        .iter()
        .flat_map(|s| s.tests.iter())
        .filter(|t| t.status.is_failed())
        .map(|t| t.name.clone())
        .collect();

    if failed.is_empty() {
        let _ = std::fs::remove_file(&path);
        return;
    }

    let json = serde_json::to_string(&failed).expect("serialize failed tests");
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&path, &json);
}

/// Load the list of previously failed test names.
pub fn load_failed_tests() -> Vec<String> {
    let path = last_run_cache_path();
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    serde_json::from_str(&content).unwrap_or_default()
}

/// Persist the list of flaky test names.
pub fn save_flaky_tests(tests: &[String]) {
    let path = flaky_cache_path();
    if tests.is_empty() {
        let _ = std::fs::remove_file(&path);
        return;
    }
    let json = serde_json::to_string(tests).expect("serialize flaky tests");
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&path, &json);
}

/// Load the list of previously flaky test names.
pub fn load_flaky_tests() -> Vec<String> {
    let path = flaky_cache_path();
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    serde_json::from_str(&content).unwrap_or_default()
}

fn baseline_cache_path() -> std::path::PathBuf {
    cache_dir().join("baseline.json")
}

/// Save benchmark baseline data from a TestRun.
pub fn save_bench_baseline(run: &TestRun) {
    use std::collections::HashMap;
    let mut baseline: HashMap<String, serde_json::Value> = HashMap::new();
    for suite in &run.suites {
        for test in &suite.tests {
            if let Some(ref stats) = test.bench_stats {
                let entry = serde_json::json!({
                    "mean_secs": stats.mean.as_secs_f64(),
                    "min_secs": stats.min.as_secs_f64(),
                    "max_secs": stats.max.as_secs_f64(),
                    "iterations": stats.iterations,
                });
                baseline.insert(test.name.clone(), entry);
            }
        }
    }
    let path = baseline_cache_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let json = serde_json::to_string_pretty(&baseline).expect("serialize baseline");
    let _ = std::fs::write(&path, &json);
}

/// Load benchmark baseline data.
pub fn load_bench_baseline() -> std::collections::HashMap<String, serde_json::Value> {
    let path = baseline_cache_path();
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return std::collections::HashMap::new(),
    };
    serde_json::from_str(&content).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Test result cache — skip previously-passed tests when sources haven't changed
// ---------------------------------------------------------------------------

fn result_cache_path() -> std::path::PathBuf {
    cache_dir().join("results.json")
}

/// A cache entry for a test result.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct CachedResult {
    /// SHA-256 hash of source files at the time the test was run.
    source_hash: String,
    /// Whether the test passed.
    passed: bool,
    /// Duration in seconds.
    duration_secs: f64,
    /// Timestamp when the result was cached.
    cached_at: u64,
}

/// The full result cache.
#[derive(serde::Serialize, serde::Deserialize, Default)]
struct ResultCache {
    /// Map of test_name → CachedResult.
    results: std::collections::HashMap<String, CachedResult>,
}

static RESULT_CACHE_ENABLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Enable or disable test result caching globally.
pub fn set_result_cache_enabled(enabled: bool) {
    RESULT_CACHE_ENABLED.store(enabled, std::sync::atomic::Ordering::SeqCst);
}

/// Returns `true` if test result caching is enabled.
pub fn is_result_cache_enabled() -> bool {
    RESULT_CACHE_ENABLED.load(std::sync::atomic::Ordering::SeqCst)
}

/// Compute a hash of source files for cache key determination.
/// Hashes all `.rs` files under `src/` and the current `Cargo.toml`.
pub fn compute_source_hash(root: &std::path::Path) -> Result<String, String> {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();

    // Hash Cargo.toml if it exists
    let cargo_toml = root.join("Cargo.toml");
    if cargo_toml.exists()
        && let Ok(content) = std::fs::read(&cargo_toml) {
            Digest::update(&mut hasher, &content);
        }

    // Hash all .rs files under src/
    let src_dir = root.join("src");
    if src_dir.exists() {
        hash_dir(&mut hasher, &src_dir)?;
    }

    Ok(format!("{:x}", Digest::finalize(hasher)))
}

fn hash_dir(hasher: &mut sha2::Sha256, dir: &std::path::Path) -> Result<(), String> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("read_dir {:?}: {}", dir, e))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("entry: {e}"))?;
        let path = entry.path();
        if path.is_dir() {
            hash_dir(hasher, &path)?;
        } else if path.extension().is_some_and(|e| e == "rs")
            && let Ok(content) = std::fs::read(&path) {
                Digest::update(&mut *hasher, &content);
            }
    }
    Ok(())
}

/// Load the result cache from disk.
fn load_result_cache() -> ResultCache {
    let path = result_cache_path();
    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => ResultCache::default(),
    }
}

/// Save the result cache to disk.
fn save_result_cache(cache: &ResultCache) {
    let path = result_cache_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string(cache) {
        let _ = std::fs::write(&path, &json);
    }
}

/// Get the cached status for a test, if it exists and the source hash matches.
/// Returns `None` if caching is disabled, no cached result exists, or the
/// source hash has changed.
pub fn get_cached_status(test_name: &str, source_hash: &str) -> Option<(bool, f64)> {
    if !is_result_cache_enabled() {
        return None;
    }
    let cache = load_result_cache();
    cache.results.get(test_name).and_then(|r| {
        if r.source_hash == source_hash {
            Some((r.passed, r.duration_secs))
        } else {
            None
        }
    })
}

/// Cache a test result for future runs.
pub fn cache_test_result(test_name: &str, source_hash: &str, passed: bool, duration: std::time::Duration) {
    if !is_result_cache_enabled() {
        return;
    }
    let mut cache = load_result_cache();
    cache.results.insert(test_name.to_owned(), CachedResult {
        source_hash: source_hash.to_owned(),
        passed,
        duration_secs: duration.as_secs_f64(),
        cached_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
    });
    save_result_cache(&cache);
}

/// Clear the result cache entirely.
pub fn clear_result_cache() {
    let path = result_cache_path();
    let _ = std::fs::remove_file(&path);
}

/// Recursively hash all `.rs` files in a directory using the given hasher.
/// Used for build cache key computation.
pub fn hash_dir_recursive(hasher: &mut impl sha2::Digest, dir: &std::path::Path) -> Result<(), String> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("read_dir {:?}: {}", dir, e))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("entry: {e}"))?;
        let path = entry.path();
        if path.is_dir() {
            hash_dir_recursive(hasher, &path)?;
        } else if path.extension().is_some_and(|e| e == "rs")
            && let Ok(content) = std::fs::read(&path) {
                sha2::Digest::update(hasher, &content);
            }
    }
    Ok(())
}

/// Get the number of cached test results.
pub fn cached_result_count() -> usize {
    let cache = load_result_cache();
    cache.results.len()
}

