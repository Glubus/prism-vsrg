//! Trait implementations for MenuState.

use super::MenuState;
use crate::state::traits::{Snapshot, Update};

// MenuState implements Snapshot by cloning itself.
// It's already Arc-wrapped for cheap clones.
impl Snapshot for MenuState {
    type Output = MenuState;

    fn create_snapshot(&self) -> Self::Output {
        self.clone()
    }
}

// MenuState doesn't need per-frame updates in the traditional sense.
// Rate cache updates happen on selection, not per-frame.
// We provide a no-op implementation for uniformity.
impl Update for MenuState {
    fn update(&mut self, _dt: f64) {
        // Menu state updates are event-driven, not frame-driven.
        // Rate cache and difficulty calculations happen on-demand.
    }
}
