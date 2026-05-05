use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Stylize,
    text::Line,
    widgets::Paragraph,
};

use super::{Action, Component};

pub struct TimerComponent {
    pub today_secs: i64,
    session_start: Option<Instant>,
    pub current_subject: Option<String>,
}

impl TimerComponent {
    pub fn new(today_secs: i64) -> Self {
        Self {
            today_secs,
            session_start: None,
            current_subject: None,
        }
    }

    pub fn start(&mut self) {
        self.session_start = Some(Instant::now());
    }

    pub fn set_subject(&mut self, name: String) {
        self.current_subject = Some(name);
    }

    pub fn stop(&mut self) {
        self.today_secs += self.session_elapsed_secs();
        self.session_start = None;
        self.current_subject = None;
    }

    pub fn is_studying(&self) -> bool {
        self.session_start.is_some()
    }

    fn session_elapsed_secs(&self) -> i64 {
        self.session_start
            .map(|s| s.elapsed().as_secs() as i64)
            .unwrap_or(0)
    }

    fn fmt(secs: i64) -> String {
        let h = secs / 3600;
        let m = (secs % 3600) / 60;
        let s = secs % 60;
        format!("{h:02}:{m:02}:{s:02}")
    }

    // Each digit is rendered as 5 rows of block characters.
    fn char_rows(ch: char) -> [&'static str; 5] {
        match ch {
            '0' => [" ███ ", "█   █", "█   █", "█   █", " ███ "],
            '1' => ["  █  ", " ██  ", "  █  ", "  █  ", " ███ "],
            '2' => [" ███ ", "    █", "  ██ ", "█    ", "█████"],
            '3' => ["████ ", "    █", " ███ ", "    █", "████ "],
            '4' => ["█   █", "█   █", "█████", "    █", "    █"],
            '5' => ["█████", "█    ", "████ ", "    █", "████ "],
            '6' => [" ████", "█    ", "████ ", "█   █", " ███ "],
            '7' => ["█████", "    █", "  ██ ", "  █  ", "  █  "],
            '8' => [" ███ ", "█   █", " ███ ", "█   █", " ███ "],
            '9' => [" ███ ", "█   █", " ████", "    █", " ███ "],
            ':' => ["   ", "   ", " █ ", "   ", " █ "],
            _ => ["     ", "     ", "     ", "     ", "     "],
        }
    }

    fn big_lines(time_str: &str) -> [String; 5] {
        let mut rows: [String; 5] = Default::default();
        for (i, ch) in time_str.chars().enumerate() {
            for (row, pat) in rows.iter_mut().zip(Self::char_rows(ch)) {
                if i > 0 {
                    row.push(' ');
                }
                row.push_str(pat);
            }
        }
        rows
    }
}

impl Component for TimerComponent {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            KeyCode::Char(' ') => {
                if self.is_studying() {
                    Some(Action::StopStudy)
                } else {
                    Some(Action::RequestStart)
                }
            }
            KeyCode::Char('a') if !self.is_studying() => Some(Action::OpenAnalytics),
            KeyCode::Char('t') => Some(Action::OpenTasks),
            KeyCode::Char('s') if !self.is_studying() => Some(Action::OpenSubjects),
            _ => None,
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let session_secs = self.session_elapsed_secs();
        let total_secs = self.today_secs + session_secs;

        let big_height: u16 = 5;
        let hint_height: u16 = 1;
        let gap: u16 = 1;
        let block_height = big_height + gap + hint_height;

        let start_y = area.y + area.height.saturating_sub(block_height) / 2;

        let timer_area = Rect {
            x: area.x,
            y: start_y,
            width: area.width,
            height: big_height,
        };
        let hint_area = Rect {
            x: area.x,
            y: start_y + big_height + gap,
            width: area.width,
            height: hint_height,
        };

        let timer_lines: Vec<Line> = Self::big_lines(&Self::fmt(total_secs))
            .into_iter()
            .map(|l| Line::from(l).alignment(Alignment::Center))
            .collect();

        let hint = if self.is_studying() {
            let subject_part = self
                .current_subject
                .as_deref()
                .map(|s| format!("  [{s}]"))
                .unwrap_or_default();
            format!(
                "session {}{subject_part}  ·  t tasks  ·  SPACE to stop",
                Self::fmt(session_secs)
            )
        } else {
            "SPACE to start  ·  a analytics  ·  t tasks  ·  s subjects".to_string()
        };

        frame.render_widget(Paragraph::new(timer_lines).bold(), timer_area);
        frame.render_widget(
            Paragraph::new(hint).dim().alignment(Alignment::Center),
            hint_area,
        );
    }
}
