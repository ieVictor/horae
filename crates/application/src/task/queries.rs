use crate::task::ports::TaskRepository;
use std::sync::Arc;
use horae_domain::{DomainError, SessionId, StudyBlockId, Task};

pub struct GetTasksUseCase {
    repo: Arc<dyn TaskRepository>,
}

impl GetTasksUseCase {
    pub fn new(repo: Arc<dyn TaskRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Task>, DomainError> {
        self.repo.find_all().await
    }
}

pub struct GetTasksBySessionUseCase {
    repo: Arc<dyn TaskRepository>,
}

impl GetTasksBySessionUseCase {
    pub fn new(repo: Arc<dyn TaskRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, session_id: SessionId) -> Result<Vec<Task>, DomainError> {
        self.repo.find_by_session_id(session_id).await
    }
}

pub struct GetTasksByStudyBlockUseCase {
    repo: Arc<dyn TaskRepository>,
}

impl GetTasksByStudyBlockUseCase {
    pub fn new(repo: Arc<dyn TaskRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, study_block_id: StudyBlockId) -> Result<Vec<Task>, DomainError> {
        self.repo.find_by_study_block_id(study_block_id).await
    }
}
