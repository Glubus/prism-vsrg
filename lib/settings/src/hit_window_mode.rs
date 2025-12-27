//! Hit window calculation modes.

use serde::{Deserialize, Serialize};

/// Hit window calculation mode.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
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

impl std::fmt::Display for HitWindowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OsuOD => write!(f, "osu! OD"),
            Self::EtternaJudge => write!(f, "Etterna Judge"),
        }
    }
}
