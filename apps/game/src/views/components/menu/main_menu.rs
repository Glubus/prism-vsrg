//! Main menu screen component.
//!
//! Displays the title and main navigation buttons (Play, Quit).
//! Uses the Prism design system with red/black color scheme.

use crate::graphics::theme::{PRISM_BG, PRISM_PRIMARY, PRISM_PRIMARY_HOVER, PRISM_TEXT};
use egui::{Color32, Label, RichText, Vec2};

/// Event returned by the main menu when user wants to navigate
#[derive(Debug, Clone, PartialEq)]
pub enum MainMenuAction {
    /// User clicked Play - go to song select
    Play,
    /// User clicked Quit - exit the game
    Quit,
    /// No action
    None,
}

pub struct MainMenuScreen;

impl MainMenuScreen {
    /// Renders the main menu screen.
    /// Returns the action to take based on user interaction.
    pub fn render(ctx: &egui::Context) -> MainMenuAction {
        let mut action = MainMenuAction::None;

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(PRISM_BG))
            .show(ctx, |ui| {
                let available_size = ui.available_size();
                let center_x = available_size.x / 2.0;
                let center_y = available_size.y / 2.0;

                // Full-screen centered layout
                ui.allocate_ui_at_rect(
                    egui::Rect::from_min_size(egui::Pos2::ZERO, available_size),
                    |ui| {
                        ui.vertical_centered(|ui| {
                            // Spacer to center content vertically
                            ui.add_space(center_y - 200.0);

                            // Title with glow effect
                            ui.add(
                                Label::new(
                                    RichText::new("PRISM").size(72.0).strong().color(PRISM_TEXT),
                                )
                                .selectable(false),
                            );

                            // Subtitle
                            ui.add_space(8.0);
                            ui.add(
                                Label::new(
                                    RichText::new("Vertical Scrolling Rhythm Game")
                                        .size(16.0)
                                        .color(Color32::from_rgb(136, 136, 136)),
                                )
                                .selectable(false),
                            );

                            ui.add_space(80.0);

                            // Play button
                            if Self::render_menu_button(ui, "▶  PLAY", true) {
                                action = MainMenuAction::Play;
                            }

                            ui.add_space(16.0);

                            // Quit button
                            if Self::render_menu_button(ui, "✕  QUIT", false) {
                                action = MainMenuAction::Quit;
                            }

                            // Footer
                            ui.add_space(80.0);
                            ui.add(
                                Label::new(
                                    RichText::new("Powered by Rust & wgpu")
                                        .size(12.0)
                                        .color(Color32::from_rgb(102, 102, 102)),
                                )
                                .selectable(false),
                            );
                        });
                    },
                );
            });

        action
    }

    fn render_menu_button(ui: &mut egui::Ui, text: &str, is_primary: bool) -> bool {
        let button_width = 280.0;
        let button_height = 56.0;

        // Prism color scheme
        let bg_color = if is_primary {
            Color32::from_rgba_unmultiplied(255, 0, 60, 25) // PRISM_PRIMARY with low alpha
        } else {
            Color32::from_rgba_unmultiplied(255, 255, 255, 10)
        };

        let hover_color = if is_primary {
            Color32::from_rgba_unmultiplied(255, 0, 60, 60)
        } else {
            Color32::from_rgba_unmultiplied(255, 255, 255, 25)
        };

        let text_color = if is_primary {
            PRISM_PRIMARY
        } else {
            Color32::from_gray(200)
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

        // Background with rounded corners
        let fill = if is_hovered { hover_color } else { bg_color };
        painter.rect_filled(rect, 8.0, fill);

        // Border with Prism primary color
        let border_color = if is_hovered {
            text_color
        } else {
            Color32::from_rgba_unmultiplied(text_color.r(), text_color.g(), text_color.b(), 80)
        };
        painter.rect_stroke(
            rect,
            8.0,
            egui::Stroke::new(2.0, border_color),
            egui::StrokeKind::Inside,
        );

        // Glow effect on hover
        if is_hovered && is_primary {
            let glow_rect = rect.expand(4.0);
            painter.rect_stroke(
                glow_rect,
                12.0,
                egui::Stroke::new(2.0, Color32::from_rgba_unmultiplied(255, 0, 60, 30)),
                egui::StrokeKind::Outside,
            );
        }

        // Text
        let final_text_color = if is_hovered {
            hover_text_color
        } else {
            text_color
        };
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(20.0),
            final_text_color,
        );

        response.clicked()
    }
}
