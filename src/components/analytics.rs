use std::str::FromStr;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        Bar, BarChart, BarGroup, Block, Borders, Cell, Clear, List, ListItem, ListState,
        Paragraph, Row, Table, TableState,
    },
};

use crate::domain::{DailyStudyTime, StudyBlockWithSubject, SubjectId, SubjectStats};
use crate::util;

use super::{Action, Component};

fn contrasting_color(bg: Color) -> Color {
    let luminance = match bg {
        Color::Rgb(r, g, b) => 0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64,
        Color::Black | Color::Red | Color::Green | Color::Blue
        | Color::Magenta | Color::DarkGray => 30.0,
        _ => 180.0,
    };
    if luminance > 128.0 { Color::Black } else { Color::White }
}

enum Mode {
    Browsing,
    FilteringBySubject { list_state: ListState },
}

pub struct AnalyticsComponent {
    blocks: Vec<StudyBlockWithSubject>,
    stats: Vec<DailyStudyTime>,
    subjects: Vec<SubjectStats>,
    active_filter: Option<SubjectId>,
    week_offset: i32,
    mode: Mode,
    list_state: TableState,
}

impl AnalyticsComponent {
    pub fn new(
        blocks: Vec<StudyBlockWithSubject>,
        stats: Vec<DailyStudyTime>,
        subjects: Vec<SubjectStats>,
    ) -> Self {
        let mut list_state = TableState::default();
        if !blocks.is_empty() {
            list_state.select(Some(0));
        }
        Self {
            blocks,
            stats,
            subjects,
            active_filter: None,
            week_offset: 0,
            mode: Mode::Browsing,
            list_state,
        }
    }

    pub fn update_stats(
        &mut self,
        stats: Vec<DailyStudyTime>,
        filter: Option<SubjectId>,
        week_offset: i32,
    ) {
        self.stats = stats;
        self.active_filter = filter;
        self.week_offset = week_offset;
    }

    fn active_subject(&self) -> Option<&SubjectStats> {
        self.active_filter
            .as_ref()
            .and_then(|id| self.subjects.iter().find(|s| &s.id == id))
    }

    fn filter_action(&self, subject_id: Option<String>) -> Action {
        Action::FilterAnalyticsBySubject { subject_id, week_offset: self.week_offset }
    }

    fn navigate_action(&self, offset: i32) -> Action {
        Action::NavigateAnalyticsWeek {
            offset,
            subject_id: self.active_filter.as_ref().map(|id| id.0.clone()),
        }
    }
}

impl AnalyticsComponent {
    fn render_chart(&self, frame: &mut Frame, area: Rect) {
        let max_duration_secs = self.stats.iter().map(|s| s.duration_secs).max().unwrap_or(0);
        let max_hours = ((max_duration_secs as f64 / 3600.0).ceil() as u64).max(1);

        let filter_color = self
            .active_subject()
            .and_then(|s| Color::from_str(&s.color_hex).ok());

        let bars: Vec<Bar> = self
            .stats
            .iter()
            .map(|stat| {
                let label = stat.day.chars().skip(5).collect::<String>();
                let hours = stat.duration_secs as f64 / 3600.0;
                let val = (hours * 10.0).round() as u64;
                let color = filter_color.unwrap_or_else(|| util::random_color(&stat.day));
                Bar::default()
                    .label(Line::from(label))
                    .value(val)
                    .text_value(format!("{:.1}h", hours))
                    .style(Style::default().fg(color))
                    .value_style(Style::default().fg(contrasting_color(color)).bg(color))
            })
            .collect();

        let title = {
            let subject_part = match self.active_subject() {
                Some(s) => format!(" · {}", s.name),
                None => String::new(),
            };
            let week_part = if self.week_offset < 0 {
                format!(" · {}w", self.week_offset)
            } else {
                String::new()
            };
            format!(" Weekly Study Time{}{} ", subject_part, week_part)
        };

        let barchart = BarChart::default()
            .block(Block::default().title(title).borders(Borders::ALL))
            .bar_width(7)
            .bar_gap(2)
            .label_style(Style::default().fg(Color::White))
            .data(BarGroup::default().bars(&bars))
            .max(max_hours * 10);

        frame.render_widget(barchart, area);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header = Row::new(vec![
            Cell::from("Start Time").style(Style::new().bold()),
            Cell::from("Duration").style(Style::new().bold()),
            Cell::from("").style(Style::new().bold()),
            Cell::from("Subject").style(Style::new().bold()),
            Cell::from("Status").style(Style::new().bold()),
        ])
        .bottom_margin(1);

        let rows: Vec<Row> = self
            .blocks
            .iter()
            .map(|b| {
                let start = util::fmt_datetime(b.block.start_time);
                let duration = util::fmt_duration(b.block.duration);
                let status = if b.block.end_time.is_some() { "✓ done" } else { "● ongoing" };
                let color = Color::from_str(&b.subject.color_hex).unwrap_or(Color::Gray);
                Row::new(vec![
                    Cell::from(start),
                    Cell::from(duration),
                    Cell::from("██").style(Style::new().fg(color)),
                    Cell::from(b.subject.name.clone()),
                    Cell::from(status),
                ])
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(20),
                Constraint::Length(10),
                Constraint::Length(3),
                Constraint::Min(16),
                Constraint::Length(10),
            ],
        )
        .header(header)
        .block(Block::default().title(" Study Blocks ").borders(Borders::ALL))
        .row_highlight_style(Style::new().reversed())
        .highlight_symbol("> ");

        frame.render_stateful_widget(table, area, &mut self.list_state);
    }
}

fn render_filter_popup(
    subjects: &[SubjectStats],
    active_filter: Option<&SubjectId>,
    frame: &mut Frame,
    chart_area: Rect,
    list_state: &mut ListState,
) {
    let height = (subjects.len() as u16 + 1 + 2).min(14);
    let width = 38u16;
    let popup_area = Rect {
        x: chart_area.x + chart_area.width.saturating_sub(width + 1),
        y: chart_area.y + 1,
        width: width.min(chart_area.width),
        height: height.min(chart_area.height.saturating_sub(2)),
    };

    frame.render_widget(Clear, popup_area);

    let block = Block::bordered().title(" Filter by Subject ");
    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let active_id = active_filter.map(|id| &id.0);

    let mut items: Vec<ListItem> = vec![ListItem::new(Line::from(if active_id.is_none() {
        "  ● All Subjects"
    } else {
        "    All Subjects"
    }))];

    for s in subjects {
        let color = Color::from_str(&s.color_hex).unwrap_or(Color::Gray);
        let marker = if active_id == Some(&s.id.0) { "● " } else { "  " };
        items.push(ListItem::new(
            Line::from(format!("{}██ {}", marker, s.name)).style(Style::new().fg(color)),
        ));
    }

    let list = List::new(items)
        .highlight_style(Style::new().reversed())
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, inner, list_state);
}

impl Component for AnalyticsComponent {
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
                KeyCode::Char('h') | KeyCode::Left => {
                    action = Some(self.navigate_action(self.week_offset - 1));
                    Mode::Browsing
                }
                KeyCode::Char('l') | KeyCode::Right if self.week_offset < 0 => {
                    action = Some(self.navigate_action(self.week_offset + 1));
                    Mode::Browsing
                }
                KeyCode::Char('s') => {
                    let mut ls = ListState::default();
                    let selected = self
                        .active_filter
                        .as_ref()
                        .and_then(|id| self.subjects.iter().position(|s| &s.id == id))
                        .map(|i| i + 1)
                        .unwrap_or(0);
                    ls.select(Some(selected));
                    Mode::FilteringBySubject { list_state: ls }
                }
                KeyCode::Esc => {
                    action = Some(Action::CloseAnalytics);
                    Mode::Browsing
                }
                KeyCode::Char('q') => {
                    action = Some(Action::Quit);
                    Mode::Browsing
                }
                _ => Mode::Browsing,
            },

            Mode::FilteringBySubject { mut list_state } => match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    list_state.select_next();
                    Mode::FilteringBySubject { list_state }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    list_state.select_previous();
                    Mode::FilteringBySubject { list_state }
                }
                KeyCode::Enter => {
                    let idx = list_state.selected().unwrap_or(0);
                    let subject_id =
                        if idx == 0 { None } else { self.subjects.get(idx - 1).map(|s| s.id.0.clone()) };
                    action = Some(self.filter_action(subject_id));
                    Mode::Browsing
                }
                KeyCode::Esc => Mode::Browsing,
                _ => Mode::FilteringBySubject { list_state },
            },
        };

        action
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(" Analytics ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let [chart_area, table_area, hint_area] = Layout::vertical([
            Constraint::Length(12),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(inner);

        self.render_chart(frame, chart_area);
        self.render_table(frame, table_area);

        let hint = match &self.mode {
            Mode::FilteringBySubject { .. } => "j/k move  ·  Enter select  ·  ESC cancel",
            Mode::Browsing => match (self.active_filter.is_some(), self.week_offset < 0) {
                (true, true) => "h/l week  ·  s change filter  ·  j/k scroll  ·  ESC back",
                (true, false) => "h prev week  ·  s change filter  ·  j/k scroll  ·  ESC back",
                (false, true) => "h/l week  ·  s filter  ·  j/k scroll  ·  ESC back",
                (false, false) => "h prev week  ·  s filter  ·  j/k scroll  ·  ESC back",
            },
        };
        frame.render_widget(Paragraph::new(hint).dim().right_aligned(), hint_area);

        let mode = std::mem::replace(&mut self.mode, Mode::Browsing);
        if let Mode::FilteringBySubject { mut list_state } = mode {
            render_filter_popup(
                &self.subjects,
                self.active_filter.as_ref(),
                frame,
                chart_area,
                &mut list_state,
            );
            self.mode = Mode::FilteringBySubject { list_state };
        } else {
            self.mode = mode;
        }
    }
}
