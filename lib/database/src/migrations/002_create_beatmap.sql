CREATE TABLE IF NOT EXISTS beatmap (
    hash TEXT PRIMARY KEY,
    beatmapset_id INTEGER NOT NULL,
    path TEXT NOT NULL UNIQUE,
    difficulty_name TEXT,
    note_count INTEGER NOT NULL,
    duration_ms INTEGER NOT NULL DEFAULT 0,
    nps REAL NOT NULL DEFAULT 0.0,
    bpm REAL NOT NULL DEFAULT 0.0,
    key_count INTEGER NOT NULL DEFAULT 4,
    FOREIGN KEY (beatmapset_id) REFERENCES beatmapset(id) ON DELETE CASCADE
);