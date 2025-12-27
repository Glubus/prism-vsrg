//! Graphics module - centralized rendering architecture.
//!
//! This module provides:
//! - `Context`: wgpu device, queue, and surface configuration
//! - `Pipelines`: all render pipelines
//! - `SkinAssets`: centralized texture loading from skin
//! - `Primitives`: basic rendering types (InstanceRaw, etc.)
//! - `draw`: gameplay rendering functions

pub mod assets;
pub mod context;
pub mod draw;
pub mod pipelines;
pub mod primitives;
pub mod renderer;
pub mod theme;

// pub use draw::GameplayBuffers;
pub use pipelines::Pipelines;
