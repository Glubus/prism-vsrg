CREATE TABLE IF NOT EXISTS beatmapset (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    image_path TEXT,
    artist TEXT,
    title TEXT
);

