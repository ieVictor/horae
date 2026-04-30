use crate::session::ports::SessionRepository;
use std::sync::Arc;
use horae_domain::{DomainError, Session};

pub struct GetSessionsUseCase {
    repo: Arc<dyn SessionRepository>,
}

impl GetSessionsUseCase {
    pub fn new(repo: Arc<dyn SessionRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Session>, DomainError> {
        self.repo.find_all().await
    }
}
