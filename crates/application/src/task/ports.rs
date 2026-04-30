use async_trait::async_trait;
use horae_domain::{Task, TaskId, StudyBlockId, DomainError};

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn save(&self, task: Task) -> Result<(), DomainError>;
    async fn find_all(&self) -> Result<Vec<Task>, DomainError>;
    async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, DomainError>;
    async fn find_by_session_id(&self, session_id: horae_domain::SessionId) -> Result<Vec<Task>, DomainError>;
    async fn find_by_study_block_id(&self, study_block_id: StudyBlockId) -> Result<Vec<Task>, DomainError>;
}
