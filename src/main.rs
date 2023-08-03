use el_cartero::app::{App, AppResult};
use el_cartero::event::{EventHandler};
use el_cartero::handler::handle_key_events;
use el_cartero::tui::Tui;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        tui.draw(&mut app)?;
        
        handle_key_events(&mut app)?;
    }

    tui.exit()?;
    Ok(())
}
