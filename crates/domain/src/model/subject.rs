use serde::{Deserialize, Serialize};
use crate::model::topic::Topic;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubjectId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub id: SubjectId,
    pub name: String,
    pub color_hex: String,
    pub topics: Vec<Topic>,
    pub created_at: u64,
    pub updated_at: u64,
}
