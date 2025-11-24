use serde::{Deserialize, Serialize};

/// Représente un hit sur une note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayHit {
    pub note_index: usize,  // Numéro de la note dans l'ordre
    pub timing_ms: f64,     // Distance en ms (peut être négatif si en avance)
}

/// Représente une pression standard de l'utilisateur (touche pressée sans note)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayKeyPress {
    pub timestamp_ms: f64,  // Temps absolu en ms depuis le début
    pub column: usize,      // Colonne pressée
}

/// Structure complète d'un replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayData {
    pub hits: Vec<ReplayHit>,           // Tous les hits dans l'ordre
    pub key_presses: Vec<ReplayKeyPress>, // Toutes les pressions standard
    pub hit_stats: Option<crate::models::stats::HitStats>, // Stats de hits pour affichage rapide
}

impl ReplayData {
    pub fn new() -> Self {
        Self {
            hits: Vec::new(),
            key_presses: Vec::new(),
            hit_stats: None,
        }
    }

    pub fn with_hit_stats(mut self, stats: crate::models::stats::HitStats) -> Self {
        self.hit_stats = Some(stats);
        self
    }

    /// Ajoute un hit au replay
    pub fn add_hit(&mut self, note_index: usize, timing_ms: f64) {
        self.hits.push(ReplayHit {
            note_index,
            timing_ms,
        });
    }

    /// Ajoute une pression de touche standard
    pub fn add_key_press(&mut self, timestamp_ms: f64, column: usize) {
        self.key_presses.push(ReplayKeyPress {
            timestamp_ms,
            column,
        });
    }

    /// Sérialise en JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Désérialise depuis JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl Default for ReplayData {
    fn default() -> Self {
        Self::new()
    }
}

/// Recalcule les hit_stats et l'accuracy en utilisant la hit window actuelle
/// 
/// # Arguments
/// * `replay_data` - Les données du replay avec les timings des hits
/// * `total_notes` - Le nombre total de notes dans la map
/// * `hit_window` - La hit window actuelle à utiliser pour rejuger
/// 
/// # Returns
/// Un tuple (HitStats, accuracy)
pub fn recalculate_accuracy_with_hit_window(
    replay_data: &ReplayData,
    total_notes: usize,
    hit_window: &crate::models::engine::hit_window::HitWindow,
) -> (crate::models::stats::HitStats, f64) {
    use crate::models::stats::{HitStats, Judgement};
    
    let mut stats = HitStats::new();
    
    // Créer un set des notes qui ont été hit
    let mut hit_notes = std::collections::HashSet::new();
    for hit in &replay_data.hits {
        hit_notes.insert(hit.note_index);
        
        // Rejuger avec la nouvelle hit window
        // Le timing_ms est déjà normalisé (divisé par le rate lors de la sauvegarde)
        let (judgement, _) = hit_window.judge(hit.timing_ms);
        
        match judgement {
            Judgement::Marv => stats.marv += 1,
            Judgement::Perfect => stats.perfect += 1,
            Judgement::Great => stats.great += 1,
            Judgement::Good => stats.good += 1,
            Judgement::Bad => stats.bad += 1,
            Judgement::Miss => stats.miss += 1,
            Judgement::GhostTap => stats.ghost_tap += 1,
        }
    }
    
    // Les notes non hit sont des miss
    for note_index in 0..total_notes {
        if !hit_notes.contains(&note_index) {
            stats.miss += 1;
        }
    }
    
    let accuracy = stats.calculate_accuracy();
    (stats, accuracy)
}

