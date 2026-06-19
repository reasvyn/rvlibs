# History of Rust

From a personal project to one of the most loved languages — the story of Rust.

## Prerequisites

- None — this is a historical overview

## The Origins (2006–2010)

Rust began in 2006 as Graydon Hoare's personal project while working at Mozilla. The name comes from rust fungi, "a robust, resilient, and reliable thing" — and a pun on the opposite of "C" (as in, a safe alternative to C/C++).

Mozilla began sponsoring the project in 2009, and it was announced publicly in 2010. The original compiler was written in OCaml and generated native code via LLVM.

## The Early Years (2010–2015)

The early language was quite different from modern Rust:

- **Green threads** — Initially Rust had lightweight threads (like Go's goroutines), removed in 2014
- **Sigils** — Pointers had sigils (`~`, `@`, `&`) that were eventually replaced with `Box`, `Rc`, and `&`
- **Typestate system** — A static analysis system later simplified into lifetimes
- **Semicolons matter** — Early Rust didn't require `;` after every statement; uniform function call syntax was debated heavily

**2012 — Rust gets its first commit on GitHub** (moved from Mercurial).

**2014 — 1.0 freeze.** The team famously "rewrote the compiler in Rust" (bootstrapping), removing the OCaml implementation entirely.

## The 1.0 Era (2015–2018)

**May 15, 2015 — Rust 1.0 is released.** This marked the beginning of the "stable" era with backwards compatibility guarantees.

The Rust 1.0 era saw:

- **Cargo** — The package manager that shipped with 1.0, revolutionising Rust's ecosystem
- **crates.io** — The package registry, launched alongside Cargo
- **The Rust Book** — "The Book" was rewritten for 1.0 and became the gold standard for language documentation
- **Growing ecosystem** — early crates like `hyper`, `serde`, `tokio` laid the foundation

## The Edition Era (2018–2021)

**2018 — Rust 2018 Edition.** The first edition introduced:

- Non-Lexical Lifetimes (NLL)
- Module system improvements (`crate::`, `self::`)
- `impl Trait` in argument and return positions
- `dyn Trait` syntax for trait objects
- `const fn` basics

**2019 — The Rust Foundation is announced.**

**2020 — Rust 2021 Edition.** Incremental improvements:

- `IntoIterator` for arrays
- `TryInto`, `TryFrom` and `Try` in const generics
- `Cargo feature resolver v2`
- `panic!` macro consistency

## The Recent Years (2022–present)

**2022 — Rust enters the Linux kernel** (experimental support for kernel modules).

**2023 — Rust 2024 Edition** with traits, impl blocks, and `RPITIT` (Return Position Impl Trait In Traits) reaching stability.

Key developments:

- **Rust in the Linux kernel** — Official support merged for kernel modules
- **ferrocene** — First safety-qualified Rust compiler (ISO 26262)
- **Rust for embedded** — Growing maturity in embedded and IoT
- **Big tech adoption** — Google (Android), Microsoft (Windows), Meta, Amazon (AWS) investing heavily

## Glossarium

| Term | Definition |
|------|------------|
| Edition | A Rust release that may include breaking changes, gated behind `edition = "20xx"` in Cargo.toml. |
| NLL | Non-Lexical Lifetimes — the borrow checker that works per-expression instead of per-statement. |
| Bootstrapping | Compiling a compiler with itself — Rust's compiler is written in Rust. |
| RFCS | Rust Foundation Change Suggestions — the community process for language changes. |

## Next Steps

- [Milestones](milestones.md) — key moments in Rust's timeline
- [Rust 1.0 Announcement](https://blog.rust-lang.org/2015/05/15/Rust-1.0.html)
- [The Rust 2018 Edition](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)
- [History of Rust (Rust Book)](https://doc.rust-lang.org/book/ch00-00-introduction.html#a-taste-of-rust)
