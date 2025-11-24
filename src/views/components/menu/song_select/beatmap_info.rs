use egui::{Color32, RichText, Ui};

use crate::database::models::{Beatmap, Beatmapset};
use crate::models::settings::HitWindowMode;

pub struct BeatmapInfo {
    selected_rating_tab: u8, // 0 = osu, 1 = etterna, 2 = quaver
}

impl BeatmapInfo {
    pub fn new() -> Self {
        Self {
            selected_rating_tab: 0,
        }
    }

    pub fn render(
        &mut self,
        ui: &mut Ui,
        beatmapset: &Beatmapset,
        beatmap: Option<&Beatmap>,
        rate: f64,
        hit_window_mode: HitWindowMode,
        hit_window_value: f64,
    ) {
        egui::Frame::default()
            .corner_radius(5.0)
            .outer_margin(10.0)
            .inner_margin(10.0)
            .fill(Color32::from_rgba_unmultiplied(38, 38, 38, 230))
            .show(ui, |ui| {
                ui.set_width(ui.available_rect_before_wrap().width());
                
                // Nom de la difficulté en gros
                if let Some(bm) = beatmap {
                    if let Some(diff_name) = &bm.difficulty_name {
                        ui.heading(RichText::new(diff_name).size(24.0));
                        ui.add_space(10.0);
                    }
                }

                // Image en bandeau (placeholder pour l'instant)
                if let Some(_image_path) = &beatmapset.image_path {
                    // Plus tard on chargera et affichera l'image ici en mode bandeau
                    egui::Frame::default()
                        .fill(Color32::from_rgba_unmultiplied(20, 20, 20, 255))
                        .inner_margin(5.0)
                        .show(ui, |ui| {
                            ui.set_height(80.0);
                            ui.centered_and_justified(|ui| {
                                ui.label(RichText::new("Background Image").small().weak());
                            });
                        });
                    ui.add_space(5.0);
                }

                // Informations de la map
                ui.separator();
                ui.add_space(5.0);

                // Notes, BPM, Mappeur les uns à côté des autres
                ui.horizontal(|ui| {
                    // Nombre de notes
                    if let Some(bm) = beatmap {
                        ui.label(RichText::new("Notes:").strong());
                        ui.label(format!("{}", bm.note_count));
                        ui.add_space(15.0);
                    }

                    // BPM (constante pour l'instant)
                    ui.label(RichText::new("BPM:").strong());
                    ui.label("180"); // Constante pour l'instant
                    ui.add_space(15.0);

                    // Mappeur (constante pour l'instant)
                    ui.label(RichText::new("Mapper:").strong());
                    ui.label("Unknown"); // Constante pour l'instant
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);

                // Onglets pour osu/etterna/quaver
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.selected_rating_tab == 0, "osu!").clicked() {
                        self.selected_rating_tab = 0;
                    }
                    if ui.selectable_label(self.selected_rating_tab == 1, "Etterna").clicked() {
                        self.selected_rating_tab = 1;
                    }
                    if ui.selectable_label(self.selected_rating_tab == 2, "Quaver").clicked() {
                        self.selected_rating_tab = 2;
                    }
                });

                ui.add_space(5.0);

                // Hit Window au-dessus du rate
                let hit_window_text = match hit_window_mode {
                    HitWindowMode::OsuOD => format!("OD {:.1}", hit_window_value),
                    HitWindowMode::EtternaJudge => format!("Judge {}", hit_window_value as u8),
                };
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(&hit_window_text).small());
                    });
                });

                // Rating et Rate sur la même ligne
                ui.horizontal(|ui| {
                    // Afficher le rating selon l'onglet sélectionné
                    match self.selected_rating_tab {
                        0 => {
                            ui.label(RichText::new("SR: 4.25").size(18.0)); // Constante pour l'instant
                        }
                        1 => {
                            ui.label(RichText::new("MSD: 12.34").size(18.0)); // Constante pour l'instant
                        }
                        2 => {
                            ui.label(RichText::new("Difficulty: 8.50").size(18.0)); // Constante pour l'instant
                        }
                        _ => {}
                    }
                    
                    // Rate à droite, au même niveau
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!("{:.1}x", rate)).size(20.0).strong());
                    });
                });
            });
    }
}

