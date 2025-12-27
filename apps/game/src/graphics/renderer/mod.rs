//! Main renderer orchestrating all graphics operations.
//!
//! This module is split into submodules for each render target:
//! - `main_menu` - Main menu screen
//! - `song_select` - Song selection / menu
//! - `editor` - Skin editor
//! - `result` - Result screen
//! - `gameplay` - In-game rendering
//! - `offscreen` - Offscreen texture management for editor preview

mod editor;
mod gameplay;
mod main_menu;
mod offscreen;
mod result;
mod song_select;

use crate::input::events::GameAction;
use crate::render::context::RenderContext;
use crate::render::draw::draw_game;
use crate::render::resources::RenderResources;
use crate::render::ui::UiOverlay;
use crate::shared::snapshot::RenderState;
use crate::ui::page::MainMenuPage;
use crate::ui::page::song_select::SongSelectScreen;
use crate::views::components::editor::SkinEditorLayout;
use crate::views::components::menu::result_screen::ResultScreen;
use std::sync::Arc;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::PhysicalKey;
use winit::window::Window;

pub struct Renderer {
    pub ctx: RenderContext,

    // UI Principale (affichée à l'écran)
    ui: UiOverlay,

    // UI Secondaire (pour le rendu dans la texture de l'éditeur)
    offscreen_ui: UiOverlay,

    pub resources: RenderResources,
    current_state: RenderState,

    // Screens
    song_select_screen: SongSelectScreen,
    result_screen: ResultScreen,
    skin_editor: SkinEditorLayout,
    main_menu_page: MainMenuPage,

    // Offscreen Rendering (pour l'éditeur)
    offscreen_texture: Option<wgpu::Texture>,
    offscreen_view: Option<wgpu::TextureView>,
    offscreen_id: Option<egui::TextureId>,
    offscreen_size: (u32, u32),

    // FPS
    last_frame_time: std::time::Instant,
    frame_count: u32,
    last_fps_update: std::time::Instant,
    current_fps: f64,

    // Key mode tracking
    current_key_count: usize,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let ctx = RenderContext::new(window.clone()).await;

        // Instance UI pour la fenêtre principale
        let ui = UiOverlay::new(window.clone(), &ctx.device, ctx.config.format);

        // Instance UI pour le rendu offscreen (prévisualisation)
        let offscreen_ui = UiOverlay::new(window.clone(), &ctx.device, ctx.config.format);

        let mut resources = RenderResources::new(&ctx, &ui.ctx);

        // Positionnement initial des éléments
        resources.update_component_positions(ctx.config.width as f32, ctx.config.height as f32);

        // Initialize main menu page with GPU resources
        let mut main_menu_page = MainMenuPage::new();
        main_menu_page.init_gpu(
            &ctx.device,
            ctx.config.format,
            ctx.config.width as f32,
            ctx.config.height as f32,
        );

        Self {
            ctx,
            ui,
            offscreen_ui,
            resources,
            current_state: RenderState::MainMenu,

            song_select_screen: SongSelectScreen::new(),
            result_screen: ResultScreen::new(),
            skin_editor: SkinEditorLayout::new(),
            main_menu_page,

            offscreen_texture: None,
            offscreen_view: None,
            offscreen_id: None,
            offscreen_size: (0, 0),

            last_frame_time: std::time::Instant::now(),
            frame_count: 0,
            last_fps_update: std::time::Instant::now(),
            current_fps: 0.0,

            current_key_count: 4, // Default to 4K
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.ctx.resize(new_size);
        self.resources
            .pixel_system
            .update_size(new_size.width, new_size.height, None);
        self.resources.text_brush.resize_view(
            new_size.width as f32,
            new_size.height as f32,
            &self.ctx.queue,
        );
        self.resources.update_component_positions(
            self.ctx.config.width as f32,
            self.ctx.config.height as f32,
        );
        self.main_menu_page
            .resize(new_size.width as f32, new_size.height as f32);
    }

    pub fn handle_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        let handled = self.ui.handle_input(window, event);

        if let WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(code),
                    ..
                },
            ..
        } = event
            && self.resources.settings.remapping_column.is_some()
        {
            let label = format!("{:?}", code);
            self.resources.settings.push_keybind_key(label);
        }

        handled
    }

    pub fn update_state(&mut self, new_state: RenderState) {
        // Detect game entry and switch key mode if needed
        // Note: Editor mode key switching is handled by offscreen.rs based on preview_key_count
        if let RenderState::InGame(ref snapshot) = new_state {
            if snapshot.key_count != self.current_key_count {
                self.current_key_count = snapshot.key_count;
                self.resources.set_key_mode(snapshot.key_count, &self.ctx);
                log::info!("RENDERER: Switched to {}K mode", snapshot.key_count);
            }
        }

        if let RenderState::Menu(ref menu) = new_state
            && let Some((set, _)) = menu.get_selected_beatmapset()
            && let Some(img_path) = &set.image_path
        {
            self.resources
                .load_background(&self.ctx.device, &self.ctx.queue, img_path);
        }
        self.current_state = new_state;
    }

    pub fn render(&mut self, window: &Window) -> Result<Vec<GameAction>, wgpu::SurfaceError> {
        // --- FPS Calculation ---
        self.frame_count += 1;
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_fps_update);
        if elapsed.as_secs_f64() >= 1.0 {
            self.current_fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.last_fps_update = now;
        }

        // Préparation de la frame
        let output = self.ctx.surface.get_current_texture()?;
        let swapchain_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut actions_to_send = Vec::new();

        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main Encoder"),
            });

        let is_editor = matches!(self.current_state, RenderState::Editor(_));

        // =================================================================================
        // 1. GAME RENDERING LAYER (Offscreen ou Onscreen)
        // =================================================================================

        if is_editor {
            self.render_editor_offscreen(&mut encoder, window);
        } else if matches!(self.current_state, RenderState::MainMenu) {
            // --- MAIN MENU: Render 3D cube and particles ---
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Main Menu 3D Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &swapchain_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.02,
                                g: 0.02,
                                b: 0.02,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    ..Default::default()
                });
                let aspect = self.ctx.config.width as f32 / self.ctx.config.height as f32;
                self.main_menu_page
                    .render_3d(&mut pass, &self.ctx.queue, aspect);
            }
        } else {
            // --- MODE NORMAL : RENDU ONSCREEN ---
            draw_game(
                &self.ctx,
                &mut self.resources,
                &mut encoder,
                &swapchain_view,
                &self.current_state,
                self.current_fps,
            );
        }

        // =================================================================================
        // 2. UI LAYER PRINCIPALE (SWAPCHAIN)
        // =================================================================================

        self.ui.begin_frame(window);
        let ctx_egui = self.ui.ctx.clone();

        // Clone state data before mutable operations to avoid borrow conflicts
        let current_state = self.current_state.clone();

        match &current_state {
            RenderState::MainMenu => {
                use crate::ui::page::main_menu::MainMenuAction;
                let action = self.main_menu_page.render_ui(&ctx_egui);
                match action {
                    MainMenuAction::Play => actions_to_send.push(GameAction::Confirm),
                    MainMenuAction::Quit => actions_to_send.push(GameAction::Back),
                    MainMenuAction::None => {}
                }
            }
            RenderState::Menu(menu_state) => {
                song_select::render(
                    self,
                    &ctx_egui,
                    menu_state,
                    &swapchain_view,
                    &mut actions_to_send,
                );
            }
            RenderState::Editor(_snapshot) => {
                editor::render(self, &ctx_egui);
            }
            RenderState::Result(data) => {
                result::render(self, &ctx_egui, data, &mut actions_to_send);
            }
            RenderState::InGame(snapshot) => {
                gameplay::render(&ctx_egui, snapshot, self.ctx.config.width as f32);
            }
            _ => {}
        }

        self.ui
            .end_frame_and_draw(&self.ctx, &mut encoder, &swapchain_view);
        self.ctx.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(actions_to_send)
    }

    /// Render editor preview to offscreen texture
    fn render_editor_offscreen(&mut self, encoder: &mut wgpu::CommandEncoder, window: &Window) {
        offscreen::render_editor_offscreen(self, encoder, window);
    }
}
