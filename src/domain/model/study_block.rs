use rusqlite::{
    Row,
    types::{FromSql, FromSqlResult, ValueRef},
};
use serde::{Deserialize, Serialize};

use super::SubjectId;
use super::subject::SubjectStats;

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

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(StudyBlock {
            id: row.get("id")?,
            subject_id: row.get::<_, Option<String>>("subject_id")?.map(SubjectId),
            start_time: row.get("start_time")?,
            end_time: row.get("end_time")?,
            duration: row.get("duration")?,
            created_at: row.get("created_at")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct StudyBlockWithSubject {
    pub block: StudyBlock,
    pub subject: SubjectStats,
}

impl TryFrom<&Row<'_>> for StudyBlockWithSubject {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let mut subject = SubjectStats::try_from(row)?;
        subject.id = row.get("subject_id")?;
        Ok(Self {
            block: StudyBlock::try_from(row)?,
            subject,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DailyStudyTime {
    pub day: String,
    pub duration_secs: i64,
}

impl TryFrom<&Row<'_>> for DailyStudyTime {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(DailyStudyTime {
            day: row.get("day")?,
            duration_secs: row.get("duration_secs")?,
        })
    }
}
