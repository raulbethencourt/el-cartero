use crate::app::InputMode;
use crate::app::{App, AppResult};
use crossterm::event::{self, Event, KeyCode};
use tui_input::backend::crossterm::EventHandler;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(app: &mut App) -> AppResult<()> {
    if let Event::Key(key) = event::read()? {
        match app.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('i') => {
                    app.input_mode = InputMode::Editing;
                }
                KeyCode::Char('q') => {
                    app.quit();
                }
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    app.messages.push(app.input.value().into());
                    app.input.reset();
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                }
                _ => {
                    app.input.handle_event(&Event::Key(key));
                }
            },
        }
    }
    Ok(())
}
