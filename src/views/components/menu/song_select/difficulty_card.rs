use egui::{Color32, Label, Margin, RichText, Stroke, TextureId, Sense, Rect, Vec2, Pos2, UiBuilder, StrokeKind};

use crate::database::models::Beatmap;

pub struct DifficultyCard;

impl DifficultyCard {
    pub fn render(
        ui: &mut egui::Ui,
        beatmap: &Beatmap,
        is_selected: bool,
        texture_normal: Option<TextureId>,
        texture_selected: Option<TextureId>,
        selected_color: Color32,
    ) -> egui::Response {
        // Hauteur fine
        let card_height = 30.0; 
        
        // On réduit la largeur visuelle en ajoutant des espaces virtuels
        // Au lieu de prendre tout ui.available_width(), on réduit
        let full_width = ui.available_width();
        let margin_side = 40.0; // Rétrécissement latéral ("moins long")
        let visual_width = (full_width - margin_side * 2.0).max(100.0);
        
        // Espace total alloué (doit inclure les marges pour que la souris ne clique pas dans le vide)
        // Mais pour le dessin, on centrera.
        
        // 1. Allocation de la zone CLIQUABLE (Toute la largeur pour ergonomie, ou juste la zone visuelle ?)
        // Généralement, on préfère cliquer n'importe où sur la ligne.
        // Si vous voulez que seule la "barre" soit cliquable, on utilise allocate_rect sur la zone réduite.
        // Essayons de centrer la zone cliquable.
        
        // On se déplace manuellement pour centrer
        let start_pos = ui.cursor().min;
        let centered_start = Pos2::new(start_pos.x + margin_side, start_pos.y);
        let centered_rect = Rect::from_min_size(centered_start, Vec2::new(visual_width, card_height));
        
        // On alloue cet espace précis
        let response = ui.allocate_rect(centered_rect, Sense::click());

        if ui.is_rect_visible(centered_rect) {
            let painter = ui.painter();
            
            let texture_id = if is_selected { 
                texture_selected.or(texture_normal) 
            } else { 
                texture_normal 
            };

            if let Some(tex_id) = texture_id {
                let base_tint = if is_selected { Color32::WHITE } else { Color32::from_gray(200) };
                painter.image(
                    tex_id,
                    centered_rect,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    base_tint
                );

                if is_selected && texture_selected.is_none() && texture_normal.is_some() {
                    let overlay_color = Color32::from_rgba_unmultiplied(
                        selected_color.r(), selected_color.g(), selected_color.b(), 100 
                    );
                    painter.rect_filled(centered_rect, 0.0, overlay_color);
                }
            } else {
                let fill_color = Color32::from_rgba_unmultiplied(30, 30, 30, 250);
                painter.rect_filled(centered_rect, 0.0, fill_color);
                
                let stroke_color = if is_selected { selected_color } else { Color32::from_rgba_unmultiplied(60, 60, 60, 255) };
                painter.rect_stroke(centered_rect, 0.0, Stroke::new(1.0, stroke_color), StrokeKind::Inside);
            }
        }

        // Contenu texte centré
        let mut content_ui = ui.new_child(
            UiBuilder::new()
                .max_rect(centered_rect)
                .layout(*ui.layout())
        );

        content_ui.vertical(|ui| {
            ui.centered_and_justified(|ui| {
                if let Some(diff_name) = &beatmap.difficulty_name {
                    ui.add(Label::new(RichText::new(diff_name).size(16.0).color(Color32::WHITE)).selectable(false));
                } else {
                    ui.add(Label::new(RichText::new("Unknown").size(16.0).weak()).selectable(false));
                }
            });
        });
        
        // Important : Avancer le curseur de l'UI parent pour la prochaine ligne
        // (allocate_rect l'a fait, mais vérifions si on doit ajouter de l'espace vertical si nécessaire)
        // Comme on a alloué 'centered_rect', le curseur de l'ui parent a avancé. 
        // Mais comme on a décalé X, il faut s'assurer que le curseur est bien revenu à la ligne suivante.
        // Egui gère ça normalement avec allocate_rect.
        
        // Petit espace vertical manuel si besoin pour séparer les diffs
        ui.add_space(4.0); 

        response
    }
}