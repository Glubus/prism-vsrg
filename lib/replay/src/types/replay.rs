//! Replay data structure - the main replay container.

use super::input::ReplayInput;
use serde::{Deserialize, Serialize};

/// Current replay format version for compatibility.
pub const REPLAY_FORMAT_VERSION: u8 = 5;

/// Minimum interval between checkpoints (in µs).
pub const CHECKPOINT_MIN_INTERVAL_US: i64 = 15_000_000; // 15 seconds

/// Minimal replay data containing only raw inputs.
///
/// Hit windows are NOT stored - they are applied server-side during
/// score calculation, allowing replays to be re-judged with different
/// timing parameters.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct ReplayData {
    /// Format version for future compatibility.
    pub version: u8,
    /// All user inputs in chronological order.
    pub inputs: Vec<ReplayInput>,
    /// Playback rate used during the play.
    pub rate: f64,
    /// Whether practice mode was enabled.
    #[serde(default)]
    pub is_practice_mode: bool,
    /// Checkpoints placed by the user (timestamps in µs).
    #[serde(default)]
    pub checkpoints: Vec<i64>,
}

impl ReplayData {
    /// Creates a new replay data structure.
    pub fn new(rate: f64) -> Self {
        Self {
            version: REPLAY_FORMAT_VERSION,
            inputs: Vec::new(),
            rate,
            is_practice_mode: false,
            checkpoints: Vec::new(),
        }
    }

    /// Creates a new replay data structure in practice mode.
    pub fn new_practice(rate: f64) -> Self {
        let mut data = Self::new(rate);
        data.is_practice_mode = true;
        data
    }

    /// Adds a checkpoint if the minimum interval is respected.
    pub fn add_checkpoint(&mut self, time_us: i64) -> bool {
        if let Some(&last) = self.checkpoints.last() {
            if time_us - last < CHECKPOINT_MIN_INTERVAL_US {
                return false;
            }
        }
        self.checkpoints.push(time_us);
        true
    }

    /// Returns the last checkpoint timestamp, if any.
    pub fn get_last_checkpoint(&self) -> Option<i64> {
        self.checkpoints.last().copied()
    }

    /// Removes all inputs after the given timestamp.
    pub fn truncate_inputs_after(&mut self, time_us: i64) {
        self.inputs.retain(|input| input.time_us < time_us);
    }

    /// Adds an input (press or release).
    pub fn add_input(&mut self, time_us: i64, column: usize, is_press: bool) {
        self.inputs
            .push(ReplayInput::new(time_us, column, is_press));
    }

    /// Adds a key press input.
    #[inline]
    pub fn add_press(&mut self, time_us: i64, column: usize) {
        self.add_input(time_us, column, true);
    }

    /// Adds a key release input.
    #[inline]
    pub fn add_release(&mut self, time_us: i64, column: usize) {
        self.add_input(time_us, column, false);
    }

    /// Get total input count.
    pub fn input_count(&self) -> usize {
        self.inputs.len()
    }

    /// Check if replay is empty.
    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty()
    }
}

impl Default for ReplayData {
    fn default() -> Self {
        Self {
            version: REPLAY_FORMAT_VERSION,
            inputs: Vec::new(),
            rate: 1.0,
            is_practice_mode: false,
            checkpoints: Vec::new(),
        }
    }
}
