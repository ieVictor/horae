pub mod helpers;
pub mod session;
pub mod study_block;
pub mod subject;
pub mod task;

pub struct SessionContainer {
    pub create_session: session::commands::CreateSessionUseCase,
    pub get_sessions: session::queries::GetSessionsUseCase,
}

pub struct SubjectContainer {
    pub create_subject: subject::commands::CreateSubjectUseCase,
    pub get_subjects: subject::queries::GetSubjectsUseCase,
}

pub struct TaskContainer {
    pub create_task: task::commands::CreateTaskUseCase,
    pub assign_task_to_session: task::commands::AssignTaskToSessionUseCase,
    pub get_tasks: task::queries::GetTasksUseCase,
    pub get_tasks_by_session: task::queries::GetTasksBySessionUseCase,
    pub get_tasks_by_study_block: task::queries::GetTasksByStudyBlockUseCase,
}

pub struct StudyBlockContainer {
    pub start_study_block: study_block::commands::StartStudyBlockUseCase,
    pub complete_study_block: study_block::commands::CompleteStudyBlockUseCase,
    pub get_study_blocks: study_block::queries::GetStudyBlocksUseCase,
}

pub struct AppContainer {
    pub session: SessionContainer,
    pub subject: SubjectContainer,
    pub task: TaskContainer,
    pub study_block: StudyBlockContainer,
}

impl AppContainer {
    pub fn new(
        session_repo: std::sync::Arc<dyn session::ports::SessionRepository>,
        task_repo: std::sync::Arc<dyn task::ports::TaskRepository>,
        subject_repo: std::sync::Arc<dyn subject::ports::SubjectRepository>,
        study_block_repo: std::sync::Arc<dyn study_block::ports::StudyBlockRepository>,
    ) -> Self {
        Self {
            session: SessionContainer {
                create_session: session::commands::CreateSessionUseCase::new(std::sync::Arc::clone(&session_repo)),
                get_sessions: session::queries::GetSessionsUseCase::new(session_repo),
            },
            subject: SubjectContainer {
                create_subject: subject::commands::CreateSubjectUseCase::new(std::sync::Arc::clone(&subject_repo)),
                get_subjects: subject::queries::GetSubjectsUseCase::new(subject_repo),
            },
            task: TaskContainer {
                create_task: task::commands::CreateTaskUseCase::new(std::sync::Arc::clone(&task_repo)),
                assign_task_to_session: task::commands::AssignTaskToSessionUseCase::new(std::sync::Arc::clone(&task_repo)),
                get_tasks: task::queries::GetTasksUseCase::new(std::sync::Arc::clone(&task_repo)),
                get_tasks_by_session: task::queries::GetTasksBySessionUseCase::new(std::sync::Arc::clone(&task_repo)),
                get_tasks_by_study_block: task::queries::GetTasksByStudyBlockUseCase::new(task_repo),
            },
            study_block: StudyBlockContainer {
                start_study_block: study_block::commands::StartStudyBlockUseCase::new(std::sync::Arc::clone(&study_block_repo)),
                complete_study_block: study_block::commands::CompleteStudyBlockUseCase::new(std::sync::Arc::clone(&study_block_repo)),
                get_study_blocks: study_block::queries::GetStudyBlocksUseCase::new(study_block_repo),
            },
        }
    }
}
