use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::domain::Priority;

pub mod analytics;
pub mod subjects;
pub mod tasks;
pub mod timer;

pub use analytics::AnalyticsComponent;
pub use subjects::SubjectsComponent;
pub use tasks::TasksComponent;
pub use timer::TimerComponent;

pub enum Action {
    // Timer
    RequestStart,
    StopStudy,
    // Navigation
    OpenAnalytics,
    CloseAnalytics,
    OpenTasks,
    CloseTasks,
    OpenSubjects,
    CloseSubjects,
    FetchSubjectSessions(String),
    // Task mutations
    CreateTask { title: String, description: Option<String>, priority: Priority },
    DeleteTask(String),
    ToggleTask(String),
    // Subject mutations
    CreateSubject(String),
    DeleteSubject(String),
    // Global
    Quit,
}

pub trait Component {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action>;
    fn render(&mut self, frame: &mut Frame, area: Rect);
}
