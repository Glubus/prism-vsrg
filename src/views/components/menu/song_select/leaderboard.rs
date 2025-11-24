use egui::{Color32, ScrollArea};

use crate::models::replay::ReplayData;
use crate::models::engine::hit_window::HitWindow;
use crate::views::components::menu::song_select::leaderboard_card::LeaderboardCard;

#[derive(Clone)]
pub struct ScoreCard {
    pub timestamp: i64,
    pub rate: f64,
    pub replay_data: ReplayData,
    pub total_notes: usize, // Nombre total de notes dans la map
}

impl ScoreCard {
    pub fn from_replay(replay: &crate::database::models::Replay, total_notes: usize) -> Option<Self> {
        let replay_data = if let Ok(data) = serde_json::from_str::<ReplayData>(&replay.data) {
            data
        } else {
            ReplayData::new()
        };
        
        Some(ScoreCard {
            timestamp: replay.timestamp,
            rate: replay.rate,
            replay_data,
            total_notes,
        })
    }
}

pub struct Leaderboard {
    scores: Vec<ScoreCard>,
}

impl Leaderboard {
    pub fn new() -> Self {
        Self {
            scores: Vec::new(),
        }
    }

    pub fn update_scores(&mut self, scores: Vec<ScoreCard>) {
        self.scores = scores;
    }

    pub fn render(&self, ui: &mut egui::Ui, _difficulty_name: Option<&str>, hit_window: &HitWindow) {
        egui::Frame::default()
            .corner_radius(5.0)
            .outer_margin(10.0)
            .inner_margin(5.0)
            .fill(Color32::from_rgba_unmultiplied(38, 38, 38, 230))
            .show(ui, |ui| {
                ui.set_width(ui.available_rect_before_wrap().width());
                ui.set_height(ui.available_rect_before_wrap().height());

                ui.heading("Top Scores");
                ui.separator();

                if self.scores.is_empty() {
                    ui.centered_and_justified(|ui| {
                        ui.label("No Score Set");
                    });
                } else {
                    ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            for (i, card) in self.scores.iter().take(10).enumerate() {
                                // Recalculer avec la hit window actuelle
                                let (hit_stats, accuracy) = crate::models::replay::recalculate_accuracy_with_hit_window(
                                    &card.replay_data,
                                    card.total_notes,
                                    hit_window,
                                );
                                
                                LeaderboardCard::render(
                                    ui,
                                    i,
                                    accuracy,
                                    card.rate,
                                    card.timestamp,
                                    &hit_stats,
                                );
                                
                                if i < self.scores.len().min(10).saturating_sub(1) {
                                    ui.add_space(5.0);
                                }
                            }
                        });
                }
            });
    }
}

