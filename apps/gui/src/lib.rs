mod commands;

use std::sync::Arc;
use horae_persistence_memory::{InMemorySessionRepository, InMemoryTaskRepository, InMemorySubjectRepository, InMemoryStudyBlockRepository};
use horae_application::AppContainer;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let session_repo = Arc::new(InMemorySessionRepository::new());
    let task_repo = Arc::new(InMemoryTaskRepository::new());
    let subject_repo = Arc::new(InMemorySubjectRepository::new());
    let study_block_repo = Arc::new(InMemoryStudyBlockRepository::new());
    let app_container = AppContainer::new(session_repo, task_repo, subject_repo, study_block_repo);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_container)
        .invoke_handler(tauri::generate_handler![
            commands::create_session,
            commands::get_sessions,
            commands::create_task,
            commands::get_tasks,
            commands::get_subjects,
            commands::assign_task_to_session,
            commands::start_study_block,
            commands::get_study_blocks,
            commands::complete_study_block,
            commands::get_tasks_by_session,
            commands::get_tasks_by_study_block
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
