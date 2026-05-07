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
            other => Err(FromSqlError::Other(format!("unknown status: {other}").into())),
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

    // Expects columns: id(0), subject_id(1), text(2), answer(3), status(4),
    //                  created_in_block_id(5), resolved_in_block_id(6),
    //                  created_at(7), resolved_at(8), updated_at(9)
    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Question {
            id: row.get(0)?,
            subject_id: row.get(1)?,
            text: row.get(2)?,
            answer: row.get::<_, Option<String>>(3)?,
            status: row.get(4)?,
            created_in_block_id: row.get::<_, Option<String>>(5)?.map(StudyBlockId),
            resolved_in_block_id: row.get::<_, Option<String>>(6)?.map(StudyBlockId),
            created_at: row.get(7)?,
            resolved_at: row.get::<_, Option<i64>>(8)?,
            updated_at: row.get::<_, Option<i64>>(9)?,
        })
    }
}
