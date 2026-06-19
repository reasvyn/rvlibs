# rustc Basics

The Rust compiler — flags, optimisation levels, code generation, and incremental compilation.

## Prerequisites

- [Package Management](../cargo/package-management.md) — cargo build system

## Optimisation Levels

```bash
# No optimisation (fast compile, slow code) — default for debug builds
cargo build

# Size optimisation
cargo build --release
# equivalent to -C opt-level=3

# More control via Cargo.toml
[profile.release]
opt-level = 3        # 0–3, s, z
lto = true            # Link-time optimisation
codegen-units = 1     # Slower compile, better optimisation
strip = true          # Remove symbols
```

## Incremental Compilation

```bash
# Enabled by default for debug builds
# Disable for release to get better optimisation
CARGO_INCREMENTAL=0 cargo build --release
```

## Common Flags

```bash
# Emit LLVM IR
rustc --emit llvm-ir main.rs

# Show generated assembly
rustc --emit asm main.rs

# Print optimisation remarks
rustc -C passes=name main.rs

# Target a specific CPU
rustc -C target-cpu=native main.rs
```

## Glossarium

| Term | Definition |
|------|------------|
| LTO | Link-Time Optimisation — optimises across crate boundaries. |
| Codegen Units | Number of parallel compilation units. Higher = faster compile, less optimisation. |
| opt-level | Optimisation level (0–3). Debug default is 0, release default is 3. |

## Next Steps

- [Lints and Diagnostics](lints.md) — compiler warnings and clippy
- [Rustc Book](https://doc.rust-lang.org/rustc/)
