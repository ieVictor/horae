use std::str::FromStr;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

use crate::domain::{DailyStudyTime, StudyBlockWithSubject};
use crate::util;

use super::{Action, Component};

pub struct AnalyticsComponent {
    blocks: Vec<StudyBlockWithSubject>,
    stats: Vec<DailyStudyTime>,
    list_state: TableState,
}

impl AnalyticsComponent {
    pub fn new(blocks: Vec<StudyBlockWithSubject>, stats: Vec<DailyStudyTime>) -> Self {
        let mut list_state = TableState::default();
        if !blocks.is_empty() {
            list_state.select(Some(0));
        }
        Self {
            blocks,
            stats,
            list_state,
        }
    }
}

impl AnalyticsComponent {
    fn render_chart(&self, frame: &mut Frame, area: Rect) {
        let max_duration_secs = self
            .stats
            .iter()
            .map(|s| s.duration_secs)
            .max()
            .unwrap_or(0);
        let max_hours = ((max_duration_secs as f64 / 3600.0).ceil() as u64).max(1);

        let bars: Vec<Bar> = self
            .stats
            .iter()
            .map(|stat| {
                let label = stat.day.chars().skip(5).collect::<String>();
                let hours = stat.duration_secs as f64 / 3600.0;
                let val = (hours * 10.0).round() as u64;
                let color = util::random_color(&stat.day);
                Bar::default()
                    .label(Line::from(label))
                    .value(val)
                    .text_value(format!("{:.1}h", hours))
                    .style(Style::default().fg(color))
                    .value_style(Style::default().fg(Color::Black).bg(color))
            })
            .collect();

        let barchart = BarChart::default()
            .block(
                Block::default()
                    .title(" Weekly Study Time ")
                    .borders(Borders::ALL),
            )
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
                let status = if b.block.end_time.is_some() {
                    "✓ done"
                } else {
                    "● ongoing"
                };
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
        .block(
            Block::default()
                .title(" Study Blocks ")
                .borders(Borders::ALL),
        )
        .row_highlight_style(Style::new().reversed())
        .highlight_symbol("> ");

        frame.render_stateful_widget(table, area, &mut self.list_state);
    }
}

impl Component for AnalyticsComponent {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.list_state.select_next();
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.list_state.select_previous();
                None
            }
            KeyCode::Esc => Some(Action::CloseAnalytics),
            KeyCode::Char('q') => Some(Action::Quit),
            _ => None,
        }
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

        frame.render_widget(
            Paragraph::new("j/k to scroll  ·  ESC to go back")
                .dim()
                .right_aligned(),
            hint_area,
        );
    }
}
