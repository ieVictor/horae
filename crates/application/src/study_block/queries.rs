use crate::study_block::ports::StudyBlockRepository;
use std::sync::Arc;
use horae_domain::{DomainError, StudyBlock};

pub struct GetStudyBlocksUseCase {
    repo: Arc<dyn StudyBlockRepository>,
}

impl GetStudyBlocksUseCase {
    pub fn new(repo: Arc<dyn StudyBlockRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<StudyBlock>, DomainError> {
        self.repo.find_all().await
    }
}
