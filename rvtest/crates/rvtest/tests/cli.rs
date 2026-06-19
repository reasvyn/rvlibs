use std::process::Command;

fn rvtest(args: &[&str]) -> (bool, String, String) {
    let output = Command::new("cargo")
        .arg("rvtest")
        .args(args)
        .output()
        .expect("failed to run cargo rvtest");
    let ok = output.status.success();
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    (ok, stdout, stderr)
}

#[test]
fn help_succeeds() {
    let (ok, stdout, _) = rvtest(&["--help"]);
    assert!(ok, "cargo rvtest --help should exit 0");
    assert!(stdout.contains("rvtest"));
    assert!(stdout.contains("--filter"));
    assert!(stdout.contains("--format"));
}

#[test]
fn filter_nonexistent_is_fast() {
    let (ok, stdout, _) = rvtest(&["-F", "compact", "-f", "NONEXISTENT_TEST_XYZ"]);
    assert!(ok, "filtering nonexistent test should succeed");
    assert!(stdout.contains("0/0"));
}

#[test]
fn format_compact() {
    let (ok, stdout, _) = rvtest(&["-F", "compact"]);
    assert!(ok, "compact format should succeed");
    assert!(
        stdout.contains("passed") || stdout.contains("Results:"),
        "compact output should contain Results line"
    );
}

#[test]
fn format_json() {
    let (ok, stdout, _) = rvtest(&["-F", "json"]);
    assert!(ok, "json format should succeed");
    assert!(stdout.contains(r#""success""#));
}

#[test]
fn format_tap() {
    let (ok, stdout, _) = rvtest(&["-F", "tap"]);
    assert!(ok, "tap format should succeed");
    assert!(stdout.starts_with("1.."), "TAP output should start with 1..N");
}

#[test]
fn format_junit() {
    let (ok, stdout, _) = rvtest(&["-F", "junit"]);
    assert!(ok, "junit format should succeed");
    assert!(stdout.contains("<?xml"));
    assert!(stdout.contains("<testsuites"));
}

#[test]
fn verbose_flag() {
    let (ok, _, _) = rvtest(&["-v", "-F", "compact"]);
    assert!(ok, "verbose mode should succeed");
}

#[test]
fn tag_filter() {
    let (ok, stdout, _) = rvtest(&["-F", "compact", "--tag", "spec"]);
    assert!(ok, "tag filter should succeed");
    assert!(stdout.contains("passed") || stdout.contains("Results:"));
}

#[test]
fn exclude_tag() {
    let (ok, _, _) = rvtest(&["-F", "compact", "-E", "NONEXISTENT_TAG"]);
    assert!(ok, "exclude tag should succeed");
}

#[test]
fn retries_flag() {
    let (ok, _, _) = rvtest(&["-F", "compact", "--retries", "1"]);
    assert!(ok, "retries flag should succeed");
}
