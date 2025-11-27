use crate::database::models::{Beatmap, BeatmapRating, BeatmapWithRatings, Beatmapset, Replay};
use sqlx::SqlitePool;
use std::collections::HashMap;

/// Vide toutes les tables (pour rescan)
pub async fn clear_all(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM beatmap_rating")
        .execute(pool)
        .await?;
    sqlx::query("DELETE FROM replay").execute(pool).await?;
    sqlx::query("DELETE FROM beatmap").execute(pool).await?;
    sqlx::query("DELETE FROM beatmapset").execute(pool).await?;
    Ok(())
}

/// Insère ou met à jour un beatmapset
pub async fn insert_beatmapset(
    pool: &SqlitePool,
    path: &str,
    image_path: Option<&str>,
    artist: Option<&str>,
    title: Option<&str>,
) -> Result<i64, sqlx::Error> {
    // Vérifier si le beatmapset existe déjà
    let existing: Option<i64> = sqlx::query_scalar("SELECT id FROM beatmapset WHERE path = ?1")
        .bind(path)
        .fetch_optional(pool)
        .await?;

    match existing {
        Some(id) => {
            // Mettre à jour
            sqlx::query(
                "UPDATE beatmapset SET image_path = ?1, artist = ?2, title = ?3 WHERE id = ?4",
            )
            .bind(image_path)
            .bind(artist)
            .bind(title)
            .bind(id)
            .execute(pool)
            .await?;
            Ok(id)
        }
        None => {
            // Insérer
            let result = sqlx::query(
                "INSERT INTO beatmapset (path, image_path, artist, title) VALUES (?1, ?2, ?3, ?4)",
            )
            .bind(path)
            .bind(image_path)
            .bind(artist)
            .bind(title)
            .execute(pool)
            .await?;
            Ok(result.last_insert_rowid())
        }
    }
}

/// Insère ou met à jour une beatmap
pub async fn insert_beatmap(
    pool: &SqlitePool,
    beatmapset_id: i64,
    hash: &str,
    path: &str,
    difficulty_name: Option<&str>,
    note_count: i32,
    duration_ms: i32,
    nps: f64,
) -> Result<String, sqlx::Error> {
    // Vérifier si la beatmap existe déjà par hash
    let existing: Option<String> = sqlx::query_scalar("SELECT hash FROM beatmap WHERE hash = ?1")
        .bind(hash)
        .fetch_optional(pool)
        .await?;

    match existing {
        Some(existing_hash) => {
            // Mettre à jour
            sqlx::query(
                "UPDATE beatmap SET beatmapset_id = ?1, path = ?2, difficulty_name = ?3, note_count = ?4, duration_ms = ?5, nps = ?6 WHERE hash = ?7"
            )
            .bind(beatmapset_id)
            .bind(path)
            .bind(difficulty_name)
            .bind(note_count)
            .bind(duration_ms)
            .bind(nps)
            .bind(&existing_hash)
            .execute(pool)
            .await?;
            Ok(existing_hash)
        }
        None => {
            // Insérer
            sqlx::query(
                "INSERT INTO beatmap (hash, beatmapset_id, path, difficulty_name, note_count, duration_ms, nps) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
            )
            .bind(hash)
            .bind(beatmapset_id)
            .bind(path)
            .bind(difficulty_name)
            .bind(note_count)
            .bind(duration_ms)
            .bind(nps)
            .execute(pool)
            .await?;
            Ok(hash.to_string())
        }
    }
}

/// Insère ou met à jour un rating pour une beatmap
pub async fn upsert_beatmap_rating(
    pool: &SqlitePool,
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
    sqlx::query(
        "INSERT INTO beatmap_rating (
            beatmap_hash, name, overall, stream, jumpstream, handstream, stamina, jackspeed, chordjack, technical
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
         ON CONFLICT(beatmap_hash, name) DO UPDATE SET
            overall = excluded.overall,
            stream = excluded.stream,
            jumpstream = excluded.jumpstream,
            handstream = excluded.handstream,
            stamina = excluded.stamina,
            jackspeed = excluded.jackspeed,
            chordjack = excluded.chordjack,
            technical = excluded.technical",
    )
    .bind(beatmap_hash)
    .bind(name)
    .bind(overall)
    .bind(stream)
    .bind(jumpstream)
    .bind(handstream)
    .bind(stamina)
    .bind(jackspeed)
    .bind(chordjack)
    .bind(technical)
    .execute(pool)
    .await?;
    Ok(())
}

/// Récupère tous les ratings d'une beatmap
pub async fn get_ratings_for_beatmap(
    pool: &SqlitePool,
    beatmap_hash: &str,
) -> Result<Vec<BeatmapRating>, sqlx::Error> {
    let ratings: Vec<BeatmapRating> = sqlx::query_as(
        "SELECT id, beatmap_hash, name, overall, stream, jumpstream, handstream, stamina, jackspeed, chordjack, technical
         FROM beatmap_rating WHERE beatmap_hash = ?1 ORDER BY name",
    )
    .bind(beatmap_hash)
    .fetch_all(pool)
    .await?;
    Ok(ratings)
}

/// Récupère tous les ratings
pub async fn get_all_beatmap_ratings(
    pool: &SqlitePool,
) -> Result<Vec<BeatmapRating>, sqlx::Error> {
    let ratings: Vec<BeatmapRating> = sqlx::query_as(
        "SELECT id, beatmap_hash, name, overall, stream, jumpstream, handstream, stamina, jackspeed, chordjack, technical FROM beatmap_rating",
    )
    .fetch_all(pool)
    .await?;
    Ok(ratings)
}

/// Récupère tous les beatmapsets avec leurs beatmaps
pub async fn get_all_beatmapsets(
    pool: &SqlitePool,
) -> Result<Vec<(Beatmapset, Vec<BeatmapWithRatings>)>, sqlx::Error> {
    let beatmapsets: Vec<Beatmapset> = sqlx::query_as(
        "SELECT id, path, image_path, artist, title FROM beatmapset ORDER BY artist, title",
    )
    .fetch_all(pool)
    .await?;

    let ratings = get_all_beatmap_ratings(pool).await?;
    let mut ratings_map: HashMap<String, Vec<BeatmapRating>> = HashMap::new();
    for rating in ratings {
        ratings_map
            .entry(rating.beatmap_hash.clone())
            .or_default()
            .push(rating);
    }

    let mut result = Vec::new();
    for beatmapset in beatmapsets {
        let beatmaps: Vec<Beatmap> = sqlx::query_as(
            "SELECT hash, beatmapset_id, path, difficulty_name, note_count, duration_ms, nps FROM beatmap WHERE beatmapset_id = ?1 ORDER BY difficulty_name"
        )
        .bind(beatmapset.id)
        .fetch_all(pool)
        .await?;

        let with_ratings = beatmaps
            .into_iter()
            .map(|beatmap| {
                let ratings = ratings_map.remove(&beatmap.hash).unwrap_or_default();
                BeatmapWithRatings::new(beatmap, ratings)
            })
            .collect();

        result.push((beatmapset, with_ratings));
    }

    Ok(result)
}

/// Compte le nombre total de beatmapsets
pub async fn count_beatmapsets(pool: &SqlitePool) -> Result<i32, sqlx::Error> {
    let count: Option<i64> = sqlx::query_scalar("SELECT COUNT(*) FROM beatmapset")
        .fetch_optional(pool)
        .await?;
    Ok(count.unwrap_or(0) as i32)
}

/// Insère un replay en calculant automatiquement son hash
pub async fn insert_replay(
    pool: &SqlitePool,
    beatmap_hash: &str,
    timestamp: i64,
    score: i32,
    accuracy: f64,
    max_combo: i32,
    rate: f64,
    data: &str,
) -> Result<String, sqlx::Error> {
    let hash_input = format!(
        "{}:{}:{}:{}:{}:{}:{}",
        beatmap_hash, timestamp, score, accuracy, max_combo, rate, data
    );
    let hash = format!("{:x}", md5::compute(hash_input));

    sqlx::query(
        "INSERT INTO replay (hash, beatmap_hash, timestamp, score, accuracy, max_combo, rate, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
    )
    .bind(&hash)
    .bind(beatmap_hash)
    .bind(timestamp)
    .bind(score)
    .bind(accuracy)
    .bind(max_combo)
    .bind(rate)
    .bind(data)
    .execute(pool)
    .await?;
    Ok(hash)
}

/// Récupère tous les replays pour une beatmap, triés par rate puis accuracy (meilleurs scores en premier)
pub async fn get_replays_for_beatmap(
    pool: &SqlitePool,
    beatmap_hash: &str,
) -> Result<Vec<Replay>, sqlx::Error> {
    let replays: Vec<Replay> = sqlx::query_as(
        "SELECT hash, beatmap_hash, timestamp, score, accuracy, max_combo, rate, data FROM replay WHERE beatmap_hash = ?1 ORDER BY rate DESC, accuracy DESC, timestamp DESC LIMIT 10"
    )
    .bind(beatmap_hash)
    .fetch_all(pool)
    .await?;
    Ok(replays)
}

/// Récupère les meilleurs scores triés par rate puis accuracy (toutes beatmaps confondues)
pub async fn get_top_scores(pool: &SqlitePool, limit: i32) -> Result<Vec<Replay>, sqlx::Error> {
    let replays: Vec<Replay> = sqlx::query_as(
        "SELECT hash, beatmap_hash, timestamp, score, accuracy, max_combo, rate, data FROM replay ORDER BY rate DESC, accuracy DESC, timestamp DESC LIMIT ?1"
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(replays)
}
