pub mod card;
pub mod glassware;
pub mod image;

use std::iter;

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, Paragraph, Wrap},
};
use tui_textarea::TextArea;

use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    sys::{self, glass::Glassware},
    ui::card::RecipeCard,
};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "CALICOMP",
        Style::default().bold().fg(Color::Green),
    ))
    .centered()
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let list = List::from_iter(iter::from_fn(|| Some("Another Cocktail")).take(20))
        .highlight_symbol(">>")
        .highlight_style(Style::new().yellow());

    let [left, right] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2); 2])
        .areas(chunks[1]);

    frame.render_widget(list, left);
    recipe_window(frame, right);

    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Viewing Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Name => {
                        Span::styled("Editing Recipe Name", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Description => Span::styled(
                        "Editing Recipe Description",
                        Style::default().fg(Color::LightGreen),
                    ),
                    _ => Span::styled(
                        "Editing something else?",
                        Style::default().fg(Color::LightBlue),
                    ),
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    if let Some(editing) = &app.currently_editing {
        edit_window(frame, editing, app);
    }

    if let CurrentScreen::Exiting = app.current_screen {
        exit_popup(frame);
    }
}

fn recipe_window(frame: &mut Frame<'_>, right: Rect) {
    let daiquiri = sys::db::new_daiq();
    let glass = glassware::Glass::from(daiquiri.glassware.unwrap_or(Glassware::Highball));

    let [left, right] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2); 2])
        .areas(right);

    let card = RecipeCard {
        recipe: Some(&daiquiri),
    };

    frame.render_widget(&card, left);

    //frame.render_widget(glass, right);
    image::image(frame, right).unwrap();
}

fn edit_window(frame: &mut Frame<'_>, editing: &CurrentlyEditing, app: &App) {
    let popup_block = Block::default()
        .title("Enter a new key-value pair")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(popup_block, area);

    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(4), Constraint::Min(5)])
        .split(area);

    let mut key_block = Block::default().title("Key").borders(Borders::ALL);

    let mut value_block = Block::default().title("Value").borders(Borders::ALL);

    let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

    match editing {
        CurrentlyEditing::Name => key_block = key_block.style(active_style),
        CurrentlyEditing::Description => value_block = value_block.style(active_style),
        _ => todo!(),
    };

    let name_text = &app.name_text;
    frame.render_widget(name_text, popup_chunks[0]);

    let desc_text = &app.desc_text;
    frame.render_widget(desc_text, popup_chunks[1]);
}

fn exit_popup(frame: &mut Frame<'_>) {
    frame.render_widget(Clear, frame.area());
    //this clears the entire screen and anything already drawn
    let popup_block = Block::default()
        .title("Y/N")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let exit_text = Text::styled(
        "Would you like to output the buffer as json? (y/n)",
        Style::default().fg(Color::Red),
    );
    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(exit_paragraph, area);
}
