use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, params};
use uuid::Uuid;

use crate::domain::{StudyBlock, SubjectStats};

fn now_secs() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

pub fn find_all_summary(conn: &Connection) -> Result<Vec<SubjectStats>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT
            s.id,
            s.name,
            COALESCE(s.color_hex, '#c0c0c0') AS color_hex,
            s.is_default,
            COALESCE(SUM(sb.duration), 0)    AS total_seconds,
            MAX(sb.start_time)               AS last_session
         FROM subjects s
         LEFT JOIN study_blocks sb ON sb.subject_id = s.id
         GROUP BY s.id
         ORDER BY s.name",
    )?;

    let rows = stmt
        .query_map([], |row| SubjectStats::try_from(row))?
        .collect::<Result<Vec<SubjectStats>, _>>()?;

    Ok(rows)
}

pub fn create(conn: &Connection, name: &str, color_hex: &str) -> Result<(), rusqlite::Error> {
    let id = Uuid::new_v4().to_string();
    let now = now_secs();
    conn.execute(
        "INSERT INTO subjects (id, name, is_default, color_hex, created_at) VALUES (?1, ?2, 0, ?3, ?4)",
        (&id, name, color_hex, now),
    )?;
    Ok(())
}

pub fn delete(conn: &Connection, id: &str) -> Result<(), rusqlite::Error> {
    // is_default = 0 guard prevents deleting the default subject even if called.
    conn.execute("DELETE FROM subjects WHERE id = ?1 AND is_default = 0", [id])?;
    Ok(())
}

pub fn find_blocks(
    conn: &Connection,
    subject_id: &str,
    limit: i64,
) -> Result<Vec<StudyBlock>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, subject_id, start_time, end_time, duration, created_at
         FROM study_blocks
         WHERE subject_id = ?1
         ORDER BY start_time DESC
         LIMIT ?2",
    )?;

    let blocks = stmt
        .query_map(params![subject_id, limit], |row| StudyBlock::try_from(row))?
        .collect::<Result<Vec<StudyBlock>, _>>()?;

    Ok(blocks)
}
