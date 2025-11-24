use crate::database::{DbManager, DbState};
use crate::menu::MenuState;
use crate::renderer::Renderer;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

/// Convertit un KeyCode en nom de string pour le mapping
fn keycode_to_string(key_code: KeyCode) -> String {
    format!("{:?}", key_code)
}

pub struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    db_manager: Option<DbManager>,
    db_state: Arc<Mutex<DbState>>,
    menu_state: Arc<Mutex<MenuState>>,
}

impl App {
    pub fn new() -> Self {
        // Créer le DbManager avec les chemins
        let db_path = PathBuf::from("maps.db");
        let songs_path = PathBuf::from("songs");
        let db_manager = DbManager::new(db_path, songs_path);
        let db_state = db_manager.get_state();

        Self {
            window: None,
            renderer: None,
            db_manager: Some(db_manager),
            db_state,
            menu_state: Arc::new(Mutex::new(MenuState::new())),
        }
    }

    fn init_database(&mut self) {
        if let Some(ref db_manager) = self.db_manager {
            db_manager.init();
        }
    }

    fn rescan_maps(&mut self) {
        if let Some(ref db_manager) = self.db_manager {
            db_manager.rescan();
        }
    }

    fn update_menu_from_db_state(&mut self) {
        // Mettre à jour le menu_state depuis le db_state
        let db_state_guard = self.db_state.lock().unwrap();
        let beatmapsets = db_state_guard.beatmapsets.clone();
        drop(db_state_guard);

        if let Ok(mut menu_state) = self.menu_state.lock() {
            let lengths_differ = menu_state.beatmapsets.len() != beatmapsets.len();
            let structure_changed =
                if lengths_differ {
                    true
                } else {
                    menu_state.beatmapsets.iter().zip(beatmapsets.iter()).any(
                        |(current, updated)| {
                            current.0.id != updated.0.id || current.1.len() != updated.1.len()
                        },
                    )
                };

            if structure_changed {
                let old_selected = menu_state.selected_index;
                let old_diff = menu_state.selected_difficulty_index;
                menu_state.beatmapsets = beatmapsets;

                // Réinitialiser les index de scroll
                menu_state.start_index = 0;
                menu_state.end_index = menu_state.visible_count.min(menu_state.beatmapsets.len());

                if menu_state.beatmapsets.is_empty() {
                    menu_state.selected_index = 0;
                    menu_state.selected_difficulty_index = 0;
                } else {
                    menu_state.selected_index = old_selected.min(menu_state.beatmapsets.len() - 1);
                    let current_beatmap_count = menu_state
                        .beatmapsets
                        .get(menu_state.selected_index)
                        .map(|(_, beatmaps)| beatmaps.len())
                        .unwrap_or(1)
                        .max(1);
                    menu_state.selected_difficulty_index = old_diff.min(current_beatmap_count - 1);
                }
            }
        }
    }
}

impl ApplicationHandler for App {
    // Appelé quand l'app démarre ou redémarre (Android/iOS/Desktop)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            // Initialiser la base de données
            self.init_database();

            let win_attr = winit::window::Window::default_attributes()
                .with_title("rVsrg - Rust Vertical Scroll Rhythm Game");

            let window = Arc::new(event_loop.create_window(win_attr).unwrap());
            self.window = Some(window.clone());

            // Init WGPU (Async bloquant pour l'exemple, ou utiliser spawn local)
            let menu_state_for_renderer = Arc::clone(&self.menu_state);
            let renderer =
                pollster::block_on(Renderer::new(window.clone(), menu_state_for_renderer));
            self.renderer = Some(renderer);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // Mettre à jour le menu depuis le db_state à chaque frame
        self.update_menu_from_db_state();

        match event {
            WindowEvent::CloseRequested => {
                println!("Shutdown requested...");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(physical_size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    match renderer.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            // TODO: Reconfigure surface
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                    // Demande immédiate de la prochaine frame (boucle infinie fluide)
                    self.window.as_ref().unwrap().request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(key_code),
                        ..
                    },
                ..
            } => {
                // Vérifier si on est dans le menu
                let in_menu = {
                    if let Some(renderer) = &self.renderer {
                        if let Ok(menu_state) = renderer.menu_state.lock() {
                            menu_state.in_menu
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };

                if in_menu {
                    match key_code {
                        KeyCode::Escape => {
                            event_loop.exit();
                        }
                        KeyCode::F8 => {
                            // Rescan des maps
                            self.rescan_maps();
                            // Le menu_state sera mis à jour par rescan_maps
                        }
                        KeyCode::ArrowUp => {
                            if let Some(renderer) = &self.renderer {
                                if let Ok(mut menu_state) = renderer.menu_state.lock() {
                                    menu_state.move_up();
                                }
                            }
                        }
                        KeyCode::ArrowDown => {
                            if let Some(renderer) = &self.renderer {
                                if let Ok(mut menu_state) = renderer.menu_state.lock() {
                                    menu_state.move_down();
                                }
                            }
                        }
                        KeyCode::ArrowLeft => {
                            if let Some(renderer) = &self.renderer {
                                if let Ok(mut menu_state) = renderer.menu_state.lock() {
                                    menu_state.previous_difficulty();
                                }
                            }
                        }
                        KeyCode::ArrowRight => {
                            if let Some(renderer) = &self.renderer {
                                if let Ok(mut menu_state) = renderer.menu_state.lock() {
                                    menu_state.next_difficulty();
                                }
                            }
                        }
                        KeyCode::Enter | KeyCode::NumpadEnter => {
                            // Charger la map sélectionnée
                            if let Some(renderer) = &mut self.renderer {
                                let map_path = {
                                    if let Ok(menu_state) = renderer.menu_state.lock() {
                                        menu_state.get_selected_beatmap_path()
                                    } else {
                                        None
                                    }
                                };

                                if let Some(path) = map_path {
                                    if let Ok(mut menu_state) = renderer.menu_state.lock() {
                                        menu_state.in_menu = false;
                                    }
                                    renderer.load_map(path);
                                }
                            }
                        }
                        KeyCode::PageUp => {
                            // Increase rate
                            if let Some(renderer) = &self.renderer {
                                if let Ok(mut menu_state) = renderer.menu_state.lock() {
                                    menu_state.increase_rate();
                                    println!("Rate: {:.1}x", menu_state.rate);
                                }
                            }
                        }
                        KeyCode::PageDown => {
                            // Decrease rate
                            if let Some(renderer) = &self.renderer {
                                if let Ok(mut menu_state) = renderer.menu_state.lock() {
                                    menu_state.decrease_rate();
                                    println!("Rate: {:.1}x", menu_state.rate);
                                }
                            }
                        }
                        _ => {}
                    }
                    return;
                }

                // Si on n'est pas dans le menu, gérer les touches du gameplay
                match key_code {
                    KeyCode::Escape => {
                        // Retour au menu
                        if let Some(renderer) = &mut self.renderer {
                            renderer.stop_audio(); // Arrêter la musique
                            if let Ok(mut menu_state) = renderer.menu_state.lock() {
                                menu_state.in_menu = true;
                            }
                        }
                    }
                    KeyCode::F3 => {
                        // Diminuer la vitesse de défilement
                        if let Some(renderer) = &mut self.renderer {
                            renderer.engine.scroll_speed_ms =
                                (renderer.engine.scroll_speed_ms - 50.0).max(100.0);
                            println!("Scroll speed: {:.1} ms", renderer.engine.scroll_speed_ms);
                        }
                    }
                    KeyCode::F4 => {
                        // Augmenter la vitesse de défilement
                        if let Some(renderer) = &mut self.renderer {
                            renderer.engine.scroll_speed_ms =
                                (renderer.engine.scroll_speed_ms + 50.0).min(2000.0);
                            println!("Scroll speed: {:.1} ms", renderer.engine.scroll_speed_ms);
                        }
                    }
                    KeyCode::F5 => {
                        // Relancer la map depuis le début
                        if let Some(renderer) = &mut self.renderer {
                            renderer.engine.reset_time();
                            println!("Map restarted from the beginning");
                        }
                    }
                    KeyCode::F8 => {
                        // Rescan des maps (même en gameplay)
                        self.rescan_maps();
                    }
                    KeyCode::F11 => {
                        // Réduire la taille des notes et receptors
                        if let Some(renderer) = &mut self.renderer {
                            renderer.decrease_note_size();
                        }
                    }
                    KeyCode::F12 => {
                        // Augmenter la taille des notes et receptors
                        if let Some(renderer) = &mut self.renderer {
                            renderer.increase_note_size();
                        }
                    }
                    _ => {
                        // Utiliser le mapping de touches du skin
                        if let Some(renderer) = &mut self.renderer {
                            let key_name = keycode_to_string(key_code);
                            if let Some(column) = renderer.skin.get_column_for_key(&key_name) {
                                if let Some(judgement) = renderer.engine.process_input(column) {
                                    println!(
                                        "Hit column {} ({}): {:?}",
                                        column, key_name, judgement
                                    );
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
