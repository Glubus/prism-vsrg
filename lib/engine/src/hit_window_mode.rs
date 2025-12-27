//! Hit window mode configuration.
//!
//! This module defines the different hit window calculation modes
//! supported by the engine.

use serde::{Deserialize, Serialize};

/// Hit window calculation mode.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub enum HitWindowMode {
    /// osu! Overall Difficulty (OD) based timing.
    OsuOD,
    /// Etterna/Quaver judge level based timing.
    EtternaJudge,
}

impl Default for HitWindowMode {
    fn default() -> Self {
        Self::OsuOD
    }
}
