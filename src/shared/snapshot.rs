use crate::input::events::{EditMode, EditorTarget}; // Ajout de EditMode
use crate::models::engine::NoteData;
use crate::models::menu::{GameResultData, MenuState};
use crate::models::stats::{HitStats, Judgement};
use std::time::Instant;

#[derive(Clone, Debug)]
pub enum RenderState {
    Empty,
    Menu(MenuState),
    InGame(GameplaySnapshot),
    Editor(EditorSnapshot),
    Result(GameResultData),
}

#[derive(Clone, Debug)]
pub struct EditorSnapshot {
    pub game: GameplaySnapshot,
    pub target: Option<EditorTarget>,
    pub mode: EditMode, // Le mode actuel (Resize/Move)
    pub status_text: String,

    // La commande de modification contient maintenant le mode
    pub modification: Option<(EditorTarget, EditMode, f32, f32)>,
    pub save_requested: bool,
}

#[derive(Clone, Debug)]
pub struct GameplaySnapshot {
    pub audio_time: f64,
    pub timestamp: Instant,
    pub rate: f64,
    pub scroll_speed: f64,

    pub visible_notes: Vec<NoteData>,
    pub keys_held: Vec<bool>,

    pub score: u32,
    pub accuracy: f64,
    pub combo: u32,
    pub hit_stats: HitStats,
    pub remaining_notes: usize,

    pub last_hit_judgement: Option<Judgement>,
    pub last_hit_timing: Option<f64>,

    pub nps: f64,
}
