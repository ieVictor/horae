use crate::model::session::SessionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StudyBlockId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StudyBlockStatus {
    Active,
    Completed,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyBlock {
    pub id: StudyBlockId,
    /// Optional link to the Session (plan) that triggered this block.
    pub session_id: Option<SessionId>,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub status: StudyBlockStatus,
    pub created_at: u64,
}
