use crate::components::card::Card;
use crate::database::models::{Beatmapset, Beatmap};
use wgpu_text::glyph_brush::{Section, Text};
use bytemuck;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadInstance {
    pub center: [f32; 2],  // Centre du quad en coordonnées normalisées [-1, 1]
    pub size: [f32; 2],     // Taille du quad en coordonnées normalisées
    pub color: [f32; 4],    // Couleur RGBA
}

/// Convertit les coordonnées écran en coordonnées normalisées
fn screen_to_normalized(x: f32, y: f32, width: f32, height: f32) -> [f32; 2] {
    [
        (x / width) * 2.0 - 1.0,
        -((y / height) * 2.0 - 1.0), // Inverser Y
    ]
}

/// Crée un quad pour un panel/card réutilisable
fn create_quad(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: [f32; 4],
    screen_width: f32,
    screen_height: f32,
) -> QuadInstance {
    let center = screen_to_normalized(
        x + width / 2.0,
        y + height / 2.0,
        screen_width,
        screen_height
    );
    // Pour la taille, on convertit les pixels en coordonnées normalisées
    // mais sans le décalage de -1 (car c'est une taille, pas une position)
    let size = [
        (width / screen_width) * 2.0,
        (height / screen_height) * 2.0,
    ];
    QuadInstance {
        center,
        size,
        color,
    }
}

/// Composant pour lister les maps avec des cards
pub struct MapListComponent {
    pub cards: Vec<Card>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub card_width: f32,
    pub card_height: f32,
    pub card_spacing: f32,
    pub screen_width: f32,
    pub screen_height: f32,
    // Stocker les strings pour qu'elles vivent assez longtemps
    text_strings: Vec<String>,
}

impl MapListComponent {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let card_width = 400.0; // Agrandi en largeur
        let card_height = 80.0; // Agrandi en hauteur pour afficher artist | difficulty
        let card_spacing = 8.0;
        let width = card_width + 40.0; // Padding
        let x = screen_width - width; // Collé à droite
        let y = 100.0;
        
        Self {
            cards: Vec::new(),
            x,
            y,
            width,
            card_width,
            card_height,
            card_spacing,
            screen_width,
            screen_height,
            text_strings: Vec::new(),
        }
    }
    
    /// Met à jour les cards avec les beatmapsets visibles
    pub fn update_cards(&mut self, visible_items: &[(Beatmapset, Vec<Beatmap>)], selected_index: usize) {
        self.cards.clear();
        
        for (idx, (beatmapset, beatmaps)) in visible_items.iter().enumerate() {
            // Calculer la position Y en tenant compte de la hauteur de la card et de l'espacement
            let card_y = self.y + (idx as f32 * (self.card_height + self.card_spacing));
            let is_selected = idx == selected_index;
            
            let card = Card::new(
                beatmapset.clone(),
                beatmaps.clone(),
                self.x + 20.0, // Padding interne
                card_y,
                self.card_width,
                self.card_height,
                is_selected,
            );
            
            self.cards.push(card);
        }
    }
    
    /// Crée les quads pour le fond du panel et toutes les cards
    pub fn create_quads(&self) -> Vec<QuadInstance> {
        let mut quads = Vec::new();
        
        if self.cards.is_empty() {
            return quads;
        }
        
        // Calculer la hauteur totale du panel
        let total_height = (self.cards.len() as f32 * self.card_height) 
            + ((self.cards.len() as f32 - 1.0) * self.card_spacing)
            + 40.0; // Padding
        
        // Panel noir semi-transparent derrière toutes les cards
        quads.push(create_quad(
            self.x,
            self.y - 20.0,
            self.width,
            total_height,
            [0.0, 0.0, 0.0, 0.8], // Noir semi-transparent
            self.screen_width,
            self.screen_height,
        ));
        
        // Fond de chaque card (noir semi-transparent)
        for card in &self.cards {
            quads.push(create_quad(
                card.x,
                card.y,
                card.width,
                card.height,
                card.background_color(),
                self.screen_width,
                self.screen_height,
            ));
        }
        
        quads
    }
    
    /// Crée les sections de texte pour toutes les cards
    pub fn create_text_sections(&mut self) -> Vec<Section> {
        // Vider et recréer les strings
        self.text_strings.clear();
        let mut card_data = Vec::new();
        
        // Première passe : créer toutes les strings
        for card in &self.cards {
            let title = card.title_text(); // Juste le titre en gros
            let artist_difficulty = card.artist_difficulty_text(); // artist | difficulty en petit
            let text_color = card.text_color();
            
            // Stocker les strings et leurs indices
            let title_idx = self.text_strings.len();
            self.text_strings.push(title);
            let diff_idx = self.text_strings.len();
            self.text_strings.push(artist_difficulty);
            
            card_data.push((card.x, card.y, card.width, card.height, text_color, title_idx, diff_idx));
        }
        
        // Deuxième passe : créer les sections avec les références aux strings
        let mut sections = Vec::new();
        for (x, y, width, height, text_color, title_idx, diff_idx) in card_data {
            // Titre en gros (juste le titre de la map)
            sections.push(Section {
                screen_position: (x + 15.0, y + 15.0),
                bounds: (width - 30.0, height),
                text: vec![
                    Text::new(&self.text_strings[title_idx])
                        .with_scale(28.0)
                        .with_color(text_color),
                ],
                ..Default::default()
            });
            
            // artist | difficulty en petit en dessous
            sections.push(Section {
                screen_position: (x + 15.0, y + 50.0),
                bounds: (width - 30.0, height),
                text: vec![
                    Text::new(&self.text_strings[diff_idx])
                        .with_scale(18.0)
                        .with_color([0.7, 0.7, 0.7, 1.0]), // Gris pour le texte secondaire
                ],
                ..Default::default()
            });
        }
        
        sections
    }
    
    /// Met à jour la taille de l'écran
    pub fn update_size(&mut self, screen_width: f32, screen_height: f32) {
        self.screen_width = screen_width;
        self.screen_height = screen_height;
        self.width = self.card_width + 40.0;
        self.x = screen_width - self.width;
    }
}

