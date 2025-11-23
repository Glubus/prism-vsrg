use crate::engine::{PixelSystem, GameEngine, JudgementColors};
use crate::components::Component;
use wgpu_text::glyph_brush::{Section, Text};

pub struct JudgementsComponent {
    pub x_pixels: f32,
    pub y_pixels: f32,
    pub judgement_colors: JudgementColors,
    judgement_strings: Vec<String>,
    remaining_text: String,
    scroll_speed_text: String,
}

impl JudgementsComponent {
    pub fn new(x_pixels: f32, y_pixels: f32, judgement_colors: JudgementColors) -> Self {
        Self {
            x_pixels,
            y_pixels,
            judgement_colors,
            judgement_strings: Vec::new(),
            remaining_text: String::new(),
            scroll_speed_text: String::new(),
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

impl Component for JudgementsComponent {
    fn render(&mut self, engine: &GameEngine, pixel_system: &PixelSystem, screen_width: f32, screen_height: f32) -> Vec<Section> {
        let mut sections = Vec::new();
        let x = self.get_x(pixel_system);
        let mut y_offset = self.get_y(pixel_system);
        let scale_ratio = screen_height / 1080.0;
        let spacing_small = 25.0 * scale_ratio;
        let spacing_large = 30.0 * scale_ratio;

        // "judgement:"
        sections.push(Section {
            screen_position: (x, y_offset),
            bounds: (screen_width, screen_height),
            text: vec![
                Text::new("judgement:")
                    .with_scale(18.0 * scale_ratio)
                    .with_color([1.0, 1.0, 1.0, 1.0]),
            ],
            ..Default::default()
        });
        
        y_offset += spacing_large;
        
        // Les 7 jugements avec leurs compteurs
        let judgements_data = vec![
            ("Marv", self.judgement_colors.marv, engine.hit_stats.marv),
            ("Perfect", self.judgement_colors.perfect, engine.hit_stats.perfect),
            ("Great", self.judgement_colors.great, engine.hit_stats.great),
            ("Good", self.judgement_colors.good, engine.hit_stats.good),
            ("Bad", self.judgement_colors.bad, engine.hit_stats.bad),
            ("Miss", self.judgement_colors.miss, engine.hit_stats.miss),
            ("Ghost Tap", self.judgement_colors.ghost_tap, engine.hit_stats.ghost_tap),
        ];
        
        // Stocker les strings dans le component
        self.judgement_strings.clear();
        for (name, _color, count) in &judgements_data {
            self.judgement_strings.push(format!("{}: {}", name, count));
        }
        
        for (i, (_name, color, _count)) in judgements_data.iter().enumerate() {
            sections.push(Section {
                screen_position: (x, y_offset),
                bounds: (screen_width, screen_height),
                text: vec![
                    Text::new(&self.judgement_strings[i])
                        .with_scale(16.0 * scale_ratio)
                        .with_color(*color),
                ],
                ..Default::default()
            });
            y_offset += spacing_small;
        }

        // Nombre de notes restantes
        let remaining_notes = engine.get_remaining_notes();
        self.remaining_text = format!("Remaining notes: {}", remaining_notes);
        sections.push(Section {
            screen_position: (x, y_offset),
            bounds: (screen_width, screen_height),
            text: vec![
                Text::new(&self.remaining_text)
                    .with_scale(16.0 * scale_ratio)
                    .with_color([1.0, 1.0, 1.0, 1.0]),
            ],
            ..Default::default()
        });
        
        y_offset += spacing_small;
        
        // Vitesse de défilement
        self.scroll_speed_text = format!("Speed: {:.1} ms", engine.scroll_speed_ms);
        sections.push(Section {
            screen_position: (x, y_offset),
            bounds: (screen_width, screen_height),
            text: vec![
                Text::new(&self.scroll_speed_text)
                    .with_scale(16.0 * scale_ratio)
                    .with_color([1.0, 1.0, 1.0, 1.0]),
            ],
            ..Default::default()
        });

        sections
    }
}

