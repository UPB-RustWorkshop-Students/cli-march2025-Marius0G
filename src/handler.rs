use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};
use std::fs::OpenOptions;
use std::io::Write;
/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("event_log.txt")
    .unwrap();

    writeln!(file, "Event: {:?}", key_event.code).unwrap();  
    match key_event.code {
        // TODO: call the proper method on app instance for each event you want to handle
        // eq: arrow keys pressed, enter key, scroll...
        // TODO: define actions for quitting the app
        KeyCode::Char('q') => {
            app.running = false;
        }
        KeyCode::Up => {
            app.prev_city();
        }
        KeyCode::Down => {
            app.next_city();
        }
        _ => {}
    }
    Ok(())
}
