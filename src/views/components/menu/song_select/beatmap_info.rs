use egui::{Color32, RichText, Ui};
use std::borrow::Cow;

use crate::database::models::{BeatmapRating, BeatmapWithRatings, Beatmapset};
use crate::models::settings::HitWindowMode;

pub struct BeatmapInfo {
    selected_rating_tab: u8, // 0 = Etterna, 1 = Osu
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
        beatmap: Option<&BeatmapWithRatings>,
        rate: f64,
        hit_window_mode: HitWindowMode,
        hit_window_value: f64,
        override_ratings: Option<&[BeatmapRating]>,
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
                    if let Some(diff_name) = &bm.beatmap.difficulty_name {
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
                        ui.label(format!("{}", bm.beatmap.note_count));
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

                let ratings_slice =
                    override_ratings.or_else(|| beatmap.map(|bm| bm.ratings.as_slice()));
                let etterna_rating = find_rating(ratings_slice, "etterna");
                let osu_rating = find_rating(ratings_slice, "osu");

                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(self.selected_rating_tab == 0, "Etterna")
                        .clicked()
                    {
                        self.selected_rating_tab = 0;
                    }
                    if ui
                        .selectable_label(self.selected_rating_tab == 1, "Osu")
                        .clicked()
                    {
                        self.selected_rating_tab = 1;
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
                    let (label, rating) = match self.selected_rating_tab {
                        0 => ("Etterna", etterna_rating),
                        1 => ("Osu", osu_rating),
                        _ => ("Etterna", etterna_rating),
                    };

                    if let Some(rating) = rating {
                        ui.label(
                            RichText::new(format!("{} Overall: {:.2}", label, rating.overall))
                                .size(18.0),
                        );
                    } else {
                        ui.label(
                            RichText::new(format!("{}: N/A", label))
                                .size(18.0)
                                .italics()
                                .weak(),
                        );
                    }

                    // Rate à droite, au même niveau
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!("{:.1}x", rate)).size(20.0).strong());
                    });
                });

                if let Some(rating) = match self.selected_rating_tab {
                    0 => etterna_rating,
                    1 => osu_rating,
                    _ => etterna_rating,
                } {
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(6.0);
                    render_ssr_details(ui, rating);
                }
            });
    }
}

fn find_rating<'a>(
    ratings: Option<&'a [BeatmapRating]>,
    target: &str,
) -> Option<&'a BeatmapRating> {
    ratings.and_then(|list| {
        list.iter()
            .find(|rating| rating.name.eq_ignore_ascii_case(target))
    })
}

fn render_ssr_details(ui: &mut Ui, rating: &BeatmapRating) {
    let pairs = [
        (("Stream", "stream"), ("Jumpstream", "jumpstream")),
        (("Jumpstream", "jumpstream"), ("Stamina", "stamina")),
        (("Handstream", "handstream"), ("JackSpeed", "jackspeed")),
        (("Chordjack", "chordjack"), ("Technical", "technical")),
    ];

    for (left, right) in pairs {
        ui.horizontal(|ui| {
            render_metric_entry(ui, left.0, get_metric_value(rating, left.1));
            ui.add_space(18.0);
            render_metric_entry(ui, right.0, get_metric_value(rating, right.1));
        });
    }
}

fn render_metric_entry(ui: &mut Ui, label: &str, value: f64) {
    let alias = metric_alias(label);
    ui.label(
        RichText::new(format!("{}: {:.2}", alias, value))
            .strong()
            .monospace(),
    );
}

fn get_metric_value(rating: &BeatmapRating, key: &str) -> f64 {
    match key {
        "overall" => rating.overall,
        "stream" => rating.stream,
        "jumpstream" => rating.jumpstream,
        "handstream" => rating.handstream,
        "stamina" => rating.stamina,
        "jackspeed" => rating.jackspeed,
        "chordjack" => rating.chordjack,
        "technical" => rating.technical,
        _ => 0.0,
    }
}

fn metric_alias(label: &str) -> Cow<'_, str> {
    match label.to_ascii_lowercase().as_str() {
        "jumpstream" => Cow::Borrowed("JS"),
        "stamina" => Cow::Borrowed("Stam"),
        "handstream" => Cow::Borrowed("HS"),
        "jackspeed" => Cow::Borrowed("SJ"),
        "chordjack" => Cow::Borrowed("CJ"),
        "technical" => Cow::Borrowed("Tech"),
        _ => Cow::Borrowed(label),
    }
}
