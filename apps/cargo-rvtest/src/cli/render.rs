use rvtest::core::ReportFormat;
use rvtest::report::{self, TestReporter};

use super::profile;

pub fn open_in_browser(path: &str) {
    let _ = std::process::Command::new("xdg-open").arg(path).spawn();
    let _ = std::process::Command::new("open").arg(path).spawn();
    let _ = std::process::Command::new("cmd")
        .args(["/c", "start", path])
        .spawn();
}

pub fn render(
    format: &ReportFormat,
    run: &rvtest::core::TestRun,
    slow_count: usize,
    use_colour: bool,
) -> String {
    let reporter: Box<dyn TestReporter> = match format {
        ReportFormat::Pretty => Box::new(report::PrettyReporter::new().colour(use_colour)),
        ReportFormat::Tap => Box::new(report::TapReporter),
        ReportFormat::Junit => Box::new(report::JunitReporter::new()),
        ReportFormat::Json => Box::new(report::JsonReporter),
        ReportFormat::Compact => Box::new(report::CompactReporter),
        ReportFormat::Github => Box::new(report::GithubReporter),
        ReportFormat::Agent => Box::new(report::AgentReporter),
        ReportFormat::Html => Box::new(report::HtmlReporter),
        ReportFormat::Nextest => Box::new(report::NextestReporter),
    };
    let mut out = reporter.report(run);

    if slow_count > 0 {
        let slow = run.slowest(slow_count);
        if !slow.is_empty() {
            use std::fmt::Write;
            let _ = writeln!(out);
            let _ = writeln!(out, "  {} Slowest tests", profile::dim("⏱"));
            for (i, test) in slow.iter().enumerate() {
                let dur = report::format_duration(test.duration);
                let name = test.name.replace(" :: ", " > ");
                let _ = writeln!(out, "    {}.  {:>8}  {}", i + 1, dur, name);
            }
        }
    }

    out
}
