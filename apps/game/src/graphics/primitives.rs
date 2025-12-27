//! Basic rendering primitives.

use bytemuck::{Pod, Zeroable};

/// Raw instance data for instanced rendering.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InstanceRaw {
    /// Center position in normalized coordinates [-1, 1]
    pub offset: [f32; 2],
    /// Size in normalized coordinates
    pub scale: [f32; 2],
}

/// Quad instance for colored rectangles.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct QuadInstance {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 4],
}

/// Progress bar instance for the progress shader.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ProgressInstance {
    pub center: [f32; 2],
    pub size: [f32; 2],
    pub filled_color: [f32; 4],
    pub empty_color: [f32; 4],
    pub progress: f32,
    pub mode: u32, // 0 = horizontal bar, 1 = radial
}
