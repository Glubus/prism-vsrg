use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPosition {
    #[serde(default)]
    pub x: Option<f32>,  // Position X en pixels (None = calculé automatiquement)
    #[serde(default)]
    pub y: Option<f32>,  // Position Y en pixels (None = calculé automatiquement)
    #[serde(default)]
    pub width: Option<f32>,  // Largeur en pixels (pour hit_bar)
    #[serde(default)]
    pub height: Option<f32>,  // Hauteur en pixels (pour hit_bar)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPositions {
    #[serde(default)]
    pub playfield: Option<UIPosition>,
    #[serde(default)]
    pub combo: Option<UIPosition>,
    #[serde(default)]
    pub hit_bar: Option<UIPosition>,
    #[serde(default)]
    pub score: Option<UIPosition>,
    #[serde(default)]
    pub accuracy: Option<UIPosition>,
    #[serde(default)]
    pub judgements: Option<UIPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinConfig {
    pub skin: SkinInfo,
    pub images: ImagePaths,
    #[serde(default)]
    pub colors: Option<ColorConfig>,
    #[serde(default)]
    pub keys: Option<KeyConfig>,
    #[serde(default)]
    pub ui_positions: Option<UIPositions>,  // Positions des éléments UI
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyConfig {
    // Mapping des colonnes vers les touches (support jusqu'à 10 colonnes)
    // Format: "column_0" = ["KeyD", "KeyF"] pour plusieurs touches
    #[serde(default)]
    pub column_0: Option<Vec<String>>,
    #[serde(default)]
    pub column_1: Option<Vec<String>>,
    #[serde(default)]
    pub column_2: Option<Vec<String>>,
    #[serde(default)]
    pub column_3: Option<Vec<String>>,
    #[serde(default)]
    pub column_4: Option<Vec<String>>,
    #[serde(default)]
    pub column_5: Option<Vec<String>>,
    #[serde(default)]
    pub column_6: Option<Vec<String>>,
    #[serde(default)]
    pub column_7: Option<Vec<String>>,
    #[serde(default)]
    pub column_8: Option<Vec<String>>,
    #[serde(default)]
    pub column_9: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    #[serde(default)]
    pub font: Option<String>,  // Chemin vers le fichier de police
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagePaths {
    #[serde(default)]
    pub receptor: Option<String>,
    // Images par colonne pour les receptors (0-9, support jusqu'à 10 colonnes)
    #[serde(default)]
    pub receptor_0: Option<String>,
    #[serde(default)]
    pub receptor_1: Option<String>,
    #[serde(default)]
    pub receptor_2: Option<String>,
    #[serde(default)]
    pub receptor_3: Option<String>,
    #[serde(default)]
    pub receptor_4: Option<String>,
    #[serde(default)]
    pub receptor_5: Option<String>,
    #[serde(default)]
    pub receptor_6: Option<String>,
    #[serde(default)]
    pub receptor_7: Option<String>,
    #[serde(default)]
    pub receptor_8: Option<String>,
    #[serde(default)]
    pub receptor_9: Option<String>,
    
    #[serde(default)]
    pub note: Option<String>,
    // Images par colonne pour les notes (0-9, support jusqu'à 10 colonnes)
    #[serde(default)]
    pub note_0: Option<String>,
    #[serde(default)]
    pub note_1: Option<String>,
    #[serde(default)]
    pub note_2: Option<String>,
    #[serde(default)]
    pub note_3: Option<String>,
    #[serde(default)]
    pub note_4: Option<String>,
    #[serde(default)]
    pub note_5: Option<String>,
    #[serde(default)]
    pub note_6: Option<String>,
    #[serde(default)]
    pub note_7: Option<String>,
    #[serde(default)]
    pub note_8: Option<String>,
    #[serde(default)]
    pub note_9: Option<String>,
    
    #[serde(default)]
    pub miss_note: Option<String>,
    #[serde(default)]
    pub background: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorConfig {
    #[serde(default = "default_receptor_color")]
    pub receptor_color: [f32; 4],
    #[serde(default = "default_note_color")]
    pub note_color: [f32; 4],
    // Couleurs des jugements
    #[serde(default = "default_marv_color")]
    pub marv: [f32; 4],
    #[serde(default = "default_perfect_color")]
    pub perfect: [f32; 4],
    #[serde(default = "default_great_color")]
    pub great: [f32; 4],
    #[serde(default = "default_good_color")]
    pub good: [f32; 4],
    #[serde(default = "default_bad_color")]
    pub bad: [f32; 4],
    #[serde(default = "default_miss_color")]
    pub miss: [f32; 4],
    #[serde(default = "default_ghost_tap_color")]
    pub ghost_tap: [f32; 4],
}

fn default_receptor_color() -> [f32; 4] {
    [0.0, 0.0, 1.0, 1.0] // Bleu
}

fn default_note_color() -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0] // Blanc
}

fn default_marv_color() -> [f32; 4] {
    [0.0, 1.0, 1.0, 1.0] // Cyan
}

fn default_perfect_color() -> [f32; 4] {
    [1.0, 1.0, 0.0, 1.0] // Jaune
}

fn default_great_color() -> [f32; 4] {
    [0.0, 1.0, 0.0, 1.0] // Vert
}

fn default_good_color() -> [f32; 4] {
    [0.0, 0.0, 0.5, 1.0] // Bleu foncé
}

fn default_bad_color() -> [f32; 4] {
    [1.0, 0.41, 0.71, 1.0] // Rose
}

fn default_miss_color() -> [f32; 4] {
    [1.0, 0.0, 0.0, 1.0] // Rouge
}

fn default_ghost_tap_color() -> [f32; 4] {
    [0.5, 0.5, 0.5, 1.0] // Gris
}

pub struct Skin {
    pub config: SkinConfig,
    pub base_path: PathBuf,
    // Mapping des touches vers les colonnes (pour lookup rapide)
    pub key_to_column: HashMap<String, usize>,
}

impl Skin {
    /// Charge un skin depuis un dossier
    pub fn load(skin_path: &Path) -> Result<Self, String> {
        let toml_path = skin_path.join("skin.toml");
        
        if !toml_path.exists() {
            return Err(format!("skin.toml not found in {:?}", skin_path));
        }

        let toml_content = fs::read_to_string(&toml_path)
            .map_err(|e| format!("Failed to read skin.toml: {}", e))?;

        let config: SkinConfig = toml::from_str(&toml_content)
            .map_err(|e| format!("Failed to parse skin.toml: {}", e))?;

        // Construire le mapping des touches vers les colonnes (support jusqu'à 10 colonnes)
        let mut key_to_column = HashMap::new();
        if let Some(keys) = &config.keys {
            let column_keys = [
                &keys.column_0, &keys.column_1, &keys.column_2, &keys.column_3,
                &keys.column_4, &keys.column_5, &keys.column_6, &keys.column_7,
                &keys.column_8, &keys.column_9,
            ];
            
            for (col_idx, col_keys_opt) in column_keys.iter().enumerate() {
                if let Some(col_keys) = col_keys_opt {
                    for key in col_keys {
                        key_to_column.insert(key.clone(), col_idx);
                    }
                }
            }
        }

        Ok(Self {
            config,
            base_path: skin_path.to_path_buf(),
            key_to_column,
        })
    }

    /// Charge le skin par défaut pour un nombre de colonnes donné
    pub fn load_default(num_columns: usize) -> Result<Self, String> {
        let default_path = Path::new("skins/default");
        let toml_name = format!("skin_{}k.toml", num_columns);
        let toml_path = default_path.join(&toml_name);
        
        if !toml_path.exists() {
            return Err(format!("skin_{}k.toml not found in {:?}", num_columns, default_path));
        }

        let toml_content = fs::read_to_string(&toml_path)
            .map_err(|e| format!("Failed to read {}: {}", toml_name, e))?;

        let config: SkinConfig = toml::from_str(&toml_content)
            .map_err(|e| format!("Failed to parse {}: {}", toml_name, e))?;

        // Construire le mapping des touches vers les colonnes
        let mut key_to_column = HashMap::new();
        if let Some(keys) = &config.keys {
            let column_keys = [
                &keys.column_0, &keys.column_1, &keys.column_2, &keys.column_3,
                &keys.column_4, &keys.column_5, &keys.column_6, &keys.column_7,
                &keys.column_8, &keys.column_9,
            ];
            
            for (col_idx, col_keys_opt) in column_keys.iter().enumerate() {
                if col_idx >= num_columns {
                    break; // Ne pas traiter les colonnes au-delà du nombre de colonnes
                }
                if let Some(col_keys) = col_keys_opt {
                    for key in col_keys {
                        key_to_column.insert(key.clone(), col_idx);
                    }
                }
            }
        }

        Ok(Self {
            config,
            base_path: default_path.to_path_buf(),
            key_to_column,
        })
    }

    /// Retourne la colonne associée à une touche (si configurée)
    pub fn get_column_for_key(&self, key_name: &str) -> Option<usize> {
        self.key_to_column.get(key_name).copied()
    }

    /// Retourne le chemin complet vers une image
    pub fn get_image_path(&self, image_name: &str) -> PathBuf {
        self.base_path.join(image_name)
    }

    /// Retourne le chemin vers l'image du receptor pour une colonne donnée
    pub fn get_receptor_path(&self, column: usize) -> Option<PathBuf> {
        let image_name = match column {
            0 => self.config.images.receptor_0.as_ref(),
            1 => self.config.images.receptor_1.as_ref(),
            2 => self.config.images.receptor_2.as_ref(),
            3 => self.config.images.receptor_3.as_ref(),
            4 => self.config.images.receptor_4.as_ref(),
            5 => self.config.images.receptor_5.as_ref(),
            6 => self.config.images.receptor_6.as_ref(),
            7 => self.config.images.receptor_7.as_ref(),
            8 => self.config.images.receptor_8.as_ref(),
            9 => self.config.images.receptor_9.as_ref(),
            _ => None,
        };
        
        // Si pas d'image spécifique pour cette colonne, utiliser l'image générale
        image_name
            .or_else(|| self.config.images.receptor.as_ref())
            .map(|name| self.get_image_path(name))
    }

    /// Retourne le chemin vers l'image de note pour une colonne donnée
    pub fn get_note_path(&self, column: usize) -> Option<PathBuf> {
        let image_name = match column {
            0 => self.config.images.note_0.as_ref(),
            1 => self.config.images.note_1.as_ref(),
            2 => self.config.images.note_2.as_ref(),
            3 => self.config.images.note_3.as_ref(),
            4 => self.config.images.note_4.as_ref(),
            5 => self.config.images.note_5.as_ref(),
            6 => self.config.images.note_6.as_ref(),
            7 => self.config.images.note_7.as_ref(),
            8 => self.config.images.note_8.as_ref(),
            9 => self.config.images.note_9.as_ref(),
            _ => None,
        };
        
        // Si pas d'image spécifique pour cette colonne, utiliser l'image générale
        image_name
            .or_else(|| self.config.images.note.as_ref())
            .map(|name| self.get_image_path(name))
    }

    /// Retourne le chemin vers l'image de note manquée
    pub fn get_miss_note_path(&self) -> Option<PathBuf> {
        self.config.images.miss_note.as_ref()
            .map(|name| self.get_image_path(name))
    }

    /// Retourne le chemin vers l'image de fond
    pub fn get_background_path(&self) -> Option<PathBuf> {
        self.config.images.background.as_ref()
            .map(|name| self.get_image_path(name))
    }

    /// Retourne la couleur du receptor
    pub fn get_receptor_color(&self) -> [f32; 4] {
        self.config.colors.as_ref()
            .map(|c| c.receptor_color)
            .unwrap_or([0.0, 0.0, 1.0, 1.0])
    }

    /// Retourne la couleur des notes
    pub fn get_note_color(&self) -> [f32; 4] {
        self.config.colors.as_ref()
            .map(|c| c.note_color)
            .unwrap_or([1.0, 1.0, 1.0, 1.0])
    }

    /// Retourne les couleurs des jugements
    pub fn get_judgement_colors(&self) -> crate::engine::JudgementColors {
        if let Some(colors) = &self.config.colors {
            crate::engine::JudgementColors {
                marv: colors.marv,
                perfect: colors.perfect,
                great: colors.great,
                good: colors.good,
                bad: colors.bad,
                miss: colors.miss,
                ghost_tap: colors.ghost_tap,
            }
        } else {
            crate::engine::JudgementColors::new()
        }
    }

    /// Retourne le chemin vers le fichier de police
    pub fn get_font_path(&self) -> Option<PathBuf> {
        self.config.skin.font.as_ref()
            .map(|font_name| self.get_image_path(font_name))
    }

    /// Retourne les positions UI configurées
    pub fn get_ui_positions(&self) -> &Option<UIPositions> {
        &self.config.ui_positions
    }
}

/// Initialise la structure de dossiers des skins si elle n'existe pas
pub fn init_skin_structure() -> Result<(), String> {
    let skins_dir = Path::new("skins");
    let default_dir = skins_dir.join("default");

    // Créer le dossier skins s'il n'existe pas
    if !skins_dir.exists() {
        fs::create_dir_all(&skins_dir)
            .map_err(|e| format!("Failed to create skins directory: {}", e))?;
    }

    // Créer le dossier default s'il n'existe pas
    if !default_dir.exists() {
        fs::create_dir_all(&default_dir)
            .map_err(|e| format!("Failed to create default skin directory: {}", e))?;
    }

    // Créer les fichiers TOML pour chaque nombre de colonnes (4k à 10k)
    for num_cols in 4..=10 {
        let toml_name = format!("skin_{}k.toml", num_cols);
        let toml_path = default_dir.join(&toml_name);
        
        if !toml_path.exists() {
            let toml_content = generate_toml_for_columns(num_cols);
            fs::write(&toml_path, toml_content)
                .map_err(|e| format!("Failed to create {}: {}", toml_name, e))?;
        }
    }

    Ok(())
}

/// Génère le contenu TOML pour un nombre de colonnes donné
fn generate_toml_for_columns(num_columns: usize) -> String {
    let mut toml = format!(r#"# Configuration du skin pour {} colonnes
[skin]
name = "Default {}K"
version = "1.0.0"
author = "RVSRG"

# Chemin vers le fichier de police (optionnel, par défaut: assets/font.ttf)
font = "font.ttf"

# Images pour les différents éléments
[images]
# Image générale pour les receptors (utilisée si pas d'image spécifique par colonne)
receptor = "receptor.png"

# Images spécifiques par colonne pour les receptors (optionnel)
"#, num_columns, num_columns);
    
    // Ajouter les commentaires pour les images par colonne
    for i in 0..num_columns {
        toml.push_str(&format!("# receptor_{} = \"receptor_col{}.png\"\n", i, i));
    }
    
    toml.push_str(r#"
# Image générale pour les notes (utilisée si pas d'image spécifique par colonne)
note = "note.png"

# Images spécifiques par colonne pour les notes (optionnel)
"#);
    
    for i in 0..num_columns {
        toml.push_str(&format!("# note_{} = \"note_col{}.png\"\n", i, i));
    }
    
    toml.push_str(r#"
# Image pour les notes manquées (optionnel)
miss_note = "miss_note.png"

# Image de fond (optionnel)
background = "background.png"

# Couleurs personnalisées
[colors]
receptor_color = [0.0, 0.0, 1.0, 1.0]  # Bleu pour les receptors
note_color = [1.0, 1.0, 1.0, 1.0]      # Blanc pour les notes

# Couleurs des jugements
marv = [0.0, 1.0, 1.0, 1.0]           # Cyan
perfect = [1.0, 1.0, 0.0, 1.0]        # Jaune
great = [0.0, 1.0, 0.0, 1.0]           # Vert
good = [0.0, 0.0, 0.5, 1.0]            # Bleu foncé
bad = [1.0, 0.41, 0.71, 1.0]           # Rose
miss = [1.0, 0.0, 0.0, 1.0]            # Rouge
ghost_tap = [0.5, 0.5, 0.5, 1.0]      # Gris

# Configuration des touches
[keys]
"#);
    
    // Générer les touches par défaut selon le nombre de colonnes
    let default_keys = match num_columns {
        4 => vec!["KeyD", "KeyF", "KeyJ", "KeyK"],
        5 => vec!["KeyD", "KeyF", "Space", "KeyJ", "KeyK"],
        6 => vec!["KeyS", "KeyD", "KeyF", "KeyJ", "KeyK", "KeyL"],
        7 => vec!["KeyA", "KeyS", "KeyD", "KeyF", "KeyJ", "KeyK", "KeyL"],
        8 => vec!["KeyA", "KeyS", "KeyD", "KeyF", "KeyH", "KeyJ", "KeyK", "KeyL"],
        9 => vec!["KeyQ", "KeyA", "KeyS", "KeyD", "KeyF", "KeyH", "KeyJ", "KeyK", "KeyL"],
        10 => vec!["KeyQ", "KeyA", "KeyS", "KeyD", "KeyF", "KeyH", "KeyJ", "KeyK", "KeyL", "KeyP"],
        _ => vec!["KeyD", "KeyF", "KeyJ", "KeyK"], // Fallback à 4k
    };
    
    for (i, key) in default_keys.iter().enumerate().take(num_columns) {
        toml.push_str(&format!("column_{} = [\"{}\"]\n", i, key));
    }
    
    toml.push_str(r#"
# Positions des éléments UI (optionnel, None = calculé automatiquement)
# Les positions sont en pixels d'écran
[ui_positions]
# Exemple de configuration (décommentez et ajustez selon vos besoins):
# [ui_positions.combo]
# x = 960.0  # Centre de l'écran (1920/2)
# y = 460.0  # 80px au-dessus du centre (1080/2 - 80)

# [ui_positions.hit_bar]
# x = 760.0  # Centre - largeur/2
# y = 520.0  # Juste en dessous du combo
# width = 400.0
# height = 20.0

# [ui_positions.score]
# x = 1200.0
# y = 50.0

# [ui_positions.accuracy]
# x = 200.0
# y = 100.0

# [ui_positions.judgements]
# x = 200.0
# y = 150.0
"#);
    
    toml
}

