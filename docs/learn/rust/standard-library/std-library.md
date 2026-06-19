# Standard Library Overview

Key modules in the Rust standard library — beyond the basics.

## Prerequisites

- Basic Rust — familiarity with common types

## Core Modules

| Module | Purpose | Key Items |
|--------|---------|-----------|
| `std::fs` | File system operations | `File`, `read_dir`, `create_dir`, `metadata` |
| `std::io` | Input/output traits | `Read`, `Write`, `BufReader`, `BufWriter` |
| `std::path` | File path handling | `Path`, `PathBuf` |
| `std::net` | Networking | `TcpStream`, `TcpListener`, `UdpSocket` |
| `std::env` | Environment access | `args`, `var`, `current_dir`, `temp_dir` |
| `std::time` | Time and duration | `Duration`, `Instant`, `SystemTime` |
| `std::process` | Child processes | `Command`, `Output`, `Child` |
| `std::sync` | Synchronisation primitives | `Mutex`, `RwLock`, `Arc`, `Barrier` |
| `std::thread` | Threading | `spawn`, `sleep`, `JoinHandle` |
| `std::collections` | Data structures | `HashMap`, `VecDeque`, `BinaryHeap`, `BTreeMap` |
| `std::fmt` | Formatting | `Display`, `Debug`, `format!`, `write!` |

## Path and PathBuf

```rust
use std::path::Path;

let path = Path::new("/tmp/foo.txt");
println!("{}", path.display());
println!("dir: {}", path.parent().unwrap().display());
println!("stem: {}", path.file_stem().unwrap().to_str().unwrap());
println!("ext: {}", path.extension().unwrap().to_str().unwrap());
```

## Environment

```rust
use std::env;

// Command-line arguments
for arg in env::args() {
    println!("{arg}");
}

// Environment variables
let path = env::var("PATH").unwrap_or_default();
let cwd = env::current_dir().unwrap();
let tmp = env::temp_dir();
```

## Time

```rust
use std::time::{Duration, Instant};

let start = Instant::now();
// ... do work ...
let elapsed = start.elapsed();
println!("took {}.{:03}s", elapsed.as_secs(), elapsed.subsec_millis());

let timeout = Duration::from_secs(5);
```

## Glossarium

| Term | Definition |
|------|------------|
| `Path` | A borrowed path (like `&str` for strings). |
| `PathBuf` | An owned, growable path (like `String`). |
| `Duration` | A span of time (seconds + nanoseconds). |
| `Instant` | A measurement of a monotonically nondecreasing clock. |

## Next Steps

- [File I/O](file-io.md) — reading and writing files
- [Rust Standard Library Docs](https://doc.rust-lang.org/std/)
