use super::{GameState, MenuStateController, StateContext, StateTransition};
use crate::models::menu::MenuState;
use crate::models::stats::HitStats;
use crate::models::replay::ReplayData;
use std::sync::{Arc, Mutex};
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct ResultStateController {
    menu_state: Arc<Mutex<MenuState>>,
    hit_stats: HitStats,
    replay_data: ReplayData,
    score: u32,
    accuracy: f64,
    max_combo: u32,
}

impl ResultStateController {
    pub fn new(
        menu_state: Arc<Mutex<MenuState>>,
        hit_stats: HitStats,
        replay_data: ReplayData,
        score: u32,
        accuracy: f64,
        max_combo: u32,
    ) -> Self {
        Self {
            menu_state,
            hit_stats,
            replay_data,
            score,
            accuracy,
            max_combo,
        }
    }

    fn with_menu_state<F>(&self, mut f: F)
    where
        F: FnMut(&mut MenuState),
    {
        if let Ok(mut state) = self.menu_state.lock() {
            f(&mut state);
        }
    }
}

impl GameState for ResultStateController {
    fn on_enter(&mut self, _ctx: &mut StateContext) {
        self.with_menu_state(|state| {
            state.in_menu = true;
            state.show_result = true;
        });
    }

    fn on_exit(&mut self, _ctx: &mut StateContext) {
        self.with_menu_state(|state| {
            state.show_result = false;
        });
    }

    fn handle_input(&mut self, event: &WindowEvent, _ctx: &mut StateContext) -> StateTransition {
        if let WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(key_code),
                    ..
                },
            ..
        } = event
        {
            match key_code {
                KeyCode::Escape | KeyCode::Enter | KeyCode::NumpadEnter => {
                    // Retour au menu
                    return StateTransition::Replace(Box::new(MenuStateController::new(
                        Arc::clone(&self.menu_state),
                    )));
                }
                _ => {}
            }
        }
        StateTransition::None
    }
}

