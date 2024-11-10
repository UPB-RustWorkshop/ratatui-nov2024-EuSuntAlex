use crate::app::{App, AppResult};
use crossterm::event::KeyEvent;
use crossterm::event::KeyCode;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // TODO: define actions for quitting the app
        KeyCode::Char('q') => app.running = false,
        KeyCode::Up => app.indicator = app.indicator.saturating_sub(1),
        KeyCode::Down => app.indicator = app.indicator.saturating_add(1),
        KeyCode::Enter => app.data = None, 
        // TODO: define actions for apps functionalities
        _ => {}
    }
    Ok(())
}
