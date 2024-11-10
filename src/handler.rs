use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // TODO: define actions for quitting the app
        // TODO: define actions for apps functionalities
        KeyCode::Enter => app.data = None,
        KeyCode::Char('q') => app.running = false,
        KeyCode::Down => {
            app.indicator += 1;
            if app.indicator == app.city.len() {
                app.indicator = 0;
            }
        },
        KeyCode::Up => {
            if app.indicator == 0 {
                app.indicator = app.city.len() - 1;
            } else {
                app.indicator -= 1;
            }
        },
        _ => {}
    }
    Ok(())
}
