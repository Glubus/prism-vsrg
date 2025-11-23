use crate::engine::{PixelSystem, GameEngine, HitWindow, Judgement};
use crate::components::Component;
use wgpu_text::glyph_brush::{Section, Text};

#[derive(Clone)]
struct HitMarker {
    timing: f64,  // Timing en ms
    judgement: Judgement,
}

pub struct HitBar {
    pub x_pixels: f32,  // Position X en pixels (référence)
    pub y_pixels: f32,  // Position Y en pixels (référence)
    pub width_pixels: f32,  // Largeur en pixels (référence)
    pub height_pixels: f32,  // Hauteur en pixels (référence)
    pub hit_window: HitWindow,
    last_hits: Vec<HitMarker>,  // Les 10 derniers hits
}

impl HitBar {
    pub fn new(x_pixels: f32, y_pixels: f32, width_pixels: f32, height_pixels: f32) -> Self {
        Self {
            x_pixels,
            y_pixels,
            width_pixels,
            height_pixels,
            hit_window: HitWindow::new(),
            last_hits: Vec::with_capacity(10),
        }
    }

    fn get_x(&self, _pixel_system: &PixelSystem) -> f32 {
        // x_pixels est déjà en pixels d'écran, pas besoin de ratio
        self.x_pixels
    }

    fn get_y(&self, _pixel_system: &PixelSystem) -> f32 {
        // y_pixels est déjà en pixels d'écran, pas besoin de ratio
        self.y_pixels
    }

    fn get_width(&self, _pixel_system: &PixelSystem) -> f32 {
        // width_pixels est déjà en pixels d'écran, pas besoin de ratio
        self.width_pixels
    }

    fn get_height(&self, _pixel_system: &PixelSystem) -> f32 {
        // height_pixels est déjà en pixels d'écran, pas besoin de ratio
        self.height_pixels
    }

    /// Retourne la couleur pour un jugement donné
    fn get_judgement_color(judgement: Judgement) -> [f32; 4] {
        match judgement {
            Judgement::Marv => [0.0, 1.0, 1.0, 1.0],      // Cyan
            Judgement::Perfect => [1.0, 1.0, 0.0, 1.0],   // Yellow
            Judgement::Great => [0.0, 1.0, 0.0, 1.0],    // Green
            Judgement::Good => [0.0, 0.0, 1.0, 1.0],     // Dark Blue
            Judgement::Bad => [1.0, 0.0, 1.0, 1.0],       // Pink
            Judgement::Miss => [1.0, 0.0, 0.0, 1.0],      // Red
            Judgement::GhostTap => [0.5, 0.5, 0.5, 1.0], // Grey
        }
    }

    /// Calcule la position X d'un tick en pixels selon le timing (en ms)
    /// timing_ms > 0 : trop tôt (gauche)
    /// timing_ms < 0 : trop tard (droite)
    fn timing_to_x(&self, timing_ms: f64, pixel_system: &PixelSystem) -> f32 {
        let x = self.get_x(pixel_system);
        let width = self.get_width(pixel_system);
        // Centre de la barre
        let center_x = x + (width / 2.0);
        
        // Convertir le timing en position (200ms = largeur complète)
        // Inversé : positif (trop tôt) va à gauche, négatif (trop tard) va à droite
        let max_timing = 200.0f64; // ms
        let ratio = (timing_ms / max_timing).clamp(-1.0, 1.0) as f32;
        
        // Inverser : timing positif (trop tôt) -> gauche (négatif), timing négatif (trop tard) -> droite (positif)
        center_x - (ratio * (width / 2.0))
    }
}

impl Component for HitBar {
    fn render(&mut self, engine: &GameEngine, pixel_system: &PixelSystem, screen_width: f32, screen_height: f32) -> Vec<Section> {
        let mut sections = Vec::new();
        let x = self.get_x(pixel_system);
        let y = self.get_y(pixel_system);
        let width = self.get_width(pixel_system);
        let height = self.get_height(pixel_system);

        // Dessiner la barre centrale (ligne verticale au centre)
        let center_x = x + (width / 2.0);
        sections.push(Section {
            screen_position: (center_x, y),
            bounds: (screen_width, screen_height),
            text: vec![
                Text::new("|")
                    .with_scale(height)
                    .with_color([1.0, 1.0, 1.0, 1.0]), // Blanc pour le centre
            ],
            ..Default::default()
        });

        // Mettre à jour la liste des derniers hits
        if let (Some(timing), Some(judgement)) = (engine.last_hit_timing, engine.last_hit_judgement) {
            // Vérifier si c'est un nouveau hit (pas déjà dans la liste)
            let is_new_hit = self.last_hits.is_empty() || 
                self.last_hits.last().map(|h| h.timing != timing || h.judgement != judgement).unwrap_or(true);
            
            if is_new_hit {
                self.last_hits.push(HitMarker {
                    timing,
                    judgement,
                });
                
                // Garder seulement les 10 derniers
                if self.last_hits.len() > 10 {
                    self.last_hits.remove(0);
                }
            }
        }

        // Dessiner les marqueurs pour les 10 derniers hits
        for hit_marker in &self.last_hits {
            let hit_x = self.timing_to_x(hit_marker.timing, pixel_system);
            let color = Self::get_judgement_color(hit_marker.judgement);
            
            // Dessiner un trait pour chaque hit
            sections.push(Section {
                screen_position: (hit_x, y),
                bounds: (screen_width, screen_height),
                text: vec![
                    Text::new("|")
                        .with_scale(height * 0.9)
                        .with_color(color),
                ],
                ..Default::default()
            });
        }

        sections
    }
}

