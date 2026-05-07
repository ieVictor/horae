use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph},
};
use tui_input::Input;

use crate::domain::{Question, QuestionStatus, StudyBlock, SubjectStats};
use crate::util;

use super::{Action, Component, input::apply_input};

enum Mode {
    Browsing,
    Expanded { sessions: Vec<StudyBlock> },
    Creating(String),
    Questions { questions: Vec<Question>, cursor: usize, answer: Option<Input>, subject_id: String },
}

pub struct SubjectsComponent {
    subjects: Vec<SubjectStats>,
    list_state: ListState,
    mode: Mode,
}

impl SubjectsComponent {
    pub fn new(subjects: Vec<SubjectStats>) -> Self {
        let mut list_state = ListState::default();
        if !subjects.is_empty() {
            list_state.select(Some(0));
        }
        Self { subjects, list_state, mode: Mode::Browsing }
    }

    pub fn update_subjects(&mut self, subjects: Vec<SubjectStats>) {
        let prev = self.list_state.selected().unwrap_or(0);
        self.subjects = subjects;
        if self.subjects.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(prev.min(self.subjects.len() - 1)));
        }
    }

    pub fn set_expanded_sessions(&mut self, sessions: Vec<StudyBlock>) {
        if let Mode::Expanded { sessions: s } = &mut self.mode {
            *s = sessions;
        }
    }

    pub fn set_questions(&mut self, questions: Vec<Question>, subject_id: String) {
        self.mode = Mode::Questions { questions, cursor: 0, answer: None, subject_id };
    }

    pub fn update_questions(&mut self, questions: Vec<Question>) {
        if let Mode::Questions { questions: q, cursor, .. } = &mut self.mode {
            let prev = *cursor;
            *q = questions;
            *cursor = prev.min(q.len().saturating_sub(1));
        }
    }

    pub fn questions_subject_id(&self) -> Option<&str> {
        if let Mode::Questions { subject_id, .. } = &self.mode {
            Some(subject_id)
        } else {
            None
        }
    }

    fn selected(&self) -> Option<&SubjectStats> {
        self.list_state.selected().and_then(|i| self.subjects.get(i))
    }

    fn item_line(subj: &SubjectStats) -> ListItem<'static> {
        let default_tag = if subj.is_default { " [default]" } else { "" };
        let name = format!("{}{}", subj.name, default_tag);
        let name_padded = format!("{name:<36}");

        let total = util::fmt_duration(subj.total_seconds);

        let last = subj
            .last_session
            .map(util::fmt_datetime)
            .unwrap_or_else(|| "—".to_string());

        ListItem::new(Line::from(vec![
            Span::raw(format!("  {name_padded}")),
            Span::raw(format!("  {total}   ")),
            Span::raw(last).dim(),
        ]))
    }
}

impl Component for SubjectsComponent {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        let mode = std::mem::replace(&mut self.mode, Mode::Browsing);
        let mut action = None;

        self.mode = match mode {
            Mode::Browsing => match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.list_state.select_next();
                    Mode::Browsing
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.list_state.select_previous();
                    Mode::Browsing
                }
                KeyCode::Char('l') | KeyCode::Right => {
                    if let Some(subj) = self.selected() {
                        let id = subj.id.0.clone();
                        action = Some(Action::FetchSubjectSessions(id));
                        Mode::Expanded { sessions: Vec::new() }
                    } else {
                        Mode::Browsing
                    }
                }
                KeyCode::Char('?') => {
                    if let Some(subj) = self.selected() {
                        let id = subj.id.0.clone();
                        action = Some(Action::OpenSubjectQuestions(id));
                    }
                    Mode::Browsing
                }
                KeyCode::Char('d') => {
                    if let Some(subj) = self.selected() {
                        if !subj.is_default {
                            action = Some(Action::DeleteSubject(subj.id.0.clone()));
                        }
                    }
                    Mode::Browsing
                }
                KeyCode::Char('n') => Mode::Creating(String::new()),
                KeyCode::Esc => {
                    action = Some(Action::CloseSubjects);
                    Mode::Browsing
                }
                KeyCode::Char('q') => {
                    action = Some(Action::Quit);
                    Mode::Browsing
                }
                _ => Mode::Browsing,
            },

            Mode::Expanded { sessions } => match key.code {
                KeyCode::Char('h') | KeyCode::Left | KeyCode::Esc => Mode::Browsing,
                _ => Mode::Expanded { sessions },
            },

            Mode::Creating(mut input) => match key.code {
                KeyCode::Char(c) => {
                    input.push(c);
                    Mode::Creating(input)
                }
                KeyCode::Backspace => {
                    input.pop();
                    Mode::Creating(input)
                }
                KeyCode::Enter => {
                    let name = input.trim().to_string();
                    if !name.is_empty() {
                        action = Some(Action::CreateSubject(name));
                    }
                    Mode::Browsing
                }
                KeyCode::Esc => Mode::Browsing,
                _ => Mode::Creating(input),
            },

            Mode::Questions { mut questions, mut cursor, mut answer, subject_id } => {
                let mut go_back = false;

                if let Some(mut input) = answer.take() {
                    match key.code {
                        KeyCode::Enter => {
                            let text = input.value().trim().to_string();
                            if !text.is_empty() {
                                if let Some(q) = questions.get_mut(cursor) {
                                    q.status = QuestionStatus::Resolved;
                                    q.answer = Some(text.clone());
                                    action = Some(Action::AnswerSubjectQuestion {
                                        id: q.id.0.clone(),
                                        answer: text,
                                    });
                                }
                            }
                        }
                        KeyCode::Esc => {}
                        code => {
                            apply_input(&mut input, code);
                            answer = Some(input);
                        }
                    }
                } else {
                    match key.code {
                        KeyCode::Char('j') | KeyCode::Down => {
                            cursor = (cursor + 1).min(questions.len().saturating_sub(1));
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            cursor = cursor.saturating_sub(1);
                        }
                        KeyCode::Char(' ') => {
                            if let Some(q) = questions.get_mut(cursor) {
                                let resolving = q.status == QuestionStatus::Open;
                                q.status = if resolving {
                                    QuestionStatus::Resolved
                                } else {
                                    QuestionStatus::Open
                                };
                                action = Some(Action::ToggleSubjectQuestionResolved {
                                    id: q.id.0.clone(),
                                    resolved: resolving,
                                });
                            }
                        }
                        KeyCode::Char('a') => {
                            answer = Some(Input::default());
                        }
                        KeyCode::Esc => {
                            go_back = true;
                        }
                        _ => {}
                    }
                }

                if go_back {
                    Mode::Browsing
                } else {
                    Mode::Questions { questions, cursor, answer, subject_id }
                }
            }
        };

        action
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(" Subjects ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Questions view
        if let Mode::Questions { questions, cursor, answer, .. } = &self.mode {
            render_questions_list(questions, *cursor, answer, frame, inner);
            return;
        }

        // Expanded: full-screen detail view for the selected subject.
        if let Mode::Expanded { sessions } = &self.mode {
            let [content_area, hint_area] =
                Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(inner);

            if let Some(subj) = self.list_state.selected().and_then(|i| self.subjects.get(i)) {
                let last_str = subj
                    .last_session
                    .map(util::fmt_datetime)
                    .unwrap_or_else(|| "—".to_string());

                let mut lines: Vec<Line> = vec![
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("  Name         ", Style::new().bold()),
                        Span::raw(subj.name.clone()),
                    ]),
                    Line::from(vec![
                        Span::styled("  Total hours  ", Style::new().bold()),
                        Span::raw(util::fmt_duration(subj.total_seconds)),
                    ]),
                    Line::from(vec![
                        Span::styled("  Last session ", Style::new().bold()),
                        Span::raw(last_str),
                    ]),
                ];

                if !sessions.is_empty() {
                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::styled(
                        "  Last 10 sessions",
                        Style::new().bold(),
                    )));
                    lines.push(Line::from("  ──────────────────────────────────"));
                    for b in sessions {
                        lines.push(Line::from(format!(
                            "  {}   {}",
                            util::fmt_datetime(b.start_time),
                            util::fmt_duration(b.duration),
                        )));
                    }
                } else {
                    lines.push(Line::from(""));
                    lines.push(Line::from("  No sessions yet.").dim());
                }

                frame.render_widget(Paragraph::new(lines), content_area);
            }

            frame.render_widget(
                Paragraph::new("h/← to go back").dim().right_aligned(),
                hint_area,
            );
            return;
        }

        let is_creating = matches!(self.mode, Mode::Creating(_));

        let [list_area, form_area, hint_area] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(if is_creating { 1 } else { 0 }),
            Constraint::Length(1),
        ])
        .areas(inner);

        let items: Vec<ListItem> = self.subjects.iter().map(Self::item_line).collect();
        let list = List::new(items)
            .highlight_style(Style::new().reversed())
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, list_area, &mut self.list_state);

        if let Mode::Creating(input) = &self.mode {
            frame.render_widget(
                Paragraph::new(format!("  Name: {input}█")),
                form_area,
            );
            frame.render_widget(
                Paragraph::new("Enter to create  ·  ESC to cancel").dim().right_aligned(),
                hint_area,
            );
        } else {
            frame.render_widget(
                Paragraph::new("n new  ·  d delete  ·  l expand  ·  ? questions  ·  ESC back")
                    .dim()
                    .right_aligned(),
                hint_area,
            );
        }
    }
}

fn render_questions_list(
    questions: &[Question],
    cursor: usize,
    answer: &Option<Input>,
    frame: &mut Frame,
    area: Rect,
) {
    let [list_area, hint_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(area);

    let answering = answer.is_some();
    let mut items: Vec<ListItem> = Vec::new();

    for (i, q) in questions.iter().enumerate() {
        let is_cursor = i == cursor;
        let is_resolved = q.status == QuestionStatus::Resolved;
        let tag = if is_resolved { "[Resolved]" } else { "[Open]    " };

        let line = if is_cursor {
            Line::from(format!("> {tag} {}", q.text)).style(Style::new().reversed())
        } else if is_resolved {
            Line::from(format!("  {tag} {}", q.text)).dim()
        } else {
            Line::from(format!("  {tag} {}", q.text))
        };
        items.push(ListItem::new(line));

        if is_cursor {
            if let Some(input) = answer {
                items.push(ListItem::new(Line::from(format!(
                    "         answer> {}█",
                    input.value()
                ))));
            } else if let Some(ans) = &q.answer {
                items.push(ListItem::new(Line::from(format!("         {ans}")).dim()));
            }
        }
    }

    if questions.is_empty() {
        items.push(ListItem::new(Line::from("  No questions yet.").dim()));
    }

    frame.render_widget(List::new(items), list_area);

    let hints = if answering {
        "enter save · esc cancel"
    } else {
        "j/k move · space toggle · a answer · esc back"
    };
    frame.render_widget(Paragraph::new(hints).dim().right_aligned(), hint_area);
}
