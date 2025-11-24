use crate::database::{DbManager, DbState};
use crate::models::menu::MenuState;
use crate::renderer::Renderer;
use crate::states::{GameState, MenuStateController, StateContext, StateTransition};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey, ModifiersState};
use winit::window::{Window, WindowId};

pub struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    db_manager: DbManager,
    db_state: Arc<Mutex<DbState>>,
    menu_state: Arc<Mutex<MenuState>>,
    state_stack: Vec<Box<dyn GameState>>,
    modifiers: ModifiersState, // Pour traquer CTRL
}

impl App {
    pub fn new() -> Self {
        let db_path = PathBuf::from("main.db");
        let songs_path = PathBuf::from("songs");
        let db_manager = DbManager::new(db_path, songs_path);
        let db_state = db_manager.get_state();
        let menu_state = Arc::new(Mutex::new(MenuState::new()));

        let mut app = Self {
            window: None,
            renderer: None,
            db_manager,
            db_state,
            menu_state: Arc::clone(&menu_state),
            state_stack: Vec::new(),
            modifiers: ModifiersState::default(),
        };

        app.enter_state(Box::new(MenuStateController::new(menu_state)));
        app
    }

    fn init_database(&mut self) {
        self.db_manager.init();
    }

    fn update_menu_from_db_state(&mut self) {
        let db_state_guard = self.db_state.lock().unwrap();
        let beatmapsets = db_state_guard.beatmapsets.clone();
        drop(db_state_guard);

        if let Ok(mut menu_state) = self.menu_state.lock() {
            let lengths_differ = menu_state.beatmapsets.len() != beatmapsets.len();
            let structure_changed = if lengths_differ {
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

    fn make_state_context(&mut self) -> StateContext {
        let renderer_ptr = self
            .renderer
            .as_mut()
            .map(|renderer| renderer as *mut Renderer);
        let db_manager_ptr = Some(&mut self.db_manager as *mut DbManager);
        StateContext::new(renderer_ptr, db_manager_ptr)
    }

    fn enter_state(&mut self, mut state: Box<dyn GameState>) {
        let mut ctx = self.make_state_context();
        state.on_enter(&mut ctx);
        self.state_stack.push(state);
    }

    fn exit_state(&mut self) {
        if let Some(mut state) = self.state_stack.pop() {
            let mut ctx = self.make_state_context();
            state.on_exit(&mut ctx);
        }
    }

    fn replace_state(&mut self, mut state: Box<dyn GameState>) {
        if let Some(mut current) = self.state_stack.pop() {
            let mut ctx = self.make_state_context();
            current.on_exit(&mut ctx);
        }
        let mut ctx = self.make_state_context();
        state.on_enter(&mut ctx);
        self.state_stack.push(state);
    }

    fn with_active_state<F>(&mut self, f: F) -> StateTransition
    where
        F: FnOnce(&mut dyn GameState, &mut StateContext) -> StateTransition,
    {
        if self.state_stack.is_empty() {
            return StateTransition::None;
        }

        let mut ctx = self.make_state_context();
        if let Some(state) = self.state_stack.last_mut() {
            f(state.as_mut(), &mut ctx)
        } else {
            StateTransition::None
        }
    }

    fn apply_transition(&mut self, transition: StateTransition, event_loop: &ActiveEventLoop) {
        match transition {
            StateTransition::None => {}
            StateTransition::Push(state) => self.enter_state(state),
            StateTransition::Pop => self.exit_state(),
            StateTransition::Replace(state) => self.replace_state(state),
            StateTransition::Exit => event_loop.exit(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.init_database();
            let win_attr = winit::window::Window::default_attributes()
                .with_title("rVsrg - Rust Vertical Scroll Rhythm Game");

            let window = Arc::new(event_loop.create_window(win_attr).unwrap());
            self.window = Some(window.clone());

            let menu_state_for_renderer = Arc::clone(&self.menu_state);
            let renderer =
                pollster::block_on(Renderer::new(window.clone(), menu_state_for_renderer));
            self.renderer = Some(renderer);

            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.update_menu_from_db_state();

        // 1. Gestion EGUI (Inputs Souris/Clavier UI)
        if let (Some(renderer), Some(window)) = (self.renderer.as_mut(), self.window.as_ref()) {
            // On passe l'événement à egui
            renderer.handle_event(window, &event);

            // Si egui veut l'input (ex: on tape dans une textbox ou on clique sur une fenêtre),
            // on ne propage pas l'événement au jeu.
            // Note: wants_keyboard_input renvoie true si le focus est dans un widget texte
            // wants_pointer_input renvoie true si la souris est sur une fenêtre egui
            let egui_wants_keyboard = renderer.egui_ctx.wants_keyboard_input();
            let egui_wants_pointer = renderer.egui_ctx.wants_pointer_input();

            // Si c'est un input clavier et qu'egui le veut, on return
            if let WindowEvent::KeyboardInput { .. } = event {
                if egui_wants_keyboard {
                    return; 
                }
            }
            // Idem pour la souris
            if let WindowEvent::CursorMoved { .. } | WindowEvent::MouseInput { .. } = event {
                if egui_wants_pointer {
                    return;
                }
            }
        }

        // 2. Traitement des événements globaux
        match &event {
            WindowEvent::CloseRequested => {
                println!("Shutdown requested...");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize(*physical_size);
                }
            }
            WindowEvent::ModifiersChanged(new_modifiers) => {
                self.modifiers = new_modifiers.state();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(key_code),
                        repeat: false,
                        ..
                    },
                ..
            } => {
                // Gestion Raccourcis Globaux (Settings)
                if *key_code == KeyCode::KeyO && self.modifiers.control_key() {
                    let allow_toggle = self
                        .menu_state
                        .lock()
                        .map(|state| state.in_menu)
                        .unwrap_or(true);
                    if allow_toggle {
                    if let Some(renderer) = self.renderer.as_mut() {
                        renderer.toggle_settings();
                    }
                    // On return pour ne pas que le 'O' soit aussi traité par le jeu (optionnel)
                    return;
                    }
                    // En gameplay, Ctrl+O est ignoré pour éviter des collisions de raccourcis
                }

                // Propagation au jeu (States)
                let transition =
                    self.with_active_state(|state, ctx| state.handle_input(&event, ctx));
                self.apply_transition(transition, event_loop);
            }
            WindowEvent::RedrawRequested => {
                let transition = self.with_active_state(|state, ctx| match state.update(ctx) {
                    StateTransition::None => state.render(ctx),
                    other => other,
                });
                self.apply_transition(transition, event_loop);

                if let (Some(renderer), Some(window)) = (self.renderer.as_mut(), self.window.as_ref()) {
                    // On passe la window au render
                    match renderer.render(window) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            // TODO: Reconfigure surface
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                    window.request_redraw();
                }
            }
            _ => {
                // Pour les autres events clavier (release, repeat)
                if let WindowEvent::KeyboardInput { .. } = event {
                     let transition =
                        self.with_active_state(|state, ctx| state.handle_input(&event, ctx));
                    self.apply_transition(transition, event_loop);
                }
            }
        }
    }
}