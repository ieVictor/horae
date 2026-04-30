use crate::model::subject::SubjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SessionId(pub String);

/// Represents a scheduled intent to study a subject.
/// Follows RFC 5545 (iCalendar) concepts for scheduling.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: SessionId,
    /// A session belongs to exactly one subject.
    pub subject_id: SubjectId,
    pub name: String,
    /// Start time of the first occurrence (Unix timestamp).
    pub dtstart: u64,
    /// Duration of the study session in minutes.
    pub duration_minutes: u32,
    /// Recurrence rule string (e.g., "FREQ=WEEKLY;BYDAY=SU").
    /// If None, the session is one-off.
    pub rrule: Option<String>,
    /// List of timestamps for excluded occurrences.
    pub exdates: Vec<u64>,
    pub created_at: u64,
}

impl Session {
    pub fn is_recurring(&self) -> bool {
        self.rrule.is_some()
    }
}
