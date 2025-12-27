# Prism VSRG

Vertical Scrolling Rhythm Game built in Rust with wgpu.

## Features

- 4K/7K gameplay
- Osu, Stepmania, Quaver, Etterna chart support
- Multiple difficulty calculators (Etterna, osu!)
- Skinnable UI
- Rate modification (0.5x - 2.0x)
- Local leaderboard with replays

## Structure

```bash
apps/game/     Main application
lib/audio/     Audio playback
lib/chart/     Chart parsing & difficulty
lib/database/  Beatmap database
lib/engine/    Gameplay & scoring
lib/replay/    Replay system
lib/settings/  Configuration
lib/skin/      Skin loading
```

## Build

```bash
cargo run -r
```

## License

AGPL-3.0
