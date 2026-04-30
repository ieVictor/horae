use async_trait::async_trait;
use horae_domain::{Session, DomainError};

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn save(&self, session: Session) -> Result<(), DomainError>;
    async fn find_all(&self) -> Result<Vec<Session>, DomainError>;
    async fn find_by_id(&self, id: horae_domain::SessionId) -> Result<Option<Session>, DomainError>;
}
