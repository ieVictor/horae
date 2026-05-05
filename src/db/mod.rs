use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;

pub fn init(conn: &Connection) -> Result<(), rusqlite::Error> {
    // subjects must be created before study_blocks (FK reference)
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS subjects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            color_hex TEXT,
            is_default INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL,
            updated_at INTEGER
        );
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT CHECK(status in ('Open', 'Completed', 'Abandoned')),
            priority TEXT CHECK(priority in ('Low', 'Medium', 'High', 'Urgent')),
            created_at INTEGER,
            updated_at INTEGER
        );
        CREATE TABLE IF NOT EXISTS study_blocks (
            id TEXT PRIMARY KEY,
            subject_id TEXT REFERENCES subjects(id),
            start_time INTEGER NOT NULL,
            end_time INTEGER,
            duration INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL
        );
        ",
    )?;

    // Schema migrations for users who have an older database on disk.
    // Errors (e.g. "duplicate column name") are intentionally ignored.
    let _ = conn.execute("ALTER TABLE subjects ADD COLUMN is_default INTEGER NOT NULL DEFAULT 0", []);
    let _ = conn.execute("ALTER TABLE study_blocks ADD COLUMN subject_id TEXT REFERENCES subjects(id)", []);

    // Seed the default subject (INSERT OR IGNORE is idempotent).
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    conn.execute(
        "INSERT OR IGNORE INTO subjects (id, name, is_default, created_at)
         VALUES ('00000000-0000-0000-0000-000000000001', 'General', 1, ?1)",
        [now],
    )?;

    Ok(())
}

pub mod study_block;
pub mod subject;
pub mod task;
