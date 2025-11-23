use crate::components::map_list::MapListComponent;
use crate::menu::MenuState;
use std::sync::{Arc, Mutex};
use wgpu::{Device, Queue, RenderPipeline, Buffer, TextureView, SurfaceError};
use wgpu_text::TextBrush;
use bytemuck;

/// Menu de sélection de chansons
pub struct SongSelectionMenu {
    map_list: MapListComponent,
    screen_width: f32,
    screen_height: f32,
}

impl SongSelectionMenu {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self {
            map_list: MapListComponent::new(screen_width, screen_height),
            screen_width,
            screen_height,
        }
    }
    
    /// Met à jour la taille de l'écran
    pub fn update_size(&mut self, screen_width: f32, screen_height: f32) {
        self.screen_width = screen_width;
        self.screen_height = screen_height;
        self.map_list.update_size(screen_width, screen_height);
    }
    
    /// Met à jour le menu avec l'état actuel
    pub fn update(&mut self, menu_state: &Arc<Mutex<MenuState>>) {
        let (visible_items, selected_index) = {
            let menu_state_guard = menu_state.lock().unwrap();
            let visible_items = menu_state_guard.get_visible_items();
            (
                visible_items.iter().map(|(bs, bms)| (bs.clone(), bms.clone())).collect::<Vec<_>>(),
                menu_state_guard.get_relative_selected_index()
            )
        };
        
        self.map_list.update_cards(&visible_items, selected_index);
    }
    
    /// Rend le menu (quads + texte)
    pub fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        text_brush: &mut TextBrush,
        view: &TextureView,
        quad_pipeline: &RenderPipeline,
        quad_buffer: &Buffer,
        fps: f64,
        menu_state: &Arc<Mutex<MenuState>>,
    ) -> Result<(), SurfaceError> {
        // Créer les quads pour le panel et les cards
        let quad_instances = self.map_list.create_quads();
        
        // Rendre les quads (panel + cards)
        if !quad_instances.is_empty() {
            queue.write_buffer(quad_buffer, 0, bytemuck::cast_slice(&quad_instances));
            
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Song Selection Menu Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                
                render_pass.set_pipeline(quad_pipeline);
                render_pass.set_vertex_buffer(0, quad_buffer.slice(..));
                render_pass.draw(0..4, 0..quad_instances.len() as u32);
            }
            queue.submit(std::iter::once(encoder.finish()));
        }
        
        // Stocker les valeurs avant l'emprunt mutable
        let map_list_x = self.map_list.x;
        let map_list_width = self.map_list.width;
        let cards_empty = self.map_list.cards.is_empty();
        
        // Créer les sections de texte
        let mut text_sections = self.map_list.create_text_sections();
        
        // Ajouter le FPS
        let fps_text = format!("FPS: {:.0}", fps);
        text_sections.push(wgpu_text::glyph_brush::Section {
            screen_position: (self.screen_width - 100.0, 20.0),
            bounds: (self.screen_width, self.screen_height),
            text: vec![
                wgpu_text::glyph_brush::Text::new(&fps_text)
                    .with_scale(24.0)
                    .with_color([1.0, 1.0, 1.0, 1.0]),
            ],
            ..Default::default()
        });
        
        // Ajouter le titre
        text_sections.push(wgpu_text::glyph_brush::Section {
            screen_position: (map_list_x + 20.0, 50.0),
            bounds: (map_list_width, self.screen_height),
            text: vec![
                wgpu_text::glyph_brush::Text::new("Map Selection")
                    .with_scale(36.0)
                    .with_color([1.0, 1.0, 1.0, 1.0]),
            ],
            ..Default::default()
        });
        
        // Ajouter les instructions
        let instructions = "Arrows: Navigate | Enter: Play | F8: Rescan | ESC: Quit | PageUp/Down: Rate";
        text_sections.push(wgpu_text::glyph_brush::Section {
            screen_position: (20.0, self.screen_height - 50.0),
            bounds: (self.screen_width, self.screen_height),
            text: vec![
                wgpu_text::glyph_brush::Text::new(instructions)
                    .with_scale(18.0)
                    .with_color([0.5, 0.5, 0.5, 1.0]),
            ],
            ..Default::default()
        });
        
        // Ajouter l'affichage du rate
        let rate_text = {
            if let Ok(menu_state) = menu_state.lock() {
                format!("Rate: {:.1}x", menu_state.rate)
            } else {
                "Rate: 1.0x".to_string()
            }
        };
        text_sections.push(wgpu_text::glyph_brush::Section {
            screen_position: (20.0, self.screen_height - 80.0),
            bounds: (self.screen_width, self.screen_height),
            text: vec![
                wgpu_text::glyph_brush::Text::new(&rate_text)
                    .with_scale(20.0)
                    .with_color([1.0, 1.0, 0.5, 1.0]),
            ],
            ..Default::default()
        });
        
        // Ajouter le message si aucune map
        if cards_empty {
            text_sections.push(wgpu_text::glyph_brush::Section {
                screen_position: (map_list_x + 20.0, self.screen_height / 2.0),
                bounds: (self.screen_width, self.screen_height),
                text: vec![
                    wgpu_text::glyph_brush::Text::new("No map found")
                        .with_scale(36.0)
                        .with_color([1.0, 0.5, 0.5, 1.0]),
                ],
                ..Default::default()
            });
            
            text_sections.push(wgpu_text::glyph_brush::Section {
                screen_position: (map_list_x + 20.0, self.screen_height / 2.0 + 50.0),
                bounds: (self.screen_width, self.screen_height),
                text: vec![
                    wgpu_text::glyph_brush::Text::new("Press F8 to scan maps")
                        .with_scale(24.0)
                        .with_color([0.7, 0.7, 0.7, 1.0]),
                ],
                ..Default::default()
            });
        }
        
        // Rendre le texte
        text_brush.queue(device, queue, text_sections).map_err(|_| SurfaceError::Lost)?;
        
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Song Selection Menu Text Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            text_brush.draw(&mut render_pass);
        }
        
        queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}

