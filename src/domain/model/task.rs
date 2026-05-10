use rusqlite::{
    Row,
    types::{FromSql, FromSqlError, FromSqlResult, ValueRef},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskId(pub String);

impl FromSql for TaskId {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(TaskId(value.as_str()?.to_string()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Open,
    Completed,
    Abandoned,
}

impl FromSql for TaskStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "Open" => Ok(TaskStatus::Open),
            "Completed" => Ok(TaskStatus::Completed),
            "Abandoned" => Ok(TaskStatus::Abandoned),
            other => Err(FromSqlError::Other(
                format!("Unknown priority: {other}").into(),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl FromSql for Priority {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "Low" => Ok(Priority::Low),
            "Medium" => Ok(Priority::Medium),
            "High" => Ok(Priority::High),
            "Urgent" => Ok(Priority::Urgent),
            other => Err(FromSqlError::Other(
                format!("Unknown priority: {other}").into(),
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

impl TryFrom<&Row<'_>> for Task {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Task {
            id: row.get("id")?,
            title: row.get("title")?,
            description: row.get::<_, Option<String>>("description")?,
            status: row.get("status")?,
            priority: row.get("priority")?,
            created_at: row.get("created_at")?,
            updated_at: row.get::<_, Option<i64>>("updated_at")?,
        })
    }
}
