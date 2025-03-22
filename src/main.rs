use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventsPublisher};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;


#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Setup the terminal
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;


    // TOD: create the events pubisher
    let tick_rate: u64 = 100;
    let events_publisher= EventsPublisher::new(tick_rate);

    // TOD: init the terminal user interface
    let mut tui = Tui::new(terminal,events_publisher);
    tui.init()?;
    // Start the main loop.
    while app.running {
        // TOD: Render the user interface.
        tui.draw(&mut app)?;
        // TODO: Handle events.
        match tui.events.next().await {
            Ok(event) => { 
                match event {

                    Event::Key(key_event) => {

                    handle_key_events(key_event,&mut app);
                    
                    }
                    Event::Mouse(mouse_event) => {
                        // Log to file what mouse_event contains
                    }
                    Event::Resize(width, height) => {
                        // Log to file what dimensions were received
                    }
                    Event::Tick => {
                        // This is a timer tick
                    }
                    _ => {
                        // Any other event types
                    }
                }
        
             }
            Err(e) => { app.running = false; }
        }
        // Hint: wait for events and handle them

    }

    // TOD: Reset the terminal if the app has been terminated
    tui.exit()?;
    Ok(())
}
