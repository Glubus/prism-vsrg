//! Background rendering.

use wgpu::RenderPass;

use crate::graphics::Pipelines;

/// Draw a fullscreen background texture.
pub fn draw_background<'a>(
    render_pass: &mut RenderPass<'a>,
    pipelines: &'a Pipelines,
    bind_group: &'a wgpu::BindGroup,
) {
    render_pass.set_pipeline(&pipelines.background);
    render_pass.set_bind_group(0, bind_group, &[]);
    render_pass.draw(0..6, 0..1);
}
