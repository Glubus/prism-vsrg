//! Audio system module - playback management and worker thread.
//!
//! This module coordinates audio playback through a dedicated worker thread,
//! ensuring non-blocking audio operations from the game logic.

pub mod manager;
pub mod worker;

pub use manager::AudioManager;
pub use worker::start_audio_thread;
