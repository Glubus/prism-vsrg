use egui::{Button, Color32, Frame, Margin, RichText, Slider, Ui};

use crate::models::menu::MenuState;
use crate::models::search::MenuSearchFilters;

pub enum SearchPanelEvent {
    None,
    Apply(MenuSearchFilters),
}

pub struct SearchPanel {
    form_filters: MenuSearchFilters,
    form_dirty: bool,
}

impl SearchPanel {
    pub fn new() -> Self {
        Self {
            form_filters: MenuSearchFilters::default(),
            form_dirty: false,
        }
    }

    pub fn render(&mut self, ui: &mut Ui, menu_state: &MenuState) -> SearchPanelEvent {
        if !self.form_dirty && self.form_filters != menu_state.search_filters {
            self.form_filters = menu_state.search_filters.clone();
        }

        let mut event = SearchPanelEvent::None;

        Frame::default()
            .corner_radius(5.0)
            .outer_margin(Margin::symmetric(0, 4))
            .inner_margin(Margin::same(10))
            .fill(Color32::from_rgba_unmultiplied(20, 20, 20, 220))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Search");
                    if menu_state.search_filters.is_active() {
                        ui.label(
                            RichText::new("Filtres actifs")
                                .size(14.0)
                                .color(Color32::from_rgba_unmultiplied(120, 200, 255, 255)),
                        );
                    }
                });

                ui.add_space(6.0);

                if ui
                    .text_edit_singleline(&mut self.form_filters.query)
                    .changed()
                {
                    self.form_dirty = true;
                }

                ui.add_space(6.0);
                self.render_rating_filter(ui);
                self.render_duration_filter(ui);

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    let apply_enabled = self.form_dirty;
                    let apply_button = ui.add_enabled(apply_enabled, Button::new("Appliquer"));
                    if apply_button.clicked() {
                        self.form_dirty = false;
                        event = SearchPanelEvent::Apply(self.form_filters.clone());
                    }

                    if ui.button("Réinitialiser").clicked() {
                        self.form_filters = MenuSearchFilters::default();
                        self.form_dirty = false;
                        event = SearchPanelEvent::Apply(self.form_filters.clone());
                    }
                });

                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("Résultats: {}", menu_state.beatmapsets.len()))
                        .size(14.0),
                );
            });

        event
    }

    fn render_rating_filter(&mut self, ui: &mut Ui) {
        let mut enabled = self.form_filters.min_rating.is_some();
        ui.horizontal(|ui| {
            if ui.checkbox(&mut enabled, "Min rating (Etterna)").changed() {
                if enabled {
                    self.form_filters.min_rating =
                        Some(self.form_filters.min_rating.unwrap_or(20.0));
                } else {
                    self.form_filters.min_rating = None;
                }
                self.form_dirty = true;
            }

            if enabled {
                let mut value = self.form_filters.min_rating.unwrap_or(20.0) as f32;
                if ui
                    .add(Slider::new(&mut value, 0.0..=30.0).suffix(" MSD"))
                    .changed()
                {
                    self.form_filters.min_rating = Some(value as f64);
                    self.form_dirty = true;
                }
            }
        });
    }

    fn render_duration_filter(&mut self, ui: &mut Ui) {
        let mut enabled = self.form_filters.max_duration_seconds.is_some();
        ui.horizontal(|ui| {
            if ui.checkbox(&mut enabled, "Durée max (secondes)").changed() {
                if enabled {
                    self.form_filters.max_duration_seconds =
                        Some(self.form_filters.max_duration_seconds.unwrap_or(180.0));
                } else {
                    self.form_filters.max_duration_seconds = None;
                }
                self.form_dirty = true;
            }

            if enabled {
                let mut duration = self.form_filters.max_duration_seconds.unwrap_or(180.0) as f32;
                if ui
                    .add(Slider::new(&mut duration, 30.0..=600.0).suffix("s"))
                    .changed()
                {
                    self.form_filters.max_duration_seconds = Some(duration as f64);
                    self.form_dirty = true;
                }
            }
        });
    }
}
