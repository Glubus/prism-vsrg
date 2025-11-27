CREATE TABLE IF NOT EXISTS beatmap_rating (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    beatmap_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    overall REAL NOT NULL DEFAULT 0.0,
    stream REAL NOT NULL DEFAULT 0.0,
    jumpstream REAL NOT NULL DEFAULT 0.0,
    handstream REAL NOT NULL DEFAULT 0.0,
    stamina REAL NOT NULL DEFAULT 0.0,
    jackspeed REAL NOT NULL DEFAULT 0.0,
    chordjack REAL NOT NULL DEFAULT 0.0,
    technical REAL NOT NULL DEFAULT 0.0,
    FOREIGN KEY (beatmap_hash) REFERENCES beatmap(hash) ON DELETE CASCADE,
    UNIQUE (beatmap_hash, name)
);


