use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::domain::{Priority, Task, TaskStatus};
use crate::util;

use super::{Action, Component};

const PRIORITY_LABELS: [&str; 4] = ["Low", "Medium", "High", "Urgent"];

fn priority_at(cursor: usize) -> Priority {
    match cursor {
        0 => Priority::Low,
        1 => Priority::Medium,
        2 => Priority::High,
        _ => Priority::Urgent,
    }
}

enum CreatingStep {
    Title(String),
    Description { title: String, input: String },
    Priority { title: String, description: Option<String>, cursor: usize },
}

enum Mode {
    Browsing,
    Expanded,
    Creating(CreatingStep),
}

pub struct TasksComponent {
    tasks: Vec<Task>,
    list_state: ListState,
    mode: Mode,
}

impl TasksComponent {
    pub fn new(tasks: Vec<Task>) -> Self {
        let mut list_state = ListState::default();
        if !tasks.is_empty() {
            list_state.select(Some(0));
        }
        Self { tasks, list_state, mode: Mode::Browsing }
    }

    pub fn update_tasks(&mut self, tasks: Vec<Task>) {
        let prev = self.list_state.selected().unwrap_or(0);
        self.tasks = tasks;
        if self.tasks.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(prev.min(self.tasks.len() - 1)));
        }
    }

    fn selected_id(&self) -> Option<&str> {
        self.list_state
            .selected()
            .and_then(|i| self.tasks.get(i))
            .map(|t| t.id.0.as_str())
    }

    fn item_line(task: &Task) -> ListItem<'static> {
        let (checkbox, title_style) = match task.status {
            TaskStatus::Open => ("[ ]", Style::new()),
            TaskStatus::Completed => (
                "[✓]",
                Style::new().dim().add_modifier(Modifier::CROSSED_OUT),
            ),
            TaskStatus::Abandoned => ("[x]", Style::new().dim()),
        };

        let (priority_str, priority_style) = match task.priority {
            Priority::Low => ("Low   ", Style::new().dim()),
            Priority::Medium => ("Medium", Style::new().yellow()),
            Priority::High => ("High  ", Style::new().red()),
            Priority::Urgent => ("Urgent", Style::new().red().bold()),
        };

        let status_str = match task.status {
            TaskStatus::Open => "Open",
            TaskStatus::Completed => "Done",
            TaskStatus::Abandoned => "Abandoned",
        };

        // Truncate title to 44 chars for the compact list view.
        let title: String = task.title.chars().take(44).collect();
        let suffix = if task.title.chars().count() > 44 { "…" } else { " " };
        let title_padded = format!("{title}{suffix}{:<width$}", "", width = 44usize.saturating_sub(title.chars().count()));

        ListItem::new(Line::from(vec![
            Span::raw(format!("{checkbox} ")),
            Span::styled(title_padded, title_style),
            Span::raw("  "),
            Span::styled(priority_str, priority_style),
            Span::raw(format!("  {status_str}")),
        ]))
    }
}

impl Component for TasksComponent {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        // Take ownership of mode to allow free transitions without borrow conflicts.
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
                    if self.list_state.selected().is_some() {
                        Mode::Expanded
                    } else {
                        Mode::Browsing
                    }
                }
                KeyCode::Enter => {
                    action = self.selected_id().map(|id| Action::ToggleTask(id.to_string()));
                    Mode::Browsing
                }
                KeyCode::Char('d') => {
                    action = self.selected_id().map(|id| Action::DeleteTask(id.to_string()));
                    Mode::Browsing
                }
                KeyCode::Char('n') => Mode::Creating(CreatingStep::Title(String::new())),
                KeyCode::Esc => {
                    action = Some(Action::CloseTasks);
                    Mode::Browsing
                }
                KeyCode::Char('q') => {
                    action = Some(Action::Quit);
                    Mode::Browsing
                }
                _ => Mode::Browsing,
            },

            Mode::Expanded => match key.code {
                KeyCode::Char('h') | KeyCode::Left | KeyCode::Esc => Mode::Browsing,
                _ => Mode::Expanded,
            },

            Mode::Creating(step) => match step {
                CreatingStep::Title(mut input) => match key.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                        Mode::Creating(CreatingStep::Title(input))
                    }
                    KeyCode::Backspace => {
                        input.pop();
                        Mode::Creating(CreatingStep::Title(input))
                    }
                    KeyCode::Enter => {
                        let title = input.trim().to_string();
                        if title.is_empty() {
                            Mode::Creating(CreatingStep::Title(input))
                        } else {
                            Mode::Creating(CreatingStep::Description {
                                title,
                                input: String::new(),
                            })
                        }
                    }
                    KeyCode::Esc => Mode::Browsing,
                    _ => Mode::Creating(CreatingStep::Title(input)),
                },

                CreatingStep::Description { title, mut input } => match key.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                        Mode::Creating(CreatingStep::Description { title, input })
                    }
                    KeyCode::Backspace => {
                        input.pop();
                        Mode::Creating(CreatingStep::Description { title, input })
                    }
                    KeyCode::Enter => {
                        let description =
                            if input.trim().is_empty() { None } else { Some(input.trim().to_string()) };
                        Mode::Creating(CreatingStep::Priority { title, description, cursor: 0 })
                    }
                    KeyCode::Esc => Mode::Browsing,
                    _ => Mode::Creating(CreatingStep::Description { title, input }),
                },

                CreatingStep::Priority { title, description, mut cursor } => match key.code {
                    KeyCode::Char('j') | KeyCode::Down => {
                        cursor = (cursor + 1).min(PRIORITY_LABELS.len() - 1);
                        Mode::Creating(CreatingStep::Priority { title, description, cursor })
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        cursor = cursor.saturating_sub(1);
                        Mode::Creating(CreatingStep::Priority { title, description, cursor })
                    }
                    KeyCode::Enter => {
                        action = Some(Action::CreateTask {
                            title,
                            description,
                            priority: priority_at(cursor),
                        });
                        Mode::Browsing
                    }
                    KeyCode::Esc => Mode::Browsing,
                    _ => Mode::Creating(CreatingStep::Priority { title, description, cursor }),
                },
            },
        };

        action
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(" Tasks ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Expanded mode: full-screen detail view for the selected task.
        if matches!(self.mode, Mode::Expanded) {
            let [content_area, hint_area] =
                Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(inner);

            let selected = self.list_state.selected().and_then(|i| self.tasks.get(i));
            if let Some(task) = selected {
                let priority_str = match task.priority {
                    Priority::Low => "Low",
                    Priority::Medium => "Medium",
                    Priority::High => "High",
                    Priority::Urgent => "Urgent",
                };
                let status_str = match task.status {
                    TaskStatus::Open => "Open",
                    TaskStatus::Completed => "Completed",
                    TaskStatus::Abandoned => "Abandoned",
                };

                let mut lines: Vec<Line> = vec![
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("  Title       ", Style::new().bold()),
                        Span::raw(task.title.clone()),
                    ]),
                    Line::from(vec![
                        Span::styled("  Priority    ", Style::new().bold()),
                        Span::raw(priority_str),
                    ]),
                    Line::from(vec![
                        Span::styled("  Status      ", Style::new().bold()),
                        Span::raw(status_str),
                    ]),
                    Line::from(vec![
                        Span::styled("  Created     ", Style::new().bold()),
                        Span::raw(util::fmt_datetime(task.created_at)),
                    ]),
                ];

                if let Some(desc) = &task.description {
                    lines.push(Line::from(""));
                    lines.push(Line::from(vec![
                        Span::styled("  Description ", Style::new().bold()),
                        Span::raw(desc.clone()),
                    ]));
                }

                frame.render_widget(
                    Paragraph::new(lines).wrap(Wrap { trim: false }),
                    content_area,
                );
            }

            frame.render_widget(
                Paragraph::new("h/← to go back").dim().right_aligned(),
                hint_area,
            );
            return;
        }

        // Browsing / Creating modes share the list + bottom form layout.
        let form_lines: u16 = match &self.mode {
            Mode::Creating(CreatingStep::Priority { .. }) => PRIORITY_LABELS.len() as u16 + 1,
            Mode::Creating(_) => 1,
            _ => 0,
        };

        let [list_area, form_area, hint_area] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(form_lines),
            Constraint::Length(1),
        ])
        .areas(inner);

        // Render the task list.
        let items: Vec<ListItem> = self.tasks.iter().map(Self::item_line).collect();
        let list = List::new(items)
            .highlight_style(Style::new().reversed())
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, list_area, &mut self.list_state);

        // Render the creation form or the browsing hint.
        match &self.mode {
            Mode::Creating(step) => {
                match step {
                    CreatingStep::Title(input) => {
                        frame.render_widget(
                            Paragraph::new(format!("  Title: {input}█")),
                            form_area,
                        );
                        frame.render_widget(
                            Paragraph::new("Enter to next  ·  ESC to cancel")
                                .dim()
                                .right_aligned(),
                            hint_area,
                        );
                    }
                    CreatingStep::Description { input, .. } => {
                        frame.render_widget(
                            Paragraph::new(format!("  Description (optional): {input}█")),
                            form_area,
                        );
                        frame.render_widget(
                            Paragraph::new("Enter to next  ·  ESC to cancel")
                                .dim()
                                .right_aligned(),
                            hint_area,
                        );
                    }
                    CreatingStep::Priority { cursor, .. } => {
                        let prio_lines: Vec<Line> = PRIORITY_LABELS
                            .iter()
                            .enumerate()
                            .map(|(i, label)| {
                                if i == *cursor {
                                    Line::from(format!("  > {label}")).bold()
                                } else {
                                    Line::from(format!("    {label}")).dim()
                                }
                            })
                            .collect();
                        frame.render_widget(Paragraph::new(prio_lines), form_area);
                        frame.render_widget(
                            Paragraph::new("j/k to select  ·  Enter to create  ·  ESC to cancel")
                                .dim()
                                .right_aligned(),
                            hint_area,
                        );
                    }
                }
            }
            _ => {
                frame.render_widget(
                    Paragraph::new("n new  ·  d delete  ·  Enter toggle  ·  l expand  ·  ESC back")
                        .dim()
                        .right_aligned(),
                    hint_area,
                );
            }
        }
    }
}
