use crate::engine::{PixelSystem, GameEngine};
use crate::components::Component;
use wgpu_text::glyph_brush::{Section, Text};

pub struct ScoreComponent {
    pub x_pixels: f32,  // Position X en pixels (référence)
    pub y_pixels: f32,  // Position Y en pixels (référence)
    pub score: u32,
    score_text: String,
}

impl ScoreComponent {
    pub fn new(x_pixels: f32, y_pixels: f32) -> Self {
        Self {
            x_pixels,
            y_pixels,
            score: 871290, // Placeholder
            score_text: String::new(),
        }
    }

    /// Calcule la position X (déjà en pixels d'écran)
    fn get_x(&self, _pixel_system: &PixelSystem) -> f32 {
        // x_pixels est déjà en pixels d'écran, pas besoin de ratio
        self.x_pixels
    }

    /// Calcule la position Y (déjà en pixels d'écran)
    fn get_y(&self, _pixel_system: &PixelSystem) -> f32 {
        // y_pixels est déjà en pixels d'écran, pas besoin de ratio
        self.y_pixels
    }
}

impl Component for ScoreComponent {
    fn render(&mut self, _engine: &GameEngine, pixel_system: &PixelSystem, screen_width: f32, screen_height: f32) -> Vec<Section> {
        self.score_text = format!("{}", self.score);
        let x = self.get_x(pixel_system);
        let y = self.get_y(pixel_system);
        let scale_ratio = screen_height / 1080.0;
        let spacing = 25.0 * scale_ratio;
        
        vec![
            Section {
                screen_position: (x, y),
                bounds: (screen_width, screen_height),
                text: vec![
                    Text::new("Score")
                        .with_scale(20.0 * scale_ratio)
                        .with_color([1.0, 1.0, 1.0, 1.0]),
                ],
                ..Default::default()
            },
            Section {
                screen_position: (x, y + spacing),
                bounds: (screen_width, screen_height),
                text: vec![
                    Text::new(&self.score_text)
                        .with_scale(24.0 * scale_ratio)
                        .with_color([1.0, 1.0, 1.0, 1.0]),
                ],
                ..Default::default()
            },
        ]
    }
}
