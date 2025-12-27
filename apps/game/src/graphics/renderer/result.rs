//! Result screen rendering.

use super::Renderer;
use crate::input::events::GameAction;
use crate::state::GameResultData;
use crate::views::settings::{SettingsSnapshot, render_settings_window};

pub fn render(
    renderer: &mut Renderer,
    ctx: &egui::Context,
    data: &GameResultData,
    actions: &mut Vec<GameAction>,
) {
    if data.show_settings {
        let (snapshot, result) = {
            let settings = &mut renderer.resources.settings;
            let snapshot = SettingsSnapshot::capture(settings);
            let result = render_settings_window(ctx, settings, &snapshot);
            (snapshot, result)
        };

        if renderer.resources.settings.current_skin != snapshot.skin {
            renderer.resources.settings.save();
            renderer.resources = crate::render::resources::RenderResources::new(&renderer.ctx, ctx);
            renderer.resources.update_component_positions(
                renderer.ctx.config.width as f32,
                renderer.ctx.config.height as f32,
            );
        }

        if let Some(volume) = result.volume_changed {
            actions.push(GameAction::UpdateVolume(volume));
        }
        if let Some((mode, value)) = result.hit_window_changed {
            actions.push(GameAction::UpdateHitWindow { mode, value });
        }
        if result.keybinds_updated {
            actions.push(GameAction::ReloadKeybinds);
        }
        if result.request_toggle {
            actions.push(GameAction::ToggleSettings);
        }
    }

    // Render result screen
    let hit_win = engine::hit_window::HitWindow::new();
    if renderer.result_screen.render(ctx, data, &hit_win) {
        actions.push(GameAction::Back);
    }
}
