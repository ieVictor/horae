use async_trait::async_trait;
use horae_domain::{Subject, SubjectId, DomainError};

#[async_trait]
pub trait SubjectRepository: Send + Sync {
    async fn save(&self, subject: Subject) -> Result<(), DomainError>;
    async fn find_all(&self) -> Result<Vec<Subject>, DomainError>;
    async fn find_by_id(&self, id: SubjectId) -> Result<Option<Subject>, DomainError>;
}
