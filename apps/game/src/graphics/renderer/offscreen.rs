//! Offscreen texture management for editor preview.

use super::Renderer;
use crate::render::draw::draw_game;
use crate::render::mock_data::create_mock_state;
use crate::shared::snapshot::RenderState;
// use crate::ui::page::song_select::UIPanelTextures;

impl Renderer {
    /// Prépare la texture offscreen pour le rendu de l'éditeur
    pub(super) fn ensure_offscreen_texture(&mut self, width: u32, height: u32) {
        if self.offscreen_texture.is_some() && self.offscreen_size == (width, height) {
            return;
        }

        // Libérer l'ancienne texture Egui si elle existe
        if let Some(id) = self.offscreen_id {
            self.ui.free_texture(id);
        }

        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.ctx.config.format,
            usage: wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("Editor Offscreen Texture"),
            view_formats: &[],
        };

        let texture = self.ctx.device.create_texture(&texture_desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Enregistrement de la nouvelle texture dans l'UI principale pour l'afficher
        let id = self
            .ui
            .register_texture(&self.ctx.device, &view, wgpu::FilterMode::Linear);

        self.offscreen_texture = Some(texture);
        self.offscreen_view = Some(view);
        self.offscreen_id = Some(id);
        self.offscreen_size = (width, height);

        log::info!("RENDER: Created offscreen texture {}x{}", width, height);
    }
}

/// Render editor preview to offscreen texture
pub fn render_editor_offscreen(
    renderer: &mut Renderer,
    encoder: &mut wgpu::CommandEncoder,
    window: &winit::window::Window,
) {
    // 1. Récupérer la résolution désirée depuis l'éditeur
    let w = renderer.skin_editor.state.preview_width;
    let h = renderer.skin_editor.state.preview_height;
    renderer.ensure_offscreen_texture(w, h);

    if let Some(target_view) = &renderer.offscreen_view {
        // Clone the view to avoid borrow issues
        let target_view = target_view.clone();

        // 2. Adapter le système de coordonnées à la résolution offscreen
        renderer.resources.pixel_system.update_size(w, h, None);

        // 3. Créer l'état factice (Mock) avec le key count de l'éditeur
        let key_count = renderer.skin_editor.state.preview_key_count;
        let mock_state = create_mock_state(renderer.skin_editor.state.current_scene, key_count);

        // Update key mode if it changed
        if key_count != renderer.current_key_count {
            renderer.current_key_count = key_count;
            renderer.resources.set_key_mode(key_count, &renderer.ctx);
        }

        // 4. Rendu WGPU (Jeu / Background / Notes)
        draw_game(
            &renderer.ctx,
            &mut renderer.resources,
            encoder,
            &target_view,
            &mock_state,
            renderer.current_fps,
        );

        // 5. Rendu EGUI OFFSCREEN
        // Note: egui textures are not shared between UI contexts, so Menu/Result
        // preview would show missing textures. For now, we only render the wgpu layer.
        // The egui layer (buttons, etc.) is skipped for preview purposes.
        // Gameplay uses wgpu_text which works fine.
        match &mock_state {
            RenderState::Menu(_menu_state) => {
                // Skip egui rendering - textures registered in main UI aren't available here
                // Just display the background from wgpu draw_game
            }
            RenderState::Result(_data) => {
                // Skip egui rendering for same reason
            }
            _ => {} // InGame: HUD is handled by draw_game via wgpu_text
        }

        // 6. Restaurer la taille réelle de la fenêtre pour le rendu principal
        renderer.resources.pixel_system.update_size(
            renderer.ctx.config.width,
            renderer.ctx.config.height,
            None,
        );
    }
}
