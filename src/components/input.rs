use crossterm::event::KeyCode;
use tui_input::Input;

pub fn apply_input(input: &mut Input, code: KeyCode) {
    match code {
        KeyCode::Char(c) => { input.handle(tui_input::InputRequest::InsertChar(c)); }
        KeyCode::Backspace => { input.handle(tui_input::InputRequest::DeletePrevChar); }
        _ => {}
    }
}
