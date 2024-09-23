use crossterm::{execute, terminal::EnterAlternateScreen};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal as TuiTerminal
use wasm_bindgen::prelude::*;

use wasm_bindgen::JsValue;
use xterm_js_sys::{
    crossterm_support::XtermJsCrosstermBackend,
    xterm::{LogLevel, TerminalOptions},
    Terminal,
};

use std::io::Write;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let terminal_div = document
        .get_element_by_id("terminal")
        .expect("should have a terminal div");

    let term= Terminal::new(Some(
        TerminalOptions::default()
            .with_log_level(LogLevel::Debug)
            .with_font_family("'Fira Mono', monospace".to_string())
            .with_font_size(11.0),
    ));
    term.open(terminal_div);

    let mut term: XtermJsCrosstermBackend = (&term).into();
    execute!((&mut term), EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(term);

    Ok(())
}
