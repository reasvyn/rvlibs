use std::process::{Command, Stdio};

pub fn auto_tune() {
    let cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let ram_mb = detect_ram_mb();

    println!("  System:");
    println!("    CPUs:      {cpus}");
    println!(
        "    RAM:       {} MB",
        ram_mb.map(|m| m.to_string()).unwrap_or("unknown".into())
    );

    println!("  Recommendations:");
    if let Some(linker) = detect_fast_linker() {
        println!("    Use:       cargo rvtest --fast");
        println!("    (fast linker: {linker})");
    }
    if ram_mb.unwrap_or(0) >= 4096 {
        println!(
            "    Parallel:  cargo rvtest --max-threads {} --cache",
            cpus.saturating_sub(1)
        );
    } else {
        println!("    Sequential: cargo rvtest --no-parallel");
    }
    if ram_mb.is_none() || ram_mb.unwrap_or(0) < 4096 {
        println!("    Incremental: set CARGO_INCREMENTAL=1");
    }
    if ram_mb.unwrap_or(0) >= 16000 {
        println!("    Ramdisk:   consider TARGET_DIR=/dev/shm/rust-target (16+ GB RAM detected)");
    }
}

fn detect_ram_mb() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        let content = std::fs::read_to_string("/proc/meminfo").ok()?;
        for line in content.lines() {
            if let Some(rest) = line.strip_prefix("MemTotal:") {
                let parts: Vec<&str> = rest.split_whitespace().collect();
                let kb: u64 = parts.first()?.parse().ok()?;
                return Some(kb / 1024);
            }
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = 0;
    }
    None
}

pub(crate) fn detect_fast_linker() -> Option<&'static str> {
    if Command::new("mold")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
    {
        return Some("mold");
    }
    if Command::new("ld.lld")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
    {
        return Some("lld");
    }
    None
}
