//! Song selection screen page.
use crate::input::events::GameAction;
use crate::state::menu::SongSelectMode;
use crate::state::{GameResultData, MenuState};
use crate::ui::song_select::beatmap_info::{BeatmapInfo, InfoTab};
use crate::ui::song_select::leaderboard::{Leaderboard, ScoreCard};
use crate::ui::song_select::search_panel::{SearchPanel, SearchPanelEvent};
use crate::ui::song_select::song_list::SongList;
use database::MenuSearchFilters;
use egui::{Color32, RichText, TextureId};
use wgpu::TextureView;

/// Textures for UI panel backgrounds
pub struct UIPanelTextures {
    pub beatmap_info_bg: Option<TextureId>,
    pub search_panel_bg: Option<TextureId>,
    pub search_bar: Option<TextureId>,
}

impl Default for UIPanelTextures {
    fn default() -> Self {
        Self {
            beatmap_info_bg: None,
            search_panel_bg: None,
            search_bar: None,
        }
    }
}

pub struct SongSelectScreen {
    song_list: SongList,
    leaderboard: Leaderboard,
    beatmap_info: BeatmapInfo,
    search_panel: SearchPanel,
}

impl SongSelectScreen {
    pub fn new() -> Self {
        Self {
            song_list: SongList::new(),
            leaderboard: Leaderboard::new(),
            beatmap_info: BeatmapInfo::new(),
            search_panel: SearchPanel::new(),
        }
    }

    // Signature extended to optionally bubble up GameResultData.
    // Returns: (UIAction, GameResultData, SearchFilters, CalculatorChanged)
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &mut self,
        ctx: &egui::Context,
        menu_state: &MenuState, // Immutable
        _view: &TextureView,
        _screen_width: f32,
        _screen_height: f32,
        hit_window: &engine::hit_window::HitWindow,
        hit_window_mode: crate::models::settings::HitWindowMode,
        hit_window_value: f64,
        btn_tex: Option<TextureId>,
        btn_sel_tex: Option<TextureId>,
        diff_tex: Option<TextureId>,
        diff_sel_tex: Option<TextureId>,
        song_sel_color: Color32,
        diff_sel_color: Color32,
        panel_textures: &UIPanelTextures,
        rating_colors: Option<&skin::menus::song_select::RatingColorsConfig>,
    ) -> (
        Option<GameAction>,
        Option<GameResultData>,
        Option<MenuSearchFilters>,
        Option<String>, // Calculator changed
    ) {
        // Set current index
        self.song_list.set_current(menu_state.selected_index);

        let mut action_triggered = None;
        let mut result_data_triggered = None;
        let mut search_request = None;
        let mut calculator_changed = None;

        // --- TOP: Mode Tabs ---
        egui::TopBottomPanel::top("mode_tabs")
            .frame(egui::Frame::NONE.fill(Color32::from_black_alpha(200)))
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    if let Some(act) = self.render_filter_bar(ui, menu_state) {
                        action_triggered = Some(act);
                    }
                });
                ui.add_space(4.0);
            });

        // --- BOTTOM: Action Bar / Footer ---
        egui::TopBottomPanel::bottom("action_bar")
            .frame(egui::Frame::NONE.fill(Color32::from_black_alpha(240)))
            .show(ctx, |ui| {
                ui.add_space(8.0);
                if let Some(act) = self.render_action_bar(ui, menu_state) {
                    action_triggered = Some(act);
                }
                ui.add_space(8.0);
            });

        // --- CENTER: Split View (Wheel | Info) ---
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let available_size = ui.available_size();

                // Left Panel: Song Wheel (50%)
                let left_panel_width = available_size.x * 0.5;

                egui::SidePanel::left("song_wheel_panel")
                    .exact_width(left_panel_width)
                    .frame(egui::Frame::NONE)
                    .show_inside(ui, |ui| {
                        // Search bar at top of wheel
                        // Search Panel moved to Mods tab
                        ui.add_space(8.0);

                        ui.add_space(10.0);

                        // Song Wheel
                        if let Some(act) = self.song_list.render(
                            ui,
                            menu_state,
                            btn_tex,
                            btn_sel_tex,
                            diff_tex,
                            diff_sel_tex,
                            song_sel_color,
                            diff_sel_color,
                            rating_colors,
                        ) {
                            action_triggered = Some(act);
                        }
                    });

                // Right Panel: Info & Stats (Remaining)
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show_inside(ui, |ui| {
                        ui.add_space(20.0);

                        let (beatmapset, beatmap, rate, diff_name) = {
                            if let Some((bs, beatmaps)) =
                                menu_state.beatmapsets.get(menu_state.selected_index)
                            {
                                let bm = beatmaps.get(menu_state.selected_difficulty_index);
                                let diff_name =
                                    bm.and_then(|bm| bm.beatmap.difficulty_name.clone());
                                (Some(bs.clone()), bm.cloned(), menu_state.rate, diff_name)
                            } else {
                                (None, None, 1.0, None)
                            }
                        };

                        // 1. Beatmap Info Panel
                        if let Some(bs) = &beatmapset {
                            let rate_specific_ratings = beatmap.as_ref().and_then(|bm| {
                                menu_state.get_cached_ratings_for(&bm.beatmap.hash, rate)
                            });

                            // Update leaderboard logic
                            if let Some(bm) = beatmap.as_ref() {
                                self.refresh_leaderboard(
                                    menu_state,
                                    &bm.beatmap.hash,
                                    bm.beatmap.note_count as usize,
                                );
                            } else {
                                self.leaderboard.update_scores(Vec::new());
                            }

                            // Get current difficulty from cache
                            let current_ssr = menu_state.get_current_difficulty();

                            if let Some(new_calc) = self.beatmap_info.render(
                                ui,
                                bs,
                                beatmap.as_ref(),
                                rate,
                                hit_window_mode,
                                hit_window_value,
                                rate_specific_ratings,
                                panel_textures.beatmap_info_bg,
                                &menu_state.available_calculators,
                                &menu_state.active_calculator,
                                current_ssr,
                            ) {
                                calculator_changed = Some(new_calc);
                            }
                            ui.add_space(10.0);
                        }

                        // 2. Tabs + Content (Scores vs Breakdown)
                        ui.add_space(10.0);

                        // Unified Frame for Tabbed Section
                        egui::Frame::default()
                            .corner_radius(egui::CornerRadius::same(12))
                            .outer_margin(egui::Margin::symmetric(8, 6))
                            .inner_margin(egui::Margin::symmetric(14, 10))
                            .fill(self.beatmap_info.colors.panel_bg)
                            .stroke(egui::Stroke::new(
                                1.0,
                                self.beatmap_info.colors.panel_border,
                            ))
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());

                                // Tab Selector Header
                                self.render_tab_selector(ui);

                                ui.add_space(10.0);
                                ui.separator();
                                ui.add_space(10.0);

                                match self.beatmap_info.active_tab {
                                    InfoTab::Scores => {
                                        let cached_chart = menu_state
                                            .get_cached_chart()
                                            .map(|c| c.chart.as_slice());

                                        let clicked_result = self.leaderboard.render(
                                            ui,
                                            diff_name.as_deref(),
                                            hit_window,
                                            cached_chart,
                                        );
                                        if let Some(result_data) = clicked_result {
                                            result_data_triggered = Some(result_data);
                                        }
                                    }
                                    InfoTab::Breakdown => {
                                        // Get rate specific ratings again (borrow checker)
                                        let rate_specific_ratings =
                                            beatmap.as_ref().and_then(|bm| {
                                                menu_state
                                                    .get_cached_ratings_for(&bm.beatmap.hash, rate)
                                            });
                                        let current_ssr = menu_state.get_current_difficulty();
                                        self.beatmap_info.render_breakdown_tab(
                                            ui,
                                            beatmap.as_ref(),
                                            rate_specific_ratings,
                                            &menu_state.active_calculator,
                                            current_ssr,
                                        );
                                    }
                                    InfoTab::Mods => {
                                        // Render mod toggle buttons
                                        ui.add_space(10.0);
                                        ui.label(
                                            RichText::new("GAMEPLAY MODIFIERS")
                                                .size(18.0)
                                                .strong()
                                                .color(Color32::WHITE),
                                        );
                                        ui.add_space(10.0);

                                        for game_mod in crate::state::mods::GameMod::all() {
                                            let is_active = menu_state.active_mods.has(*game_mod);
                                            let color = if is_active {
                                                Color32::from_rgb(255, 0, 60) // Prism Red
                                            } else {
                                                Color32::GRAY
                                            };

                                            ui.horizontal(|ui| {
                                                let button_text =
                                                    RichText::new(game_mod.display_name())
                                                        .size(16.0)
                                                        .strong()
                                                        .color(color);
                                                if ui
                                                    .add(
                                                        egui::Button::new(button_text)
                                                            .min_size(egui::vec2(150.0, 30.0)),
                                                    )
                                                    .clicked()
                                                {
                                                    action_triggered =
                                                        Some(GameAction::ToggleMod(*game_mod));
                                                }
                                                ui.add_space(10.0);
                                                ui.label(
                                                    RichText::new(game_mod.description())
                                                        .size(14.0)
                                                        .color(Color32::LIGHT_GRAY),
                                                );
                                            });
                                            ui.add_space(5.0);
                                        }
                                    }
                                }
                            });
                    });
            });

        (
            action_triggered,
            result_data_triggered,
            search_request,
            calculator_changed,
        )
    }

    fn render_tab_selector(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.style_mut().spacing.item_spacing.x = 20.0;

            let tabs = [InfoTab::Scores, InfoTab::Breakdown, InfoTab::Mods];

            for tab in tabs {
                let label = match tab {
                    InfoTab::Scores => "TOP SCORES",
                    InfoTab::Breakdown => "PATTERN BREAKDOWN",
                    InfoTab::Mods => "MODS",
                };

                let is_active = self.beatmap_info.active_tab == tab;
                let color = if is_active {
                    Color32::WHITE
                } else {
                    Color32::GRAY
                };

                if ui
                    .add(
                        egui::Label::new(RichText::new(label).strong().color(color))
                            .sense(egui::Sense::click()),
                    )
                    .clicked()
                {
                    self.beatmap_info.active_tab = tab;
                }
            }
        });
    }

    fn render_filter_bar(
        &mut self,
        ui: &mut egui::Ui,
        menu_state: &MenuState,
    ) -> Option<GameAction> {
        ui.style_mut().spacing.item_spacing.x = 20.0;

        let filters = [
            (SongSelectMode::Key4, "4K"),
            (SongSelectMode::Key7, "7K"),
            (SongSelectMode::Practice, "PRACTICE"),
            (SongSelectMode::Coop, "COOP"),
        ];

        let mut action = None;

        for (mode, label) in filters {
            let is_active = menu_state.active_modes.contains(&mode);

            let color = if is_active {
                Color32::from_rgb(255, 0, 60) // Prism Red
            } else {
                Color32::GRAY
            };

            let text = RichText::new(label).size(18.0).strong().color(color);

            if ui
                .add(egui::Label::new(text).sense(egui::Sense::click()))
                .clicked()
            {
                action = Some(GameAction::ChangeSongSelectMode(mode));
            }
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(10.0);
            let text = RichText::new("SKIN EDITOR")
                .size(18.0)
                .strong()
                .color(Color32::GRAY);
            if ui
                .add(egui::Label::new(text).sense(egui::Sense::click()))
                .clicked()
            {
                action = Some(GameAction::ToggleEditor);
            }
        });

        action
    }

    fn render_action_bar(
        &mut self,
        ui: &mut egui::Ui,
        menu_state: &MenuState,
    ) -> Option<GameAction> {
        let mut action = None;
        ui.horizontal(|ui| {
            ui.add_space(20.0);
            if ui
                .add(egui::Button::new(RichText::new("◀ BACK").size(18.0)))
                .clicked()
            {
                action = Some(GameAction::Back);
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Mods button removed per user request
                // ui.add_space(20.0);
                // if ui
                //     .add(egui::Button::new(RichText::new("MODS ⚙").size(18.0)))
                //     .clicked()
                // {
                //     action = Some(GameAction::ToggleSettings);
                // }

                ui.add_space(20.0);
                if ui
                    .add(egui::Button::new(
                        RichText::new("PLAY ▶")
                            .size(30.0)
                            .strong()
                            .color(Color32::GREEN),
                    ))
                    .clicked()
                {
                    if menu_state.active_modes.contains(&SongSelectMode::Practice) {
                        action = Some(GameAction::LaunchPractice);
                    } else {
                        action = Some(GameAction::Confirm);
                    }
                }

                ui.add_space(20.0);

                let is_scanning =
                    matches!(menu_state.db_status, database::DbStatus::Scanning { .. });

                if is_scanning {
                    ui.label(
                        RichText::new("Scanning...")
                            .size(18.0)
                            .color(Color32::YELLOW)
                            .italics(),
                    );
                    ui.add(egui::Spinner::new());
                } else if ui
                    .add(egui::Button::new(RichText::new("REFRESH ⟳").size(18.0)))
                    .clicked()
                {
                    action = Some(GameAction::Rescan);
                }

                if let database::DbStatus::Error(ref e) = menu_state.db_status {
                    ui.label(
                        RichText::new(format!("Error: {}", e))
                            .size(14.0)
                            .color(Color32::RED),
                    );
                }
            });
        });
        action
    }

    fn refresh_leaderboard(
        &mut self,
        menu_state: &MenuState,
        beatmap_hash: &str,
        total_notes: usize,
    ) {
        if menu_state.leaderboard_hash.as_deref() == Some(beatmap_hash) {
            let cards = menu_state
                .leaderboard_scores
                .iter()
                .filter_map(|replay| ScoreCard::from_replay(replay, total_notes))
                .collect();
            self.leaderboard.update_scores(cards);
        } else {
            self.leaderboard.update_scores(Vec::new());
        }
    }
}
