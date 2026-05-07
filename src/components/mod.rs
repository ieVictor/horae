use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::domain::Priority;

pub mod analytics;
pub mod input;
pub mod questions;
pub mod subjects;
pub mod tasks;
pub mod timer;

pub use analytics::AnalyticsComponent;
pub use questions::QuestionsComponent;
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
    // Q&A session-end flow
    ToggleQuestionResolved { id: String, block_id: String, resolved: bool },
    AnswerQuestion { id: String, answer: String, block_id: String },
    SaveCapturedQuestions { questions: Vec<String>, subject_id: String, block_id: String },
    QADone,
    // Subjects questions view
    OpenSubjectQuestions(String),
    ToggleSubjectQuestionResolved { id: String, resolved: bool },
    AnswerSubjectQuestion { id: String, answer: String },
    // Global
    Quit,
}

pub trait Component {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action>;
    fn render(&mut self, frame: &mut Frame, area: Rect);
}
