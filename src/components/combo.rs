use crate::engine::{PixelSystem, GameEngine};
use crate::components::Component;
use wgpu_text::glyph_brush::{Section, Text};

pub struct ComboComponent {
    pub x_pixels: f32,
    pub y_pixels: f32,
    combo_text: String,
}

impl ComboComponent {
    pub fn new(x_pixels: f32, y_pixels: f32) -> Self {
        Self { 
            x_pixels, 
            y_pixels,
            combo_text: String::new(),
        }
    }

    fn get_x(&self, _pixel_system: &PixelSystem) -> f32 {
        // Ne plus utiliser cette méthode, le centrage est fait dans render()
        self.x_pixels
    }

    fn get_y(&self, _pixel_system: &PixelSystem) -> f32 {
        // y_pixels est déjà en pixels d'écran, pas besoin de ratio
        self.y_pixels
    }
}

impl Component for ComboComponent {
    fn render(&mut self, engine: &GameEngine, pixel_system: &PixelSystem, screen_width: f32, screen_height: f32) -> Vec<Section> {
        self.combo_text = format!("{}", engine.combo);
        let scale_ratio = pixel_system.window_height as f32 / 1080.0;
        
        // Pour centrer le texte, on ajuste la position X en soustrayant la moitié de la largeur estimée du texte
        // Estimation : chaque caractère fait environ 30 pixels à cette échelle
        let text_width_estimate = self.combo_text.len() as f32 * 30.0 * scale_ratio;
        let centered_x = self.x_pixels - (text_width_estimate / 2.0);
        
        vec![Section {
            screen_position: (centered_x, self.get_y(pixel_system)),
            bounds: (screen_width, screen_height),
            text: vec![
                Text::new(&self.combo_text)
                    .with_scale(48.0 * scale_ratio)
                    .with_color([1.0, 1.0, 1.0, 1.0]),
            ],
            ..Default::default()
        }]
    }
}

