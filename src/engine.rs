use std::time::Instant;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use rand::Rng;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub const NUM_COLUMNS: usize = 4;
pub const HIT_LINE_Y: f32 = -0.8;
pub const SPAWN_Y: f32 = 1.2;
pub const VISIBLE_DISTANCE: f32 = SPAWN_Y - HIT_LINE_Y;

// --- Système de Pixel ---
pub struct PixelSystem {
    pub pixel_size: f32, // Taille d'un pixel en coordonnées normalisées
    pub window_width: u32,
    pub window_height: u32,
}

impl PixelSystem {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        // Un pixel = 1/height de l'écran (pour avoir des tailles constantes)
        let pixel_size = 2.0 / window_height as f32;
        Self {
            pixel_size,
            window_width,
            window_height,
        }
    }

    pub fn pixels_to_normalized(&self, pixels: f32) -> f32 {
        pixels * self.pixel_size
    }

    pub fn update_size(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;
        self.pixel_size = 2.0 / height as f32;
    }
}

// --- Couleurs des Jugements ---
#[derive(Clone)]
pub struct JudgementColors {
    pub marv: [f32; 4],        // Bleu cyan
    pub perfect: [f32; 4],      // Jaune
    pub great: [f32; 4],        // Vert
    pub good: [f32; 4],         // Bleu foncé
    pub bad: [f32; 4],          // Rose
    pub miss: [f32; 4],         // Rouge
    pub ghost_tap: [f32; 4],    // À définir
}

impl JudgementColors {
    pub fn new() -> Self {
        Self {
            marv: [0.0, 1.0, 1.0, 1.0],           // Cyan
            perfect: [1.0, 1.0, 0.0, 1.0],        // Jaune
            great: [0.0, 1.0, 0.0, 1.0],           // Vert
            good: [0.0, 0.0, 0.5, 1.0],            // Bleu foncé
            bad: [1.0, 0.41, 0.71, 1.0],          // Rose
            miss: [1.0, 0.0, 0.0, 1.0],           // Rouge
            ghost_tap: [0.5, 0.5, 0.5, 1.0],      // Gris par défaut
        }
    }
}

// --- Configuration du Playfield ---
#[derive(Clone)]
pub struct PlayfieldConfig {
    pub column_width_pixels: f32,  // Largeur d'une colonne en pixels
    pub note_width_pixels: f32,    // Largeur d'une note en pixels (peut être > column_width)
    pub note_height_pixels: f32,   // Hauteur d'une note en pixels
}

impl PlayfieldConfig {
    pub fn new() -> Self {
        Self {
            column_width_pixels: 100.0,  // 100 pixels par défaut
            note_width_pixels: 90.0,     // 90 pixels par défaut
            note_height_pixels: 20.0,    // 20 pixels par défaut
        }
    }

    /// Réduit la taille des notes et receptors de 5 pixels
    pub fn decrease_note_size(&mut self) {
        self.note_width_pixels = (self.note_width_pixels - 5.0).max(10.0);
        self.note_height_pixels = self.note_width_pixels; // Garder les notes carrées
        // L'écart entre colonnes est égal à la taille des notes
        self.column_width_pixels = self.note_width_pixels;
        println!("Note size: {:.0}x{:.0} pixels, column spacing: {:.0} pixels", self.note_width_pixels, self.note_height_pixels, self.column_width_pixels);
    }

    /// Augmente la taille des notes et receptors de 5 pixels
    pub fn increase_note_size(&mut self) {
        self.note_width_pixels = (self.note_width_pixels + 5.0).min(200.0);
        self.note_height_pixels = self.note_width_pixels; // Garder les notes carrées
        // L'écart entre colonnes est égal à la taille des notes
        self.column_width_pixels = self.note_width_pixels;
        println!("Note size: {:.0}x{:.0} pixels, column spacing: {:.0} pixels", self.note_width_pixels, self.note_height_pixels, self.column_width_pixels);
    }
}

// --- Système de HitWindow ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Judgement {
    Marv,
    Perfect,
    Great,
    Good,
    Bad,
    Miss,
    GhostTap,
}

pub struct HitWindow {
    pub marv_ms: f64,      // -16 à 16 ms
    pub perfect_ms: f64,   // -50 à 50 ms
    pub great_ms: f64,     // -65 à 65 ms
    pub good_ms: f64,      // -100 à 100 ms
    pub bad_ms: f64,       // -150 à 150 ms
    pub miss_ms: f64,      // -200 à 150 ms
}

impl HitWindow {
    pub fn new() -> Self {
        Self {
            marv_ms: 16.0,
            perfect_ms: 50.0,
            great_ms: 65.0,
            good_ms: 100.0,
            bad_ms: 150.0,
            miss_ms: 200.0,
        }
    }

    /// Juge une note selon le timing (différence en ms entre le hit et le timestamp de la note)
    /// timing_diff_ms > 0 : on tape trop tôt (note pas encore arrivée)
    /// timing_diff_ms < 0 : on tape trop tard (note déjà passée)
    /// Retourne le jugement et si la note a été touchée
    pub fn judge(&self, timing_diff_ms: f64) -> (Judgement, bool) {
        // Si on tape trop en avance (plus de 200ms avant la note), c'est un ghost tap
        if timing_diff_ms > 200.0 {
            return (Judgement::GhostTap, false);
        }
        
        // Si on tape entre 200ms et 150ms en avance, c'est un miss
        if timing_diff_ms > 150.0 && timing_diff_ms <= 200.0 {
            return (Judgement::Miss, true);
        }
        
        // Si on tape trop tard (plus de 150ms après la note), c'est un miss
        if timing_diff_ms < -150.0 {
            return (Judgement::Miss, true);
        }
        
        // Sinon, juger selon la précision (entre -150ms et +150ms)
        let abs_diff = timing_diff_ms.abs();
        
        if abs_diff <= self.marv_ms {
            (Judgement::Marv, true)
        } else if abs_diff <= self.perfect_ms {
            (Judgement::Perfect, true)
        } else if abs_diff <= self.great_ms {
            (Judgement::Great, true)
        } else if abs_diff <= self.good_ms {
            (Judgement::Good, true)
        } else if abs_diff <= self.bad_ms {
            (Judgement::Bad, true)
        } else {
            // Entre bad_ms et 150ms, c'est un miss
            (Judgement::Miss, true)
        }
    }
}

// --- Structures de Données ---

#[derive(Clone)]
pub struct NoteData {
    pub timestamp_ms: f64,
    pub column: usize,
    pub hit: bool,  // Si la note a été touchée
}

/// Charge une map osu depuis le fichier spécifié et retourne le chemin de l'audio et les notes
/// Les notes doivent être converties depuis le format osu vers NoteData
/// Le chemin de l'audio est lu depuis la section [General] du fichier .osu
/// 
/// Format de retour : (PathBuf, Vec<NoteData>)
/// - PathBuf : chemin vers le fichier audio (relatif au dossier de la map)
/// - Vec<NoteData> :
///   - timestamp_ms : timestamp de la note en millisecondes (depuis le début de la map)
///   - column : index de la colonne (0 à NUM_COLUMNS-1)
///   - hit : toujours false au chargement
pub fn load_map(path: PathBuf, rate: f64) -> (PathBuf, Vec<NoteData>) {

    let map = rosu_map::Beatmap::from_path(&path).unwrap();
    let audio_path = path.parent().unwrap().join(map.audio_file);

    let mut notes = Vec::new();
    for hit_object in map.hit_objects {
        if let Ok(column) = map_x_to_column(&hit_object) {
            // Apply rate: divide timestamp by rate multiplier
            // If rate = 1.5x, notes come 1.5x faster, so timestamps are divided by 1.5
            let adjusted_timestamp = hit_object.start_time / rate;
            let note = NoteData {
                timestamp_ms: adjusted_timestamp,
                column: column,
                hit: false,
            };
            notes.push(note);
        }
    }
    
    (audio_path, notes)
}

fn map_x_to_column(hit_object: &rosu_map::section::hit_objects::HitObject) -> Result<usize, String> {
    match hit_object.kind {
        rosu_map::section::hit_objects::HitObjectKind::Circle(circle) => Ok(x_to_column(circle.pos.x as i32)),
        _ => Err(format!("Hit object is not a circle: {:?}", hit_object.kind)),
    }
} 

fn x_to_column(x: i32) -> usize {
    match x {
        64 => 0,
        192 => 1,
        320 => 2,
        448 => 3,
        _ => panic!("Invalid column: {}", x),
    }
}
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub offset: [f32; 2],
    pub scale: [f32; 2],
}

// --- Moteur ---

// Structure pour tracker les hits
pub struct HitStats {
    pub marv: u32,
    pub perfect: u32,
    pub great: u32,
    pub good: u32,
    pub bad: u32,
    pub miss: u32,
    pub ghost_tap: u32,
}

impl HitStats {
    pub fn new() -> Self {
        Self {
            marv: 0,
            perfect: 0,
            great: 0,
            good: 0,
            bad: 0,
            miss: 0,
            ghost_tap: 0,
        }
    }

    pub fn calculate_accuracy(&self) -> f64 {
        let total = (self.marv + self.perfect + self.great + self.good + self.bad + self.miss) as f64;
        
        if total == 0.0 {
            return 0.0;
        }
        
        let score = (self.marv + self.perfect) as f64 * 6.0
            + self.great as f64 * 4.0
            + self.good as f64 * 2.0
            + self.bad as f64;
        
        (score / (total * 6.0)) * 100.0
    }
}

pub struct GameEngine {
    pub chart: Vec<NoteData>,
    pub head_index: usize,
    pub start_time: Instant,
    pub scroll_speed_ms: f64,
    pub notes_passed: u32,  // Nombre de notes passées (pour le combo)
    pub combo: u32,  // Combo actuel (réinitialisé à 0 si on rate)
    pub max_combo: u32,  // Combo maximum atteint
    pub hit_window: HitWindow,
    pub active_notes: Vec<(usize, NoteData)>, // (index dans chart, note) - notes actives qui peuvent être touchées
    pub hit_stats: HitStats,  // Statistiques des hits
    pub last_hit_timing: Option<f64>,  // Timing du dernier hit en ms (None si aucun hit récent, Some(timing_diff))
    pub last_hit_judgement: Option<Judgement>,  // Jugement du dernier hit
    _audio_stream: OutputStream,  // Garde le stream audio actif
    pub audio_sink: Arc<Mutex<Sink>>,  // Contrôle la lecture audio
    audio_path: Option<PathBuf>,  // Chemin vers le fichier audio (pour pouvoir le recharger)
    audio_started: bool,  // Si l'audio a démarré
    rate: f64,  // Rate multiplier (1.0 = normal speed)
}

impl GameEngine {
    pub fn new() -> Self {
        // Simulation d'une partition (Chart) complètement aléatoire
        let mut rng = rand::rng();
        let mut chart = Vec::new();
        let mut current_time = 1000.0;
        
        for _ in 0..2000 {
            chart.push(NoteData {
                timestamp_ms: current_time,
                column: rng.random_range(0..NUM_COLUMNS),
                hit: false,
            });
            // Intervalle aléatoire entre 50ms et 500ms
            current_time += rng.random_range(50.0..500.0);
        }

        // Créer un stream audio vide pour la compatibilité
        let (_stream, stream_handle) = OutputStream::try_default().unwrap_or_else(|_| {
            // Si pas de device audio, créer un stream vide
            OutputStream::try_default().unwrap()
        });
        let sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            chart,
            head_index: 0,
            start_time: Instant::now(),
            scroll_speed_ms: 500.0, // 2 secondes pour descendre
            notes_passed: 0,
            combo: 0,
            max_combo: 0,
            hit_window: HitWindow::new(),
            active_notes: Vec::new(),
            hit_stats: HitStats::new(),
            last_hit_timing: None,
            last_hit_judgement: None,
            _audio_stream: _stream,
            audio_sink: Arc::new(Mutex::new(sink)),
            audio_path: None,
            audio_started: false,
            rate: 1.0, // Default rate for new engine
        }
    }

    /// Crée un GameEngine depuis une map osu et charge l'audio
    pub fn from_map(map_path: PathBuf, rate: f64) -> Self {
        let (audio_path, chart) = load_map(map_path, rate);
        
        // Charger et jouer l'audio
        let (_stream, stream_handle) = OutputStream::try_default()
            .expect("Impossible de créer le stream audio");
        
        let sink = Sink::try_new(&stream_handle)
            .expect("Impossible de créer le sink audio");
        
        // Set playback speed based on rate
        sink.set_speed(rate as f32);
        
        // Charger le fichier audio mais ne pas le jouer immédiatement
        match File::open(&audio_path) {
            Ok(file) => {
                match Decoder::new(BufReader::new(file)) {
                    Ok(source) => {
                        sink.append(source);
                        // Mettre en pause pour éviter le démarrage automatique
                        sink.pause();
                    }
                    Err(e) => {
                        eprintln!("Error: Unable to decode audio from {:?}: {}", audio_path, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: Unable to load audio from {:?}: {}", audio_path, e);
            }
        }

        let start_time = Instant::now();
        
        // Ne pas jouer l'audio immédiatement, on le démarrera quand game_time >= 0
        let sink_arc = Arc::new(Mutex::new(sink));

        Self {
            chart,
            head_index: 0,
            start_time,
            scroll_speed_ms: 500.0, // 2 secondes pour descendre
            notes_passed: 0,
            combo: 0,
            max_combo: 0,
            hit_window: HitWindow::new(),
            active_notes: Vec::new(),
            hit_stats: HitStats::new(),
            last_hit_timing: None,
            last_hit_judgement: None,
            _audio_stream: _stream,
            audio_sink: sink_arc,
            audio_path: Some(audio_path),
            audio_started: false,
            rate,
        }
    }

    /// Retourne le game_time en millisecondes
    /// Commence à -5000ms et avance normalement
    pub fn get_game_time(&self) -> f64 {
        let now = Instant::now();
        let elapsed_ms = now.duration_since(self.start_time).as_secs_f64() * 1000.0;
        // game_time commence à -5000ms
        elapsed_ms - 5000.0
    }
    
    /// Démarre l'audio si game_time >= 0 et que l'audio n'a pas encore démarré
    pub fn start_audio_if_needed(&mut self) {
        if !self.audio_started {
            let game_time = self.get_game_time();
            if game_time >= 0.0 {
                if let Ok(sink) = self.audio_sink.lock() {
                    // Reprendre la lecture (play() reprend si en pause)
                    sink.play();
                    self.audio_started = true;
                }
            }
        }
    }

    pub fn reset_time(&mut self) {
        self.start_time = Instant::now();
        self.head_index = 0;
        self.notes_passed = 0;
        self.combo = 0;
        self.max_combo = 0;
        self.active_notes.clear();
        self.hit_stats = HitStats::new();
        self.last_hit_timing = None;
        self.last_hit_judgement = None;
        // Réinitialiser toutes les notes (remettre hit à false)
        for note in &mut self.chart {
            note.hit = false;
        }
        // Redémarrer l'audio depuis le début
        if let Ok(sink) = self.audio_sink.lock() {
            sink.stop();
            sink.clear(); // Vider le sink pour recharger la source
        }
        
        // Recharger le fichier audio si le chemin est disponible
        if let Some(ref audio_path) = self.audio_path {
            match File::open(audio_path) {
                Ok(file) => {
                    match Decoder::new(BufReader::new(file)) {
                        Ok(source) => {
                            if let Ok(sink) = self.audio_sink.lock() {
                                sink.append(source);
                                // Mettre en pause pour éviter le démarrage automatique
                                sink.pause();
                            }
                            // L'audio sera démarré automatiquement quand game_time >= 0
                            self.audio_started = false;
                        }
                        Err(e) => {
                            eprintln!("Error: Unable to decode audio from {:?}: {}", audio_path, e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: Unable to load audio from {:?}: {}", audio_path, e);
                }
            }
        }
    }

    /// Traite un input pour une colonne donnée (0-3 pour dfjk)
    pub fn process_input(&mut self, column: usize) -> Option<Judgement> {
        let song_time = self.get_game_time();

        // Chercher la note la plus proche dans cette colonne qui n'a pas encore été touchée
        let mut best_note: Option<(usize, f64)> = None; // (index, time_diff)

        for (idx, note) in self.chart.iter().enumerate().skip(self.head_index) {
            if note.column == column && !note.hit {
                let time_diff = note.timestamp_ms - song_time;
                
                // Vérifier si la note est dans une hit window valide (pas ghost tap)
                let (judgement, _) = self.hit_window.judge(time_diff);
                if judgement != Judgement::GhostTap {
                    // Prendre la note la plus proche (time_diff le plus petit en valeur absolue)
                    if let Some((_, best_diff)) = best_note {
                        if time_diff.abs() < best_diff.abs() {
                            best_note = Some((idx, time_diff));
                        }
                    } else {
                        best_note = Some((idx, time_diff));
                    }
                }
            }
        }

        if let Some((note_idx, time_diff)) = best_note {
            let (judgement, _) = self.hit_window.judge(time_diff);
            self.chart[note_idx].hit = true;
            self.active_notes.retain(|(idx, _)| *idx != note_idx);
            
            // Enregistrer le timing du dernier hit pour la hitbar
            self.last_hit_timing = Some(time_diff);
            self.last_hit_judgement = Some(judgement);
            
            // Mettre à jour les statistiques et le combo
            match judgement {
                Judgement::Marv => {
                    self.hit_stats.marv += 1;
                    self.combo += 1;
                }
                Judgement::Perfect => {
                    self.hit_stats.perfect += 1;
                    self.combo += 1;
                }
                Judgement::Great => {
                    self.hit_stats.great += 1;
                    self.combo += 1;
                }
                Judgement::Good => {
                    self.hit_stats.good += 1;
                    self.combo += 1;
                }
                Judgement::Bad => {
                    self.hit_stats.bad += 1;
                    self.combo += 1;
                }
                Judgement::Miss => {
                    self.hit_stats.miss += 1;
                    self.combo = 0;  // Réinitialiser le combo
                }
                Judgement::GhostTap => {
                    // Ne devrait pas arriver ici car on filtre les ghost taps
                    self.hit_stats.ghost_tap += 1;
                }
            }
            
            // Mettre à jour le combo maximum
            if self.combo > self.max_combo {
                self.max_combo = self.combo;
            }
            
            // Compter les notes passées (sauf ghost tap)
            if judgement != Judgement::GhostTap {
                self.notes_passed += 1;
            }
            
            return Some(judgement);
        }
        
        // Si aucune note valide n'a été trouvée, c'est un ghost tap
        // Vérifier s'il y a des notes dans cette colonne qui sont trop en avance (avant la hit window)
        let has_notes_in_column = self.chart.iter()
            .skip(self.head_index)
            .any(|note| note.column == column && !note.hit);
        
        if has_notes_in_column {
            // Il y a des notes dans cette colonne mais elles sont en dehors de la hit window
            // C'est un ghost tap
            self.hit_stats.ghost_tap += 1;
            self.last_hit_timing = None;
            self.last_hit_judgement = Some(Judgement::GhostTap);
            return Some(Judgement::GhostTap);
        }
        
        // Aucune note dans cette colonne du tout - c'est aussi un ghost tap
        self.hit_stats.ghost_tap += 1;
        self.last_hit_timing = None;
        self.last_hit_judgement = Some(Judgement::GhostTap);
        Some(Judgement::GhostTap)
    }

    /// Met à jour la liste des notes actives (dans la hit window)
    pub fn update_active_notes(&mut self) {
        let song_time = self.get_game_time();

        self.active_notes.clear();
        
        for (idx, note) in self.chart.iter().enumerate().skip(self.head_index) {
            if !note.hit {
                let time_diff = note.timestamp_ms - song_time;
                let (judgement, _) = self.hit_window.judge(time_diff);
                if judgement != Judgement::GhostTap {
                    self.active_notes.push((idx, note.clone()));
                }
            }
        }
    }

    /// Détecte et marque les notes manquées (passées la hit line sans être touchées)
    pub fn detect_misses(&mut self) {
        let song_time = self.get_game_time();

        for note in self.chart.iter_mut().skip(self.head_index) {
            if !note.hit {
                let time_diff = note.timestamp_ms - song_time;
                
                // Si la note est passée la hit window (après 150ms), c'est un miss
                if time_diff < -150.0 {
                    note.hit = true;  // Marquer comme hit pour ne plus l'afficher
                    self.hit_stats.miss += 1;
                    self.combo = 0;  // Réinitialiser le combo
                    self.notes_passed += 1;
                }
            }
        }
    }

    /// Retourne le nombre de notes restantes (non touchées)
    pub fn get_remaining_notes(&self) -> usize {
        self.chart.iter().skip(self.head_index).filter(|note| !note.hit).count()
    }

    /// Retourne la liste des instances à dessiner pour la frame actuelle
    pub fn get_visible_notes(&mut self, pixel_system: &PixelSystem, playfield_config: &PlayfieldConfig, playfield_x: f32, _playfield_width: f32) -> Vec<InstanceRaw> {
        let now = Instant::now();
        let song_time = now.duration_since(self.start_time).as_secs_f64() * 1000.0;

        let max_future_time = song_time + self.scroll_speed_ms;
        let min_past_time = song_time - 200.0;

        // 1. Avancer le curseur (Optimisation)
        while self.head_index < self.chart.len() {
            if self.chart[self.head_index].timestamp_ms < min_past_time {
                self.head_index += 1;
                self.notes_passed += 1; // Compter les notes passées
            } else {
                break;
            }
        }

        // Calculer les dimensions en coordonnées normalisées
        let column_width_norm = pixel_system.pixels_to_normalized(playfield_config.column_width_pixels);
        let note_width_norm = pixel_system.pixels_to_normalized(playfield_config.note_width_pixels);
        let note_height_norm = pixel_system.pixels_to_normalized(playfield_config.note_height_pixels);

        let mut instances = Vec::with_capacity(500);

        for note in self.chart.iter().skip(self.head_index) {
            if note.timestamp_ms > max_future_time {
                break;
            }

            let time_to_hit = note.timestamp_ms - song_time;
            let progress = time_to_hit / self.scroll_speed_ms;
            
            // Calcul Y : Ligne d'impact + (Distance * Progression)
            let y_pos = HIT_LINE_Y + (VISIBLE_DISTANCE * progress as f32);
            
            // Position X : playfield_x + (colonne * largeur_colonne) + (largeur_colonne / 2)
            let center_x = playfield_x + (note.column as f32 * column_width_norm) + (column_width_norm / 2.0);

            instances.push(InstanceRaw {
                offset: [center_x, y_pos],
                scale: [note_width_norm, note_height_norm],
            });
        }

        instances
    }
}