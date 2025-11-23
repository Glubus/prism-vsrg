use wgpu::{Device, Queue, SurfaceError, RenderPipeline, BindGroup, TextureView, Buffer};
use wgpu_text::TextBrush;
use crate::menu::MenuState;
use std::sync::{Arc, Mutex};
use crate::components::song_selection_menu::SongSelectionMenu;

/// Rend le menu de sélection de map
pub fn render_menu(
    device: &Device,
    queue: &Queue,
    text_brush: &mut TextBrush,
    menu_state: &Arc<Mutex<MenuState>>,
    screen_width: f32,
    screen_height: f32,
    fps: f64,
    view: &TextureView,
    background_pipeline: Option<&RenderPipeline>,
    background_bind_group: Option<&BindGroup>,
    quad_pipeline: &RenderPipeline,
    quad_buffer: &Buffer,
) -> Result<(), SurfaceError> {
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    
    // Rendre le background en premier si disponible
    if let (Some(pipeline), Some(bind_group)) = (background_pipeline, background_bind_group) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Background Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        
        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    } else {
        // Pas de background, juste clear
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Menu Clear Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
    }
    
    // Soumettre l'encoder du background avant de continuer
    queue.submit(std::iter::once(encoder.finish()));
    
    // Créer et mettre à jour le menu de sélection
    let mut song_menu = SongSelectionMenu::new(screen_width, screen_height);
    song_menu.update(menu_state);
    
    // Rendre le menu
    song_menu.render(device, queue, text_brush, view, quad_pipeline, quad_buffer, fps, menu_state)?;
    
    Ok(())
}
