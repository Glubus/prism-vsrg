//! Trait implementations for GameEngine.

use super::GameEngine;
use crate::shared::snapshot::GameplaySnapshot;
use crate::state::traits::{Snapshot, Update};

// GameEngine implements Snapshot by creating a GameplaySnapshot.
impl Snapshot for GameEngine {
    type Output = GameplaySnapshot;

    fn create_snapshot(&self) -> Self::Output {
        self.get_snapshot()
    }
}

// GameEngine needs per-frame updates for gameplay timing.
impl Update for GameEngine {
    fn update(&mut self, dt: f64) {
        // Delegate to the existing update method
        GameEngine::update(self, dt);
    }
}
