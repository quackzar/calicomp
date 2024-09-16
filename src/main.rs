pub mod app;
pub mod sys;
pub mod ui;

use std::io;

use better_panic::Settings;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use eyre::Result;
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::app::{events::run_app, App};


pub fn initialize_panic_handler() {
  std::panic::set_hook(Box::new(|panic_info| {
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
    Settings::auto().most_recent_first(false).lineno_suffix(true).create_panic_handler()(panic_info);
  }));
}

fn init() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;
    // This is a special case. Normally using stdout is fine
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    initialize_panic_handler();

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}


fn main() -> Result<()> {
    // TODO setup clap
    init()
}
