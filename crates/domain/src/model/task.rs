use serde::{Deserialize, Serialize};
use crate::model::session::SessionId;
use crate::model::study_block::StudyBlockId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Open,
    Completed,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
    /// Optional link to a planned Session.
    pub session_id: Option<SessionId>,
    /// Optional link to an active or past StudyBlock.
    pub study_block_id: Option<StudyBlockId>,
    pub due_date: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}
