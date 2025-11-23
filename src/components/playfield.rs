use crate::engine::{PixelSystem, GameEngine, PlayfieldConfig, NUM_COLUMNS, InstanceRaw, HIT_LINE_Y, VISIBLE_DISTANCE, NoteData};
use crate::components::Component;
use wgpu_text::glyph_brush::Section;

pub struct PlayfieldComponent {
    pub config: PlayfieldConfig,
}

impl PlayfieldComponent {
    pub fn new(config: PlayfieldConfig) -> Self {
        Self { config }
    }

    /// Calcule la position et la taille du playfield en coordonnées normalisées
    pub fn get_bounds(&self, pixel_system: &PixelSystem) -> (f32, f32) {
        let width = pixel_system.pixels_to_normalized(self.config.column_width_pixels * NUM_COLUMNS as f32);
        let x = -width / 2.0; // Centré horizontalement
        (x, width)
    }

    /// Fonction renderer principale qui convertit les notes visibles en instances pour le rendu
    /// Retourne les instances groupées par colonne pour faciliter le rendu avec différentes textures
    pub fn render_notes(
        &self,
        visible_notes: &[NoteData],
        song_time: f64,
        scroll_speed_ms: f64,
        pixel_system: &PixelSystem,
    ) -> Vec<(usize, InstanceRaw)> {
        let (playfield_x, _playfield_width) = self.get_bounds(pixel_system);
        
        let column_width_norm = pixel_system.pixels_to_normalized(self.config.column_width_pixels);
        // Les notes sont des carrés (même largeur et hauteur)
        let note_size_norm = pixel_system.pixels_to_normalized(self.config.note_width_pixels);

        let mut instances = Vec::with_capacity(visible_notes.len());

        for note in visible_notes {
            // Ne pas afficher les notes déjà touchées
            if note.hit {
                continue;
            }

            let time_to_hit = note.timestamp_ms - song_time;
            let progress = time_to_hit / scroll_speed_ms;
            
            // Calcul Y : Ligne d'impact + (Distance * Progression)
            let y_pos = HIT_LINE_Y + (VISIBLE_DISTANCE * progress as f32);
            
            // Position X : playfield_x + (colonne * largeur_colonne) + (largeur_colonne / 2)
            let center_x = playfield_x + (note.column as f32 * column_width_norm) + (column_width_norm / 2.0);

            instances.push((note.column, InstanceRaw {
                offset: [center_x, y_pos],
                scale: [note_size_norm, note_size_norm],  // Carré : même largeur et hauteur
            }));
        }

        instances
    }

    /// Rendre les receptors (notes fixes) en bleu à la ligne de hit
    pub fn render_receptors(&self, pixel_system: &PixelSystem) -> Vec<InstanceRaw> {
        let (playfield_x, _playfield_width) = self.get_bounds(pixel_system);
        
        let column_width_norm = pixel_system.pixels_to_normalized(self.config.column_width_pixels);
        // Les receptors sont des carrés (même largeur et hauteur), comme les notes
        let receptor_size_norm = pixel_system.pixels_to_normalized(self.config.note_width_pixels);

        let mut instances = Vec::with_capacity(NUM_COLUMNS);

        for col in 0..NUM_COLUMNS {
            let center_x = playfield_x + (col as f32 * column_width_norm) + (column_width_norm / 2.0);
            
            instances.push(InstanceRaw {
                offset: [center_x, HIT_LINE_Y],
                scale: [receptor_size_norm, receptor_size_norm],  // Carré : même largeur et hauteur
            });
        }

        instances
    }
}

impl Component for PlayfieldComponent {
    fn render(&mut self, _engine: &GameEngine, _pixel_system: &PixelSystem, _screen_width: f32, _screen_height: f32) -> Vec<Section> {
        // Le playfield ne rend pas de texte, seulement des instances
        // Le rendu des instances est géré séparément
        Vec::new()
    }
}

