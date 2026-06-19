# Milestones

Key moments in Rust's history — from first commit to global adoption.

## Prerequisites

- [History](history.md) — Rust's origins

## Timeline

| Year | Milestone | Significance |
|------|-----------|-------------|
| **2006** | Graydon Hoare starts Rust | Personal project at Mozilla |
| **2009** | Mozilla sponsors Rust | First institutional backing |
| **2010** | Rust announced publicly | Revealed at Mozilla Summit |
| **2012** | First GitHub commit | Moved from Mercurial to Git |
| **2013** | Servo engine begins | Mozilla's research browser engine in Rust |
| **2014** | Green threads removed | Shift to 1:1 threading model |
| **2015** | **Rust 1.0** | First stable release with backwards compatibility guarantee |
| **2016** | First RustConf | Community conference established |
| **2017** | Rust reaches 100k crates on crates.io | Ecosystem milestone |
| **2018** | **Rust 2018 Edition** | NLL, impl Trait, module improvements |
| **2019** | Rust Foundation announced | Independent governance structure |
| **2020** | Rust 2021 Edition | IntoIterator for arrays, resolver v2 |
| **2021** | Rust in Android | Google officially supporting Rust in AOSP |
| **2022** | **Rust in Linux kernel** | Kernel module support merged |
| **2023** | Rust 2024 Edition | RPITIT, trait improvements |
| **2024** | TIOBE top 10 | Rust enters the top 10 programming languages |

## The Linux Kernel Milestone

2022 was a watershed year: Rust was merged into the Linux kernel as a second language for kernel module development. This meant:

- Kernel drivers could be written in Rust with memory safety guarantees
- The kernel now compiles with `rustc`
- A `rust/` directory appears alongside the traditional C source tree
- Companies like Google, Samsung, and Red Hat invested in Rust-for-Linux

## The Android Milestone

Google announced in 2021 that Rust was being used in Android (AOSP):

- Over 1.5 million lines of Rust in Android by 2023
- Rust is used for core system components (Keystore, DNS, Wi-Fi, Bluetooth)
- Android's vulnerability data shows Rust code has zero memory safety vulnerabilities in production
- Google actively encourages new Android components to be written in Rust

## Glossarium

| Term | Definition |
|------|------------|
| Servo | Mozilla's research browser engine written in Rust, using parallelism. |
| AOSP | Android Open Source Project — the source code for Android. |
| Rust-for-Linux | The project to enable Rust as a kernel module language. |
| TIOBE Index | A measure of programming language popularity based on search engine queries. |

## Next Steps

- [Corporate Adoption](corporate-adoption.md) — how companies are adopting Rust
- [Rust Blog: 1.0 Announcement](https://blog.rust-lang.org/2015/05/15/Rust-1.0.html)
