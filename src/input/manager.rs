use std::collections::HashMap;
use winit::event::ElementState;
use winit::keyboard::KeyCode;
use super::events::{RawInputEvent, GameAction, EditorTarget};

pub struct InputManager {
    bindings: HashMap<KeyCode, GameAction>,
}

impl InputManager {
    pub fn new() -> Self {
        let mut manager = Self {
            bindings: HashMap::new(),
        };
        manager.load_default_bindings();
        manager
    }

    pub fn process(&mut self, event: RawInputEvent) -> Option<GameAction> {
        if let Some(&base_action) = self.bindings.get(&event.keycode) {
            match (event.state, base_action) {
                (ElementState::Pressed, GameAction::Hit { column }) => Some(GameAction::Hit { column }),
                (ElementState::Released, GameAction::Hit { column }) => Some(GameAction::Release { column }),
                (ElementState::Pressed, action) => Some(action),
                _ => None,
            }
        } else {
            match (event.state, event.keycode) {
                (ElementState::Pressed, KeyCode::Escape) => Some(GameAction::Back),
                (ElementState::Pressed, KeyCode::Enter) => Some(GameAction::Confirm),
                _ => None
            }
        }
    }

    fn load_default_bindings(&mut self) {
        // Gameplay 4K
        self.bindings.insert(KeyCode::KeyD, GameAction::Hit { column: 0 });
        self.bindings.insert(KeyCode::KeyF, GameAction::Hit { column: 1 });
        self.bindings.insert(KeyCode::KeyJ, GameAction::Hit { column: 2 });
        self.bindings.insert(KeyCode::KeyK, GameAction::Hit { column: 3 });
        self.bindings.insert(KeyCode::F5, GameAction::Restart);

        // Navigation UI (Sert aussi pour l'éditeur)
        self.bindings.insert(KeyCode::ArrowUp, GameAction::Navigation { x: 0, y: -1 });
        self.bindings.insert(KeyCode::ArrowDown, GameAction::Navigation { x: 0, y: 1 });
        self.bindings.insert(KeyCode::ArrowLeft, GameAction::Navigation { x: -1, y: 0 });
        self.bindings.insert(KeyCode::ArrowRight, GameAction::Navigation { x: 1, y: 0 });
        
        // Onglets / Settings
        self.bindings.insert(KeyCode::PageUp, GameAction::TabPrev);
        self.bindings.insert(KeyCode::PageDown, GameAction::TabNext);
        self.bindings.insert(KeyCode::KeyO, GameAction::ToggleSettings);
        
        // System / DB
        self.bindings.insert(KeyCode::KeyE, GameAction::ToggleEditor); // F2 ou E
        self.bindings.insert(KeyCode::F2, GameAction::ToggleEditor);
        self.bindings.insert(KeyCode::F8, GameAction::Rescan);
        
        // Editor Selection Shortcuts
        self.bindings.insert(KeyCode::KeyW, GameAction::EditorSelect(EditorTarget::Notes));
        self.bindings.insert(KeyCode::KeyX, GameAction::EditorSelect(EditorTarget::Receptors));
        self.bindings.insert(KeyCode::KeyC, GameAction::EditorSelect(EditorTarget::Combo));
        self.bindings.insert(KeyCode::KeyV, GameAction::EditorSelect(EditorTarget::Score));
        self.bindings.insert(KeyCode::KeyB, GameAction::EditorSelect(EditorTarget::Accuracy));
        self.bindings.insert(KeyCode::KeyN, GameAction::EditorSelect(EditorTarget::Judgement));
        self.bindings.insert(KeyCode::KeyK, GameAction::EditorSelect(EditorTarget::HitBar));
        self.bindings.insert(KeyCode::KeyL, GameAction::EditorSelect(EditorTarget::Lanes));
        self.bindings.insert(KeyCode::KeyS, GameAction::EditorSave);
    }
}