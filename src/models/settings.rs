#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HitWindowMode {
    OsuOD,
    EtternaJudge,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AspectRatioMode {
    Auto,      // Utilise la taille réelle de la fenêtre (Correct par défaut)
    Ratio16_9, // Force le ratio 16:9
    Ratio4_3,  // Force le ratio 4:3
}

pub struct GameSettings {
    pub is_open: bool,
    pub show_keybindings: bool,
    pub remapping_column: Option<usize>,
    pub master_volume: f32,
    pub hit_window_mode: HitWindowMode,
    pub hit_window_value: f64,
    pub aspect_ratio_mode: AspectRatioMode, // Nouveau champ
}

impl GameSettings {
    pub fn new() -> Self {
        Self {
            is_open: false,
            show_keybindings: false,
            remapping_column: None,
            master_volume: 0.5,
            hit_window_mode: HitWindowMode::OsuOD,
            hit_window_value: 5.0,
            aspect_ratio_mode: AspectRatioMode::Auto, // Auto par défaut pour corriger l'étirement
        }
    }
}
