pub struct GameSettings {
    pub is_open: bool,          // Le menu est-il ouvert ?
    pub show_keybindings: bool, // Le menu de remapping est-il ouvert ?
    pub master_volume: f32,     // 0.0 à 1.0
                                // Tu pourras ajouter ici d'autres options (offset, speed, etc.)
}

impl GameSettings {
    pub fn new() -> Self {
        Self {
            is_open: false,
            show_keybindings: false,
            master_volume: 0.5,
        }
    }
}
