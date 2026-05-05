use std::time::Duration;

use color_eyre::Result;
use color_eyre::eyre::Context;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, List, ListItem, Paragraph},
};
use rusqlite::Connection;

use components::{Action, AnalyticsComponent, Component, SubjectsComponent, TasksComponent, TimerComponent};
use domain::SubjectStats;

mod components;
mod db;
mod domain;
mod util;

fn main() -> Result<()> {
    color_eyre::install()?;

    let conn = Connection::open("horae.db")?;
    db::init(&conn)?;

    ratatui::run(|terminal| App::new(conn)?.run(terminal)).context("failed to run app")
}

// ── Subject selector shown alongside the timer when Space is pressed ────────

struct StartSelectorState {
    subjects: Vec<SubjectStats>,
    cursor: usize,
}

impl StartSelectorState {
    fn new(subjects: Vec<SubjectStats>) -> Self {
        Self { subjects, cursor: 0 }
    }

    fn select_next(&mut self) {
        if !self.subjects.is_empty() {
            self.cursor = (self.cursor + 1).min(self.subjects.len() - 1);
        }
    }

    fn select_previous(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    fn selected(&self) -> Option<&SubjectStats> {
        self.subjects.get(self.cursor)
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(" Select Subject ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let [list_area, hint_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(inner);

        let items: Vec<ListItem> = self
            .subjects
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let line = if i == self.cursor {
                    Line::from(format!("  > {}", s.name)).style(Style::new().reversed())
                } else {
                    Line::from(format!("    {}", s.name))
                };
                ListItem::new(line)
            })
            .collect();

        frame.render_widget(List::new(items), list_area);
        frame.render_widget(
            Paragraph::new("Enter to start  ·  ESC to cancel").dim().right_aligned(),
            hint_area,
        );
    }
}

// ── Overlay screens ──────────────────────────────────────────────────────────

enum Overlay {
    Analytics(AnalyticsComponent),
    Tasks(TasksComponent),
    Subjects(SubjectsComponent),
    StartSelector(StartSelectorState),
}

// ── App ───────────────────────────────────────────────────────────────────────

struct App {
    conn: Connection,
    timer: TimerComponent,
    overlay: Option<Overlay>,
    active_block_id: Option<String>,
}

impl App {
    fn new(conn: Connection) -> Result<Self> {
        let today_secs = db::study_block::today_total_secs(&conn)?;
        Ok(Self {
            conn,
            timer: TimerComponent::new(today_secs),
            overlay: None,
            active_block_id: None,
        })
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| {
                let area = frame.area();
                match &mut self.overlay {
                    Some(Overlay::Analytics(c)) => c.render(frame, area),
                    Some(Overlay::Tasks(c)) => c.render(frame, area),
                    Some(Overlay::Subjects(c)) => c.render(frame, area),
                    Some(Overlay::StartSelector(s)) => {
                        let [timer_area, selector_area] = Layout::horizontal([
                            Constraint::Percentage(60),
                            Constraint::Percentage(40),
                        ])
                        .areas(area);
                        self.timer.render(frame, timer_area);
                        s.render(frame, selector_area);
                    }
                    None => self.timer.render(frame, area),
                }
            })?;

            if !event::poll(Duration::from_millis(200)).context("event poll failed")? {
                continue;
            }

            if let Event::Key(key) = event::read().context("event read failed")? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // StartSelector is handled inline — it's not a Component.
                if matches!(self.overlay, Some(Overlay::StartSelector(_))) {
                    match key.code {
                        KeyCode::Char('j') | KeyCode::Down => {
                            if let Some(Overlay::StartSelector(s)) = &mut self.overlay {
                                s.select_next();
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            if let Some(Overlay::StartSelector(s)) = &mut self.overlay {
                                s.select_previous();
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(Overlay::StartSelector(s)) = self.overlay.take() {
                                if let Some(subj) = s.selected() {
                                    let subject_id = subj.id.0.clone();
                                    let subject_name = subj.name.clone();
                                    let block =
                                        db::study_block::create(&self.conn, &subject_id)?;
                                    self.active_block_id = Some(block.id.0);
                                    self.timer.set_subject(subject_name);
                                    self.timer.start();
                                }
                            }
                        }
                        KeyCode::Esc | KeyCode::Char('q') => {
                            self.overlay = None;
                        }
                        _ => {}
                    }
                    continue;
                }

                // All other overlays go through the Component trait.
                let action = match &mut self.overlay {
                    Some(Overlay::Analytics(c)) => c.handle_key(key),
                    Some(Overlay::Tasks(c)) => c.handle_key(key),
                    Some(Overlay::Subjects(c)) => c.handle_key(key),
                    None => self.timer.handle_key(key),
                    Some(Overlay::StartSelector(_)) => unreachable!(),
                };

                match action {
                    Some(Action::Quit) => break,

                    Some(Action::RequestStart) => {
                        let subjects = db::subject::find_all_summary(&self.conn)?;
                        self.overlay =
                            Some(Overlay::StartSelector(StartSelectorState::new(subjects)));
                    }
                    Some(Action::StopStudy) => {
                        if let Some(id) = self.active_block_id.take() {
                            db::study_block::end(&self.conn, &id)?;
                        }
                        self.timer.stop();
                    }

                    Some(Action::OpenAnalytics) => {
                        let blocks = db::study_block::find_all(&self.conn)?;
                        self.overlay =
                            Some(Overlay::Analytics(AnalyticsComponent::new(blocks)));
                    }
                    Some(Action::CloseAnalytics) => self.overlay = None,

                    Some(Action::OpenTasks) => {
                        let tasks = db::task::find_all(&self.conn)?;
                        self.overlay = Some(Overlay::Tasks(TasksComponent::new(tasks)));
                    }
                    Some(Action::CloseTasks) => self.overlay = None,

                    Some(Action::OpenSubjects) => {
                        let subjects = db::subject::find_all_summary(&self.conn)?;
                        self.overlay =
                            Some(Overlay::Subjects(SubjectsComponent::new(subjects)));
                    }
                    Some(Action::CloseSubjects) => self.overlay = None,

                    Some(Action::FetchSubjectSessions(id)) => {
                        let sessions = db::subject::find_blocks(&self.conn, &id, 10)?;
                        if let Some(Overlay::Subjects(c)) = &mut self.overlay {
                            c.set_expanded_sessions(sessions);
                        }
                    }

                    Some(Action::CreateTask { title, description, priority }) => {
                        db::task::create(&self.conn, &title, description.as_deref(), priority)?;
                        self.refresh_tasks()?;
                    }
                    Some(Action::DeleteTask(id)) => {
                        db::task::delete(&self.conn, &id)?;
                        self.refresh_tasks()?;
                    }
                    Some(Action::ToggleTask(id)) => {
                        db::task::toggle_status(&self.conn, &id)?;
                        self.refresh_tasks()?;
                    }

                    Some(Action::CreateSubject(name)) => {
                        db::subject::create(&self.conn, &name)?;
                        self.refresh_subjects()?;
                    }
                    Some(Action::DeleteSubject(id)) => {
                        db::subject::delete(&self.conn, &id)?;
                        self.refresh_subjects()?;
                    }

                    None => {}
                }
            }
        }
        Ok(())
    }

    fn refresh_tasks(&mut self) -> Result<()> {
        if let Some(Overlay::Tasks(c)) = &mut self.overlay {
            let tasks = db::task::find_all(&self.conn)?;
            c.update_tasks(tasks);
        }
        Ok(())
    }

    fn refresh_subjects(&mut self) -> Result<()> {
        if let Some(Overlay::Subjects(c)) = &mut self.overlay {
            let subjects = db::subject::find_all_summary(&self.conn)?;
            c.update_subjects(subjects);
        }
        Ok(())
    }
}
