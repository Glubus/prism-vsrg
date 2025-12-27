//! Inspector submodule - Per-column editing for notes and receptors
//!
//! Allows editing images/colors for each column in a specific keymode (4K, 5K, 6K, 7K)

use super::ColumnElementType;
use super::common::*;
use skin::Skin;
use egui::Ui;

/// Edit a single column element (Note or Receptor)
/// col is 0-indexed
pub fn edit_single_column_element(
    ui: &mut Ui,
    skin: &mut Skin,
    col: usize,
    element_type: ColumnElementType,
) -> bool {
    let mut changed = false;

    // We need to determine the keymode from context
    // For now, we'll use a reasonable default or iterate through available keymodes
    // The simplest approach: work with whatever keymodes have this column

    // Find keymodes that have this column
    let keymodes: Vec<usize> = skin
        .key_modes
        .keys()
        .filter(|&&k| col < k)
        .copied()
        .collect();

    if keymodes.is_empty() {
        // If no keymodes exist yet, create for the default current preview keymode
        // We'll default to the smallest keymode that can contain this column
        let default_keymode = col + 1;
        let km_config = skin.key_modes.entry(default_keymode).or_default();

        // Ensure vectors are properly sized
        while km_config.notes.len() <= col {
            km_config.notes.push(Default::default());
        }
        while km_config.receptors.len() <= col {
            km_config.receptors.push(Default::default());
        }
    }

    match element_type {
        ColumnElementType::Note => {
            section_header(ui, &format!("🎵 Column {} - Note", col + 1));

            // Edit note for all applicable keymodes
            for &keymode in &keymodes {
                if let Some(km_config) = skin.key_modes.get_mut(&keymode) {
                    while km_config.notes.len() <= col {
                        km_config.notes.push(Default::default());
                    }
                    if let Some(note_cfg) = km_config.notes.get_mut(col) {
                        ui.collapsing(format!("{}K Mode", keymode), |ui| {
                            changed |= size_edit(ui, &mut note_cfg.size.x, &mut note_cfg.size.y);
                            changed |= image_picker(
                                ui,
                                "Note Image",
                                &mut note_cfg.image,
                                Some(&skin.base_path),
                            );
                            changed |= color_edit(ui, "Note Color", &mut note_cfg.color);
                        });
                    }
                }
            }

            hint(ui, "Configure the note appearance for this column");
        }

        ColumnElementType::Receptor => {
            section_header(ui, &format!("⬇️ Column {} - Receptor", col + 1));

            // Edit receptor for all applicable keymodes
            for &keymode in &keymodes {
                if let Some(km_config) = skin.key_modes.get_mut(&keymode) {
                    while km_config.receptors.len() <= col {
                        km_config.receptors.push(Default::default());
                    }
                    if let Some(rec_cfg) = km_config.receptors.get_mut(col) {
                        ui.collapsing(format!("{}K Mode", keymode), |ui| {
                            changed |= size_edit(ui, &mut rec_cfg.size.x, &mut rec_cfg.size.y);
                            changed |= image_picker(
                                ui,
                                "Normal Image",
                                &mut rec_cfg.image,
                                Some(&skin.base_path),
                            );
                            changed |= image_picker(
                                ui,
                                "Pressed Image",
                                &mut rec_cfg.pressed_image,
                                Some(&skin.base_path),
                            );
                            changed |= color_edit(ui, "Normal Color", &mut rec_cfg.color);
                            changed |= color_edit(ui, "Pressed Color", &mut rec_cfg.pressed_color);
                        });
                    }
                }
            }

            hint(ui, "Configure the receptor appearance for this column");
        }
    }

    changed
}

/// Edit columns for a specific keymode
/// Returns true if anything changed
pub fn edit_columns(ui: &mut Ui, skin: &mut Skin, keymode: usize) -> bool {
    let mut changed = false;

    // Get or create the keymode config from the HashMap
    let km_config = skin.key_modes.entry(keymode).or_default();

    section_header(ui, &format!("🎹 {}K Column Configuration", keymode));

    // Ensure we have enough entries for each column
    while km_config.notes.len() < keymode {
        km_config.notes.push(Default::default());
    }
    while km_config.receptors.len() < keymode {
        km_config.receptors.push(Default::default());
    }

    // Edit each column
    for col in 0..keymode {
        let col_name = format!("Column {} ({}K)", col + 1, keymode);

        ui.collapsing(&col_name, |ui| {
            // Note image
            section_header(ui, "🎵 Note");
            if let Some(note_cfg) = km_config.notes.get_mut(col) {
                changed |= size_edit(ui, &mut note_cfg.size.x, &mut note_cfg.size.y);
                changed |=
                    image_picker(ui, "Note Image", &mut note_cfg.image, Some(&skin.base_path));
                changed |= color_edit(ui, "Note Color", &mut note_cfg.color);
            }

            // Receptor images
            section_header(ui, "⬇️ Receptor");
            if let Some(rec_cfg) = km_config.receptors.get_mut(col) {
                changed |= size_edit(ui, &mut rec_cfg.size.x, &mut rec_cfg.size.y);
                changed |= image_picker(
                    ui,
                    "Normal Image",
                    &mut rec_cfg.image,
                    Some(&skin.base_path),
                );
                changed |= image_picker(
                    ui,
                    "Pressed Image",
                    &mut rec_cfg.pressed_image,
                    Some(&skin.base_path),
                );
                changed |= color_edit(ui, "Normal Color", &mut rec_cfg.color);
                changed |= color_edit(ui, "Pressed Color", &mut rec_cfg.pressed_color);
            }
        });
    }

    ui.add_space(10.0);
    hint(ui, "Each column can have different images and colors");

    changed
}

/// Edit 4K columns
pub fn edit_4k_columns(ui: &mut Ui, skin: &mut Skin) -> bool {
    edit_columns(ui, skin, 4)
}

/// Edit 5K columns
pub fn edit_5k_columns(ui: &mut Ui, skin: &mut Skin) -> bool {
    edit_columns(ui, skin, 5)
}

/// Edit 6K columns
pub fn edit_6k_columns(ui: &mut Ui, skin: &mut Skin) -> bool {
    edit_columns(ui, skin, 6)
}

/// Edit 7K columns
pub fn edit_7k_columns(ui: &mut Ui, skin: &mut Skin) -> bool {
    edit_columns(ui, skin, 7)
}
