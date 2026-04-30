use async_trait::async_trait;
use std::sync::Mutex;
use horae_application::study_block::ports::StudyBlockRepository;
use horae_domain::{StudyBlock, StudyBlockId, DomainError};

pub struct InMemoryStudyBlockRepository {
    blocks: Mutex<Vec<StudyBlock>>,
}

impl InMemoryStudyBlockRepository {
    pub fn new() -> Self {
        Self {
            blocks: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl StudyBlockRepository for InMemoryStudyBlockRepository {
    async fn save(&self, block: StudyBlock) -> Result<(), DomainError> {
        let mut blocks = self.blocks.lock().unwrap();
        if let Some(index) = blocks.iter().position(|b| b.id == block.id) {
            blocks[index] = block;
        } else {
            blocks.push(block);
        }
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<StudyBlock>, DomainError> {
        let blocks = self.blocks.lock().unwrap();
        Ok(blocks.clone())
    }

    async fn find_by_id(&self, id: StudyBlockId) -> Result<Option<StudyBlock>, DomainError> {
        let blocks = self.blocks.lock().unwrap();
        Ok(blocks.iter().find(|b| b.id == id).cloned())
    }
}
