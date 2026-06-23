use std::process::{Command, Stdio};

pub fn is_nightly() -> bool {
    let output = Command::new("rustc").arg("--version").output().ok();
    match output {
        Some(o) if o.status.success() => {
            let s = String::from_utf8_lossy(&o.stdout);
            s.contains("nightly")
        }
        _ => false,
    }
}

pub fn has_cranelift_component() -> bool {
    let mut cmd = Command::new("rustc");
    cmd.args(["-Zcodegen-backend=cranelift", "--version"]);
    cmd.stdout(Stdio::null()).stderr(Stdio::null());
    cmd.status().map(|s| s.success()).unwrap_or(false)
}
