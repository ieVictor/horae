use async_trait::async_trait;
use std::sync::Mutex;
use horae_application::session::ports::SessionRepository;
use horae_domain::{Session, SessionId, DomainError};

pub struct InMemorySessionRepository {
    sessions: Mutex<Vec<Session>>,
}

impl InMemorySessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl SessionRepository for InMemorySessionRepository {
    async fn save(&self, session: Session) -> Result<(), DomainError> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(index) = sessions.iter().position(|s| s.id == session.id) {
            sessions[index] = session;
        } else {
            sessions.push(session);
        }
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Session>, DomainError> {
        let sessions = self.sessions.lock().unwrap();
        Ok(sessions.clone())
    }

    async fn find_by_id(&self, id: SessionId) -> Result<Option<Session>, DomainError> {
        let sessions = self.sessions.lock().unwrap();
        Ok(sessions.iter().find(|s| s.id == id).cloned())
    }
}
