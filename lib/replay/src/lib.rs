//! Replay - Recording, simulation, and storage for rhythm game replays.
//!
//! # Modules
//!
//! - [`types`] - Core data structures (ReplayData, ReplayInput, etc.)
//! - [`simulation`] - Deterministic score calculation from replays
//! - [`storage`] - Compression and file I/O
//!
//! # Quick Start
//!
//! ```rust
//! use replay::{ReplayData, simulate, compress, decompress};
//!
//! // Create a new replay
//! let mut replay = ReplayData::new(1.0);
//! replay.add_press(1000, 0);
//! replay.add_release(1500, 0);
//!
//! // Compress for storage
//! let bytes = compress(&replay).unwrap();
//!
//! // Load it back
//! let loaded = decompress(&bytes).unwrap();
//! ```

pub mod simulation;
pub mod storage;
pub mod types;

// Re-export types
pub use types::{
    CHECKPOINT_MIN_INTERVAL_US, GhostTap, HitTiming, REPLAY_FORMAT_VERSION, ReplayData,
    ReplayInput, ReplayResult,
};

// Re-export simulation functions
pub use simulation::{rejudge, rejudge_timings, simulate};

// Re-export storage functions
pub use storage::{compress, decompress};

// Legacy aliases for backwards compatibility
#[deprecated(since = "0.2.0", note = "Use `simulate` instead")]
pub fn simulate_replay(
    replay_data: &ReplayData,
    chart: &[engine::NoteData],
    hit_window: &engine::HitWindow,
) -> ReplayResult {
    simulate(replay_data, chart, hit_window)
}

#[deprecated(since = "0.2.0", note = "Use `rejudge` instead")]
pub fn rejudge_replay(
    replay_data: &ReplayData,
    chart: &[engine::NoteData],
    hit_window: &engine::HitWindow,
) -> ReplayResult {
    rejudge(replay_data, chart, hit_window)
}

#[deprecated(since = "0.2.0", note = "Use `rejudge_timings` instead")]
pub fn rejudge_hit_timings(
    hit_timings: &[HitTiming],
    hit_window: &engine::HitWindow,
) -> (engine::HitStats, f64) {
    rejudge_timings(hit_timings, hit_window)
}

#[deprecated(since = "0.2.0", note = "Use `compress` instead")]
pub fn compress_replay(data: &ReplayData) -> std::io::Result<Vec<u8>> {
    compress(data)
}

#[deprecated(since = "0.2.0", note = "Use `decompress` instead")]
pub fn decompress_replay(compressed: &[u8]) -> std::io::Result<ReplayData> {
    decompress(compressed)
}
