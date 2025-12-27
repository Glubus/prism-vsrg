//! Replay simulation - deterministic score recalculation.
//!
//! Uses the engine's hit matching algorithm for 1:1 consistency
//! with live gameplay.

use crate::types::{GhostTap, HitTiming, ReplayData, ReplayResult};
use engine::{HitStats, HitWindow, Judgement, NoteAccessor, NoteData};

/// Wrapper for simulation that tracks hit state separately.
struct SimNote<'a> {
    note: &'a NoteData,
    hit: bool,
}

impl NoteAccessor for SimNote<'_> {
    fn time_us(&self) -> i64 {
        self.note.time_us()
    }

    fn column(&self) -> usize {
        self.note.column()
    }

    fn is_hit(&self) -> bool {
        self.hit
    }
}

/// Simulates a replay on a chart with the given hit window.
///
/// Uses the engine's `find_best_note` algorithm for 1:1 consistency
/// with live gameplay scoring.
pub fn simulate(
    replay_data: &ReplayData,
    chart: &[NoteData],
    hit_window: &HitWindow,
) -> ReplayResult {
    let mut result = ReplayResult::new();
    let mut combo: u32 = 0;
    let miss_us = hit_window.miss_us;

    // Create simulation notes with mutable hit tracking
    let mut sim_notes: Vec<SimNote> = chart
        .iter()
        .map(|n| SimNote {
            note: n,
            hit: false,
        })
        .collect();
    let mut head_index: usize = 0;

    for input in &replay_data.inputs {
        let (input_column, is_press) = input.unpack();
        let input_time_us = input.time_us;

        // Advance head_index and check for missed notes
        while head_index < sim_notes.len() {
            if sim_notes[head_index].hit {
                head_index += 1;
                continue;
            }

            let note = sim_notes[head_index].note;
            let miss_deadline = note.time_us() + miss_us;

            if input_time_us > miss_deadline {
                sim_notes[head_index].hit = true;
                result.hit_stats.miss += 1;
                combo = 0;

                result.hit_timings.push(HitTiming {
                    note_index: head_index,
                    timing_us: miss_us,
                    judgement: Judgement::Miss,
                    note_time_us: note.time_us(),
                });

                head_index += 1;
            } else {
                break;
            }
        }

        // Only process presses (releases don't hit notes in basic mode)
        if !is_press {
            continue;
        }

        // Use engine's find_best_note for 1:1 matching with gameplay
        if let Some((idx, timing_diff)) =
            hit_window.find_best_note(&sim_notes, head_index, input_column, input_time_us)
        {
            sim_notes[idx].hit = true;
            let (judgement, _) = hit_window.judge(timing_diff);

            apply_judgement(&mut result, &mut combo, judgement);

            result.hit_timings.push(HitTiming {
                note_index: idx,
                timing_us: timing_diff,
                judgement,
                note_time_us: sim_notes[idx].note.time_us(),
            });
        } else {
            // Ghost tap - no note matched
            result.hit_stats.ghost_tap += 1;
            result.ghost_taps.push(GhostTap {
                time_us: input_time_us,
                column: input_column as u8,
            });
        }
    }

    // Mark remaining unhit notes as misses
    for (idx, sim_note) in sim_notes.iter().enumerate() {
        if !sim_note.hit {
            result.hit_stats.miss += 1;
            result.hit_timings.push(HitTiming {
                note_index: idx,
                timing_us: miss_us,
                judgement: Judgement::Miss,
                note_time_us: sim_note.note.time_us(),
            });
        }
    }

    result.accuracy = result.hit_stats.calculate_accuracy();
    result
}

/// Apply a judgement to the result and update combo.
fn apply_judgement(result: &mut ReplayResult, combo: &mut u32, judgement: Judgement) {
    match judgement {
        Judgement::Miss => {
            result.hit_stats.miss += 1;
            *combo = 0;
        }
        Judgement::GhostTap => {
            result.hit_stats.ghost_tap += 1;
        }
        Judgement::Marv => {
            result.hit_stats.marv += 1;
            *combo += 1;
            result.max_combo = result.max_combo.max(*combo);
            result.score += 300;
        }
        Judgement::Perfect => {
            result.hit_stats.perfect += 1;
            *combo += 1;
            result.max_combo = result.max_combo.max(*combo);
            result.score += 300;
        }
        Judgement::Great => {
            result.hit_stats.great += 1;
            *combo += 1;
            result.max_combo = result.max_combo.max(*combo);
            result.score += 200;
        }
        Judgement::Good => {
            result.hit_stats.good += 1;
            *combo += 1;
            result.max_combo = result.max_combo.max(*combo);
            result.score += 100;
        }
        Judgement::Bad => {
            result.hit_stats.bad += 1;
            *combo += 1;
            result.max_combo = result.max_combo.max(*combo);
            result.score += 50;
        }
    }
}

/// Re-judges a replay with a new hit window.
///
/// Useful for comparing scores under different timing systems
/// (e.g., Etterna Judge 4 vs Judge 9).
pub fn rejudge(
    replay_data: &ReplayData,
    chart: &[NoteData],
    new_hit_window: &HitWindow,
) -> ReplayResult {
    simulate(replay_data, chart, new_hit_window)
}

/// Recalculates stats from existing hit timings with a new hit window.
///
/// This is faster than full re-simulation when you already have
/// the timing data and just want to apply different judgement thresholds.
pub fn rejudge_timings(hit_timings: &[HitTiming], hit_window: &HitWindow) -> (HitStats, f64) {
    let mut stats = HitStats::new();

    for hit in hit_timings {
        let (judgement, _) = hit_window.judge(hit.timing_us);

        match judgement {
            Judgement::Marv => stats.marv += 1,
            Judgement::Perfect => stats.perfect += 1,
            Judgement::Great => stats.great += 1,
            Judgement::Good => stats.good += 1,
            Judgement::Bad => stats.bad += 1,
            Judgement::Miss => stats.miss += 1,
            Judgement::GhostTap => stats.ghost_tap += 1,
        }
    }

    let accuracy = stats.calculate_accuracy();
    (stats, accuracy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ReplayData;

    #[test]
    fn test_empty_replay() {
        let replay = ReplayData::new(1.0);
        let chart: Vec<NoteData> = vec![];
        let hit_window = HitWindow::new();

        let result = simulate(&replay, &chart, &hit_window);
        assert_eq!(result.accuracy, 0.0);
        assert_eq!(result.max_combo, 0);
    }

    #[test]
    fn test_perfect_hit() {
        let mut replay = ReplayData::new(1.0);
        replay.add_press(1000, 0); // Hit at exactly 1000µs

        let chart = vec![NoteData::tap(1000, 0)]; // Note at 1000µs
        let hit_window = HitWindow::new();

        let result = simulate(&replay, &chart, &hit_window);
        assert_eq!(result.hit_stats.marv, 1);
        assert_eq!(result.max_combo, 1);
    }

    #[test]
    fn test_ghost_tap() {
        let mut replay = ReplayData::new(1.0);
        replay.add_press(1000, 1); // Wrong column

        let chart = vec![NoteData::tap(1000, 0)];
        let hit_window = HitWindow::new();

        let result = simulate(&replay, &chart, &hit_window);
        assert_eq!(result.hit_stats.ghost_tap, 1);
        assert_eq!(result.hit_stats.miss, 1); // Note was never hit
    }
}
