pub(super) mod leaderboard;
pub(super) mod leaderboard_card;
pub(super) mod song_list;
pub(super) mod song_card;
pub(super) mod difficulty_card;
pub(super) mod beatmap_info;

use std::sync::{Arc, Mutex};

use egui::{Color32, Direction, Label, RichText};
use egui_extras::{Size, StripBuilder};
use image::DynamicImage;
use md5::Digest;
use winit::dpi::PhysicalSize;
use wgpu::TextureView;

use crate::models::menu::MenuState;
use crate::views::components::menu::song_select::leaderboard::{Leaderboard, ScoreCard};
use crate::views::components::menu::song_select::song_list::SongList;
use crate::views::components::menu::song_select::beatmap_info::BeatmapInfo;

pub struct CurrentBackground {
    pub image: DynamicImage,
    pub image_hash: md5::Digest,
}

pub struct SongSelectScreen {
    menu_state: Arc<Mutex<MenuState>>,
    song_list: SongList,
    leaderboard: Leaderboard,
    beatmap_info: BeatmapInfo,
    current_background_image: Option<CurrentBackground>,
    current_beatmap_hash: Option<String>,
}

impl SongSelectScreen {
    pub fn new(menu_state: Arc<Mutex<MenuState>>) -> Self {
        Self {
            menu_state: Arc::clone(&menu_state),
            song_list: SongList::new(Arc::clone(&menu_state)),
            leaderboard: Leaderboard::new(),
            beatmap_info: BeatmapInfo::new(),
            current_background_image: None,
            current_beatmap_hash: None,
        }
    }

    pub fn set_scroll_to(&mut self, to: usize) {
        self.song_list.set_scroll_to(to);
    }

    pub fn increment_beatmap(&mut self) {
        self.song_list.increment();
    }

    pub fn decrement_beatmap(&mut self) {
        self.song_list.decrement();
    }

    pub fn set_background(&mut self, image: DynamicImage, md5: Digest) {
        // Do not perform any operations if background is the same
        if let Some(current_background) = &self.current_background_image {
            if current_background.image_hash == md5 {
                return;
            }
        }

        self.current_background_image = Some(CurrentBackground {
            image,
            image_hash: md5,
        });
    }

    pub fn update_leaderboard(&mut self, replays: Vec<crate::database::models::Replay>, note_count_map: std::collections::HashMap<String, i32>) {
        let scores: Vec<ScoreCard> = replays
            .iter()
            .filter_map(|r| {
                let total_notes = note_count_map.get(&r.beatmap_hash).copied().unwrap_or(0) as usize;
                ScoreCard::from_replay(r, total_notes)
            })
            .collect();
        self.leaderboard.update_scores(scores);
    }

    pub fn set_current_beatmap_hash(&mut self, hash: Option<String>) {
        self.current_beatmap_hash = hash;
    }

    pub fn on_resize(&mut self, _new_size: &PhysicalSize<u32>) {
        // Handle resize if needed
    }


    pub fn render(
        &mut self,
        ctx: &egui::Context,
        _view: &TextureView,
        screen_width: f32,
        screen_height: f32,
        hit_window: &crate::models::engine::hit_window::HitWindow,
        hit_window_mode: crate::models::settings::HitWindowMode,
        hit_window_value: f64,
    ) {
        // Update current selection from menu_state
        if let Ok(state) = self.menu_state.lock() {
            self.song_list.set_current(state.selected_index);
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.25)) // Leaderboard on left
                    .size(Size::relative(0.75)) // Song select on right
                    .horizontal(|mut strip| {
                        // Left panel (Beatmap info + Leaderboard)
                        strip.cell(|ui| {
                            // Get selected beatmap data
                            let (beatmapset, beatmap, rate, diff_name) = {
                                if let Ok(state) = self.menu_state.lock() {
                                    if let Some((bs, beatmaps)) = state.beatmapsets.get(state.selected_index) {
                                        let bm = beatmaps.get(state.selected_difficulty_index);
                                        let diff_name = bm.and_then(|bm| bm.difficulty_name.clone());
                                        (Some(bs.clone()), bm.cloned(), state.rate, diff_name)
                                    } else {
                                        (None, None, 1.0, None)
                                    }
                                } else {
                                    (None, None, 1.0, None)
                                }
                            };
                            
                            // Display beatmap info if we have data
                            if let Some(bs) = &beatmapset {
                                self.beatmap_info.render(ui, bs, beatmap.as_ref(), rate, hit_window_mode, hit_window_value);
                                ui.add_space(10.0);
                            }
                            
                            // Leaderboard avec le nom de la difficulté et la hit window actuelle
                            self.leaderboard.render(ui, diff_name.as_deref(), hit_window);
                        });

                        // Song select panel
                        strip.strip(|builder| {
                            builder
                                .size(Size::relative(0.9))
                                .size(Size::relative(0.1))
                                .vertical(|mut strip| {
                                    strip.cell(|ui| {
                                        self.song_list.render(ui);
                                    });

                                    strip.cell(|ui| {
                                        egui::Frame::default()
                                            .corner_radius(5.0)
                                            .outer_margin(10.0)
                                            .inner_margin(5.0)
                                            .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 255))
                                            .show(ui, |ui| {
                                                ui.set_width(ui.available_rect_before_wrap().width());
                                                ui.set_height(ui.available_rect_before_wrap().height());
                                                self.render_beatmap_footer(ui);
                                            });
                                    })
                                });
                        });
                    })
            });
    }


    fn render_beatmap_footer(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
            let beatmap_count = if let Ok(state) = self.menu_state.lock() {
                state.beatmapsets.len()
            } else {
                0
            };
            let text = format!("Beatmaps: {}", beatmap_count);
            ui.add(Label::new(RichText::new(text).heading()).selectable(false));
        });
    }
}


