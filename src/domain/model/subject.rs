use rusqlite::{
    Row,
    types::{FromSql, FromSqlResult, ValueRef},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubjectId(pub String);

impl FromSql for SubjectId {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(SubjectId(value.as_str()?.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct SubjectStats {
    pub id: SubjectId,
    pub name: String,
    pub color_hex: String,
    pub is_default: bool,
    pub total_seconds: i64,
    pub last_session: Option<i64>,
}

impl TryFrom<&Row<'_>> for SubjectStats {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(SubjectStats {
            id: row.get("id")?,
            name: row.get("name")?,
            color_hex: row.get::<_, Option<String>>("color_hex")?.unwrap_or_else(|| "#c0c0c0".to_string()),
            is_default: row.get::<_, i64>("is_default")? != 0,
            total_seconds: row.get("total_seconds")?,
            last_session: row.get("last_session")?,
        })
    }
}
