//! Main menu page - title screen with cube and navigation.
//!
//! Layout:
//! - Left panel: Title + navigation buttons
//! - Right side: Rotating 3D cube
//! - Background: Particle animation

use crate::graphics::theme::{PRISM_PRIMARY, PRISM_PRIMARY_HOVER, PRISM_TEXT};
use crate::ui::common::{CubeConfig, CubeRenderer, ParticleConfig, ParticleSystem};
use egui::epaint::StrokeKind;

use egui::{Color32, Vec2};

/// Action returned by the main menu
#[derive(Debug, Clone, PartialEq)]
pub enum MainMenuAction {
    Play,
    Quit,
    None,
}

/// Main menu page with cube and particles.
pub struct MainMenuPage {
    cube: Option<CubeRenderer>,
    particles: Option<ParticleSystem>,
}

impl MainMenuPage {
    pub fn new() -> Self {
        Self {
            cube: None,
            particles: None,
        }
    }

    /// Initialize GPU resources (call once after device is available)
    pub fn init_gpu(
        &mut self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        width: f32,
        height: f32,
    ) {
        if self.cube.is_none() {
            self.cube = Some(CubeRenderer::new(device, format, CubeConfig::large()));
        }
        if self.particles.is_none() {
            self.particles = Some(ParticleSystem::new(
                device,
                format,
                width,
                height,
                ParticleConfig::default(),
            ));
        }
    }

    /// Resize the particle system
    pub fn resize(&mut self, width: f32, height: f32) {
        if let Some(ref mut particles) = self.particles {
            particles.resize(width, height);
        }
    }

    /// Render the 3D elements (cube + particles) to the render pass.
    /// Call this BEFORE rendering egui.
    pub fn render_3d<'a>(
        &'a mut self,
        render_pass: &mut wgpu::RenderPass<'a>,
        queue: &wgpu::Queue,
        aspect_ratio: f32,
    ) {
        // Render particles first (background)
        if let Some(ref mut particles) = self.particles {
            particles.render(render_pass, queue);
        }

        // Render cube
        if let Some(ref cube) = self.cube {
            cube.render(render_pass, queue, aspect_ratio);
        }
    }

    /// Render the egui UI overlay.
    /// Returns the action to take based on user interaction.
    pub fn render_ui(&self, ctx: &egui::Context) -> MainMenuAction {
        let mut action = MainMenuAction::None;

        let title_color = PRISM_TEXT; // White title

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                let available_size = ui.available_size();

                // Button dimensions (much larger)
                let button_width = 350.0;
                let button_height = 80.0;
                let button_spacing = 24.0;

                // Position everything at the center (on top of the cube)
                let center_x = available_size.x / 2.0;
                let center_y = available_size.y / 2.0;

                // Title positioned directly above buttons
                let title_y = center_y - button_height - button_spacing - 50.0;

                // Main title in white (no glow/outline)
                ui.painter().text(
                    egui::pos2(center_x, title_y),
                    egui::Align2::CENTER_CENTER,
                    "PRISM",
                    egui::FontId::proportional(72.0),
                    title_color,
                );

                // Buttons stacked vertically below title, centered
                let button_start_y = center_y - button_height / 2.0;
                let button_left = center_x - button_width / 2.0;

                // PLAY button
                let play_rect = egui::Rect::from_min_size(
                    egui::pos2(button_left, button_start_y),
                    egui::vec2(button_width, button_height),
                );

                // QUIT button below PLAY
                let quit_rect = egui::Rect::from_min_size(
                    egui::pos2(button_left, button_start_y + button_height + button_spacing),
                    egui::vec2(button_width, button_height),
                );

                // Draw PLAY button
                let play_response = ui.allocate_rect(play_rect, egui::Sense::click());
                let play_hovered = play_response.hovered();
                let play_bg = if play_hovered {
                    Color32::from_rgba_unmultiplied(0, 255, 255, 40)
                } else {
                    Color32::from_rgba_unmultiplied(20, 20, 30, 200)
                };
                let play_border = if play_hovered {
                    Color32::from_rgb(0, 255, 255)
                } else {
                    Color32::from_rgb(60, 60, 80)
                };

                ui.painter().rect_filled(play_rect, 8.0, play_bg);
                ui.painter().rect_stroke(
                    play_rect,
                    8.0,
                    egui::Stroke::new(2.0, play_border),
                    StrokeKind::Inside,
                );
                ui.painter().text(
                    play_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "▶  PLAY",
                    egui::FontId::proportional(22.0),
                    if play_hovered {
                        Color32::from_rgb(0, 255, 255)
                    } else {
                        PRISM_TEXT
                    },
                );

                if play_response.clicked() {
                    action = MainMenuAction::Play;
                }

                // Draw QUIT button
                let quit_response = ui.allocate_rect(quit_rect, egui::Sense::click());
                let quit_hovered = quit_response.hovered();
                let quit_bg = if quit_hovered {
                    Color32::from_rgba_unmultiplied(255, 80, 80, 30)
                } else {
                    Color32::from_rgba_unmultiplied(20, 20, 30, 200)
                };
                let quit_border = if quit_hovered {
                    Color32::from_rgb(255, 100, 100)
                } else {
                    Color32::from_rgb(60, 60, 80)
                };

                ui.painter().rect_filled(quit_rect, 8.0, quit_bg);
                ui.painter().rect_stroke(
                    quit_rect,
                    8.0,
                    egui::Stroke::new(2.0, quit_border),
                    StrokeKind::Inside,
                );
                ui.painter().text(
                    quit_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "✕  QUIT",
                    egui::FontId::proportional(22.0),
                    if quit_hovered {
                        Color32::from_rgb(255, 100, 100)
                    } else {
                        Color32::from_rgb(180, 180, 180)
                    },
                );

                if quit_response.clicked() {
                    action = MainMenuAction::Quit;
                }
            });

        action
    }

    fn render_menu_button(ui: &mut egui::Ui, text: &str, is_primary: bool) -> bool {
        let button_width = 260.0;
        let button_height = 52.0;

        // Prism color scheme
        let bg_color = if is_primary {
            Color32::from_rgba_unmultiplied(255, 0, 60, 20)
        } else {
            Color32::from_rgba_unmultiplied(255, 255, 255, 8)
        };

        let hover_color = if is_primary {
            Color32::from_rgba_unmultiplied(255, 0, 60, 50)
        } else {
            Color32::from_rgba_unmultiplied(255, 255, 255, 20)
        };

        let text_color = if is_primary {
            PRISM_PRIMARY
        } else {
            Color32::from_gray(180)
        };

        let hover_text_color = if is_primary {
            PRISM_PRIMARY_HOVER
        } else {
            Color32::WHITE
        };

        let response =
            ui.allocate_response(Vec2::new(button_width, button_height), egui::Sense::click());

        let rect = response.rect;
        let painter = ui.painter();
        let is_hovered = response.hovered();

        // Background
        let fill = if is_hovered { hover_color } else { bg_color };
        painter.rect_filled(rect, 6.0, fill);

        // Border
        let border_color = if is_hovered {
            text_color
        } else {
            Color32::from_rgba_unmultiplied(text_color.r(), text_color.g(), text_color.b(), 60)
        };
        painter.rect_stroke(
            rect,
            6.0,
            egui::Stroke::new(1.5, border_color),
            egui::StrokeKind::Inside,
        );

        // Glow effect on hover (primary only)
        if is_hovered && is_primary {
            let glow_rect = rect.expand(3.0);
            painter.rect_stroke(
                glow_rect,
                9.0,
                egui::Stroke::new(2.0, Color32::from_rgba_unmultiplied(255, 0, 60, 25)),
                egui::StrokeKind::Outside,
            );
        }

        // Text
        let final_text_color = if is_hovered {
            hover_text_color
        } else {
            text_color
        };
        let text_pos = rect.left_center() + Vec2::new(24.0, 0.0);
        painter.text(
            text_pos,
            egui::Align2::LEFT_CENTER,
            text,
            egui::FontId::proportional(18.0),
            final_text_color,
        );

        response.clicked()
    }
}

impl Default for MainMenuPage {
    fn default() -> Self {
        Self::new()
    }
}
