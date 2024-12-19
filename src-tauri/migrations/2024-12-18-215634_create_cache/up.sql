-- Your SQL goes here
CREATE TABLE series (
    id INTEGER PRIMARY KEY,
    series_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    poster_path TEXT
);

CREATE TABLE seasons (
    id INTEGER PRIMARY KEY,
    season_id INTEGER NOT NULL,
    series_id INTEGER NOT NULL,
    season_number INTEGER NOT NULL,
    name TEXT NOT NULL,
    overview TEXT,
    poster_path TEXT,
    FOREIGN KEY (series_id) REFERENCES series (id)
);

CREATE TABLE episodes (
    id INTEGER PRIMARY KEY,
    episode_id INTEGER NOT NULL,
    season_id INTEGER NOT NULL,
    episode_number INTEGER NOT NULL,
    name TEXT NOT NULL,
    air_date DATE NOT NULL,
    overview TEXT,
    still_path TEXT,
    plex_path TEXT,
    watched BOOLEAN NOT NULL,
    watched_date DATE,
    FOREIGN KEY (season_id) REFERENCES seasons (id)
);
