mod cocktail;
mod glassware;

use std::io;

use color_eyre::Result;
use crossterm::event::Event;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    prelude::{Buffer, Rect},
    widgets::{block::Title, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::Position::Bottom, Block},
};

use crate::{db, ui::glassware::draw_cocktail};

#[derive(Debug, Default)]
pub struct App {
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
            }
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
        Self: Sized,
    {
        let title = Title::from("CALICOMP");
        let insts = Title::from(Line::from(vec![" Quit (q) ".into()]));

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(insts.alignment(Alignment::Left).position(Bottom))
            .border_set(border::THICK);

        let daiq = &db::generate()[0];

        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2])
            .margin(2)
            .areas(area);

        draw_cocktail(left, buf);

        let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(1); 2]).areas(right);
        let c = cocktail::cocktail(daiq);
        Paragraph::new(c)
            .centered()
            .block(block)
            .render(top_right, buf)
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
