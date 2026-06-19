# Corporate Adoption

How major technology companies are adopting and investing in Rust.

## Prerequisites

- [Milestones](milestones.md) — key moments in Rust's development

## Google

Google is one of the largest Rust adopters:

- **Android** — Over 1.5 million lines of Rust in AOSP. Core components like Keystore, DNS resolver, Wi-Fi stack, and Bluetooth are written in Rust.
- **Fuchsia** — Google's next-gen operating system uses Rust extensively in its Zircon kernel and networking stack.
- **ChromeOS** — Rust code is being integrated into ChromeOS for safer system components.
- **Chromium** — Rust is now allowed in the Chromium codebase (as of 2023).
- **TensorFlow** — Rust bindings and Rust-based ML infrastructure.

Google also actively funds the Rust Foundation and employs several Rust compiler and language team members.

## Microsoft

Microsoft is investing heavily in Rust for systems programming:

- **Windows kernel** — Rust code is being used in the Windows kernel for memory-constrained components.
- **Azure** — IoT Edge, networking, and security tooling are written in Rust.
- **Security** — Microsoft's Security Response Center (MSRC) identified that ~70% of CVEs are memory safety issues, driving their Rust investment.
- **Windows Driver Framework** — Rust support for writing Windows drivers.
- **ferrocene** — Microsoft is a major sponsor of the safety-qualified Rust compiler.

## Meta

Meta uses Rust primarily in its developer tooling and infrastructure:

- **Source control** — Meta's Mercurial server infrastructure uses Rust for performance.
- **Monorepo tooling** — Rust is used for performance-critical internal tools.
- **Open source** — Meta develops and maintains several Rust crates (e.g., `gotham`, `proptest`).

## Amazon (AWS)

AWS has been a significant Rust adopter:

- **AWS Nitro** — The virtualisation platform uses Rust in its cryptographic components.
- **Firecracker** — The microVM powering AWS Lambda and Fargate is written in Rust.
- **s2n-quic** — A QUIC protocol implementation in Rust for AWS networking infrastructure.
- **AWS SDK for Rust** — Official AWS SDK for the Rust language.
- **Bottlerocket** — A Linux-based container OS with components in Rust.

## Cloudflare

Cloudflare has been an early and vocal Rust adopter:

- **Pingora** — Cloudflare's HTTP proxy (replacing NGINX) is written in Rust.
- **Workers** — The edge compute platform uses Rust (via WebAssembly).
- **DNS resolver** — 1.1.1.1's core components are in Rust.
- **Open source** — Cloudflare maintains `pingora`, `quinn`, and many other Rust crates.

## Mozilla

Rust's birthplace, Mozilla used Rust extensively:

- **Servo** — A browser engine written in Rust (later spun off as a community project).
- **Firefox** — Rust components in Firefox include the CSS engine (`style`) and media stack.
- **Firefox Android** — Rewritten components in Rust for performance and safety.

## Other Notable Adopters

| Company | Use Case |
|---------|----------|
| **Apple** | Low-level system components |
| **Dropbox** | File synchronisation engine |
| **Figma** | Performance-critical rendering |
| **Discord** | Backend services, voice processing |
| **Shopify** | Performance-critical checkout systems |
| **Samsung** | Tizen OS, IoT devices |
| **NPM** | Registry services (written in Rust since 2024) |
| **Tailscale** | Mesh VPN implementation |
| **1Password** | Core cryptography components |

## Glossarium

| Term | Definition |
|------|------------|
| CVE | Common Vulnerabilities and Exposures — a catalog of security vulnerabilities. |
| Memory Safety | Freedom from bugs caused by incorrect memory access (use-after-free, buffer overflows). |
| Ferrocene | A safety-qualified (ISO 26262) Rust compiler toolchain. |
| MicroVM | A lightweight virtual machine that starts in milliseconds (e.g., Firecracker). |

## Next Steps

- [Community](community.md) — the Rust community and governance
- [Rust Foundation Member List](https://foundation.rust-lang.org/members/)
