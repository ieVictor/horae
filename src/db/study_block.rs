use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;
use uuid::Uuid;

use crate::domain::{StudyBlock, StudyBlockId, SubjectId};

fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

// All queries select columns in the order expected by StudyBlock::try_from:
// id, subject_id, start_time, end_time, duration, created_at

pub fn find_all(conn: &Connection) -> Result<Vec<StudyBlock>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, subject_id, start_time, end_time, duration, created_at
         FROM study_blocks
         ORDER BY start_time DESC",
    )?;

    let blocks = stmt
        .query_map([], |row| StudyBlock::try_from(row))?
        .collect::<Result<Vec<StudyBlock>, _>>()?;

    Ok(blocks)
}

pub fn find_today(conn: &Connection) -> Result<Vec<StudyBlock>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, subject_id, start_time, end_time, duration, created_at
         FROM study_blocks
         WHERE date(start_time, 'unixepoch') = date('now')",
    )?;

    let blocks = stmt
        .query_map([], |row| StudyBlock::try_from(row))?
        .collect::<Result<Vec<StudyBlock>, _>>()?;

    Ok(blocks)
}

pub fn today_total_secs(conn: &Connection) -> Result<i64, rusqlite::Error> {
    let total: i64 = conn.query_row(
        "SELECT COALESCE(SUM(duration), 0)
         FROM study_blocks
         WHERE date(start_time, 'unixepoch') = date('now')",
        [],
        |row| row.get(0),
    )?;

    Ok(total)
}

pub fn create(conn: &Connection, subject_id: &str) -> Result<StudyBlock, rusqlite::Error> {
    let id = Uuid::new_v4().to_string();
    let now = now_secs();

    conn.execute(
        "INSERT INTO study_blocks (id, subject_id, start_time, end_time, duration, created_at)
         VALUES (?1, ?2, ?3, NULL, 0, ?4)",
        (&id, subject_id, now, now),
    )?;

    Ok(StudyBlock {
        id: StudyBlockId(id),
        subject_id: Some(SubjectId(subject_id.to_string())),
        start_time: now,
        end_time: None,
        duration: 0,
        created_at: now,
    })
}

pub fn end(conn: &Connection, id: &str) -> Result<(), rusqlite::Error> {
    let now = now_secs();

    conn.execute(
        "UPDATE study_blocks
         SET end_time = ?1,
             duration = ?1 - start_time
         WHERE id = ?2",
        (now, id),
    )?;

    Ok(())
}
