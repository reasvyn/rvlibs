use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::core::{BenchStats, TestStatus};

pub(crate) fn run_with_retry(test: &Arc<dyn Fn() + Send + Sync>, retries: u32) -> TestStatus {
    let max_attempts = retries.saturating_add(1);

    for attempt in 1..=max_attempts {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (test)();
        }));

        match result {
            Ok(_) => return TestStatus::Passed,
            Err(panic_info) => {
                if attempt == max_attempts {
                    let reason = extract_panic_message(&panic_info);
                    let reason = crate::secrets::mask_if_enabled(&reason, crate::secrets::is_mask_secrets_enabled());
                    return TestStatus::Failed { reason, location: None };
                }
            }
        }
    }

    TestStatus::Failed {
        reason: "exhausted retries".to_owned(),
        location: None,
    }
}

pub(crate) fn run_with_timeout(
    test: &Arc<dyn Fn() + Send + Sync>,
    timeout: Duration,
    retries: u32,
) -> TestStatus {
    let test = Arc::clone(test);

    let (tx, rx) = std::sync::mpsc::channel();

    let _handle = std::thread::spawn(move || {
        let status = run_with_retry(&test, retries);
        let _ = tx.send(status);
    });

    match rx.recv_timeout(timeout) {
        Ok(status) => status,
        Err(_) => TestStatus::TimedOut { duration: timeout, location: None },
    }
}

pub(crate) fn execute_with_capture(
    test_fn: &Arc<dyn Fn() + Send + Sync>,
    timeout: Option<Duration>,
    retries: u32,
) -> (TestStatus, Option<String>) {
    if !crate::capture::is_capture_enabled() {
        let status = match timeout {
            Some(to) => run_with_timeout(test_fn, to, retries),
            None => run_with_retry(test_fn, retries),
        };
        return (status, None);
    }

    let test_fn = Arc::clone(test_fn);
    let (status, stdout, stderr) = crate::capture::capture(move || {
        match timeout {
            Some(to) => run_with_timeout(&test_fn, to, retries),
            None => run_with_retry(&test_fn, retries),
        }
    });

    let output = {
        let mut parts: Vec<String> = Vec::new();
        if !stdout.is_empty() {
            let masked = crate::secrets::mask_if_enabled(&stdout, crate::secrets::is_mask_secrets_enabled());
            parts.push(format!("stdout:\n{}", masked));
        }
        if !stderr.is_empty() {
            let masked = crate::secrets::mask_if_enabled(&stderr, crate::secrets::is_mask_secrets_enabled());
            parts.push(format!("stderr:\n{}", masked));
        }
        if parts.is_empty() { None } else { Some(parts.join("\n")) }
    };

    (status, output)
}

pub(crate) fn extract_panic_message(panic_info: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = panic_info.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = panic_info.downcast_ref::<String>() {
        s.clone()
    } else {
        "test panicked".to_owned()
    }
}

pub(crate) fn run_benchmark(
    bench_fn: &Arc<dyn Fn() + Send + Sync>,
    iterations: u32,
    threshold: Option<Duration>,
) -> (TestStatus, BenchStats) {
    let mut durations = Vec::with_capacity(iterations as usize);

    for _ in 0..iterations {
        let start = Instant::now();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (bench_fn)()));
        durations.push(start.elapsed());
    }

    let total: Duration = durations.iter().sum();
    let min = *durations.iter().min().unwrap_or(&Duration::ZERO);
    let max = *durations.iter().max().unwrap_or(&Duration::ZERO);
    let mean = Duration::from_nanos((total.as_nanos() / iterations as u128) as u64);

    let stats = BenchStats {
        iterations,
        total,
        min,
        max,
        mean,
    };

    let status = match threshold {
        Some(th) if mean > th => TestStatus::Failed {
            reason: format!("benchmark mean {mean:?} exceeds threshold {th:?}"),
            location: None,
        },
        _ => TestStatus::Passed,
    };

    (status, stats)
}
