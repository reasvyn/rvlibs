use rvtest::core::CoverageFormat;
use rvtest::coverage::{CoverageCollector, CoverageConfig};

use super::args::Cli;

pub fn run_coverage(args: &Cli) -> ! {
    let cov_format: CoverageFormat = args.coverage_format.parse().unwrap_or_else(|e| {
        eprintln!("{e}, falling back to 'summary'");
        CoverageFormat::Summary
    });

    let cov_config = CoverageConfig {
        enabled: true,
        format: cov_format,
        output_dir: args.coverage_dir.clone(),
        min_threshold: args.coverage_min,
        open_report: args.coverage_open,
        ..Default::default()
    };

    let collector = CoverageCollector::new(cov_config);
    match collector.collect() {
        Ok(report) => {
            println!(
                "Coverage: {:.1}% lines, {:.1}% functions, {:.1}% regions",
                report.line_coverage, report.function_coverage, report.region_coverage,
            );
            if let Some(path) = &report.report_path {
                println!("Report: {}", path.display());
            }
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Coverage collection failed:\n{e}");
            std::process::exit(1);
        }
    }
}
