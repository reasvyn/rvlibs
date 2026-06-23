//! rvfx — **Body**: Services, rendering, windowing, editor UI.
//!
//! rvfx is the body of the Rveco ecosystem. It implements the ports
//! from rvnx (the brain) via wgpu, winit, and other crates.

pub mod wgpu;
pub mod winit;
pub mod asset;
pub mod ui;
