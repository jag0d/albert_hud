mod app;
mod event;
mod model;
mod tui;
mod ui;

use app::App;
use crossterm::event::{KeyCode, KeyModifiers};
use event::{Event, EventHandler};
use model::ability::AbilitiesDoc;
use std::path::Path;
use std::time::Duration;

use crate::app::ActivePanel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Install panic hook to restore terminal cleanly on crash
    tui::install_hooks()?;

    // Load abilities from TOML file (or embedded fallback)
    let toml_path = Path::new("abilities.toml");
    let abilities_doc = AbilitiesDoc::load_from_file_or_default(toml_path);

    // Initialize application state
    let mut app = App::new(abilities_doc, toml_path);

    // Initialize terminal
    let mut terminal = tui::init()?;

    // Event handler polling at 50ms tick rate
    let events = EventHandler::new(Duration::from_millis(50));

    // Main event loop
    while app.running {
        terminal.draw(|f| ui::render(f, &app))?;

        match events.next()? {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Esc => {
                        if app.active_panel == ActivePanel::Inspector {
                            app.active_panel = app.prev_active_panel;
                        } else {
                            app.quit();
                        }
                    }
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.quit()
                    }
                    KeyCode::Tab => app.next_panel(),
                    KeyCode::Up | KeyCode::Char('k') => app.navigate_previous(),
                    KeyCode::Down | KeyCode::Char('j') => app.navigate_next(),
                    KeyCode::Left | KeyCode::Char('h') => app.navigate_left(),
                    KeyCode::Right | KeyCode::Char('l') => app.navigate_right(),
                    KeyCode::Enter => { if app.active_panel != ActivePanel::Inspector { app.navigate_inspector(); } },
                    _ => {}
                }
            }
            Event::Tick => {}
            Event::Resize(_, _) => {}
        }
    }

    // Restore terminal back to normal state before exiting
    tui::restore()?;

    Ok(())
}
