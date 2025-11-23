use crate::database::models::{Beatmapset, Beatmap};

/// Une card individuelle pour afficher une map
pub struct Card {
    pub beatmapset: Beatmapset,
    pub beatmaps: Vec<Beatmap>,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub is_selected: bool,
}

impl Card {
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}

impl Card {
    pub fn new(beatmapset: Beatmapset, beatmaps: Vec<Beatmap>, x: f32, y: f32, width: f32, height: f32, is_selected: bool) -> Self {
        Self {
            beatmapset,
            beatmaps,
            width,
            height,
            x,
            y,
            is_selected,
        }
    }
    
    /// Retourne seulement le titre de la map (sans l'artiste)
    pub fn title_text(&self) -> String {
        self.beatmapset.title.as_deref().unwrap_or("Unknown Title").to_string()
    }
    
    /// Retourne le texte avec artist | difficulty (première difficulté)
    pub fn artist_difficulty_text(&self) -> String {
        let artist = self.beatmapset.artist.as_deref().unwrap_or("Unknown Artist");
        let difficulty = if let Some(first_beatmap) = self.beatmaps.first() {
            first_beatmap.difficulty_name.as_deref().unwrap_or("Unknown")
        } else {
            "Unknown"
        };
        format!("{} | {}", artist, difficulty)
    }
    
    /// Retourne la couleur du texte (jaune si sélectionné, blanc sinon)
    pub fn text_color(&self) -> [f32; 4] {
        if self.is_selected {
            [1.0, 1.0, 0.0, 1.0] // Jaune pour la sélection
        } else {
            [0.9, 0.9, 0.9, 1.0] // Blanc cassé pour les autres
        }
    }
    
    /// Retourne la couleur du fond (noir semi-transparent)
    pub fn background_color(&self) -> [f32; 4] {
        if self.is_selected {
            [0.0, 0.0, 0.0, 0.9] // Plus opaque si sélectionné
        } else {
            [0.0, 0.0, 0.0, 0.8] // Noir semi-transparent
        }
    }
}

