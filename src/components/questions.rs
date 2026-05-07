use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, List, ListItem, Paragraph},
};
use tui_input::Input;

use crate::domain::{Question, QuestionStatus};

use super::{Action, Component, input::apply_input};

enum QAMode {
    Resolving,
    Answering(Input),
    Capturing { captured: Vec<String>, input: Input },
}

pub struct QuestionsComponent {
    questions: Vec<Question>,
    cursor: usize,
    mode: QAMode,
    subject_id: String,
    block_id: String,
}

impl QuestionsComponent {
    pub fn new(subject_id: String, block_id: String, questions: Vec<Question>) -> Self {
        let mode = if questions.is_empty() {
            QAMode::Capturing { captured: vec![], input: Input::default() }
        } else {
            QAMode::Resolving
        };
        Self { questions, cursor: 0, mode, subject_id, block_id }
    }
}

impl Component for QuestionsComponent {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        let mode = std::mem::replace(&mut self.mode, QAMode::Resolving);
        let mut action = None;

        self.mode = match mode {
            QAMode::Resolving => match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    if self.cursor + 1 < self.questions.len() {
                        self.cursor += 1;
                    }
                    QAMode::Resolving
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.cursor = self.cursor.saturating_sub(1);
                    QAMode::Resolving
                }
                KeyCode::Char(' ') => {
                    if let Some(q) = self.questions.get_mut(self.cursor) {
                        let resolving = q.status == QuestionStatus::Open;
                        q.status = if resolving {
                            QuestionStatus::Resolved
                        } else {
                            QuestionStatus::Open
                        };
                        action = Some(Action::ToggleQuestionResolved {
                            id: q.id.0.clone(),
                            block_id: self.block_id.clone(),
                            resolved: resolving,
                        });
                    }
                    QAMode::Resolving
                }
                KeyCode::Char('a') => QAMode::Answering(Input::default()),
                KeyCode::Enter | KeyCode::Esc => {
                    QAMode::Capturing { captured: vec![], input: Input::default() }
                }
                _ => QAMode::Resolving,
            },

            QAMode::Answering(mut input) => match key.code {
                KeyCode::Enter => {
                    let answer = input.value().trim().to_string();
                    if !answer.is_empty() {
                        if let Some(q) = self.questions.get_mut(self.cursor) {
                            q.status = QuestionStatus::Resolved;
                            q.answer = Some(answer.clone());
                            action = Some(Action::AnswerQuestion {
                                id: q.id.0.clone(),
                                answer,
                                block_id: self.block_id.clone(),
                            });
                        }
                    }
                    QAMode::Resolving
                }
                KeyCode::Esc => QAMode::Resolving,
                code => {
                    apply_input(&mut input, code);
                    QAMode::Answering(input)
                }
            },

            QAMode::Capturing { mut captured, mut input } => match key.code {
                KeyCode::Enter => {
                    let text = input.value().trim().to_string();
                    if text.is_empty() {
                        action = if captured.is_empty() {
                            Some(Action::QADone)
                        } else {
                            Some(Action::SaveCapturedQuestions {
                                questions: captured,
                                subject_id: self.subject_id.clone(),
                                block_id: self.block_id.clone(),
                            })
                        };
                        QAMode::Capturing { captured: vec![], input: Input::default() }
                    } else {
                        captured.push(text);
                        QAMode::Capturing { captured, input: Input::default() }
                    }
                }
                KeyCode::Esc => {
                    action = if captured.is_empty() {
                        Some(Action::QADone)
                    } else {
                        Some(Action::SaveCapturedQuestions {
                            questions: captured,
                            subject_id: self.subject_id.clone(),
                            block_id: self.block_id.clone(),
                        })
                    };
                    QAMode::Capturing { captured: vec![], input: Input::default() }
                }
                code => {
                    apply_input(&mut input, code);
                    QAMode::Capturing { captured, input }
                }
            },
        };

        action
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        if matches!(self.mode, QAMode::Capturing { .. }) {
            self.render_capturing(frame, area);
        } else {
            self.render_resolving(frame, area);
        }
    }
}

impl QuestionsComponent {
    fn render_resolving(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(" Resolve open questions ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let [list_area, hint_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(inner);

        let mut items: Vec<ListItem> = Vec::new();
        for (i, q) in self.questions.iter().enumerate() {
            let is_cursor = i == self.cursor;
            let check = if q.status == QuestionStatus::Resolved { "✓" } else { " " };

            let question_line = if is_cursor {
                Line::from(format!("> [{}] {}", check, q.text)).style(Style::new().reversed())
            } else {
                Line::from(format!("  [{}] {}", check, q.text))
            };
            items.push(ListItem::new(question_line));

            if is_cursor {
                if let QAMode::Answering(input) = &self.mode {
                    items.push(ListItem::new(Line::from(format!(
                        "       answer> {}█",
                        input.value()
                    ))));
                    items.push(ListItem::new(
                        Line::from("       [enter save · esc cancel]").dim(),
                    ));
                } else if let Some(ans) = &q.answer {
                    items.push(ListItem::new(Line::from(format!("       {ans}")).dim()));
                }
            }
        }

        frame.render_widget(List::new(items), list_area);

        let hints = if matches!(self.mode, QAMode::Answering(_)) {
            "enter save · esc cancel"
        } else {
            "j/k move · space resolve · a answer · enter/esc done"
        };
        frame.render_widget(Paragraph::new(hints).dim().right_aligned(), hint_area);
    }

    fn render_capturing(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(" Capture new questions ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let (captured, input) = if let QAMode::Capturing { captured, input } = &self.mode {
            (captured, input)
        } else {
            return;
        };

        let list_height =
            if captured.is_empty() { 0 } else { captured.len().min(10) as u16 + 1 };
        let [list_area, input_area, hint_area] = Layout::vertical([
            Constraint::Length(list_height),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(inner);

        if !captured.is_empty() {
            let items: Vec<ListItem> = captured
                .iter()
                .enumerate()
                .map(|(i, q)| ListItem::new(Line::from(format!("  {}. {q}", i + 1))))
                .collect();
            frame.render_widget(List::new(items), list_area);
        }

        frame.render_widget(
            Paragraph::new(format!("> {}█", input.value())),
            input_area,
        );
        frame.render_widget(
            Paragraph::new("Enter to add · Esc to finish").dim().right_aligned(),
            hint_area,
        );
    }
}
