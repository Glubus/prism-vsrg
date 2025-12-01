# Architecture Overview

rVsrg uses a multi-threaded architecture to ensure smooth gameplay and responsive input handling.

## Thread Model

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Main Thread   │     │  Logic Thread   │     │  Audio Thread   │
│   (Render)      │     │  (200 TPS)      │     │  (Dedicated)    │
├─────────────────┤     ├─────────────────┤     ├─────────────────┤
│ • Window events │────▶│ • Game state    │────▶│ • Audio decode  │
│ • WGPU rendering│◀────│ • Hit detection │◀────│ • Playback sync │
│ • egui UI       │     │ • Score calc    │     │ • Pitch shift   │
└─────────────────┘     └─────────────────┘     └─────────────────┘
         │                       │
         │      SystemBus        │
         └───────────┬───────────┘
                     │
         ┌─────────────────┐
         │  Input Thread   │
         ├─────────────────┤
         │ • Key mapping   │
         │ • Debouncing    │
         └─────────────────┘
```

## SystemBus

The `SystemBus` is the central communication hub using lock-free channels:

```rust
pub struct SystemBus {
    // Raw input events (Main → Input)
    pub raw_input_tx: Sender<RawInputEvent>,
    pub raw_input_rx: Receiver<RawInputEvent>,
    
    // Game actions (Input → Logic)
    pub action_tx: Sender<GameAction>,
    pub action_rx: Receiver<GameAction>,
    
    // Render snapshots (Logic → Render)
    pub render_tx: Sender<RenderState>,
    pub render_rx: Receiver<RenderState>,
    
    // Audio commands (Logic → Audio)
    pub audio_cmd_tx: Sender<AudioCommand>,
    pub audio_cmd_rx: Receiver<AudioCommand>,
}
```

## Render State Flow

1. **Logic thread** updates game state at 200 TPS
2. Creates immutable `GameplaySnapshot`
3. Sends snapshot via bounded channel (max 2 frames queued)
4. **Render thread** receives snapshot
5. Interpolates note positions for smooth animation
6. Renders using WGPU

## Audio Synchronization

The game clock is smoothed and synchronized with the audio device:

```rust
// Smooth clock advancement
self.audio_clock += dt_seconds * 1000.0 * self.rate;

// Sync with audio device
let drift = raw_audio_time - self.audio_clock;
if drift.abs() > 80.0 {
    self.audio_clock = raw_audio_time;  // Hard sync
} else if drift.abs() > 5.0 {
    self.audio_clock += drift * 0.35;   // Soft sync
}
```

## Hit Detection

Notes are processed in order with a head index optimization:

1. Check for missed notes (past hit window)
2. On key press, find closest note in window
3. Calculate timing difference
4. Apply judgement and update stats

## Replay System

Replays store only raw inputs:

```rust
pub struct ReplayData {
    pub inputs: Vec<ReplayInput>,  // Press/release events
    pub rate: f64,
    pub hit_window_mode: HitWindowMode,
    pub hit_window_value: f64,
}
```

This allows deterministic resimulation with different hit windows.

## State Machine

The game uses a simple state machine:

```rust
enum AppState {
    Menu(MenuState),      // Song select
    Game(GameEngine),     // Active gameplay
    Editor { ... },       // Beatmap editor
    Result(GameResultData), // Post-game
}
```

