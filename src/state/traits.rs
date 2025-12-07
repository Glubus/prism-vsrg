//! Traits for state management.
//!
//! These traits define the common interface for all game states.

use crate::database::DbManager;
use crate::input::events::GameAction;
use crate::models::settings::SettingsState;
use crate::system::bus::SystemBus;

/// Context passed to action handlers with shared resources.
pub struct ActionContext<'a> {
    pub db_manager: &'a mut DbManager,
    pub settings: &'a mut SettingsState,
    pub bus: &'a SystemBus,
}

/// Transition result from handling an action.
#[derive(Debug, Clone)]
pub enum Transition {
    /// Stay in current state.
    None,
    /// Transition to menu state with optional menu state.
    ToMenu,
    /// Transition to gameplay with engine data.
    ToGame,
    /// Transition to editor.
    ToEditor,
    /// Transition to result screen.
    ToResult,
    /// Exit the application.
    Exit,
}

/// Trait for creating render-ready snapshots.
///
/// Snapshots are immutable captures of state sent to the render thread.
/// They decouple game logic from rendering.
pub trait Snapshot {
    /// The snapshot type produced.
    type Output;

    /// Creates an immutable snapshot for rendering.
    fn create_snapshot(&self) -> Self::Output;
}

/// Trait for per-frame updates.
///
/// States that need frame-by-frame updates (like gameplay) implement this.
pub trait Update {
    /// Updates the state for one frame.
    ///
    /// # Arguments
    /// * `dt` - Delta time in seconds since last update.
    fn update(&mut self, dt: f64);
}

/// Trait for handling game actions.
///
/// Each state can handle actions differently and return transitions
/// to other states.
pub trait HandleAction {
    /// Handles a game action and returns any state transition.
    fn handle_action(&mut self, action: &GameAction, ctx: &mut ActionContext) -> Transition;
}
