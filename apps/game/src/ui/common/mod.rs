//! Common UI components module.
//!
//! Reusable wgpu-based visual components:
//! - `cube`: Rotating 3D wireframe cube
//! - `particles`: Animated particle background with connection lines

pub mod cube;
pub mod particles;

pub use cube::{CubeConfig, CubeRenderer};
pub use particles::{ParticleConfig, ParticleSystem};
