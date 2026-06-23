# Architecture

rvlibs is a Rust monorepo organised as a Cargo workspace — but it is not just a collection of libraries. It is an **ecosystem with gravity**, where every crate has a concrete role and a clear destination: the **Rveco** application.

Rveco is the estuary. The main binary that unifies all crates into one creative development suite. Without Rveco, we have a pile of libraries waiting for a purpose. With Rveco, every crate knows where it is heading.

This architecture uses a biological metaphor: **brain, body, and estuary**, built on a foundation of shared contracts.

The project follows a three-role architecture with a foundation:

```
                    ┌──────────────────────────────────────────┐
                    │           rveco (Application)            │
                    │  Estuary — main binary that unifies      │
                    │  all crates into one experience          │
                    └──────────────────┬───────────────────────┘
                                       │
          ┌────────────────────────────┼────────────────────────────┐
          │                            │                            │
          ▼                            ▼                            ▼
┌─────────────────────┐   ┌──────────────────────┐   ┌──────────────────────┐
│   rvnx (Brain)      │   │   rvfx (Body)         │   │   rv* (Atomic)      │
│   Intelligence,     │   │   Physical impl,      │   │   Standalone library │
│   logic, ECS,       │   │   rendering, input,   │   │   rvmath, rvtest,    │
│   scene graph,      │   │   window, audio, UI   │   │   rvstat, rvphysic…  │
│   port traits       │   │                       │   │                      │
└──────────┬──────────┘   └──────────┬────────────┘   └──────────┬───────────┘
           │                         │                           │
           └─────────────────────────┼───────────────────────────┘
                                     │
                                     ▼
                    ┌──────────────────────────────────────────┐
                    │        rvlibs (Foundation)               │
                    │  Shared contracts, error types, meta     │
                    │  Zero external dependencies              │
                    └──────────────────────────────────────────┘
```

## Roles

| Role | Crate | Description |
|------|-------|-------------|
| **Foundation** | `rvlibs`, `rvmath` | Lowest level. Zero external deps. Used by all crates. |
| **Brain** | `rvnx` | System intelligence: ECS, scene graph, AI, physics, port traits. May use external dependencies. |
| **Body** | `rvfx` | Physical implementation: rendering (wgpu), windowing (winit), input, audio, editor UI. Depends on `rvnx` to implement its ports. |
| **Atomic** | `rvmath`, `rvtest`, etc. | Standalone library per domain. Added as needed. |
| **Composite** | `rvnx`, `rvfx` | Consumer — coordinates multiple atomic crates. May be split later if too large. |
| **Estuary** | `rveco` (in `apps/`) | Main binary. Binds all crates into one application. |

## Workspace Layout

```
Cargo.toml (root workspace)

crates/
├── rvlibs/           # Shared contracts (zero deps)
├── rvmath/           # Mathematics library
├── rvtest/           # Testing library
├── rvtest-macros/    # Proc-macros for rvtest
├── rvnx/             # Brain — engine core, ports, ECS, scene
└── rvfx/             # Body — services, rendering, window, UI

apps/
├── cargo-rvtest/     # CLI binary for rvtest
└── rveco/            # Main application — ecosystem estuary

docs/                 # Documentation
```

## Foundation Layer — rvmath Module Hierarchy

```
prelude ← re-exports from all layers
  ├── algebra    — symbolic expressions (Expr enum), simplification, differentiation
  ├── calculus   — analytical & numerical derivatives, integrals, series
  ├── consts     — mathematical and physical constants
  ├── geometry   — constants, 2D/3D shape formulas, unit-aware
  ├── la         — VecN<T,N>, MatN<T,R,C>, Tensor<T> (linear algebra)
  ├── num        — Num<T> wrapper, Numeric trait, Percentage, Set, Fraction, Complex
  ├── ops        — functional-style arithmetic, trig, log, hyperbolic
  ├── polynomial — Polynomial<T>, closed-form & numerical root-finding, interpolation
  ├── special    — special functions (gamma, erf, zeta, bessel)
  ├── graph      — Graph<N,E>, traversal, shortest path, MST, max flow
  ├── unit       — type-safe dimensional analysis, families and units via macros
  └── utils      — string expression parser and evaluator
```

## Foundation Layer — rvtest Module Hierarchy

```
lib.rs
├── arch      — architecture dependency checks
├── assert    — assertion macros with diff output
├── capture   — per-test stdout/stderr capture
├── checksum  — snapshot checksum verification
├── config    — TOML config file support (rvtest.toml)
├── core      — TestSuite, TestCase, TestStatus (no deps)
├── coverage  — multi-strategy coverage collector
├── coverage_raw — pure-Rust .profraw parser
├── daemon    — persistent compile daemon
├── env       — scoped environment variable overrides
├── fs        — scoped temporary test directories
├── matcher   — composable matchers (eq, gt, contains, etc.)
├── mock      — Spy, Stub, patch!
├── param     — parametrized tests
├── property  — property-based testing: Strategy, check
├── report    — TestReporter trait + Pretty/TAP/JUnit/JSON/Compact/GitHub/HTML/Nextest
├── runner    — TestRunner, execution orchestration
├── sandbox   — filesystem, network, env isolation
├── secrets   — secrets masking in test output
├── snapshot  — file-based snapshot assertions
├── spec      — BDD builder: describe/it/run
├── tag       — tag/name filtering
└── clock     — Clock trait + RealClock/MockClock
```

## rvnx (Brain) — Module Hierarchy

rvnx is the **brain** of the system. It handles intelligence, logic, and core data structures. It defines ports (traits) that rvfx will implement.

```
src/
├── ecs/             — Entity Component System
│   ├── world.rs     — World (spawn, despawn, query)
│   ├── entity.rs    — Entity (handle/ID)
│   ├── component.rs — Component trait, storage
│   └── system.rs    — System trait, schedule
├── scene/           — Scene graph
│   ├── graph.rs     — SceneGraph, NodeId
│   ├── node.rs      — SceneNode (parent, children, components)
│   ├── transform.rs — Transform (position, rotation, scale)
│   └── camera.rs    — Camera (projection, viewport)
├── ports/           — Trait definitions (what the brain needs from the body)
│   ├── gpu.rs       — GpuPort: device, swapchain, draw, buffers
│   ├── window.rs    — WindowPort: create, resize, input events
│   └── asset.rs     — AssetPort: load files, textures, models
├── render/          — Render graph & material definitions
│   ├── graph.rs     — RenderGraph, RenderPass
│   ├── material.rs  — Material, shader parameter definitions
│   └── mesh.rs      — Mesh (vertices, indices)
├── document/        — Document model (for the code editor)
│   ├── text.rs      — TextBuffer, rope, undo/redo
│   └── cursor.rs    — CursorPosition, selection
└── math/            — Re-exports from rvmath with engine-specific additions
```

**Dependencies:** rvlibs, rvmath, and other external crates as needed.

## rvfx (Body) — Module Hierarchy

rvfx is the **body** of the system. It implements the ports from rvnx and provides physical services: rendering, windowing, input, UI.

```
src/
├── wgpu/            — GpuPort impl via wgpu
│   ├── device.rs    — Adapter, device, queue creation
│   ├── surface.rs   — Swapchain, resize, present
│   ├── shader.rs    — Shader compilation via naga
│   └── pipeline.rs  — Pipeline management
├── winit/           — WindowPort impl via winit
│   ├── window.rs    — Window creation, dpi, resize
│   └── input.rs     — Keyboard, mouse, gamepad mapping
├── asset/           — AssetPort impl
│   ├── loader.rs    — File system loader
│   ├── image.rs     — Image format parsing (png, jpeg)
│   └── model.rs     — Model format parsing (gltf, obj)
├── ui/              — Editor UI toolkit (built on wgpu)
│   ├── layout/      — Box, flex, dock layout engine
│   ├── widgets/     — Panel, Button, Text, Tree, Menu, Input
│   ├── text/        — Font atlas, glyph shaping, text rendering
│   └── renderer/    — Batched wgpu UI renderer
└── audio/           — Audio output (future)
```

**Dependencies:** rvnx (port implementations), wgpu, winit, naga, font library.

## rveco (Estuary) — Module Hierarchy

rveco is the **main binary** that brings rvnx and rvfx together into a real application.

```
src/
├── main.rs          — Entry point
├── app.rs           — App lifecycle: init, run, shutdown
├── shell.rs         — Editor shell: docking, menu bar, status bar
├── workspace.rs     — Project/workspace model
├── plugin_host.rs   — Plugin system (WASI-based)
└── panels/          — Editor panels
    ├── mod.rs       — Panel trait, registry
    ├── scene.rs     — Scene hierarchy panel
    ├── inspector.rs — Property inspector panel
    ├── console.rs   — Console/log panel
    └── file_explorer.rs — File tree panel
```

## Dependency Graph

```
                    rvlibs (zero deps)
                   ┌────┴────┐
                   │         │
                rvmath    rvtest (rvtest-macros)
                   │
              ┌────┴────┐
              │         │
            rvnx     rvtest (dev-dep)
           ┌──┴──┐
           │     │
         rvfx  rveco
           │
         wgpu, winit, naga, ...
```

## Key Design Decisions

- **Brain & Body** — rvnx defines ports (traits); rvfx implements them. The brain does not need to know about physical implementation details.
- **rvnx may have dependencies** — Unlike a "pure Rust engine layer", rvnx as the brain may use external crates.
- **Atomic & Composite** — Atomic crates are standalone per domain (rvmath, rvtest). Composite crates (rvnx, rvfx) consume them. Not preemptive: new crates are only created when truly needed.
- **apps/ for binaries** — Libraries in `crates/`, applications/binaries in `apps/`.
- **Rveco as estuary** — The ecosystem has a clear direction: all crates flow toward Rveco as the final destination.
- **Dogfooding** — The entire ecosystem uses rvtest for testing.
- **Safe Rust** — No `unsafe` in any crate.
