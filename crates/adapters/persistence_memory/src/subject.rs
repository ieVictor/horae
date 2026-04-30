use async_trait::async_trait;
use std::sync::Mutex;
use horae_application::subject::ports::SubjectRepository;
use horae_domain::{Subject, SubjectId, Topic, TopicId, DomainError};

pub struct InMemorySubjectRepository {
    subjects: Mutex<Vec<Subject>>,
}

impl InMemorySubjectRepository {
    pub fn new() -> Self {
        let subjects = vec![
            Subject {
                id: SubjectId("cs".to_string()),
                name: "Computer Science".to_string(),
                color_hex: "#3b82f6".to_string(),
                topics: vec![
                    Topic { id: TopicId(1), name: "Algorithms".to_string(), created_at: 0, updated_at: 0 },
                    Topic { id: TopicId(2), name: "Data Structures".to_string(), created_at: 0, updated_at: 0 },
                ],
                created_at: 0,
                updated_at: 0,
            },
            Subject {
                id: SubjectId("math".to_string()),
                name: "Mathematics".to_string(),
                color_hex: "#ef4444".to_string(),
                topics: vec![
                    Topic { id: TopicId(3), name: "Calculus".to_string(), created_at: 0, updated_at: 0 },
                    Topic { id: TopicId(4), name: "Linear Algebra".to_string(), created_at: 0, updated_at: 0 },
                ],
                created_at: 0,
                updated_at: 0,
            },
            Subject {
                id: SubjectId("eng".to_string()),
                name: "English".to_string(),
                color_hex: "#10b981".to_string(),
                topics: vec![
                    Topic { id: TopicId(5), name: "Grammar".to_string(), created_at: 0, updated_at: 0 },
                    Topic { id: TopicId(6), name: "Literature".to_string(), created_at: 0, updated_at: 0 },
                ],
                created_at: 0,
                updated_at: 0,
            },
        ];

        Self {
            subjects: Mutex::new(subjects),
        }
    }
}

#[async_trait]
impl SubjectRepository for InMemorySubjectRepository {
    async fn save(&self, _subject: Subject) -> Result<(), DomainError> {
        Err(DomainError::Validation("Subject lookup table is read-only".to_string()))
    }

    async fn find_all(&self) -> Result<Vec<Subject>, DomainError> {
        let subjects = self.subjects.lock().unwrap();
        Ok(subjects.clone())
    }

    async fn find_by_id(&self, id: SubjectId) -> Result<Option<Subject>, DomainError> {
        let subjects = self.subjects.lock().unwrap();
        Ok(subjects.iter().find(|s| s.id == id).cloned())
    }
}
