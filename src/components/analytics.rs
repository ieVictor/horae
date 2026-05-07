use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::domain::{DailyStudyTime, StudyBlockWithSubject};
use crate::util;

use super::{Action, Component};

pub struct AnalyticsComponent {
    blocks: Vec<StudyBlockWithSubject>,
    stats: Vec<DailyStudyTime>,
    list_state: ListState,
}

impl AnalyticsComponent {
    pub fn new(blocks: Vec<StudyBlockWithSubject>, stats: Vec<DailyStudyTime>) -> Self {
        let mut list_state = ListState::default();
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

        let [chart_area, list_area, hint_area] = Layout::vertical([
            Constraint::Length(12),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(inner);

        // --- Render Chart ---
        // Max duration in hours for scaling
        let max_duration_secs = self
            .stats
            .iter()
            .map(|s| s.duration_secs)
            .max()
            .unwrap_or(0);
        let max_hours = (max_duration_secs as f64 / 3600.0).ceil() as u64;
        let max_hours = max_hours.max(1); // At least 1 for the scale

        let bars: Vec<Bar> = self
            .stats
            .iter()
            .map(|stat| {
                // Get short day name, e.g., "Mon" from "YYYY-MM-DD"
                // For simplicity in SQLite, day is "YYYY-MM-DD"
                // We can parse or just use the last 5 chars "MM-DD"
                let label = stat.day.chars().skip(5).collect::<String>();
                let hours = stat.duration_secs as f64 / 3600.0;
                // ratatui BarChart uses u64, we can multiply by 10 for 1 decimal precision, or just use whole hours.
                // Let's use minutes for better visualization resolution, but display in hours maybe?
                // Actually, ratatui BarChart text can be custom.
                let val = (hours * 10.0).round() as u64; // e.g. 1.5h -> 15
                let color = util::random_color(&stat.day);
                Bar::default()
                    .label(Line::from(label))
                    .value(val)
                    .text_value(format!("{:.1}h", hours))
                    .style(Style::default().fg(color))
                    .value_style(Style::default().fg(Color::Black).bg(color))
            })
            .collect();

        let mut barchart = BarChart::default()
            .block(
                Block::default()
                    .title(" Weekly Study Time ")
                    .borders(Borders::ALL),
            )
            .bar_width(7)
            .bar_gap(2)
            .label_style(Style::default().fg(Color::White));

        barchart = barchart.data(BarGroup::default().bars(&bars));
        // Max value scale (since we scaled by 10)
        barchart = barchart.max(max_hours * 10);

        frame.render_widget(barchart, chart_area);

        // --- Render List ---
        let items: Vec<ListItem> = self
            .blocks
            .iter()
            .map(|b| {
                let start = util::fmt_datetime(b.block.start_time);
                let duration = util::fmt_duration(b.block.duration);
                let status = if b.block.end_time.is_some() {
                    "✓"
                } else {
                    "● ongoing"
                };
                let subject = b.subject_name.as_deref().unwrap_or("No Subject");
                ListItem::new(Line::from(format!(
                    "  {start}   {duration}   [{subject}]   {status}"
                )))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(" Study Blocks ")
                    .borders(Borders::ALL),
            )
            .highlight_style(Style::new().reversed())
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, list_area, &mut self.list_state);

        frame.render_widget(
            Paragraph::new("j/k to scroll  ·  ESC to go back")
                .dim()
                .right_aligned(),
            hint_area,
        );
    }
}
