use chrono::{Local, TimeZone};

pub fn fmt_duration(secs: i64) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

pub fn fmt_datetime(unix_secs: i64) -> String {
    match Local.timestamp_opt(unix_secs, 0) {
        chrono::LocalResult::Single(dt) | chrono::LocalResult::Ambiguous(dt, _) => {
            dt.format("%Y-%m-%d %H:%M").to_string()
        }
        chrono::LocalResult::None => format!("{unix_secs}"),
    }
}

// A simple deterministic pseudo-random color generator based on a string seed (e.g. day string)
pub fn random_color(seed: &str) -> ratatui::style::Color {
    let mut hash: u32 = 5381;
    for c in seed.chars() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u32);
    }
    
    let colors = [
        ratatui::style::Color::Red,
        ratatui::style::Color::Green,
        ratatui::style::Color::Yellow,
        ratatui::style::Color::Blue,
        ratatui::style::Color::Magenta,
        ratatui::style::Color::Cyan,
        ratatui::style::Color::LightRed,
        ratatui::style::Color::LightGreen,
        ratatui::style::Color::LightYellow,
        ratatui::style::Color::LightBlue,
        ratatui::style::Color::LightMagenta,
        ratatui::style::Color::LightCyan,
    ];
    
    colors[(hash as usize) % colors.len()]
}
