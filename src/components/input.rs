use crossterm::event::KeyCode;
use tui_input::{Input, InputRequest};

pub fn apply_input(input: &mut Input, code: KeyCode) {
    match code {
        KeyCode::Char(c) => { input.handle(InputRequest::InsertChar(c)); }
        KeyCode::Backspace => { input.handle(InputRequest::DeletePrevChar); }
        _ => {}
    }
}
