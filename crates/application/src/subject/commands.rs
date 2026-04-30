use crate::helpers::{generate_uuid, now_secs};
use crate::subject::ports::SubjectRepository;
use std::sync::Arc;
use horae_domain::{DomainError, Subject, SubjectId};

pub struct CreateSubjectUseCase {
    repo: Arc<dyn SubjectRepository>,
}

impl CreateSubjectUseCase {
    pub fn new(repo: Arc<dyn SubjectRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, name: String, color_hex: String) -> Result<Subject, DomainError> {
        let id = SubjectId(generate_uuid());
        let now = now_secs();

        let subject = Subject {
            id,
            name,
            color_hex,
            topics: vec![],
            created_at: now,
            updated_at: now,
        };

        self.repo.save(subject.clone()).await?;
        Ok(subject)
    }
}
