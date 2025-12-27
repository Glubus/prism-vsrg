use super::layout::{EditorScene, SkinEditorState};
use skin::Skin;
use egui::{ComboBox, DragValue, RichText, Ui};

pub struct AssetBrowser;

impl AssetBrowser {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut Ui, state: &mut SkinEditorState, _skin: &mut Skin) {
        ui.label("Current Scene");
        ComboBox::from_id_salt("scene_selector_right")
            .selected_text(state.current_scene.name())
            .width(ui.available_width())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut state.current_scene, EditorScene::Gameplay, "Gameplay");
                ui.selectable_value(
                    &mut state.current_scene,
                    EditorScene::SongSelect,
                    "Song Select",
                );
                ui.selectable_value(
                    &mut state.current_scene,
                    EditorScene::ResultScreen,
                    "Result Screen",
                );
            });

        // Key count selector (only shown for Gameplay scene)
        if state.current_scene == EditorScene::Gameplay {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Key Count:");
                ui.add(
                    DragValue::new(&mut state.preview_key_count)
                        .speed(0.1)
                        .range(4..=10)
                        .suffix("K"),
                );
            });
        }

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);

        ui.label(RichText::new("Scene Hierarchy").strong());
        egui::ScrollArea::vertical().show(ui, |ui| {
            match state.current_scene {
                EditorScene::Gameplay => {
                    // ========== PLAYFIELD (Note types) ==========
                    ui.collapsing("🎮 Playfield", |ui| {
                        ui.collapsing("🔗 Holds (LN)", |ui| {
                            self.item(ui, state, "Hold - Body");
                            self.item(ui, state, "Hold - End");
                        });
                        ui.collapsing("⚡ Bursts", |ui| {
                            self.item(ui, state, "Burst - Body");
                            self.item(ui, state, "Burst - End");
                        });
                        self.item(ui, state, "💣 Mines");
                    });

                    // ========== PER-COLUMN for current keymode ==========
                    let key_count = state.preview_key_count;
                    ui.collapsing(format!("🎹 {}K Columns", key_count), |ui| {
                        for col in 0..key_count {
                            ui.collapsing(format!("Column {}", col + 1), |ui| {
                                self.item(ui, state, &format!("Col {} - Note", col + 1));
                                self.item(ui, state, &format!("Col {} - Receptor", col + 1));
                            });
                        }
                    });

                    // ========== HUD (Gameplay only) ==========
                    ui.collapsing("📺 HUD", |ui| {
                        self.item(ui, state, "📊 Hit Bar");

                        ui.collapsing("📈 Score & Stats", |ui| {
                            self.item(ui, state, "Score Display");
                            self.item(ui, state, "Combo Counter");
                            self.item(ui, state, "Accuracy");
                            self.item(ui, state, "NPS Display");
                            ui.separator();
                            self.item(ui, state, "Notes Remaining");
                            self.item(ui, state, "Scroll Speed");
                            self.item(ui, state, "Time Left");
                        });

                        ui.collapsing("⚡ Judgement Flash", |ui| {
                            self.item(ui, state, "Flash - All");
                            ui.separator();
                            self.item(ui, state, "Flash - Marvelous");
                            self.item(ui, state, "Flash - Perfect");
                            self.item(ui, state, "Flash - Great");
                            self.item(ui, state, "Flash - Good");
                            self.item(ui, state, "Flash - Bad");
                            self.item(ui, state, "Flash - Miss");
                            self.item(ui, state, "Flash - Ghost Tap");
                        });

                        self.item(ui, state, "📋 Judgement Panel");
                    });
                }

                EditorScene::SongSelect => {
                    // ========== SONG SELECT MENU ==========
                    ui.collapsing("🎵 Song Select", |ui| {
                        self.item(ui, state, "Background");
                        self.item(ui, state, "Song Button");
                        self.item(ui, state, "Song Button Selected");
                        self.item(ui, state, "Difficulty Button");
                        self.item(ui, state, "Search Bar");
                        self.item(ui, state, "Search Panel");
                        self.item(ui, state, "Beatmap Info");
                        self.item(ui, state, "Leaderboard");
                        self.item(ui, state, "🎨 Panel Style");
                    });
                }

                EditorScene::ResultScreen => {
                    // ========== RESULT SCREEN ==========
                    ui.collapsing("🏆 Result Screen", |ui| {
                        self.item(ui, state, "Background");
                        self.item(ui, state, "Score Display");
                        self.item(ui, state, "Accuracy");
                        self.item(ui, state, "📋 Judgement Panel");
                        self.item(ui, state, "Max Combo");
                    });
                }
            }

            // ========== GENERAL (always shown) ==========
            ui.add_space(10.0);
            ui.collapsing("⚙️ General", |ui| {
                self.item(ui, state, "Skin Info");
                self.item(ui, state, "Font");
            });
        });
    }

    fn item(&self, ui: &mut Ui, state: &mut SkinEditorState, id: &str) {
        let display_name = id.trim_start_matches(|c: char| !c.is_alphabetic() && c != '-');
        let is_selected = state.selected_element_id.as_deref() == Some(id);
        if ui.selectable_label(is_selected, display_name).clicked() {
            state.selected_element_id = Some(id.to_string());
        }
    }
}
