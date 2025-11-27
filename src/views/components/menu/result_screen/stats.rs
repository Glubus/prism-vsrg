use crate::models::menu::GameResultData;
use egui::{Align2, Color32, FontId, Pos2, Rect, RichText, Rounding, Ui, Vec2};

pub fn render_stats(ui: &mut Ui, data: &GameResultData) {
    ui.vertical(|ui| {
        // --- SCORE & ACCURACY ---
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);

            // Score en très gros
            ui.label(
                RichText::new(format!("{:07}", data.score))
                    .size(52.0)
                    .strong()
                    .color(Color32::WHITE),
            );

            ui.add_space(5.0);

            // Accuracy et Combo sur la même ligne visuelle
            ui.horizontal_centered(|ui| {
                ui.label(
                    RichText::new(format!("{:.2}%", data.accuracy))
                        .size(36.0)
                        .strong()
                        .color(if data.accuracy >= 98.0 {
                            Color32::GOLD
                        } else {
                            Color32::WHITE
                        }),
                );

                ui.add_space(15.0);
                ui.label(RichText::new("|").size(24.0).color(Color32::GRAY));
                ui.add_space(15.0);

                ui.label(
                    RichText::new(format!("{}x", data.max_combo))
                        .size(36.0)
                        .color(Color32::LIGHT_BLUE),
                );
            });

            // --- RATE & JUDGE INFO ---
            ui.add_space(10.0);
            egui::Frame::default()
                .fill(Color32::from_white_alpha(10))
                .corner_radius(4.0)
                .inner_margin(6.0)
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(format!("{}  •  {:.1}x Rate", data.judge_text, data.rate))
                            .size(16.0)
                            .strong()
                            .color(Color32::from_gray(220)),
                    );
                });
        });

        ui.add_space(30.0);

        // --- JUDGEMENTS BARS (FULL WIDTH) ---
        let total = (data.hit_stats.marv
            + data.hit_stats.perfect
            + data.hit_stats.great
            + data.hit_stats.good
            + data.hit_stats.bad
            + data.hit_stats.miss) as f32;
        let total = if total == 0.0 { 1.0 } else { total };

        let judgements = [
            (
                "Marvelous",
                data.hit_stats.marv,
                Color32::from_rgb(0, 255, 255),
            ),
            (
                "Perfect",
                data.hit_stats.perfect,
                Color32::from_rgb(255, 255, 0),
            ),
            ("Great", data.hit_stats.great, Color32::from_rgb(0, 255, 0)),
            ("Good", data.hit_stats.good, Color32::from_rgb(0, 0, 128)),
            ("Bad", data.hit_stats.bad, Color32::from_rgb(255, 105, 180)),
            ("Miss", data.hit_stats.miss, Color32::from_rgb(255, 0, 0)),
        ];

        let bar_height = 32.0; // Barres plus grosses comme demandé
        let bar_spacing = 8.0;

        for (label, count, color) in judgements.iter() {
            // Allouer toute la largeur disponible
            let (rect, _response) = ui.allocate_at_least(
                Vec2::new(ui.available_width(), bar_height),
                egui::Sense::hover(),
            );

            let painter = ui.painter();
            let rounding = egui::CornerRadius::same(4_u8);

            // 1. Fond (Partie "vide" mais colorée sombre)
            painter.rect_filled(rect, rounding, color.linear_multiply(0.15));

            // 2. Partie remplie (Proportionnelle)
            let ratio = (*count as f32 / total).clamp(0.0, 1.0);
            if ratio > 0.005 {
                // Afficher seulement si > 0.5% pour éviter les artefacts
                let filled_width = rect.width() * ratio;
                let filled_rect =
                    Rect::from_min_max(rect.min, Pos2::new(rect.min.x + filled_width, rect.max.y));
                painter.rect_filled(filled_rect, rounding, *color);
            } else if *count > 0 {
                // Petit trait minimum si count > 0 mais ratio très faible
                let filled_rect =
                    Rect::from_min_max(rect.min, Pos2::new(rect.min.x + 4.0, rect.max.y));
                painter.rect_filled(filled_rect, rounding, *color);
            }

            // 3. Texte (Label à gauche, Compteur à droite) - DEDANS la barre
            // On utilise une couleur contrastée (Noir ou Blanc selon luminosité, ici Noir semi-transparent pour effet "cutout" ou Blanc avec ombre)
            // Pour faire propre sur des couleurs vives : Blanc avec un léger contour ou Noir.
            // Le plus simple et lisible souvent : Blanc avec Drop Shadow simulé.

            let text_color = Color32::WHITE;
            let text_shadow = Color32::from_black_alpha(150);
            let font_id = FontId::proportional(16.0);

            // Label (ex: Marvelous)
            let label_pos = Pos2::new(rect.min.x + 10.0, rect.center().y);

            // Ombre
            painter.text(
                label_pos + Vec2::new(1.0, 1.0),
                Align2::LEFT_CENTER,
                *label,
                font_id.clone(),
                text_shadow,
            );
            // Texte
            painter.text(
                label_pos,
                Align2::LEFT_CENTER,
                *label,
                font_id.clone(),
                text_color,
            );

            // Compteur (ex: 1450)
            let count_pos = Pos2::new(rect.max.x - 10.0, rect.center().y);

            // Ombre
            painter.text(
                count_pos + Vec2::new(1.0, 1.0),
                Align2::RIGHT_CENTER,
                count.to_string(),
                font_id.clone(),
                text_shadow,
            );
            // Texte
            painter.text(
                count_pos,
                Align2::RIGHT_CENTER,
                count.to_string(),
                font_id.clone(),
                text_color,
            );

            ui.add_space(bar_spacing);
        }

        // Ghost Taps en bas, discret
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("Ghost Taps:").color(Color32::GRAY));
            ui.label(
                RichText::new(data.hit_stats.ghost_tap.to_string())
                    .strong()
                    .color(Color32::WHITE),
            );
        });
    });
}
