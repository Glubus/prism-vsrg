use crate::database::models::{BeatmapRating, BeatmapWithRatings, Beatmapset};
use crate::database::query;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::path::{Path, PathBuf};

const MIGRATION_CREATE_BEATMAPSET: &str = include_str!("migrations/001_create_beatmapset.sql");
const MIGRATION_CREATE_BEATMAP: &str = include_str!("migrations/002_create_beatmap.sql");
const MIGRATION_CREATE_REPLAY: &str = include_str!("migrations/003_create_replay.sql");
const MIGRATION_CREATE_BEATMAP_RATING: &str =
    include_str!("migrations/005_create_beatmap_rating.sql");

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Ouvre ou crée la base de données
    pub async fn new(db_path: &Path) -> Result<Self, sqlx::Error> {
        // S'assurer que le répertoire parent existe
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    return Err(sqlx::Error::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Unable to create parent directory: {}", e),
                    )));
                }
            }
        }

        // Pour sqlx avec SQLite, convertir le chemin en chemin absolu
        let absolute_path = if db_path.is_absolute() {
            db_path.to_path_buf()
        } else {
            // Convertir en chemin absolu depuis le répertoire de travail courant
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(db_path)
        };

        // Utiliser SqliteConnectOptions directement avec le chemin du fichier
        // create_if_missing(true) crée automatiquement le fichier s'il n'existe pas
        let options = SqliteConnectOptions::new()
            .filename(&absolute_path)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;
        let db = Database { pool };
        db.init_schema().await?;
        Ok(db)
    }

    /// Initialise les tables si elles n'existent pas
    async fn init_schema(&self) -> Result<(), sqlx::Error> {
        for migration in [
            MIGRATION_CREATE_BEATMAPSET,
            MIGRATION_CREATE_BEATMAP,
            MIGRATION_CREATE_REPLAY,
            MIGRATION_CREATE_BEATMAP_RATING,
        ] {
            sqlx::query(migration).execute(&self.pool).await?;
        }

        Ok(())
    }

    /// Retourne une référence au pool pour les requêtes
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Vide toutes les tables (pour rescan)
    pub async fn clear_all(&self) -> Result<(), sqlx::Error> {
        query::clear_all(&self.pool).await
    }

    /// Insère ou met à jour un beatmapset
    pub async fn insert_beatmapset(
        &self,
        path: &str,
        image_path: Option<&str>,
        artist: Option<&str>,
        title: Option<&str>,
    ) -> Result<i64, sqlx::Error> {
        query::insert_beatmapset(&self.pool, path, image_path, artist, title).await
    }

    /// Insère ou met à jour une beatmap
    pub async fn insert_beatmap(
        &self,
        beatmapset_id: i64,
        hash: &str,
        path: &str,
        difficulty_name: Option<&str>,
        note_count: i32,
        duration_ms: i32,
        nps: f64,
    ) -> Result<String, sqlx::Error> {
        query::insert_beatmap(
            &self.pool,
            beatmapset_id,
            hash,
            path,
            difficulty_name,
            note_count,
            duration_ms,
            nps,
        )
        .await
    }

    /// Insère ou met à jour un rating pour une beatmap
    pub async fn upsert_beatmap_rating(
        &self,
        beatmap_hash: &str,
        name: &str,
        overall: f64,
        stream: f64,
        jumpstream: f64,
        handstream: f64,
        stamina: f64,
        jackspeed: f64,
        chordjack: f64,
        technical: f64,
    ) -> Result<(), sqlx::Error> {
        query::upsert_beatmap_rating(
            &self.pool,
            beatmap_hash,
            name,
            overall,
            stream,
            jumpstream,
            handstream,
            stamina,
            jackspeed,
            chordjack,
            technical,
        )
        .await
    }

    /// Récupère les ratings d'une beatmap
    pub async fn get_ratings_for_beatmap(
        &self,
        beatmap_hash: &str,
    ) -> Result<Vec<BeatmapRating>, sqlx::Error> {
        query::get_ratings_for_beatmap(&self.pool, beatmap_hash).await
    }

    /// Récupère tous les ratings
    pub async fn get_all_beatmap_ratings(&self) -> Result<Vec<BeatmapRating>, sqlx::Error> {
        query::get_all_beatmap_ratings(&self.pool).await
    }

    /// Récupère tous les beatmapsets avec leurs beatmaps
    pub async fn get_all_beatmapsets(
        &self,
    ) -> Result<Vec<(Beatmapset, Vec<BeatmapWithRatings>)>, sqlx::Error> {
        query::get_all_beatmapsets(&self.pool).await
    }

    /// Compte le nombre total de beatmapsets
    pub async fn count_beatmapsets(&self) -> Result<i32, sqlx::Error> {
        query::count_beatmapsets(&self.pool).await
    }

    /// Insère un replay
    pub async fn insert_replay(
        &self,
        beatmap_hash: &str,
        timestamp: i64,
        score: i32,
        accuracy: f64,
        max_combo: i32,
        rate: f64,
        data: &str,
    ) -> Result<String, sqlx::Error> {
        query::insert_replay(
            &self.pool,
            beatmap_hash,
            timestamp,
            score,
            accuracy,
            max_combo,
            rate,
            data,
        )
        .await
    }

    /// Récupère tous les replays pour une beatmap
    pub async fn get_replays_for_beatmap(
        &self,
        beatmap_hash: &str,
    ) -> Result<Vec<crate::database::models::Replay>, sqlx::Error> {
        query::get_replays_for_beatmap(&self.pool, beatmap_hash).await
    }

    /// Récupère les meilleurs scores par accuracy
    pub async fn get_top_scores(
        &self,
        limit: i32,
    ) -> Result<Vec<crate::database::models::Replay>, sqlx::Error> {
        query::get_top_scores(&self.pool, limit).await
    }
}
