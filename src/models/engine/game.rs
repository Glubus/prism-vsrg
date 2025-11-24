use super::{
    constants::{HIT_LINE_Y, NUM_COLUMNS, VISIBLE_DISTANCE},
    hit_window::HitWindow,
    instance::InstanceRaw,
    note::{load_map, NoteData},
    pixel_system::PixelSystem,
    playfield::PlayfieldConfig,
};
use crate::models::replay::ReplayData;
use crate::models::stats::{HitStats, Judgement};
use md5::Context;
use rand::Rng;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// --- STRUCTURE ESPION (Audio Monitor) ---
// Cette structure enveloppe le Decoder de Rodio pour compter les samples
pub struct AudioMonitor<I> {
    inner: I,
    pub played_samples: Arc<AtomicUsize>,
}

impl<I> Iterator for AudioMonitor<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next();
        if item.is_some() {
            // Chaque fois qu'un sample est consommé, on incrémente le compteur atomique
            // Relaxed suffit car on n'a pas besoin de synchro mémoire stricte, juste du chiffre
            self.played_samples.fetch_add(1, Ordering::Relaxed);
        }
        item
    }
}

impl<I> Source for AudioMonitor<I>
where
    I: Source,
    I::Item: rodio::Sample,
{
    fn current_frame_len(&self) -> Option<usize> { self.inner.current_frame_len() }
    fn channels(&self) -> u16 { self.inner.channels() }
    fn sample_rate(&self) -> u32 { self.inner.sample_rate() }
    fn total_duration(&self) -> Option<Duration> { self.inner.total_duration() }
}

// --- MOTEUR DE JEU ---

pub struct GameEngine {
    pub chart: Vec<NoteData>,
    pub head_index: usize,
    
    // Temps et Synchro
    pub start_time: Instant,            // Moment du lancement (pour le décompte initial)
    pub played_samples: Arc<AtomicUsize>, // Compteur partagé avec le thread audio
    pub sample_rate: u32,
    pub channels: u16,
    pub audio_latency_offset: f64,      // Latence hardware (offset manuel en ms)
    
    pub scroll_speed_ms: f64,
    pub notes_passed: u32,
    pub combo: u32,
    pub max_combo: u32,
    pub hit_window: HitWindow,
    pub active_notes: Vec<(usize, NoteData)>,
    pub hit_stats: HitStats,
    pub last_hit_timing: Option<f64>,
    pub last_hit_judgement: Option<Judgement>,
    
    _audio_stream: OutputStream,
    pub audio_sink: Arc<Mutex<Sink>>,
    audio_path: Option<PathBuf>,
    audio_started: bool,
    rate: f64,
    
    pub replay_data: ReplayData,
    beatmap_hash: Option<String>,
}

impl GameEngine {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut chart = Vec::new();
        let mut current_time = 1000.0;

        for _ in 0..2000 {
            chart.push(NoteData {
                timestamp_ms: current_time,
                column: rng.random_range(0..NUM_COLUMNS),
                hit: false,
            });
            current_time += rng.random_range(50.0..500.0);
        }

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let now = Instant::now();

        Self {
            chart,
            head_index: 0,
            start_time: now,
            played_samples: Arc::new(AtomicUsize::new(0)),
            sample_rate: 44100, // Valeur par défaut
            channels: 2,        // Valeur par défaut
            audio_latency_offset: 0.0, // À ajuster si besoin (ex: 20.0ms)
            
            scroll_speed_ms: 500.0,
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
            rate: 1.0,
            replay_data: ReplayData::new(),
            beatmap_hash: None,
        }
    }

    pub fn from_map(map_path: PathBuf, rate: f64) -> Self {
        let beatmap_hash = Self::calculate_file_hash(&map_path).ok();
        
        // 1. Charger la map avec timestamps originaux (rate 1.0)
        let (audio_path, mut chart) = load_map(map_path, 1.0);

        // 2. PRÉ-CALCUL : Convertir les timestamps des notes en "Temps Réel"
        // Si rate = 1.5, une note à 1500ms devient 1000ms.
        for note in &mut chart {
            note.timestamp_ms /= rate;
        }

        let (_stream, stream_handle) = OutputStream::try_default().expect("Stream failed");
        let sink = Sink::try_new(&stream_handle).expect("Sink failed");
        
        // Configurer la vitesse de lecture hardware
        sink.set_speed(rate as f32);

        let played_samples = Arc::new(AtomicUsize::new(0));
        let mut sample_rate = 44100;
        let mut channels = 2;

        match File::open(&audio_path) {
            Ok(file) => match Decoder::new(BufReader::new(file)) {
                Ok(source) => {
                    sample_rate = source.sample_rate();
                    channels = source.channels();

                    // 3. CRÉATION DE L'ESPION
                    let monitor = AudioMonitor {
                        inner: source,
                        played_samples: played_samples.clone(),
                    };

                    sink.append(monitor);
                    sink.pause();
                }
                Err(e) => eprintln!("Error decoding audio: {}", e),
            },
            Err(e) => eprintln!("Error opening audio: {}", e),
        }

        Self {
            chart,
            head_index: 0,
            start_time: Instant::now(),
            played_samples,
            sample_rate,
            channels,
            audio_latency_offset: 20.0, // Petite compensation pour le buffer audio
            
            scroll_speed_ms: 500.0,
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
            audio_path: Some(audio_path),
            audio_started: false,
            rate,
            replay_data: ReplayData::new(),
            beatmap_hash,
        }
    }
    
    fn calculate_file_hash(file_path: &PathBuf) -> Result<String, std::io::Error> {
        let mut file = File::open(file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let mut context = Context::new();
        context.consume(buffer.as_bytes());
        Ok(format!("{:x}", context.finalize()))
    }

    /// Cœur du système de temps
    /// Retourne le temps écoulé en ms RÉELLES
    pub fn get_game_time(&self) -> f64 {
        if !self.audio_started {
            // Phase de décompte (Lead-in)
            let now = Instant::now();
            let elapsed_since_init = now.duration_since(self.start_time).as_secs_f64() * 1000.0;
            // On renvoie un temps négatif (-5000ms -> 0ms)
            return -5000.0 + elapsed_since_init;
        }

        // Phase de jeu : Calcul basé sur les samples consommés
        let samples = self.played_samples.load(Ordering::Relaxed) as f64;
        
        // 1. Convertir samples -> secondes musicales
        // (Samples / Channels) / SampleRate = Secondes
        let musical_seconds = samples / self.channels as f64 / self.sample_rate as f64;
        
        // 2. Convertir secondes musicales -> ms musicales
        let musical_ms = musical_seconds * 1000.0;
        
        // 3. Convertir ms musicales -> ms RÉELLES (compensé par le rate)
        let real_ms = musical_ms / self.rate;
        
        // 4. Appliquer la compensation de latence (le compteur est en avance sur le son entendu)
        real_ms - self.audio_latency_offset
    }

    pub fn start_audio_if_needed(&mut self, master_volume: f32) {
        if !self.audio_started {
            let current_time = self.get_game_time();
            // Fin du décompte
            if current_time >= 0.0 {
                if let Ok(sink) = self.audio_sink.lock() {
                    sink.set_volume(master_volume);
                    sink.play();
                    self.audio_started = true;
                }
            }
        }
    }

    pub fn reset_time(&mut self) {
        let now = Instant::now();
        self.start_time = now;
        self.head_index = 0;
        self.notes_passed = 0;
        self.combo = 0;
        self.max_combo = 0;
        self.active_notes.clear();
        self.hit_stats = HitStats::new();
        self.last_hit_timing = None;
        self.last_hit_judgement = None;
        
        for note in &mut self.chart {
            note.hit = false;
        }

        // Réinitialiser le compteur atomique
        self.played_samples.store(0, Ordering::Relaxed);
        self.audio_started = false;
        
        // Recharger l'audio proprement
        if let Ok(sink) = self.audio_sink.lock() {
            sink.stop();
            sink.clear();
            sink.set_speed(self.rate as f32);
        }

        if let Some(ref audio_path) = self.audio_path {
            match File::open(audio_path) {
                Ok(file) => match Decoder::new(BufReader::new(file)) {
                    Ok(source) => {
                        // On doit ré-appliquer le wrapper AudioMonitor
                        self.sample_rate = source.sample_rate();
                        self.channels = source.channels();
                        
                        let monitor = AudioMonitor {
                            inner: source,
                            played_samples: self.played_samples.clone(),
                        };
                        
                        if let Ok(sink) = self.audio_sink.lock() {
                            sink.append(monitor);
                            sink.pause();
                        }
                    }
                    Err(e) => eprintln!("Error decoding: {}", e),
                },
                Err(e) => eprintln!("Error loading: {}", e),
            }
        }
        
        self.replay_data = ReplayData::new();
    }
    
    pub async fn save_replay(&self, db: &crate::database::connection::Database) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref hash) = self.beatmap_hash {
            let mut replay_data_with_stats = self.replay_data.clone();
            replay_data_with_stats.hit_stats = Some(self.hit_stats.clone());
            let json_data = replay_data_with_stats.to_json()?;
            let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64;
            db.insert_replay(hash, timestamp, self.notes_passed as i32, self.hit_stats.calculate_accuracy(), self.max_combo as i32, &json_data).await?;
            Ok(())
        } else { Err("No hash".into()) }
    }
    
    pub fn set_volume(&self, volume: f32) {
        if let Ok(sink) = self.audio_sink.lock() { sink.set_volume(volume); }
    }

    // --- LOGIQUE DE JEU (Simplifiée grâce au get_game_time robuste) ---

    pub fn process_input(&mut self, column: usize) -> Option<Judgement> {
        let current_time = self.get_game_time();
        let mut best_note: Option<(usize, f64)> = None;

        for (idx, note) in self.chart.iter().enumerate().skip(self.head_index) {
            if note.column == column && !note.hit {
                // time_diff est maintenant en "ms réelles pures"
                // car note.timestamp_ms a été divisé par rate au chargement
                // et current_time est ajusté par rate dans get_game_time
                let time_diff = note.timestamp_ms - current_time;

                let (judgement, _) = self.hit_window.judge(time_diff);
                if judgement != Judgement::GhostTap {
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
            self.last_hit_timing = Some(time_diff);
            self.last_hit_judgement = Some(judgement);
            
            if judgement != Judgement::GhostTap { self.replay_data.add_hit(note_idx, time_diff); }

            match judgement {
                Judgement::Marv => { self.hit_stats.marv += 1; self.combo += 1; }
                Judgement::Perfect => { self.hit_stats.perfect += 1; self.combo += 1; }
                Judgement::Great => { self.hit_stats.great += 1; self.combo += 1; }
                Judgement::Good => { self.hit_stats.good += 1; self.combo += 1; }
                Judgement::Bad => { self.hit_stats.bad += 1; self.combo += 1; }
                Judgement::Miss => { self.hit_stats.miss += 1; self.combo = 0; }
                Judgement::GhostTap => { self.hit_stats.ghost_tap += 1; }
            }

            if self.combo > self.max_combo { self.max_combo = self.combo; }
            if judgement != Judgement::GhostTap { self.notes_passed += 1; }
            return Some(judgement);
        }

        // Gestion Ghost Tap
        let has_notes_in_column = self.chart.iter().skip(self.head_index)
            .any(|note| note.column == column && !note.hit);

        if has_notes_in_column {
            self.hit_stats.ghost_tap += 1;
            self.last_hit_timing = None;
            self.last_hit_judgement = Some(Judgement::GhostTap);
            self.replay_data.add_key_press(current_time, column);
            return Some(Judgement::GhostTap);
        }

        self.hit_stats.ghost_tap += 1;
        self.last_hit_timing = None;
        self.last_hit_judgement = Some(Judgement::GhostTap);
        self.replay_data.add_key_press(current_time, column);
        Some(Judgement::GhostTap)
    }

    pub fn update_active_notes(&mut self) {
        let current_time = self.get_game_time();
        self.active_notes.clear();

        for (idx, note) in self.chart.iter().enumerate().skip(self.head_index) {
            if !note.hit {
                let time_diff = note.timestamp_ms - current_time;
                let (judgement, _) = self.hit_window.judge(time_diff);
                if judgement != Judgement::GhostTap {
                    self.active_notes.push((idx, note.clone()));
                }
            }
        }
    }

    pub fn detect_misses(&mut self) {
        let current_time = self.get_game_time();
        for (idx, note) in self.chart.iter_mut().enumerate().skip(self.head_index) {
            if !note.hit {
                let time_diff = note.timestamp_ms - current_time;
                if time_diff < -150.0 {
                    note.hit = true;
                    self.hit_stats.miss += 1;
                    self.combo = 0;
                    self.notes_passed += 1;
                    self.replay_data.add_hit(idx, time_diff);
                }
            }
        }
    }

    pub fn get_remaining_notes(&self) -> usize {
        self.chart.iter().skip(self.head_index).filter(|note| !note.hit).count()
    }

    pub fn is_game_finished(&self) -> bool {
        if self.head_index >= self.chart.len() { return true; }
        self.chart.iter().skip(self.head_index).all(|note| note.hit)
    }

    pub fn get_visible_notes(
        &mut self,
        pixel_system: &PixelSystem,
        playfield_config: &PlayfieldConfig,
        playfield_x: f32,
        _playfield_width: f32,
    ) -> Vec<InstanceRaw> {
        let current_time = self.get_game_time();

        // Calcul de la vitesse de scroll effective.
        // On veut garder une densité visuelle constante ou accélérée ?
        // Standard jeu de rythme : Scroll Constant = Vitesse écran constante.
        // Comme 'rate' accélère le temps, si on ne touche pas scroll_speed_ms,
        // les notes défileront plus vite visuellement (ce qui est logique pour un rate > 1).
        // Si tu veux que les notes aillent "visuellement" à la même vitesse pixel/sec,
        // décommente la ligne ci-dessous :
        // let effective_scroll_speed = self.scroll_speed_ms / self.rate; 
        
        // Pour l'instant, on garde scroll_speed_ms tel quel pour un feeling naturel d'accélération
        let effective_scroll_speed = self.scroll_speed_ms / self.rate; 

        let max_future_time = current_time + effective_scroll_speed;
        let min_past_time = current_time - 200.0;

        while self.head_index < self.chart.len() {
            if self.chart[self.head_index].timestamp_ms < min_past_time {
                self.head_index += 1;
                self.notes_passed += 1;
            } else { break; }
        }

        let column_width_norm = pixel_system.pixels_to_normalized(playfield_config.column_width_pixels);
        let note_width_norm = pixel_system.pixels_to_normalized(playfield_config.note_width_pixels);
        let note_height_norm = pixel_system.pixels_to_normalized(playfield_config.note_height_pixels);
        let mut instances = Vec::with_capacity(500);

        for note in self.chart.iter().skip(self.head_index) {
            if note.timestamp_ms > max_future_time { break; }

            let time_to_hit = note.timestamp_ms - current_time;
            let progress = time_to_hit / effective_scroll_speed;

            let y_pos = HIT_LINE_Y + (VISIBLE_DISTANCE * progress as f32);
            let center_x = playfield_x + (note.column as f32 * column_width_norm) + (column_width_norm / 2.0);

            instances.push(InstanceRaw {
                offset: [center_x, y_pos],
                scale: [note_width_norm, note_height_norm],
            });
        }
        instances
    }
}