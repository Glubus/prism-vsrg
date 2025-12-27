//! Result types from replay simulation.

use engine::{HitStats, Judgement, US_PER_MS};
use serde::{Deserialize, Serialize};

/// Individual hit timing for graphs and analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HitTiming {
    /// Index of the hit note.
    pub note_index: usize,
    /// Timing offset in µs (negative = early, positive = late).
    pub timing_us: i64,
    /// Assigned judgement.
    pub judgement: Judgement,
    /// Timestamp of the note in the map (µs).
    pub note_time_us: i64,
}

impl HitTiming {
    /// Timing offset in milliseconds (for display).
    pub fn timing_ms(&self) -> f64 {
        self.timing_us as f64 / US_PER_MS as f64
    }
}

/// Ghost tap (press without a corresponding note).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GhostTap {
    /// Timestamp of the ghost tap (µs).
    pub time_us: i64,
    /// Column index.
    pub column: u8,
}

/// Complete result of a replay simulation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplayResult {
    /// Hit statistics.
    pub hit_stats: HitStats,
    /// Calculated accuracy (0-100).
    pub accuracy: f64,
    /// Total score.
    pub score: u32,
    /// Maximum combo achieved.
    pub max_combo: u32,
    /// Hit timing details for graphs.
    pub hit_timings: Vec<HitTiming>,
    /// List of ghost taps.
    pub ghost_taps: Vec<GhostTap>,
}

impl ReplayResult {
    pub fn new() -> Self {
        Self {
            hit_stats: HitStats::new(),
            accuracy: 0.0,
            score: 0,
            max_combo: 0,
            hit_timings: Vec::new(),
            ghost_taps: Vec::new(),
        }
    }
}

impl Default for ReplayResult {
    fn default() -> Self {
        Self::new()
    }
}
