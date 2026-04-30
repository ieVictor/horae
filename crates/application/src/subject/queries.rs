use crate::subject::ports::SubjectRepository;
use std::sync::Arc;
use horae_domain::{DomainError, Subject};

pub struct GetSubjectsUseCase {
    repo: Arc<dyn SubjectRepository>,
}

impl GetSubjectsUseCase {
    pub fn new(repo: Arc<dyn SubjectRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Subject>, DomainError> {
        self.repo.find_all().await
    }
}
