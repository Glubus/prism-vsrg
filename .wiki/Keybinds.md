# Keybinds

This page documents the default keybinds and how to customize them.

## Gameplay Controls

### Default Lane Keybinds

| Key Count | Default Keys |
|-----------|--------------|
| 4K | D, F, J, K |
| 5K | D, F, Space, J, K |
| 6K | S, D, F, J, K, L |
| 7K | S, D, F, Space, J, K, L |

### Menu Navigation

| Action | Key |
|--------|-----|
| Navigate Up | ↑ Arrow |
| Navigate Down | ↓ Arrow |
| Previous Difficulty | ← Arrow |
| Next Difficulty | → Arrow |
| Confirm/Play | Enter |
| Back | Escape |
| Increase Rate | Tab |
| Decrease Rate | Shift+Tab |
| Toggle Settings | F1 |
| Launch Practice Mode | F3 |
| Rescan Songs | F5 |

### In-Game Controls

| Action | Key |
|--------|-----|
| Pause | Escape |
| Place Checkpoint (Practice) | F4 |
| Return to Checkpoint (Practice) | F5 |

### Editor Controls

| Action | Key |
|--------|-----|
| Toggle Editor | Tab (from menu) |
| Select Notes | W |
| Select Receptors | X |
| Select Combo | C |
| Select Score | V |
| Select Accuracy | B |
| Select Judgement | N |
| Select Hit Bar | K |
| Save Layout | S |
| Move Mode | Toggle same key |
| Resize Mode | Toggle same key |

## Customizing Keybinds

### Via Settings UI

1. Press F1 to open settings
2. Navigate to "Keybindings"
3. Click on the key count you want to modify
4. Press the keys in order (left to right)

### Via Configuration File

Edit `settings.toml`:

```toml
[keybinds]
4 = ["KeyD", "KeyF", "KeyJ", "KeyK"]
5 = ["KeyD", "KeyF", "Space", "KeyJ", "KeyK"]
6 = ["KeyS", "KeyD", "KeyF", "KeyJ", "KeyK", "KeyL"]
7 = ["KeyS", "KeyD", "KeyF", "Space", "KeyJ", "KeyK", "KeyL"]
```

### Key Names

Use the physical key names from winit. Common keys:

| Key | Name |
|-----|------|
| A-Z | `KeyA` through `KeyZ` |
| 0-9 | `Digit0` through `Digit9` |
| Space | `Space` |
| Arrows | `ArrowUp`, `ArrowDown`, `ArrowLeft`, `ArrowRight` |
| Enter | `Enter` |
| Shift | `ShiftLeft`, `ShiftRight` |
| Control | `ControlLeft`, `ControlRight` |

## Practice Mode

Practice mode allows you to:

1. **Place checkpoints** (F4) - Max 1 every 15 seconds
2. **Return to checkpoint** (F5) - Restarts 1 second before the checkpoint

Scores from practice mode are labeled separately in the leaderboard.

