pub mod app;
pub mod sys;
pub mod tui;
pub mod ui;

use std::{
    fs::File,
    io::{self, stdout, BufWriter, Read},
};

use better_panic::Settings;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use eyre::Result;
use ratatui::{
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
use serde::{de::DeserializeOwned, Serialize};

pub fn initialize_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        Settings::auto()
            .most_recent_first(false)
            .lineno_suffix(true)
            .create_panic_handler()(panic_info);
    }));
}

fn edit_with_editor<B: Backend, T>(terminal: &mut Terminal<B>, edit: &mut T) -> Result<()>
where
    T: Serialize + DeserializeOwned,
{
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    let mut file = tempfile::Builder::new()
        .prefix("calicomp-")
        .suffix(".toml")
        .rand_bytes(5)
        .tempfile()?;

    let payload = toml::to_string_pretty(&edit)?;
    use std::io::Write;
    writeln!(file, "{payload}")?;
    let path = file.into_temp_path();

    let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());
    let status = std::process::Command::new(editor).arg(&path).status()?;
    if !status.success() {
        return Ok(());
    }

    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let payload: T = toml::from_str(&buf)?;

    *edit = payload;

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    terminal.clear()?;
    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut app = app::App::new();
    let mut event_handler = tui::EventHandler::new();

    loop {
        terminal.draw(|f| {
            ui::entry(f, &mut app);
        })?;
        let event = event_handler.next().await?;
        app::events::update(&mut app, event).await?;
        if app.should_quit {
            break Ok(());
        }
    }
}

async fn init() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;
    // This is a special case. Normally using stdout is fine
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(BufWriter::new(stderr));
    let mut terminal = Terminal::new(backend)?;

    initialize_panic_handler();
    let res = run_app(&mut terminal).await;
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // TODO setup clap
    init().await
}
