use rusqlite::Connection;
use uuid::Uuid;

use crate::domain::{
    DailyStudyTime, StudyBlock, StudyBlockId, StudyBlockWithSubject, SubjectId,
};

use super::now_secs;


pub fn find_all_with_subject(
    conn: &Connection,
) -> Result<Vec<StudyBlockWithSubject>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT *, 0 AS total_seconds, NULL AS last_session
         FROM study_blocks
         LEFT JOIN subjects ON study_blocks.subject_id = subjects.id
         ORDER BY start_time DESC",
    )?;

    let blocks = stmt
        .query_map([], |row| StudyBlockWithSubject::try_from(row))?
        .collect::<Result<Vec<StudyBlockWithSubject>, _>>()?;

    Ok(blocks)
}

pub fn weekly_stats(conn: &Connection) -> Result<Vec<DailyStudyTime>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "WITH RECURSIVE
          days(day, i) AS (
            SELECT date('now', 'localtime', '-6 days', 'weekday 0'), 0
            UNION ALL
            SELECT date(day, '+1 day'), i + 1 FROM days WHERE i < 6
          )
         SELECT
          d.day,
          COALESCE(SUM(b.duration), 0)
         FROM days d
         LEFT JOIN study_blocks b ON date(b.start_time, 'unixepoch', 'localtime') = d.day
         GROUP BY d.day
         ORDER BY d.day ASC",
    )?;

    let stats = stmt
        .query_map([], |row| {
            Ok(DailyStudyTime {
                day: row.get(0)?,
                duration_secs: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<DailyStudyTime>, _>>()?;

    Ok(stats)
}

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
         WHERE date(start_time, 'unixepoch', 'localtime') = date('now', 'localtime')",
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
         WHERE date(start_time, 'unixepoch', 'localtime') = date('now', 'localtime')",
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
