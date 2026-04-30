use async_trait::async_trait;
use std::sync::Mutex;
use horae_application::task::ports::TaskRepository;
use horae_domain::{Task, TaskId, SessionId, StudyBlockId, DomainError};

pub struct InMemoryTaskRepository {
    tasks: Mutex<Vec<Task>>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl TaskRepository for InMemoryTaskRepository {
    async fn save(&self, task: Task) -> Result<(), DomainError> {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(index) = tasks.iter().position(|t| t.id == task.id) {
            tasks[index] = task;
        } else {
            tasks.push(task);
        }
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Task>, DomainError> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks.clone())
    }

    async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, DomainError> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks.iter().find(|t| t.id == id).cloned())
    }

    async fn find_by_session_id(&self, session_id: SessionId) -> Result<Vec<Task>, DomainError> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks.iter().filter(|t| t.session_id == Some(session_id.clone())).cloned().collect())
    }

    async fn find_by_study_block_id(&self, study_block_id: StudyBlockId) -> Result<Vec<Task>, DomainError> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks.iter().filter(|t| t.study_block_id == Some(study_block_id.clone())).cloned().collect())
    }
}
