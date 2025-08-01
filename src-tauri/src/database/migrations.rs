use rusqlite_migration::{Migrations, M};

pub const MIGRATIONS_SLICE: &[M<'_>] = &[
    M::up("
    CREATE TABLE musics (
        id INTEGER PRIMARY KEY,
        path TEXT NOT NULL,
        duration INTEGER,

        title TEXT,
        artist TEXT,
        album TEXT,
        album_artist TEXT,
        track_number VARCHAR(10),
        genre TEXT,
        date TEXT,
        bits_per_sample INTEGER,
        sample_rate INTEGER,
        modified_at TEXT NOT NULL
    )")
];
pub const DATABASE_MIGRATIONS: Migrations<'_> = Migrations::from_slice(MIGRATIONS_SLICE);