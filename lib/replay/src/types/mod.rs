//! Replay data types.
//!
//! Core data structures for replay recording and playback.

mod input;
mod replay;
mod result;

pub use input::ReplayInput;
pub use replay::{CHECKPOINT_MIN_INTERVAL_US, REPLAY_FORMAT_VERSION, ReplayData};
pub use result::{GhostTap, HitTiming, ReplayResult};
