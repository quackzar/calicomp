use std::{
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

use crate::{app::*, tui::EventHandler, ui::entry};

pub async fn update(app: &mut App, event: Event) -> Result<()> {
    if let Event::Key(key) = event {
        if key.kind == event::KeyEventKind::Release {
            // Skip events that are not KeyEventKind::Press
        }
        match app.current_mode {
            CurrentMode::Main => match key.code {
                KeyCode::Char('e') => {
                    app.current_mode = CurrentMode::Editing;
                    app.currently_editing = Some(CurrentlyEditing::Name);
                }
                KeyCode::Char('q') => {
                    app.current_mode = CurrentMode::Exiting;
                }
                KeyCode::Char('p') => {
                    panic!("at the disco");
                }
                KeyCode::Char('v') => {
                    todo!()
                    // return edit_with_editor(terminal, &mut app.current_recipe)
                }
                KeyCode::Char('s') => {
                    let Some(()) = app.save_current_recipe() else {
                        return Ok(());
                    };
                    app.recipes.push(app.current_recipe.name.clone());
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
                        return Ok(());
                    };
                    let name = &app.recipes[i];
                    app.current_recipe = app.repo.recipes[name].clone().dumb();
                }
                _ => {}
            },
            CurrentMode::Exiting => match key.code {
                KeyCode::Char('y') => {
                    app.should_quit = true;
                }
                KeyCode::Char('n') | KeyCode::Char('q') => {
                    app.should_quit = true;
                }
                _ => {}
            },
            CurrentMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {
                    if let Some(editing) = &app.currently_editing {
                        match editing {
                            CurrentlyEditing::Name => {
                                app.currently_editing = Some(CurrentlyEditing::Description);
                            }
                            CurrentlyEditing::Description => {
                                app.save_current_recipe();
                                app.current_mode = CurrentMode::Main;
                            }
                            _ => todo!(),
                        }
                    }
                }
                KeyCode::Esc => {
                    app.current_mode = CurrentMode::Main;
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
    Ok(())
}

