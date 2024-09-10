use std::io;

use color_eyre::Result;
use crossterm::event::Event;
use measurements::Volume;
use ratatui::{prelude::*, symbols::border, widgets::{block::Position::Bottom, Block, Row, Table}};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind}, prelude::{Buffer, Rect}, widgets::{block::Title, Paragraph, Widget}, DefaultTerminal, Frame
};

use crate::{db, recipe::Product};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_evnet(key_event)
            },
            _ => (),
        }
        Ok(())
    }

    fn handle_key_evnet(&mut self, key_event: event::KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('p') => panic!("at the disco"),
            _ => (),
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized {
            let title = Title::from("CALICOMP");
            let insts = Title::from(Line::from(vec![
                    " Random Cocktail".into(),
                    " Quit".into()
            ]));

            let block = Block::bordered()
                .title(title.alignment(Alignment::Center))
                .title(insts.alignment(Alignment::Left).position(Bottom))
                .border_set(border::THICK);

            let daiq = &db::generate()[0];

            fn ingredient((volume, prod): &(Volume, Product)) -> Line<'static> {
                Line::from(vec![
                    Span::from(format!("{volume}")),
                    Span::from(format!(" {}", prod.name))
                ]).alignment(Alignment::Right)
            }
    
            let rows= [Row::new(vec!["Amounts", "Ingredient"])];
            let widths = [Constraint::Length(5), Constraint::Length(5)];
            let recipe = Table::new(rows, widths);

            let cocktail = Text::from(vec![
                Line::from(daiq.name.to_string().yellow()),
                ingredient(&daiq.ingredients[0]),
                ingredient(&daiq.ingredients[1]),
                ingredient(&daiq.ingredients[2]),
            ]);

            Paragraph::new(cocktail)
                .centered()
                .block(block)
                .render(area, buf)
    }
}

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let mut app = App::default();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result?;
    Ok(())
}
