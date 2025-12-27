//! In-game / gameplay rendering (practice mode overlay).

use crate::shared::snapshot::GameplaySnapshot;

pub fn render(ctx: &egui::Context, snapshot: &GameplaySnapshot, screen_width: f32) {
    if snapshot.practice_mode {
        egui::Area::new(egui::Id::new("practice_overlay"))
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui| {
                crate::views::components::PracticeOverlay::render(
                    ui,
                    snapshot.audio_time,
                    snapshot.map_duration,
                    &snapshot.checkpoints,
                    screen_width,
                );
            });
    }
}
