use std::{
    collections::HashMap,
    fs::File,
    io::{self, stdout, Read, Write},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use eyre::Result;
use ratatui::{prelude::Backend, Terminal};
use serde::{de::DeserializeOwned, Serialize};

use crate::{app::*, ui::ui};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Name);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Char('p') => {
                        panic!("at the disco");
                    }
                    KeyCode::Char('v') => {
                        edit_with_editor(terminal, &mut app.current_recipe).unwrap();
                    }
                    KeyCode::Char('s') => {
                        app.recipes.push(app.current_recipe.name.clone());
                        app.save_current_recipe();
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.list_state.select_next();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.list_state.select_previous();
                    }
                    KeyCode::Esc => {
                        app.list_state.select(None);
                    }
                    KeyCode::Enter => {
                        let Some(i) = app.list_state.selected() else {
                            continue;
                        };
                        let name = &app.recipes[i];
                        app.current_recipe = app.repo.recipes[name].clone();
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Name => {
                                    app.currently_editing = Some(CurrentlyEditing::Description);
                                }
                                CurrentlyEditing::Description => {
                                    app.save_current_recipe();
                                    app.current_screen = CurrentScreen::Main;
                                }
                                _ => todo!(),
                            }
                        }
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
                    KeyCode::Tab => {
                        app.toggle_editing();
                    }
                    _ => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Name => {
                                    app.name_text.input(key);
                                }
                                CurrentlyEditing::Description => {
                                    app.desc_text.input(key);
                                }
                                _ => todo!(),
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        // --snip--
    }
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
