CREATE TABLE IF NOT EXISTS replay (
    hash TEXT PRIMARY KEY,
    beatmap_hash TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    score INTEGER NOT NULL,
    accuracy REAL NOT NULL,
    max_combo INTEGER NOT NULL,
    rate REAL NOT NULL DEFAULT 1.0,
    data TEXT NOT NULL,
    FOREIGN KEY (beatmap_hash) REFERENCES beatmap(hash) ON DELETE CASCADE
);

