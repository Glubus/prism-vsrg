//! HUD rendering - score, combo, accuracy, judgements.

use wgpu::RenderPass;

use crate::graphics::Pipelines;
use crate::graphics::primitives::QuadInstance;

/// Draw HUD quads (score background, combo panel, etc.)
pub fn draw_hud_quads<'a>(
    render_pass: &mut RenderPass<'a>,
    pipelines: &'a Pipelines,
    quad_buffer: &'a wgpu::Buffer,
    queue: &wgpu::Queue,
    quads: &[QuadInstance],
) {
    if quads.is_empty() {
        return;
    }

    queue.write_buffer(quad_buffer, 0, bytemuck::cast_slice(quads));
    render_pass.set_pipeline(&pipelines.quad);
    render_pass.set_vertex_buffer(0, quad_buffer.slice(..));
    render_pass.draw(0..4, 0..quads.len() as u32);
}
