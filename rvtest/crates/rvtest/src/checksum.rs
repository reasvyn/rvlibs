use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use sha2::Digest as Sha2Digest;

/// Whether checksum verification is enabled.
static CHECKSUM_ENABLED: AtomicBool = AtomicBool::new(false);

/// Enable or disable checksum verification globally.
pub fn set_checksum_enabled(enabled: bool) {
    CHECKSUM_ENABLED.store(enabled, Ordering::SeqCst);
}

/// Returns `true` if checksum verification is enabled.
pub fn is_checksum_enabled() -> bool {
    CHECKSUM_ENABLED.load(Ordering::SeqCst)
}

/// SHA-256 hex digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Digest(String);

impl std::fmt::Display for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Compute the SHA-256 digest of a file.
pub fn hash_file(path: &Path) -> Result<Digest, String> {
    use std::io::Read;
    let mut file = std::fs::File::open(path)
        .map_err(|e| format!("cannot open {:?}: {}", path, e))?;
    let mut hasher = sha2::Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf)
            .map_err(|e| format!("cannot read {:?}: {}", path, e))?;
        if n == 0 { break; }
        Sha2Digest::update(&mut hasher, &buf[..n]);
    }
    Ok(Digest(format!("{:x}", Sha2Digest::finalize(hasher))))
}

/// Compute the SHA-256 digest of a byte slice.
pub fn hash_bytes(data: &[u8]) -> Digest {
    let mut hasher = sha2::Sha256::new();
    Sha2Digest::update(&mut hasher, data);
    Digest(format!("{:x}", Sha2Digest::finalize(hasher)))
}

/// Path to the checksum manifest for a given artifacts directory.
fn checksum_manifest_path(dir: &Path) -> PathBuf {
    dir.join(".rvtest-checksums.json")
}

/// A checksum manifest for a set of artifacts.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChecksumManifest {
    /// Map of relative file path → SHA-256 hex digest.
    pub files: HashMap<String, String>,
}

impl ChecksumManifest {
    /// Load the manifest from a directory, if it exists.
    pub fn load(dir: &Path) -> Result<Option<Self>, String> {
        let path = checksum_manifest_path(dir);
        if !path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("cannot read {:?}: {}", path, e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("cannot parse {:?}: {}", path, e))
            .map(Some)
    }

    /// Save the manifest to a directory.
    pub fn save(&self, dir: &Path) -> Result<(), String> {
        let path = checksum_manifest_path(dir);
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("cannot serialize checksums: {}", e))?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("cannot create {:?}: {}", parent, e))?;
        }
        std::fs::write(&path, &content)
            .map_err(|e| format!("cannot write {:?}: {}", path, e))?;
        Ok(())
    }

    /// Register a file's checksum.
    pub fn register(&mut self, rel_path: impl Into<String>, digest: Digest) {
        self.files.insert(rel_path.into(), digest.0);
    }

    /// Verify a file against its registered checksum.
    /// Returns `Ok(())` if the file matches, or `Err` with details.
    pub fn verify(&self, rel_path: &str, dir: &Path) -> Result<(), String> {
        let Some(expected) = self.files.get(rel_path) else {
            return Err(format!("no registered checksum for {rel_path}"));
        };
        let full_path = dir.join(rel_path);
        if !full_path.exists() {
            return Err(format!("file missing: {rel_path} (expected checksum {expected})"));
        }
        let actual = hash_file(&full_path)?;
        if &actual.0 != expected {
            return Err(format!(
                "checksum mismatch for {rel_path}: expected {expected}, got {actual}"
            ));
        }
        Ok(())
    }

    /// Returns `true` if this manifest is empty.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Number of tracked files.
    pub fn len(&self) -> usize {
        self.files.len()
    }
}

/// Update the checksum manifest for a directory of artifacts.
/// Computes hashes for all files matching the given patterns.
pub fn update_checksums(dir: &Path, patterns: &[&str]) -> Result<ChecksumManifest, String> {
    let mut manifest = ChecksumManifest { files: HashMap::new() };

    for pattern in patterns {
        let glob_pattern = dir.join(pattern);
        let glob_str = glob_pattern.to_string_lossy();
        let entries = glob::glob(&glob_str)
            .map_err(|e| format!("invalid glob pattern {glob_str}: {e}"))?;
        for entry in entries {
            let path = entry.map_err(|e| format!("glob error: {e}"))?;
            if path.is_file() {
                let rel = path.strip_prefix(dir)
                    .map_err(|e| format!("path error: {e}"))?
                    .to_string_lossy()
                    .into_owned();
                let digest = hash_file(&path)?;
                manifest.register(rel, digest);
            }
        }
    }

    manifest.save(dir)?;
    Ok(manifest)
}

/// Verify all files in a directory against its checksum manifest.
/// Returns a list of verification errors (empty = all good).
pub fn verify_checksums(dir: &Path) -> Vec<String> {
    let manifest = match ChecksumManifest::load(dir) {
        Ok(Some(m)) => m,
        Ok(None) => return vec!["no checksum manifest found".into()],
        Err(e) => return vec![e],
    };

    let mut errors = Vec::new();
    for rel_path in manifest.files.keys() {
        if let Err(e) = manifest.verify(rel_path, dir) {
            errors.push(e);
        }
    }
    errors
}

/// Only verify if the global flag is enabled.
pub fn verify_if_enabled(dir: &Path) -> Vec<String> {
    if is_checksum_enabled() {
        verify_checksums(dir)
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_bytes_deterministic() {
        let a = hash_bytes(b"hello world");
        let b = hash_bytes(b"hello world");
        assert_eq!(a, b);
    }

    #[test]
    fn hash_bytes_different_inputs_different() {
        let a = hash_bytes(b"hello");
        let b = hash_bytes(b"world");
        assert_ne!(a, b);
    }

    #[test]
    fn hash_file_roundtrip() {
        let dir = std::env::temp_dir().join("rvtest_checksum_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("test.txt");
        std::fs::write(&file, b"content").unwrap();

        let digest = hash_file(&file).unwrap();
        assert!(!digest.0.is_empty());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn manifest_roundtrip() {
        let dir = std::env::temp_dir().join("rvtest_manifest_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let file = dir.join("snap.snap");
        std::fs::write(&file, b"snapshot content").unwrap();

        let mut manifest = ChecksumManifest { files: HashMap::new() };
        let digest = hash_file(&file).unwrap();
        manifest.register("snap.snap", digest);
        manifest.save(&dir).unwrap();

        let loaded = ChecksumManifest::load(&dir).unwrap().unwrap();
        assert!(loaded.verify("snap.snap", &dir).is_ok());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn manifest_verify_mismatch() {
        let dir = std::env::temp_dir().join("rvtest_mismatch_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let file = dir.join("data.txt");
        std::fs::write(&file, b"original").unwrap();

        let mut manifest = ChecksumManifest { files: HashMap::new() };
        manifest.register("data.txt", hash_bytes(b"different content"));
        manifest.save(&dir).unwrap();

        // Should fail because file content doesn't match
        assert!(manifest.verify("data.txt", &dir).is_err());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn manifest_missing_file() {
        let m = ChecksumManifest { files: HashMap::new() };
        assert!(m.is_empty());
    }

    #[test]
    fn set_and_check() {
        set_checksum_enabled(true);
        assert!(is_checksum_enabled());
        set_checksum_enabled(false);
        assert!(!is_checksum_enabled());
    }

    #[test]
    fn update_checksums_creates_manifest() {
        let dir = std::env::temp_dir().join("rvtest_update_checksums");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("snap1.snap"), b"snap1").unwrap();
        std::fs::write(dir.join("snap2.snap"), b"snap2").unwrap();

        let manifest = update_checksums(&dir, &["*.snap"]).unwrap();
        assert_eq!(manifest.len(), 2);

        // Verify saved manifest
        let loaded = ChecksumManifest::load(&dir).unwrap().unwrap();
        assert!(loaded.verify("snap1.snap", &dir).is_ok());
        assert!(loaded.verify("snap2.snap", &dir).is_ok());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
