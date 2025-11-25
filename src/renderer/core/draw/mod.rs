use crate::renderer::Renderer;
use std::time::Instant;

// Declare the sub-modules so their 'impl Renderer' blocks are compiled
pub mod game;
pub mod ui;

impl Renderer {
    pub fn render(&mut self, window: &winit::window::Window) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // 1. Calculate FPS (instantaneous + slight smoothing so it updates every frame)
        let now = Instant::now();
        let delta_secs = now.duration_since(self.last_fps_update).as_secs_f64();
        if delta_secs > 0.0 {
            let instantaneous = 1.0 / delta_secs;
            const SMOOTHING: f64 = 0.15;
            self.fps = if self.fps == 0.0 {
                instantaneous
            } else {
                self.fps * (1.0 - SMOOTHING) + instantaneous * SMOOTHING
            };
        }
        self.last_fps_update = now;

        // 2. Update UI (Egui) logic
        // We pass 'window' and '&view' to match the signature in ui.rs
        let (ui_tris, ui_textures) = self.update_ui(window, &view);

        // 3. Generate CommandBuffers
        // Game Layer (Background, Gameplay, or Results)
        let game_cmd = self.render_game_layer(&view)?;

        // UI Layer (Egui) - Drawn on top
        let ui_cmd = self.render_ui_layer(&view, &ui_tris, &ui_textures, window);

        // 4. Submit everything in one go
        self.queue.submit([game_cmd, ui_cmd]);

        output.present();
        Ok(())
    }
}
