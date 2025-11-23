use crate::engine::{PixelSystem, GameEngine, Judgement};
use crate::components::Component;
use wgpu_text::glyph_brush::{Section, Text};

pub struct JudgementComponent {
    pub x_pixels: f32,
    pub y_pixels: f32,
    judgement_text: String,
}

impl JudgementComponent {
    pub fn new(x_pixels: f32, y_pixels: f32) -> Self {
        Self { 
            x_pixels, 
            y_pixels,
            judgement_text: String::new(),
        }
    }

    fn get_x(&self, _pixel_system: &PixelSystem) -> f32 {
        self.x_pixels
    }

    fn get_y(&self, _pixel_system: &PixelSystem) -> f32 {
        self.y_pixels
    }

    fn get_judgement_text(judgement: &Judgement) -> &'static str {
        match judgement {
            Judgement::Marv => "Marvelous",
            Judgement::Perfect => "Perfect",
            Judgement::Great => "Great",
            Judgement::Good => "Good",
            Judgement::Bad => "Bad",
            Judgement::Miss => "Miss",
            Judgement::GhostTap => "Ghost Tap",
        }
    }

    fn get_judgement_color(judgement: &Judgement) -> [f32; 4] {
        match judgement {
            Judgement::Marv => [0.0, 1.0, 1.0, 1.0],      // Cyan
            Judgement::Perfect => [1.0, 1.0, 0.0, 1.0],   // Yellow
            Judgement::Great => [0.0, 1.0, 0.0, 1.0],     // Green
            Judgement::Good => [0.0, 0.0, 0.5, 1.0],      // Dark Blue
            Judgement::Bad => [1.0, 0.41, 0.71, 1.0],     // Pink
            Judgement::Miss => [1.0, 0.0, 0.0, 1.0],      // Red
            Judgement::GhostTap => [0.5, 0.5, 0.5, 1.0],  // Grey
        }
    }
}

impl Component for JudgementComponent {
    fn render(&mut self, engine: &GameEngine, pixel_system: &PixelSystem, screen_width: f32, screen_height: f32) -> Vec<Section> {
        let scale_ratio = pixel_system.window_height as f32 / 1080.0;
        
        // Afficher le dernier jugement s'il existe
        if let Some(judgement) = engine.last_hit_judgement {
            self.judgement_text = Self::get_judgement_text(&judgement).to_string();
            let color = Self::get_judgement_color(&judgement);
            let font_scale = 36.0 * scale_ratio;
            
            // Pour centrer le texte, on ajuste la position X en soustrayant la moitié de la largeur estimée du texte
            // Estimation : chaque caractère fait environ 0.6 * font_scale pixels (basé sur la taille de police)
            let text_width_estimate = self.judgement_text.len() as f32 * 0.6 * font_scale;
            let centered_x = self.x_pixels - (text_width_estimate / 2.0);
            
            vec![Section {
                screen_position: (centered_x, self.get_y(pixel_system)),
                bounds: (screen_width, screen_height),
                text: vec![
                    Text::new(&self.judgement_text)
                        .with_scale(font_scale)
                        .with_color(color),
                ],
                ..Default::default()
            }]
        } else {
            // Pas de jugement récent, ne rien afficher
            vec![]
        }
    }
}

