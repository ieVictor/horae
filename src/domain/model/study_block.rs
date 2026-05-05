use rusqlite::{
    Row,
    types::{FromSql, FromSqlResult, ValueRef},
};
use serde::{Deserialize, Serialize};

use super::SubjectId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StudyBlockId(pub String);

impl FromSql for StudyBlockId {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(StudyBlockId(value.as_str()?.to_string()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyBlock {
    pub id: StudyBlockId,
    pub subject_id: Option<SubjectId>,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub duration: i64,
    pub created_at: i64,
}

impl TryFrom<&Row<'_>> for StudyBlock {
    type Error = rusqlite::Error;

    // Expects columns in order: id, subject_id, start_time, end_time, duration, created_at
    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(StudyBlock {
            id: row.get(0)?,
            subject_id: row.get::<_, Option<String>>(1)?.map(SubjectId),
            start_time: row.get(2)?,
            end_time: row.get::<_, Option<i64>>(3)?,
            duration: row.get(4)?,
            created_at: row.get(5)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct StudyBlockWithSubject {
    pub block: StudyBlock,
    pub subject_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DailyStudyTime {
    pub day: String,
    pub duration_secs: i64,
}
