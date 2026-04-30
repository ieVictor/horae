use tauri::State;
use horae_application::AppContainer;
use horae_domain::{Session, SessionId, Subject, SubjectId, Priority, Task, TaskId, StudyBlock, StudyBlockId};

#[tauri::command]
pub async fn create_session(
    name: String,
    subject_id: String,
    dtstart: u64,
    duration_minutes: u32,
    rrule: Option<String>,
    app: State<'_, AppContainer>,
) -> Result<Session, String> {
    app.session.create_session
        .execute(name, SubjectId(subject_id), dtstart, duration_minutes, rrule)
        .await
        .map_err(|e| format!("{:?}", e))
}

#[tauri::command]
pub async fn get_sessions(
    app: State<'_, AppContainer>,
) -> Result<Vec<Session>, String> {
    app.session.get_sessions
        .execute()
        .await
        .map_err(|e| format!("{:?}", e))
}

#[tauri::command]
pub async fn create_task(
    title: String,
    priority: Priority,
    session_id: Option<String>,
    study_block_id: Option<String>,
    app: State<'_, AppContainer>,
) -> Result<Task, String> {
    app.task.create_task
        .execute(
            title, 
            priority, 
            session_id.map(SessionId), 
            study_block_id.map(StudyBlockId)
        )
        .await
        .map_err(|e| format!("{:?}", e))
}

#[tauri::command]
pub async fn get_tasks(
    app: State<'_, AppContainer>,
) -> Result<Vec<Task>, String> {
    app.task.get_tasks
        .execute()
        .await
        .map_err(|e| format!("{:?}", e))
}

#[tauri::command]
pub async fn get_subjects(
    app: State<'_, AppContainer>,
) -> Result<Vec<Subject>, String> {
    app.subject.get_subjects
        .execute()
        .await
        .map_err(|e| format!("{:?}", e))
}

#[tauri::command]
pub async fn assign_task_to_session(
    task_id: String,
    session_id: Option<String>,
    app: State<'_, AppContainer>,
) -> Result<(), String> {
    app.task.assign_task_to_session
        .execute(TaskId(task_id), session_id.map(SessionId))
        .await
        .map_err(|e| format!("{:?}", e))
}

#[tauri::command]
pub async fn start_study_block(
    session_id: Option<String>,
    app: State<'_, AppContainer>,
) -> Result<StudyBlock, String> {
    app.study_block.start_study_block
        .execute(session_id.map(SessionId))
        .await
        .map_err(|e| format!("{:?}", e))
}

#[tauri::command]
pub async fn get_study_blocks(
    app: State<'_, AppContainer>,
) -> Result<Vec<StudyBlock>, String> {
    app.study_block.get_study_blocks
        .execute()
        .await
        .map_err(|e| format!("{:?}", e))
}

#[tauri::command]
pub async fn complete_study_block(
    id: String,
    app: State<'_, AppContainer>,
) -> Result<(), String> {
    app.study_block.complete_study_block
        .execute(StudyBlockId(id))
        .await
        .map_err(|e| format!("{:?}", e))
}

#[tauri::command]
pub async fn get_tasks_by_session(
    session_id: String,
    app: State<'_, AppContainer>,
) -> Result<Vec<Task>, String> {
    app.task.get_tasks_by_session
        .execute(SessionId(session_id))
        .await
        .map_err(|e| format!("{:?}", e))
}

#[tauri::command]
pub async fn get_tasks_by_study_block(
    study_block_id: String,
    app: State<'_, AppContainer>,
) -> Result<Vec<Task>, String> {
    app.task.get_tasks_by_study_block
        .execute(StudyBlockId(study_block_id))
        .await
        .map_err(|e| format!("{:?}", e))
}
