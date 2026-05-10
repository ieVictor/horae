use rusqlite::{
    Row,
    types::{FromSql, FromSqlError, FromSqlResult, ValueRef},
};

use super::{StudyBlockId, SubjectId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionId(pub String);

impl FromSql for QuestionId {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(QuestionId(value.as_str()?.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuestionStatus {
    Open,
    Resolved,
}

impl FromSql for QuestionStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "Open" => Ok(QuestionStatus::Open),
            "Resolved" => Ok(QuestionStatus::Resolved),
            other => Err(FromSqlError::Other(
                format!("unknown status: {other}").into(),
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Question {
    pub id: QuestionId,
    pub subject_id: SubjectId,
    pub text: String,
    pub answer: Option<String>,
    pub status: QuestionStatus,
    pub created_in_block_id: Option<StudyBlockId>,
    pub resolved_in_block_id: Option<StudyBlockId>,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl TryFrom<&Row<'_>> for Question {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Question {
            id: row.get("id")?,
            subject_id: row.get("subject_id")?,
            text: row.get("text")?,
            answer: row.get::<_, Option<String>>("answer")?,
            status: row.get("status")?,
            created_in_block_id: row
                .get::<_, Option<String>>("created_in_block_id")?
                .map(StudyBlockId),
            resolved_in_block_id: row
                .get::<_, Option<String>>("resolved_in_block_id")?
                .map(StudyBlockId),
            created_at: row.get("created_at")?,
            resolved_at: row.get::<_, Option<i64>>("resolved_at")?,
            updated_at: row.get::<_, Option<i64>>("updated_at")?,
        })
    }
}
