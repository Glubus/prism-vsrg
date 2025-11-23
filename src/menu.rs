use crate::database::{Database, Beatmapset, Beatmap};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MenuState {
    pub beatmapsets: Vec<(Beatmapset, Vec<Beatmap>)>,
    pub start_index: usize, // Index de début du scroll (premier item visible)
    pub end_index: usize,   // Index de fin du scroll (dernier item visible)
    pub selected_index: usize, // Index absolu dans toute la liste
    pub visible_count: usize, // Nombre d'items visibles à l'écran
    pub in_menu: bool,
    pub rate: f64, // Rate multiplier (1.0 = normal speed, 1.5 = 1.5x speed, etc.)
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            beatmapsets: Vec::new(),
            start_index: 0,
            end_index: 0,
            selected_index: 0,
            visible_count: 10, // Afficher 10 items visibles à l'écran
            in_menu: true,
            rate: 1.0, // Default rate: normal speed
        }
    }

    pub fn increase_rate(&mut self) {
        self.rate = (self.rate + 0.1).min(2.0); // Max 2.0x speed
    }

    pub fn decrease_rate(&mut self) {
        self.rate = (self.rate - 0.1).max(0.5); // Min 0.5x speed
    }

    pub async fn load_from_db(menu_state: Arc<Mutex<Self>>, db: &Database) -> Result<(), sqlx::Error> {
        let beatmapsets = db.get_all_beatmapsets().await?;
        if let Ok(mut state) = menu_state.lock() {
            state.beatmapsets = beatmapsets.clone();
            state.selected_index = 0;
            // Initialiser les index de scroll
            state.end_index = state.visible_count.min(state.beatmapsets.len());
            state.start_index = 0;
        }
        Ok(())
    }

    /// Retourne les items visibles dans la fenêtre de scroll
    pub fn get_visible_items(&self) -> &[(Beatmapset, Vec<Beatmap>)] {
        if self.start_index >= self.beatmapsets.len() {
            return &[];
        }
        let end = self.end_index.min(self.beatmapsets.len());
        &self.beatmapsets[self.start_index..end]
    }

    /// Retourne l'index relatif de l'item sélectionné dans la fenêtre visible
    pub fn get_relative_selected_index(&self) -> usize {
        if self.selected_index < self.start_index {
            0
        } else if self.selected_index >= self.end_index {
            self.end_index.saturating_sub(self.start_index).saturating_sub(1)
        } else {
            self.selected_index - self.start_index
        }
    }

    pub fn move_up(&mut self) {
        if self.beatmapsets.is_empty() {
            return;
        }
        
        if self.selected_index > 0 {
            self.selected_index -= 1;
            
            // Si l'item sélectionné est en dehors de la fenêtre visible, ajuster le scroll
            if self.selected_index < self.start_index {
                self.start_index = self.selected_index;
                self.end_index = (self.start_index + self.visible_count).min(self.beatmapsets.len());
            }
        }
    }

    pub fn move_down(&mut self) {
        if self.beatmapsets.is_empty() {
            return;
        }
        
        if self.selected_index < self.beatmapsets.len() - 1 {
            self.selected_index += 1;
            
            // Si l'item sélectionné est en dehors de la fenêtre visible, ajuster le scroll
            if self.selected_index >= self.end_index {
                self.end_index = (self.selected_index + 1).min(self.beatmapsets.len());
                self.start_index = self.end_index.saturating_sub(self.visible_count);
            }
        }
    }

    pub fn get_selected_beatmapset(&self) -> Option<&(Beatmapset, Vec<Beatmap>)> {
        self.beatmapsets.get(self.selected_index)
    }

    pub fn get_selected_beatmap_path(&self) -> Option<PathBuf> {
        self.get_selected_beatmapset()
            .and_then(|(_, beatmaps)| beatmaps.first())
            .map(|bm| PathBuf::from(&bm.path))
    }
}

