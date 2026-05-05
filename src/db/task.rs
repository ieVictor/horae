use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;
use uuid::Uuid;

use crate::domain::{Priority, Task};

fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

fn priority_str(p: Priority) -> &'static str {
    match p {
        Priority::Low => "Low",
        Priority::Medium => "Medium",
        Priority::High => "High",
        Priority::Urgent => "Urgent",
    }
}

pub fn find_all(conn: &Connection) -> Result<Vec<Task>, rusqlite::Error> {
    let mut stmt =
        conn.prepare("SELECT * FROM tasks ORDER BY created_at DESC")?;

    let tasks = stmt
        .query_map([], |row| Task::try_from(row))?
        .collect::<Result<Vec<Task>, _>>()?;

    Ok(tasks)
}

pub fn create(
    conn: &Connection,
    title: &str,
    description: Option<&str>,
    priority: Priority,
) -> Result<(), rusqlite::Error> {
    let id = Uuid::new_v4().to_string();
    let now = now_secs();

    conn.execute(
        "INSERT INTO tasks (id, title, description, status, priority, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (id, title, description, "Open", priority_str(priority), now, now),
    )?;

    Ok(())
}

pub fn delete(conn: &Connection, id: &str) -> Result<(), rusqlite::Error> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
    Ok(())
}

pub fn toggle_status(conn: &Connection, id: &str) -> Result<(), rusqlite::Error> {
    let now = now_secs();
    conn.execute(
        "UPDATE tasks
         SET status     = CASE WHEN status = 'Open' THEN 'Completed' ELSE 'Open' END,
             updated_at = ?1
         WHERE id = ?2",
        (now, id),
    )?;
    Ok(())
}
