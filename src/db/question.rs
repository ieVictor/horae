use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, params};
use uuid::Uuid;

use crate::domain::Question;

fn now_secs() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

const COLS: &str =
    "id, subject_id, text, answer, status, \
     created_in_block_id, resolved_in_block_id, \
     created_at, resolved_at, updated_at";

pub fn find_open_for_subject(
    conn: &Connection,
    subject_id: &str,
) -> Result<Vec<Question>, rusqlite::Error> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {COLS} FROM questions
         WHERE subject_id = ?1 AND status = 'Open'
         ORDER BY created_at DESC"
    ))?;
    stmt.query_map(params![subject_id], |row| Question::try_from(row))?
        .collect::<Result<Vec<_>, _>>()
}

pub fn find_all_for_subject(
    conn: &Connection,
    subject_id: &str,
) -> Result<Vec<Question>, rusqlite::Error> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {COLS} FROM questions
         WHERE subject_id = ?1
         ORDER BY created_at DESC"
    ))?;
    stmt.query_map(params![subject_id], |row| Question::try_from(row))?
        .collect::<Result<Vec<_>, _>>()
}

pub fn create(
    conn: &Connection,
    text: &str,
    subject_id: &str,
    block_id: Option<&str>,
) -> Result<(), rusqlite::Error> {
    let id = Uuid::new_v4().to_string();
    let now = now_secs();
    conn.execute(
        "INSERT INTO questions
         (id, subject_id, text, answer, status,
          created_in_block_id, resolved_in_block_id,
          created_at, resolved_at, updated_at)
         VALUES (?1, ?2, ?3, NULL, 'Open', ?4, NULL, ?5, NULL, NULL)",
        params![id, subject_id, text, block_id, now],
    )?;
    Ok(())
}

pub fn resolve(
    conn: &Connection,
    id: &str,
    answer: Option<&str>,
    block_id: Option<&str>,
) -> Result<(), rusqlite::Error> {
    let now = now_secs();
    conn.execute(
        "UPDATE questions
         SET status = 'Resolved',
             answer = ?2,
             resolved_in_block_id = ?3,
             resolved_at = ?4,
             updated_at = ?4
         WHERE id = ?1",
        params![id, answer, block_id, now],
    )?;
    Ok(())
}

pub fn reopen(conn: &Connection, id: &str) -> Result<(), rusqlite::Error> {
    let now = now_secs();
    conn.execute(
        "UPDATE questions
         SET status = 'Open',
             answer = NULL,
             resolved_in_block_id = NULL,
             resolved_at = NULL,
             updated_at = ?2
         WHERE id = ?1",
        params![id, now],
    )?;
    Ok(())
}
