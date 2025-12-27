//! Progress bar/circle rendering.

use wgpu::RenderPass;

use crate::graphics::Pipelines;
use crate::graphics::primitives::ProgressInstance;

/// Draw progress indicators (song progress bar, etc.)
pub fn draw_progress<'a>(
    render_pass: &mut RenderPass<'a>,
    pipelines: &'a Pipelines,
    progress_buffer: &'a wgpu::Buffer,
    queue: &wgpu::Queue,
    instances: &[ProgressInstance],
) {
    if instances.is_empty() {
        return;
    }

    queue.write_buffer(progress_buffer, 0, bytemuck::cast_slice(instances));
    render_pass.set_pipeline(&pipelines.progress);
    render_pass.set_vertex_buffer(0, progress_buffer.slice(..));
    render_pass.draw(0..4, 0..instances.len() as u32);
}
