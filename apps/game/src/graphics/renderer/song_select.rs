//! Song select / menu rendering.

use super::Renderer;
use crate::input::events::GameAction;
use crate::state::MenuState;
use crate::ui::page::song_select::UIPanelTextures;
use crate::views::settings::{SettingsSnapshot, render_settings_window};

pub fn render(
    renderer: &mut Renderer,
    ctx: &egui::Context,
    menu_state: &MenuState,
    _swapchain_view: &wgpu::TextureView,
    actions: &mut Vec<GameAction>,
) {
    // Gestion de la fenêtre de Settings (Popup)
    if menu_state.show_settings {
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

    let menus = &renderer.resources.skin.menus;
    let to_egui = |c: [f32; 4]| {
        egui::Color32::from_rgba_unmultiplied(
            (c[0] * 255.) as u8,
            (c[1] * 255.) as u8,
            (c[2] * 255.) as u8,
            (c[3] * 255.) as u8,
        )
    };
    let hit_window = match renderer.resources.settings.hit_window_mode {
        crate::models::settings::HitWindowMode::OsuOD => {
            engine::hit_window::HitWindow::from_osu_od(renderer.resources.settings.hit_window_value)
        }
        crate::models::settings::HitWindowMode::EtternaJudge => {
            engine::hit_window::HitWindow::from_etterna_judge(
                renderer.resources.settings.hit_window_value as u8,
            )
        }
    };
    let panel_textures = UIPanelTextures {
        beatmap_info_bg: renderer
            .resources
            .beatmap_info_bg_texture
            .as_ref()
            .map(|t| t.id()),
        search_panel_bg: renderer
            .resources
            .search_panel_bg_texture
            .as_ref()
            .map(|t| t.id()),
        search_bar: renderer
            .resources
            .search_bar_texture
            .as_ref()
            .map(|t| t.id()),
    };

    let (action_opt, result_data, search_request, calculator_changed) =
        renderer.song_select_screen.render(
            ctx,
            menu_state,
            _swapchain_view,
            renderer.ctx.config.width as f32,
            renderer.ctx.config.height as f32,
            &hit_window,
            renderer.resources.settings.hit_window_mode,
            renderer.resources.settings.hit_window_value,
            renderer
                .resources
                .song_button_texture
                .as_ref()
                .map(|t| t.id()),
            renderer
                .resources
                .song_button_selected_texture
                .as_ref()
                .map(|t| t.id()),
            renderer
                .resources
                .difficulty_button_texture
                .as_ref()
                .map(|t| t.id()),
            renderer
                .resources
                .difficulty_button_selected_texture
                .as_ref()
                .map(|t| t.id()),
            to_egui(menus.song_select.song_button.selected_border_color),
            to_egui(menus.song_select.difficulty_button.selected_text_color),
            &panel_textures,
            Some(&menus.song_select.rating_colors),
        );

    if let Some(calc_id) = calculator_changed {
        actions.push(GameAction::SetCalculator(calc_id));
    }

    if let Some(a) = action_opt {
        actions.push(a);
    }

    if let Some(result_data) = result_data {
        actions.push(GameAction::SetResult(result_data));
    }

    if let Some(filters) = search_request {
        actions.push(GameAction::ApplySearch(filters));
    }
}
