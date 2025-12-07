//! Traits for state management.
//!
//! These traits define the common interface for all game states.

use crate::database::DbManager;
use crate::input::events::GameAction;
use crate::models::settings::SettingsState;
use crate::system::bus::SystemBus;

/// Context passed to action handlers with shared resources.
pub struct ActionContext<'a> {
    pub db_manager: &'a DbManager,
    pub settings: &'a SettingsState,
    pub bus: &'a SystemBus,
}

/// Transition result from handling an action.
pub enum Transition {
    /// Stay in current state.
    None,
    /// Transition to menu state.
    ToMenu,
    /// Transition to gameplay.
    ToGame,
    /// Transition to editor.
    ToEditor,
    /// Transition to result screen.
    ToResult,
    /// Exit the application.
    Exit,
}

/// Trait for creating render-ready snapshots.
pub trait Snapshot {
    /// The snapshot type produced.
    type Output;

    /// Creates an immutable snapshot for rendering.
    fn create_snapshot(&self) -> Self::Output;
}

/// Trait for per-frame updates.
pub trait Update {
    /// Updates the state for one frame.
    fn update(&mut self, dt: f64);
}

/// Trait for handling game actions.
pub trait HandleAction {
    /// Handles a game action and returns any state transition.
    fn handle_action(&mut self, action: &GameAction, ctx: &mut ActionContext) -> Transition;
}
