//! Core replay input type.

use serde::{Deserialize, Serialize};

/// A single user input (press or release).
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
pub struct ReplayInput {
    /// Absolute time in microseconds since map start.
    pub time_us: i64,
    /// Packed data: (column << 1) | is_press
    pub payload: u8,
}

impl ReplayInput {
    /// Create a new ReplayInput.
    pub fn new(time_us: i64, column: usize, is_press: bool) -> Self {
        let payload = ((column as u8) << 1) | (is_press as u8);
        Self { time_us, payload }
    }

    /// Unpack column and is_press from payload.
    #[inline]
    pub fn unpack(&self) -> (usize, bool) {
        let is_press = (self.payload & 1) != 0;
        let column = (self.payload >> 1) as usize;
        (column, is_press)
    }

    /// Get the column index.
    #[inline]
    pub fn column(&self) -> usize {
        (self.payload >> 1) as usize
    }

    /// Check if this is a press (true) or release (false).
    #[inline]
    pub fn is_press(&self) -> bool {
        (self.payload & 1) != 0
    }
}
