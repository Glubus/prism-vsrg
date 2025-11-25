
use crate::models::engine::NUM_COLUMNS;
use crate::renderer::Renderer;
use egui_wgpu::ScreenDescriptor;
use wgpu::{CommandBuffer, CommandEncoderDescriptor, TextureView};

impl Renderer {
    pub fn update_ui(
        &mut self,
        window: &winit::window::Window,
        view: &TextureView,
    ) -> (Vec<egui::ClippedPrimitive>, egui::TexturesDelta) {
        let raw_input = self.egui_state.take_egui_input(window);

        // ... (Logique de remapping inchangée) ...
        let mut captured_key: Option<String> = None;
        if self.settings.remapping_column.is_some() {
            for event in &raw_input.events {
                if let egui::Event::Text(text) = event {
                    if !text.is_empty() {
                        captured_key = Some(text.chars().next().unwrap().to_string());
                        break;
                    }
                }
            }
            if captured_key.is_none() {
                for event in &raw_input.events {
                    if let egui::Event::Key {
                        key, pressed: true, ..
                    } = event
                    {
                        captured_key = Some(key.name().to_string());
                        break;
                    }
                }
            }
        }

        let mut settings_is_open = self.settings.is_open;
        let mut settings_show_keybindings = self.settings.show_keybindings;
        let mut remapping_column = self.settings.remapping_column;
        let mut master_volume = self.settings.master_volume;
        let mut hit_window_mode = self.settings.hit_window_mode;
        let mut hit_window_value = self.settings.hit_window_value;
        let mut aspect_ratio_mode = self.settings.aspect_ratio_mode;

        let keybinding_rows: Vec<(usize, String)> = (0..NUM_COLUMNS)
            .map(|col| {
                let key = self
                    .skin
                    .key_to_column
                    .iter()
                    .find(|(_, c)| **c == col)
                    .map(|(k, _)| k.clone())
                    .unwrap_or_else(|| "None".to_string());
                (col, key)
            })
            .collect();

        let (in_menu, show_result, last_result) = if let Ok(menu_state) = self.menu_state.lock() {
            (
                menu_state.in_menu,
                menu_state.show_result,
                menu_state.last_result.clone(),
            )
        } else {
            (false, false, None)
        };

        if in_menu && self.song_select_screen.is_none() {
            self.song_select_screen = Some(
                crate::views::components::menu::song_select::SongSelectScreen::new(
                    std::sync::Arc::clone(&self.menu_state),
                ),
            );
        }

        if show_result && self.result_screen.is_none() {
            self.result_screen =
                Some(crate::views::components::menu::result_screen::ResultScreen::new());
        }

        let btn_tex = self.song_button_texture.as_ref().map(|t| t.id());
        let btn_sel_tex = self.song_button_selected_texture.as_ref().map(|t| t.id());
        let diff_tex = self.difficulty_button_texture.as_ref().map(|t| t.id());
        let diff_sel_tex = self
            .difficulty_button_selected_texture
            .as_ref()
            .map(|t| t.id());

        // 1. Couleur Musique (Song)
        let sel_col_array = self.skin.get_selected_color();
        let song_selected_color = egui::Color32::from_rgba_unmultiplied(
            (sel_col_array[0] * 255.0) as u8,
            (sel_col_array[1] * 255.0) as u8,
            (sel_col_array[2] * 255.0) as u8,
            (sel_col_array[3] * 255.0) as u8,
        );

        // 2. Couleur Difficulté
        let diff_col_array = self.skin.get_difficulty_selected_color();
        let difficulty_selected_color = egui::Color32::from_rgba_unmultiplied(
            (diff_col_array[0] * 255.0) as u8,
            (diff_col_array[1] * 255.0) as u8,
            (diff_col_array[2] * 255.0) as u8,
            (diff_col_array[3] * 255.0) as u8,
        );

        let egui_ctx = std::mem::take(&mut self.egui_ctx);
        let full_output = egui_ctx.run(raw_input, |ctx| {
            if show_result {
                if let Some(data) = &last_result {
                    let current_hit_window = match hit_window_mode {
                        crate::models::settings::HitWindowMode::OsuOD => {
                            crate::models::engine::hit_window::HitWindow::from_osu_od(
                                hit_window_value,
                            )
                        }
                        crate::models::settings::HitWindowMode::EtternaJudge => {
                            crate::models::engine::hit_window::HitWindow::from_etterna_judge(
                                hit_window_value as u8,
                            )
                        }
                    };

                    if let Some(ref mut screen) = self.result_screen {
                        let should_close = screen.render(ctx, data, &current_hit_window);
                        if should_close {
                            if let Ok(mut state) = self.menu_state.lock() {
                                state.should_close_result = true;
                            }
                        }
                    }
                }
            } else if in_menu {
                if !self.leaderboard_scores_loaded {
                    self.load_leaderboard_scores();
                }

                let current_hit_window = match hit_window_mode {
                    crate::models::settings::HitWindowMode::OsuOD => {
                        crate::models::engine::hit_window::HitWindow::from_osu_od(hit_window_value)
                    }
                    crate::models::settings::HitWindowMode::EtternaJudge => {
                        crate::models::engine::hit_window::HitWindow::from_etterna_judge(
                            hit_window_value as u8,
                        )
                    }
                };

                if let Some(ref mut song_select) = self.song_select_screen {
                    song_select.render(
                        ctx,
                        view,
                        self.config.width as f32,
                        self.config.height as f32,
                        &current_hit_window,
                        hit_window_mode,
                        hit_window_value,
                        btn_tex,
                        btn_sel_tex,
                        diff_tex,
                        diff_sel_tex,
                        song_selected_color,       // Couleur 1
                        difficulty_selected_color, // Couleur 2
                    );
                }
            }

            if settings_is_open {
                egui::SidePanel::left("settings_panel")
                    .resizable(false)
                    .default_width(250.0)
                    .show(ctx, |ui| {
                        ui.heading("Settings");
                        ui.separator();

                        ui.label("Audio");
                        if ui
                            .add(egui::Slider::new(&mut master_volume, 0.0..=1.0).text("Volume"))
                            .changed()
                        {
                            self.engine.set_volume(master_volume);
                        }

                        ui.separator();
                        ui.label("Display");
                        ui.horizontal(|ui| {
                            ui.label("Aspect Ratio:");
                            egui::ComboBox::from_id_salt("aspect_ratio_combo")
                                .selected_text(match aspect_ratio_mode {
                                    crate::models::settings::AspectRatioMode::Auto => "Auto",
                                    crate::models::settings::AspectRatioMode::Ratio16_9 => "16:9",
                                    crate::models::settings::AspectRatioMode::Ratio4_3 => "4:3",
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut aspect_ratio_mode,
                                        crate::models::settings::AspectRatioMode::Auto,
                                        "Auto (Window)",
                                    );
                                    ui.selectable_value(
                                        &mut aspect_ratio_mode,
                                        crate::models::settings::AspectRatioMode::Ratio16_9,
                                        "16:9",
                                    );
                                    ui.selectable_value(
                                        &mut aspect_ratio_mode,
                                        crate::models::settings::AspectRatioMode::Ratio4_3,
                                        "4:3",
                                    );
                                });
                        });

                        ui.separator();
                        ui.label("Gameplay");

                        ui.horizontal(|ui| {
                            ui.label("Rate:");
                            let current_rate = if let Ok(menu_state) = self.menu_state.lock() {
                                menu_state.rate
                            } else {
                                1.0
                            };
                            ui.label(format!("{:.1}x", current_rate));
                            if ui.button("−").clicked() {
                                if let Ok(mut menu_state) = self.menu_state.lock() {
                                    menu_state.decrease_rate();
                                }
                            }
                            if ui.button("+").clicked() {
                                if let Ok(mut menu_state) = self.menu_state.lock() {
                                    menu_state.increase_rate();
                                }
                            }
                        });

                        ui.add_space(10.0);
                        ui.label("Hit Window");
                        ui.horizontal(|ui| {
                            ui.radio_value(
                                &mut hit_window_mode,
                                crate::models::settings::HitWindowMode::OsuOD,
                                "OD",
                            );
                            ui.radio_value(
                                &mut hit_window_mode,
                                crate::models::settings::HitWindowMode::EtternaJudge,
                                "Judge",
                            );
                        });

                        let (min_val, max_val, label) = match hit_window_mode {
                            crate::models::settings::HitWindowMode::OsuOD => (0.0, 10.0, "OD"),
                            crate::models::settings::HitWindowMode::EtternaJudge => {
                                (1.0, 9.0, "Judge Level")
                            }
                        };

                        if ui
                            .add(
                                egui::Slider::new(&mut hit_window_value, min_val..=max_val)
                                    .text(label),
                            )
                            .changed()
                        {
                            self.engine
                                .update_hit_window(hit_window_mode, hit_window_value);
                        }

                        ui.separator();
                        ui.label("Controls");
                        if ui.button("Remap Keys").clicked() {
                            settings_show_keybindings = true;
                        }

                        ui.add_space(20.0);
                        if ui.button("Close (Ctrl+O)").clicked() {
                            settings_is_open = false;
                        }
                    });
            }

            if settings_show_keybindings {
                egui::Window::new("Key Bindings")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        if keybinding_rows.is_empty() {
                            ui.label("No key bindings declared in the current skin.");
                        } else {
                            if let Some(col) = remapping_column {
                                ui.label(format!("Press a key for Column {}...", col + 1));
                                if let Some(key_name) = &captured_key {
                                    self.skin.key_to_column.retain(|_, &mut c| c != col);
                                    self.skin.key_to_column.retain(|k, _| k != key_name);
                                    self.skin.key_to_column.insert(key_name.clone(), col);
                                    remapping_column = None;
                                }
                                ui.add_space(10.0);
                                if ui.button("Cancel").clicked() {
                                    remapping_column = None;
                                }
                            } else {
                                egui::Grid::new("keybinds_grid")
                                    .striped(true)
                                    .show(ui, |ui| {
                                        for (column, key) in keybinding_rows.iter() {
                                            ui.label(format!("Column {}", column + 1));
                                            if ui.button(key).clicked() {
                                                remapping_column = Some(*column);
                                            }
                                            ui.end_row();
                                        }
                                    });
                            }
                        }
                        ui.add_space(10.0);
                        if ui.button("Done").clicked() {
                            settings_show_keybindings = false;
                            remapping_column = None;
                        }
                    });
            }
        });

        if aspect_ratio_mode != self.settings.aspect_ratio_mode {
            self.settings.aspect_ratio_mode = aspect_ratio_mode;
            self.update_pixel_system_ratio();
        }

        self.settings.is_open = settings_is_open;
        self.settings.show_keybindings = settings_show_keybindings;
        self.settings.remapping_column = remapping_column;
        self.settings.master_volume = master_volume;
        self.settings.hit_window_mode = hit_window_mode;
        self.settings.hit_window_value = hit_window_value;

        self.egui_ctx = egui_ctx;

        let tris = self
            .egui_ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);
        (tris, full_output.textures_delta)
    }

    pub fn render_ui_layer(
        &mut self,
        view: &TextureView,
        tris: &[egui::ClippedPrimitive],
        textures_delta: &egui::TexturesDelta,
        window: &winit::window::Window,
    ) -> CommandBuffer {
        let mut egui_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Egui Render Encoder"),
            });

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: window.scale_factor() as f32,
        };

        for (id, image) in &textures_delta.set {
            self.egui_renderer
                .update_texture(&self.device, &self.queue, *id, image);
        }

        self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut egui_encoder,
            tris,
            &screen_descriptor,
        );

        {
            let mut rpass = egui_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Egui Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            let rpass_static = unsafe {
                std::mem::transmute::<&mut wgpu::RenderPass<'_>, &mut wgpu::RenderPass<'static>>(
                    &mut rpass,
                )
            };

            self.egui_renderer
                .render(rpass_static, tris, &screen_descriptor);
        }

        for id in &textures_delta.free {
            self.egui_renderer.free_texture(id);
        }

        egui_encoder.finish()
    }
}
