use async_trait::async_trait;
use horae_domain::{StudyBlock, StudyBlockId, DomainError};

#[async_trait]
pub trait StudyBlockRepository: Send + Sync {
    async fn save(&self, study_block: StudyBlock) -> Result<(), DomainError>;
    async fn find_all(&self) -> Result<Vec<StudyBlock>, DomainError>;
    async fn find_by_id(&self, id: StudyBlockId) -> Result<Option<StudyBlock>, DomainError>;
}
