use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, List, ListItem, ListState, Paragraph},
};

use crate::domain::StudyBlock;
use crate::util;

use super::{Action, Component};

pub struct AnalyticsComponent {
    blocks: Vec<StudyBlock>,
    list_state: ListState,
}

impl AnalyticsComponent {
    pub fn new(blocks: Vec<StudyBlock>) -> Self {
        let mut list_state = ListState::default();
        if !blocks.is_empty() {
            list_state.select(Some(0));
        }
        Self { blocks, list_state }
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
        let block = Block::bordered().title(" Study Blocks ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let [list_area, hint_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(inner);

        let items: Vec<ListItem> = self
            .blocks
            .iter()
            .map(|b| {
                let start = util::fmt_datetime(b.start_time);
                let duration = util::fmt_duration(b.duration);
                let status = if b.end_time.is_some() { "✓" } else { "● ongoing" };
                ListItem::new(Line::from(format!("  {start}   {duration}   {status}")))
            })
            .collect();

        let list = List::new(items)
            .highlight_style(Style::new().reversed())
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, list_area, &mut self.list_state);

        frame.render_widget(
            Paragraph::new("j/k to scroll  ·  ESC to go back").dim().right_aligned(),
            hint_area,
        );
    }
}
