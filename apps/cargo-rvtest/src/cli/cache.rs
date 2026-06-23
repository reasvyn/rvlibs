use sha2::Digest;

use rvtest::core;

fn build_cache_dir() -> std::path::PathBuf {
    std::path::PathBuf::from("target/.rvtest-build-cache")
}

fn build_cache_path() -> std::path::PathBuf {
    build_cache_dir().join("manifest.json")
}

pub(crate) fn source_hash_for_build() -> Result<String, String> {
    let mut hasher = sha2::Sha256::new();
    let src_dirs = ["src", "tests"];
    for dir in &src_dirs {
        let dir_path = std::path::Path::new(dir);
        if dir_path.exists() {
            core::hash_dir_recursive(&mut hasher, dir_path)?;
        }
    }
    Ok(format!("{:x}", Digest::finalize(hasher)))
}

pub(crate) fn load_build_cache() -> Option<(String, Vec<std::path::PathBuf>)> {
    let path = build_cache_path();
    let content = std::fs::read_to_string(path).ok()?;
    let manifest: serde_json::Value = serde_json::from_str(&content).ok()?;
    let hash = manifest.get("hash")?.as_str()?.to_owned();
    let bins: Vec<std::path::PathBuf> = manifest
        .get("binaries")?
        .as_array()?
        .iter()
        .filter_map(|v| v.as_str().map(std::path::PathBuf::from))
        .filter(|p| p.exists())
        .collect();
    if bins.is_empty() {
        return None;
    }
    Some((hash, bins))
}

pub(crate) fn save_build_cache(hash: &str, binaries: &[std::path::PathBuf]) {
    let manifest = serde_json::json!({
        "hash": hash,
        "binaries": binaries.iter().map(|b| b.to_string_lossy()).collect::<Vec<_>>(),
    });
    let dir = build_cache_dir();
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(
        build_cache_path(),
        serde_json::to_string_pretty(&manifest).unwrap_or_default(),
    );
}

static BUILD_CACHE_ENABLED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

pub fn set_build_cache_enabled(enabled: bool) {
    BUILD_CACHE_ENABLED.store(enabled, std::sync::atomic::Ordering::SeqCst);
}

pub(crate) fn is_build_cache_enabled() -> bool {
    BUILD_CACHE_ENABLED.load(std::sync::atomic::Ordering::SeqCst)
}

fn warm_state_path() -> std::path::PathBuf {
    std::path::PathBuf::from("target/.rvtest-warm/state.json")
}

pub(crate) fn try_warm_binaries() -> Option<Vec<std::path::PathBuf>> {
    let path = warm_state_path();
    let content = std::fs::read_to_string(path).ok()?;
    let state: serde_json::Value = serde_json::from_str(&content).ok()?;
    let binaries: Vec<std::path::PathBuf> = state["binaries"]
        .as_array()?
        .iter()
        .filter_map(|v| v.as_str().map(std::path::PathBuf::from))
        .filter(|p| p.exists())
        .collect();
    if binaries.is_empty() {
        return None;
    }

    if let Ok(hash) = source_hash_for_build()
        && state.get("hash")?.as_str()? == hash
    {
        return Some(binaries);
    }
    None
}

pub fn save_warm_state(binaries: &[std::path::PathBuf]) {
    if let Ok(hash) = source_hash_for_build() {
        let state = serde_json::json!({
            "hash": hash,
            "binaries": binaries.iter().map(|b| b.to_string_lossy()).collect::<Vec<_>>(),
        });
        let path = warm_state_path();
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = std::fs::write(
            &path,
            serde_json::to_string_pretty(&state).unwrap_or_default(),
        );
    }
}
