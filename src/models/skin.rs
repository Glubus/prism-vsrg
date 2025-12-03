//! Skin configuration and loading.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinGeneral {
    pub name: String,
    pub version: String,
    pub author: String,
    #[serde(default)]
    pub font: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinColors {
    #[serde(default = "default_white")]
    pub receptor_color: [f32; 4],
    #[serde(default = "default_white")]
    pub note_color: [f32; 4],
    #[serde(default = "default_selected")]
    pub selected_color: [f32; 4],
    #[serde(default = "default_diff_selected")]
    pub difficulty_selected_color: [f32; 4],
    #[serde(default = "default_cyan")]
    pub marv: [f32; 4],
    #[serde(default = "default_yellow")]
    pub perfect: [f32; 4],
    #[serde(default = "default_green")]
    pub great: [f32; 4],
    #[serde(default = "default_blue")]
    pub good: [f32; 4],
    #[serde(default = "default_pink")]
    pub bad: [f32; 4],
    #[serde(default = "default_red")]
    pub miss: [f32; 4],
    #[serde(default = "default_gray")]
    pub ghost_tap: [f32; 4],

    // UI Panel Colors
    #[serde(default = "default_panel_bg")]
    pub panel_background: [f32; 4],
    #[serde(default = "default_panel_secondary")]
    pub panel_secondary: [f32; 4],
    #[serde(default = "default_panel_border")]
    pub panel_border: [f32; 4],
    #[serde(default = "default_accent")]
    pub accent: [f32; 4],
    #[serde(default = "default_accent_dim")]
    pub accent_dim: [f32; 4],
    #[serde(default = "default_text_primary")]
    pub text_primary: [f32; 4],
    #[serde(default = "default_text_secondary")]
    pub text_secondary: [f32; 4],
    #[serde(default = "default_text_muted")]
    pub text_muted: [f32; 4],
    #[serde(default = "default_rating_stream")]
    pub rating_stream: [f32; 4],
    #[serde(default = "default_rating_js")]
    pub rating_jumpstream: [f32; 4],
    #[serde(default = "default_rating_hs")]
    pub rating_handstream: [f32; 4],
    #[serde(default = "default_rating_stam")]
    pub rating_stamina: [f32; 4],
    #[serde(default = "default_rating_jack")]
    pub rating_jackspeed: [f32; 4],
    #[serde(default = "default_rating_cj")]
    pub rating_chordjack: [f32; 4],
    #[serde(default = "default_rating_tech")]
    pub rating_technical: [f32; 4],
    #[serde(default = "default_search_active")]
    pub search_active_indicator: [f32; 4],
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UIElementPos {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinUserConfig {
    #[serde(default = "default_note_size")]
    pub note_width_px: f32,
    #[serde(default = "default_note_size")]
    pub note_height_px: f32,
    #[serde(default = "default_note_size")]
    pub receptor_width_px: f32,
    #[serde(default = "default_note_size")]
    pub receptor_height_px: f32,
    pub column_width_px: f32,
    #[serde(default)]
    pub receptor_spacing_px: f32,
    #[serde(default = "default_text_size")]
    pub combo_text_size: f32,
    #[serde(default = "default_text_size")]
    pub score_text_size: f32,
    #[serde(default = "default_text_size")]
    pub accuracy_text_size: f32,
    #[serde(default = "default_text_size")]
    pub judgement_text_size: f32,
    #[serde(default = "default_hitbar_height")]
    pub hit_bar_height_px: f32,
    #[serde(default)]
    pub playfield_pos: Option<UIElementPos>,
    #[serde(default)]
    pub combo_pos: Option<UIElementPos>,
    #[serde(default)]
    pub score_pos: Option<UIElementPos>,
    #[serde(default)]
    pub accuracy_pos: Option<UIElementPos>,
    #[serde(default)]
    pub judgement_pos: Option<UIElementPos>,
    #[serde(default)]
    pub judgement_flash_pos: Option<UIElementPos>,
    #[serde(default)]
    pub hit_bar_pos: Option<UIElementPos>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinKeyMode {
    pub receptor_images: Vec<String>,
    #[serde(default)]
    pub receptor_pressed_images: Vec<String>,
    pub note_images: Vec<String>,
    pub hit_bar_pos: Option<f32>,
}

pub struct Skin {
    pub base_path: PathBuf,
    pub general: SkinGeneral,
    pub colors: SkinColors,
    pub config: SkinUserConfig,
    pub key_modes: HashMap<usize, SkinKeyMode>,
    pub background: Option<PathBuf>,
    pub miss_note: Option<PathBuf>,
    pub song_button: Option<PathBuf>,
    pub song_button_selected: Option<PathBuf>,
    pub difficulty_button: Option<PathBuf>,
    pub difficulty_button_selected: Option<PathBuf>,
    // UI Panel custom images
    pub beatmap_info_background: Option<PathBuf>,
    pub search_panel_background: Option<PathBuf>,
    pub search_bar: Option<PathBuf>,
    pub leaderboard_background: Option<PathBuf>,
    // Note type images
    pub mine: Option<PathBuf>,
    pub hold_body: Option<PathBuf>,
    pub hold_end: Option<PathBuf>,
    pub burst_body: Option<PathBuf>,
    pub burst_end: Option<PathBuf>,
}

impl Skin {
    pub fn load(skin_name: &str) -> Result<Self, String> {
        let base_path = Path::new("skins").join(skin_name);
        if !base_path.exists() {
            if skin_name == "default" {
                eprintln!("Default skin folder missing, recreating structure...");
                let _ = init_skin_structure();
                if !base_path.exists() {
                    return Err(format!(
                        "Failed to recreate default skin at {:?}",
                        base_path
                    ));
                }
            } else {
                return Err(format!("Skin folder not found: {:?}", base_path));
            }
        }
        let general: SkinGeneral = load_toml(&base_path.join("general.toml"))?;
        let colors: SkinColors = load_toml(&base_path.join("colors.toml"))?;
        let config: SkinUserConfig = load_toml(&base_path.join("conf.toml"))?;
        Ok(Self {
            base_path: base_path.clone(),
            general,
            colors,
            config,
            key_modes: HashMap::new(),
            background: check_file(&base_path, "background.png"),
            miss_note: check_file(&base_path, "miss_note.png"),
            song_button: check_file(&base_path, "song_button.png"),
            song_button_selected: check_file(&base_path, "song_button_selected.png"),
            difficulty_button: check_file(&base_path, "difficulty_button.png"),
            difficulty_button_selected: check_file(&base_path, "difficulty_button_selected.png"),
            // UI Panel custom images
            beatmap_info_background: check_file(&base_path, "beatmap_info_bg.png"),
            search_panel_background: check_file(&base_path, "search_panel_bg.png"),
            search_bar: check_file(&base_path, "search_bar.png"),
            leaderboard_background: check_file(&base_path, "leaderboard_bg.png"),
            // Note type images
            mine: check_file(&base_path, "mine.png"),
            hold_body: check_file(&base_path, "hold_body.png"),
            hold_end: check_file(&base_path, "hold_end.png"),
            burst_body: check_file(&base_path, "burst_body.png"),
            burst_end: check_file(&base_path, "burst_end.png"),
        })
    }
    pub fn save_user_config(&self) -> Result<(), String> {
        let path = self.base_path.join("conf.toml");
        let content = toml::to_string_pretty(&self.config).map_err(|e| e.to_string())?;
        fs::write(path, content).map_err(|e| e.to_string())
    }
    pub fn load_key_mode(&mut self, key_count: usize) {
        if self.key_modes.contains_key(&key_count) {
            return;
        }
        let path = self.base_path.join(format!("{}k.toml", key_count));
        if path.exists() {
            if let Ok(mode) = load_toml::<SkinKeyMode>(&path) {
                self.key_modes.insert(key_count, mode);
            } else {
                eprintln!("Failed to parse {}k.toml", key_count);
            }
        }
    }
    pub fn get_receptor_image(&self, key_count: usize, col: usize) -> Option<PathBuf> {
        self.key_modes
            .get(&key_count)
            .and_then(|m| get_image_from_list(&m.receptor_images, col))
            .map(|name| self.base_path.join(name))
            .or_else(|| check_file(&self.base_path, "receptor.png"))
    }
    pub fn get_receptor_pressed_image(&self, key_count: usize, col: usize) -> Option<PathBuf> {
        self.key_modes
            .get(&key_count)
            .and_then(|m| get_image_from_list(&m.receptor_pressed_images, col))
            .map(|name| self.base_path.join(name))
            .or_else(|| check_file(&self.base_path, "receptor_pressed.png"))
    }
    pub fn get_note_image(&self, key_count: usize, col: usize) -> Option<PathBuf> {
        self.key_modes
            .get(&key_count)
            .and_then(|m| get_image_from_list(&m.note_images, col))
            .map(|name| self.base_path.join(name))
            .or_else(|| check_file(&self.base_path, "note.png"))
    }
    
    /// Get mine image (falls back to note if not found)
    pub fn get_mine_image(&self) -> Option<PathBuf> {
        self.mine.clone().or_else(|| check_file(&self.base_path, "note.png"))
    }
    
    /// Get hold body image (the middle part that stretches)
    pub fn get_hold_body_image(&self) -> Option<PathBuf> {
        self.hold_body.clone()
    }
    
    /// Get hold end image (the cap at the end)
    pub fn get_hold_end_image(&self) -> Option<PathBuf> {
        self.hold_end.clone().or_else(|| check_file(&self.base_path, "note.png"))
    }
    
    /// Get burst body image (the middle part that stretches)
    pub fn get_burst_body_image(&self) -> Option<PathBuf> {
        self.burst_body.clone()
    }
    
    /// Get burst end image (the cap at the end)
    pub fn get_burst_end_image(&self) -> Option<PathBuf> {
        self.burst_end.clone().or_else(|| check_file(&self.base_path, "note.png"))
    }
    
    pub fn get_font_path(&self) -> Option<PathBuf> {
        self.general.font.as_ref().map(|f| self.base_path.join(f))
    }
}

fn load_toml<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    toml::from_str(&content).map_err(|e| e.to_string())
}
fn check_file(base: &Path, name: &str) -> Option<PathBuf> {
    let p = base.join(name);
    if p.exists() { Some(p) } else { None }
}
fn get_image_from_list(list: &[String], idx: usize) -> Option<&String> {
    if list.is_empty() {
        return None;
    }
    if idx < list.len() {
        Some(&list[idx])
    } else {
        Some(&list[0])
    }
}

fn default_white() -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}
fn default_selected() -> [f32; 4] {
    [1.0, 0.0, 0.0, 1.0]
}
fn default_diff_selected() -> [f32; 4] {
    [1.0, 1.0, 0.0, 1.0]
}
fn default_cyan() -> [f32; 4] {
    [0.0, 1.0, 1.0, 1.0]
}
fn default_yellow() -> [f32; 4] {
    [1.0, 1.0, 0.0, 1.0]
}
fn default_green() -> [f32; 4] {
    [0.0, 1.0, 0.0, 1.0]
}
fn default_blue() -> [f32; 4] {
    [0.0, 0.0, 0.5, 1.0]
}
fn default_pink() -> [f32; 4] {
    [1.0, 0.41, 0.71, 1.0]
}
fn default_red() -> [f32; 4] {
    [1.0, 0.0, 0.0, 1.0]
}
fn default_gray() -> [f32; 4] {
    [0.5, 0.5, 0.5, 1.0]
}

// UI Panel default colors
fn default_panel_bg() -> [f32; 4] {
    [0.08, 0.08, 0.10, 0.95]
}
fn default_panel_secondary() -> [f32; 4] {
    [0.12, 0.12, 0.15, 0.90]
}
fn default_panel_border() -> [f32; 4] {
    [0.25, 0.25, 0.30, 0.80]
}
fn default_accent() -> [f32; 4] {
    [0.40, 0.70, 1.0, 1.0]
}
fn default_accent_dim() -> [f32; 4] {
    [0.25, 0.45, 0.70, 1.0]
}
fn default_text_primary() -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}
fn default_text_secondary() -> [f32; 4] {
    [0.75, 0.75, 0.80, 1.0]
}
fn default_text_muted() -> [f32; 4] {
    [0.50, 0.50, 0.55, 1.0]
}
fn default_rating_stream() -> [f32; 4] {
    [0.30, 0.85, 0.50, 1.0]
}
fn default_rating_js() -> [f32; 4] {
    [0.95, 0.75, 0.20, 1.0]
}
fn default_rating_hs() -> [f32; 4] {
    [0.90, 0.45, 0.30, 1.0]
}
fn default_rating_stam() -> [f32; 4] {
    [0.85, 0.30, 0.55, 1.0]
}
fn default_rating_jack() -> [f32; 4] {
    [0.60, 0.40, 0.90, 1.0]
}
fn default_rating_cj() -> [f32; 4] {
    [0.40, 0.60, 0.95, 1.0]
}
fn default_rating_tech() -> [f32; 4] {
    [0.20, 0.80, 0.85, 1.0]
}
fn default_search_active() -> [f32; 4] {
    [0.30, 0.75, 0.95, 1.0]
}

fn default_note_size() -> f32 {
    90.0
}
fn default_text_size() -> f32 {
    20.0
}
fn default_hitbar_height() -> f32 {
    20.0
}

pub fn init_skin_structure() -> Result<(), String> {
    let skins_dir = Path::new("skins");
    let default_dir = skins_dir.join("default");
    if !skins_dir.exists() {
        fs::create_dir_all(skins_dir).map_err(|e| e.to_string())?;
    }
    if !default_dir.exists() {
        fs::create_dir_all(&default_dir).map_err(|e| e.to_string())?;
    }
    if !default_dir.join("general.toml").exists() {
        fs::write(
            default_dir.join("general.toml"),
            "name=\"Default Skin\"\nversion=\"1.0\"\nauthor=\"System\"\nfont=\"font.ttf\"\n",
        )
        .map_err(|e| e.to_string())?;
    }
    if !default_dir.join("colors.toml").exists() {
        fs::write(default_dir.join("colors.toml"), "receptor_color=[0.0,0.0,1.0,1.0]\nnote_color=[1.0,1.0,1.0,1.0]\nselected_color=[1.0,0.0,0.0,1.0]\ndifficulty_selected_color=[1.0,1.0,0.0,1.0]\nmarv=[0.0,1.0,1.0,1.0]\nperfect=[1.0,1.0,0.0,1.0]\ngreat=[0.0,1.0,0.0,1.0]\ngood=[0.0,0.0,0.5,1.0]\nbad=[1.0,0.41,0.71,1.0]\nmiss=[1.0,0.0,0.0,1.0]\nghost_tap=[0.5,0.5,0.5,1.0]\n").map_err(|e| e.to_string())?;
    }
    if !default_dir.join("conf.toml").exists() {
        fs::write(default_dir.join("conf.toml"), "column_width_px=100.0\nreceptor_spacing_px=0.0\nnote_width_px=90.0\nnote_height_px=90.0\nreceptor_width_px=90.0\nreceptor_height_px=90.0\ncombo_text_size=48.0\nscore_text_size=24.0\naccuracy_text_size=20.0\njudgement_text_size=16.0\nhit_bar_height_px=20.0\n").map_err(|e| e.to_string())?;
    }
    for k in 4..=10 {
        let path = default_dir.join(format!("{}k.toml", k));
        if !path.exists() {
            fs::write(&path, "receptor_images=[\"receptor.png\"]\nreceptor_pressed_images=[\"receptor_pressed.png\"]\nnote_images=[\"note.png\"]\n").map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
