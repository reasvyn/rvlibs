use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::core::{CoverageFormat, CoverageReport};

use super::parser::parse_raw_profile;
use super::types::CoverageTotals;

pub fn compute_coverage_from_profraw(path: &Path) -> Result<CoverageTotals, String> {
    let data = std::fs::read(path).map_err(|e| format!("read {:?}: {e}", path))?;
    let profile = parse_raw_profile(&data)?;

    if profile.functions.is_empty() {
        return Ok(CoverageTotals::new());
    }

    let total_counters = profile
        .functions
        .iter()
        .map(|f| f.num_counters as u64)
        .sum::<u64>();
    let covered_counters = profile
        .functions
        .iter()
        .map(|f| f.covered as u64)
        .sum::<u64>();

    let total_funcs = profile.functions.len() as u64;
    let covered_funcs = profile
        .functions
        .iter()
        .filter(|f| f.covered > 0)
        .count() as u64;

    Ok(CoverageTotals {
        total_counters,
        covered_counters,
        total_functions: total_funcs,
        covered_functions: covered_funcs,
    })
}

pub struct RawCoverageRunner {
    pub output_dir: PathBuf,
    pub extra_test_args: Vec<String>,
}

impl RawCoverageRunner {
    pub fn run(&self, format: CoverageFormat) -> Result<CoverageReport, String> {
        let out_dir = &self.output_dir;
        std::fs::create_dir_all(out_dir)
            .map_err(|e| format!("mkdir {:?}: {e}", out_dir))?;

        let profraw_pattern = out_dir.join("test_%p.profraw");

        let build = self.cargo_test_no_run()?;
        let binaries = parse_test_binaries(&build.stdout);

        if binaries.is_empty() {
            return Err("no test binaries produced".into());
        }

        for bin in &binaries {
            let status = Command::new(bin)
                .env(
                    "LLVM_PROFILE_FILE",
                    profraw_pattern.to_str().unwrap(),
                )
                .args(&self.extra_test_args)
                .status()
                .map_err(|e| format!("run {:?}: {e}", bin))?;
            if !status.success() {
                eprintln!("warning: {:?} exited non-zero", bin);
            }
        }

        let mut totals = CoverageTotals::new();

        let entries = std::fs::read_dir(out_dir)
            .map_err(|e| format!("read_dir {:?}: {e}", out_dir))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("entry: {e}"))?;
            let path = entry.path();
            if path.extension().is_none_or(|e| e != "profraw") {
                continue;
            }
            match compute_coverage_from_profraw(&path) {
                Ok(t) => totals.add(&t),
                Err(e) => {
                    eprintln!("warning: skipping {:?}: {e}", path);
                }
            }
            let _ = std::fs::remove_file(&path);
        }

        if totals.total_counters == 0 {
            return Err("no .profraw files generated or all were empty".into());
        }

        let line_cov = totals.line_pct();
        let func_cov = totals.func_pct();
        let region_cov = totals.region_pct();

        let report_path = match format {
            CoverageFormat::Summary => None,
            _ => {
                let path = out_dir.join(report_filename(format));
                let summary = format!(
                    "Lines:    {:.1}%\nFunctions:  {:.1}%\nRegions:   {:.1}%\n",
                    line_cov, func_cov, region_cov
                );
                std::fs::write(&path, &summary)
                    .map_err(|e| format!("write {:?}: {e}", path))?;
                Some(path)
            }
        };

        Ok(CoverageReport {
            line_coverage: line_cov,
            function_coverage: func_cov,
            region_coverage: region_cov,
            format,
            report_path,
        })
    }

    fn cargo_test_no_run(&self) -> Result<std::process::Output, String> {
        let mut cmd = Command::new("cargo");
        cmd.args(["test", "--no-run", "--message-format=json"])
            .env("CARGO_INCREMENTAL", "0")
            .env("RUSTFLAGS", "-Cinstrument-coverage")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit());

        if !self.extra_test_args.is_empty() {
            cmd.arg("--").args(&self.extra_test_args);
        }

        cmd.output()
            .map_err(|e| format!("cargo test --no-run: {e}"))
    }
}

pub fn write_report(format: CoverageFormat, line_cov: f64, func_cov: f64, region_cov: f64, path: &Path) -> Result<(), String> {
    let content = match format {
        CoverageFormat::Summary => String::new(),
        _ => format!(
            "Lines:    {:.1}%\nFunctions:  {:.1}%\nRegions:   {:.1}%\n",
            line_cov, func_cov, region_cov
        ),
    };
    if !content.is_empty() {
        std::fs::write(path, &content).map_err(|e| format!("write {:?}: {e}", path))?;
    }
    Ok(())
}

pub fn report_filename(format: CoverageFormat) -> String {
    match format {
        CoverageFormat::Summary => "summary.txt".into(),
        CoverageFormat::Html => "index.html".into(),
        CoverageFormat::Lcov => "lcov.info".into(),
        CoverageFormat::Json => "coverage.json".into(),
        CoverageFormat::Cobertura => "cobertura.xml".into(),
    }
}

pub fn parse_test_binaries(json_output: &[u8]) -> Vec<PathBuf> {
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct CargoArtifact {
        reason: String,
        filenames: Vec<String>,
        #[serde(default)]
        target_kind: Vec<String>,
        #[serde(default)]
        profile: Option<ArtifactProfile>,
    }

    #[derive(Deserialize)]
    struct ArtifactProfile {
        #[serde(rename = "test")]
        is_test: bool,
    }

    let text = String::from_utf8_lossy(json_output);
    let mut binaries = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(artifact) = serde_json::from_str::<CargoArtifact>(line) {
            if artifact.reason != "compiler-artifact" {
                continue;
            }
            let is_test_bin = artifact
                .profile
                .as_ref()
                .map(|p| p.is_test)
                .unwrap_or(false)
                || artifact.target_kind.iter().any(|k| k == "bin" || k == "test");
            if !is_test_bin {
                continue;
            }
            let only_doc_test = artifact.target_kind.iter().all(|k| k == "test")
                && artifact.filenames.iter().any(|f| {
                    let stem = Path::new(f).file_stem().and_then(|s| s.to_str()).unwrap_or("");
                    !stem.contains("integration") && !stem.contains("cargo_rvtest") && !stem.contains("rvtest-")
                });
            if only_doc_test {
                continue;
            }

            for filename in &artifact.filenames {
                let path = PathBuf::from(filename);
                if path.is_file() {
                    binaries.push(path);
                }
            }
        }
    }
    binaries
}

impl fmt::Display for CoverageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CoverageFormat::Summary => "summary",
            CoverageFormat::Html => "html",
            CoverageFormat::Lcov => "lcov",
            CoverageFormat::Json => "json",
            CoverageFormat::Cobertura => "cobertura",
        };
        write!(f, "{s}")
    }
}
