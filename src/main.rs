// main.rs

use ratatui_templates::app::{App, AppResult};
use ratatui_templates::connection::get_data;
use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let ev = EventHandler::new(100);
    let myterminal = ratatui::init();
    let mut tui = Tui::new(myterminal, ev); // Use Default::default() to create a default instance of EventHandler

    // TODO: init the terminal
    tui.init()?;

    // Start the main loop.
    while app.running {
        // TODO: Render the user interface.
        tui.draw(&mut app);

        // TODO: Handle events.
        let ev = tui.events.next().await; 
        match ev {
            Ok(event) => {
                match event {
                    Event::Key(key_event) => {
                        handle_key_events(key_event, &mut app)?;
                    },
                    Event::Tick => continue, 
                    Event::Mouse(_) => continue,
                    Event::Resize(_, _) => continue, 
                }
            },
            // do nothing if err
            Err(_) => print!("Nu a mers "),
        }
        
        // Apelează get_data și folosește .await pentru așteptarea rezultatului asincron
        if app.data.is_none() {
            app.data = get_data(&app).await.ok();
        }
    }

    // TODO: Reset the terminal if the app has been terminated
    let _ = tui.exit(); 
    
    Ok(())
}

