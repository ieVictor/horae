use crate::helpers::{generate_uuid, now_secs};
use crate::task::ports::TaskRepository;
use std::sync::Arc;
use horae_domain::{
    DomainError, Priority, SessionId, StudyBlockId, Task, TaskId, TaskStatus,
};

pub struct CreateTaskUseCase {
    repo: Arc<dyn TaskRepository>,
}

impl CreateTaskUseCase {
    pub fn new(repo: Arc<dyn TaskRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        title: String,
        priority: Priority,
        session_id: Option<SessionId>,
        study_block_id: Option<StudyBlockId>,
    ) -> Result<Task, DomainError> {
        let id = TaskId(generate_uuid());
        let now = now_secs();

        let task = Task {
            id,
            title,
            description: None,
            status: TaskStatus::Open,
            priority,
            session_id,
            study_block_id,
            due_date: None,
            created_at: now,
            updated_at: now,
        };

        self.repo.save(task.clone()).await?;
        Ok(task)
    }
}

pub struct AssignTaskToSessionUseCase {
    repo: Arc<dyn TaskRepository>,
}

impl AssignTaskToSessionUseCase {
    pub fn new(repo: Arc<dyn TaskRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        task_id: TaskId,
        session_id: Option<SessionId>,
    ) -> Result<(), DomainError> {
        let mut task = self
            .repo
            .find_by_id(task_id)
            .await?
            .ok_or(DomainError::NotFound)?;
        task.session_id = session_id;
        task.updated_at = now_secs();
        self.repo.save(task).await
    }
}
