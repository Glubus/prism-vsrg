use crate::engine::{PixelSystem, GameEngine};
use wgpu_text::glyph_brush::Section;

pub trait Component {
    fn render(&mut self, engine: &GameEngine, pixel_system: &PixelSystem, screen_width: f32, screen_height: f32) -> Vec<Section>;
}

pub mod accuracy;
pub mod combo;
pub mod hit_bar;
pub mod judgement;
pub mod judgements;
pub mod playfield;
pub mod score;
pub mod card;
pub mod map_list;
pub mod song_selection_menu;

pub use accuracy::AccuracyComponent;
pub use combo::ComboComponent;
pub use hit_bar::HitBar;
pub use judgement::JudgementComponent;
pub use judgements::JudgementsComponent;
pub use playfield::PlayfieldComponent;
pub use score::ScoreComponent;

