use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::Backend, Terminal};

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
