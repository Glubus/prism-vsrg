use crate::engine::{PixelSystem, GameEngine};
use crate::components::Component;
use wgpu_text::glyph_brush::{Section, Text};

pub struct AccuracyComponent {
    pub x_pixels: f32,
    pub y_pixels: f32,
    accuracy_text: String,
}

impl AccuracyComponent {
    pub fn new(x_pixels: f32, y_pixels: f32) -> Self {
        Self { 
            x_pixels, 
            y_pixels,
            accuracy_text: String::new(),
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
}

impl Component for AccuracyComponent {
    fn render(&mut self, engine: &GameEngine, pixel_system: &PixelSystem, screen_width: f32, screen_height: f32) -> Vec<Section> {
        let accuracy = engine.hit_stats.calculate_accuracy();
        self.accuracy_text = format!("accuracy: {:.2}%", accuracy);
        
        vec![Section {
            screen_position: (self.get_x(pixel_system), self.get_y(pixel_system)),
            bounds: (screen_width, screen_height),
            text: vec![
                Text::new(&self.accuracy_text)
                    .with_scale(20.0 * (pixel_system.window_height as f32 / 1080.0))
                    .with_color([1.0, 1.0, 1.0, 1.0]),
            ],
            ..Default::default()
        }]
    }
}

