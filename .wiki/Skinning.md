# Skinning Guide

rVsrg supports customizable skins for visual customization.

## Skin Structure

```Bash
skins/
└── my-skin/
    ├── conf.toml           # Main configuration
    ├── colors.toml         # Color scheme
    ├── general.toml        # General settings
    ├── 4k.toml             # 4K-specific settings
    ├── 5k.toml             # 5K-specific settings
    ├── 6k.toml             # 6K-specific settings
    ├── 7k.toml             # 7K-specific settings
    ├── font.ttf            # Custom font (optional)
    ├── notes/              # Note images
    │   ├── down.png
    │   ├── left.png
    │   ├── right.png
    │   └── up.png
    └── receptors/          # Receptor images
        ├── down.png
        ├── down_pressed.png
        ├── left.png
        ├── left_pressed.png
        ├── right.png
        ├── right_pressed.png
        ├── up.png
        └── up_pressed.png
```

## Configuration Files

### conf.toml

Main skin configuration:

```toml
name = "My Skin"
author = "Your Name"
version = "1.0"
```

### colors.toml

Color definitions (RGBA, 0.0-1.0):

```toml
[judgement]
marv = [0.0, 1.0, 1.0, 1.0]      # Cyan
perfect = [1.0, 1.0, 0.0, 1.0]   # Yellow
great = [0.0, 1.0, 0.0, 1.0]     # Green
good = [0.0, 0.0, 0.5, 1.0]      # Dark Blue
bad = [1.0, 0.41, 0.71, 1.0]     # Pink
miss = [1.0, 0.0, 0.0, 1.0]      # Red

[background]
color = [0.1, 0.1, 0.1, 1.0]

[lanes]
divider = [0.3, 0.3, 0.3, 1.0]
```

### Key Count Configs (4k.toml, etc.)

Per-key-count settings:

```toml
[playfield]
width = 400
x_offset = 0

[notes]
height = 50
```

## Image Requirements

### Notes

- **Format**: PNG with transparency
- **Size**: Any (scaled automatically)
- **Naming**: `down.png`, `left.png`, `right.png`, `up.png`

### Receptors

- **Format**: PNG with transparency
- **Normal state**: `{direction}.png`
- **Pressed state**: `{direction}_pressed.png`

## Applying a Skin

1. Place your skin folder in `skins/`
2. Open settings (F1)
3. Select your skin from the dropdown
4. Restart the game for full effect

## Tips

- Keep images small for better performance
- Use power-of-2 dimensions (64x64, 128x128, etc.)
- Test with different key counts
- Back up your work!

## Default Skin

The `default/` skin serves as a reference implementation. Copy and modify it to create your own.

