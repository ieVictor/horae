use crate::helpers::{generate_uuid, now_secs};
use crate::session::ports::SessionRepository;
use std::sync::Arc;
use horae_domain::{DomainError, Session, SessionId, SubjectId};

pub struct CreateSessionUseCase {
    repo: Arc<dyn SessionRepository>,
}

impl CreateSessionUseCase {
    pub fn new(repo: Arc<dyn SessionRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        name: String,
        subject_id: SubjectId,
        dtstart: u64,
        duration_minutes: u32,
        rrule: Option<String>,
    ) -> Result<Session, DomainError> {
        let id = SessionId(generate_uuid());
        let created_at = now_secs();

        let session = Session {
            id,
            name,
            subject_id,
            dtstart,
            duration_minutes,
            rrule,
            exdates: vec![],
            created_at,
        };

        self.repo.save(session.clone()).await?;
        Ok(session)
    }
}
