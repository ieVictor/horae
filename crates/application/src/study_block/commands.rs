use crate::helpers::{generate_uuid, now_secs};
use crate::study_block::ports::StudyBlockRepository;
use std::sync::Arc;
use horae_domain::{DomainError, SessionId, StudyBlock, StudyBlockId, StudyBlockStatus};

pub struct StartStudyBlockUseCase {
    repo: Arc<dyn StudyBlockRepository>,
}

impl StartStudyBlockUseCase {
    pub fn new(repo: Arc<dyn StudyBlockRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        session_id: Option<SessionId>,
    ) -> Result<StudyBlock, DomainError> {
        let id = StudyBlockId(generate_uuid());
        let now = now_secs();

        let block = StudyBlock {
            id,
            session_id,
            start_time: now,
            end_time: None,
            status: StudyBlockStatus::Active,
            created_at: now,
        };

        self.repo.save(block.clone()).await?;
        Ok(block)
    }
}

pub struct CompleteStudyBlockUseCase {
    repo: Arc<dyn StudyBlockRepository>,
}

impl CompleteStudyBlockUseCase {
    pub fn new(repo: Arc<dyn StudyBlockRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: StudyBlockId) -> Result<(), DomainError> {
        let mut block = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or(DomainError::NotFound)?;
        let now = now_secs();

        block.end_time = Some(now);
        block.status = StudyBlockStatus::Completed;
        self.repo.save(block).await
    }
}
