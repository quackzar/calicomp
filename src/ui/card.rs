use std::{iter, num::Wrapping};

use color_eyre::owo_colors::{colors::css::Teal, OwoColorize};
use ratatui::{
    prelude::*,
    widgets::{List, ListItem, Paragraph},
};

use crate::sys::recipe::{DumbRecipe, Recipe};

pub struct RecipeCard<'a> {
    pub recipe: Option<&'a DumbRecipe>,
}

impl<'a> Widget for &RecipeCard<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if let Some(recipe) = self.recipe {
            let [top, short, mid, bottom] = Layout::new(
                Direction::Vertical,
                [
                    Constraint::Length(1),
                    Constraint::Max(3),
                    Constraint::Min(4),
                    Constraint::Fill(1),
                ],
            )
            .areas(area);

            let cocktail_name = Span::from(&recipe.name).blue().bold().into_centered_line();
            cocktail_name.render(top, buf);

            if let Some(short_desc) = recipe.short_desc.as_deref() {
                Paragraph::new(short_desc)
                    .wrap(ratatui::widgets::Wrap { trim: true })
                    .italic()
                    .centered()
                    .render(short, buf);
            }

            // list

            let [heading, mid] = Layout::default()
                .constraints([Constraint::Length(1), Constraint::Min(0)])
                .areas(mid);
            Line::from("Ingredients")
                .italic()
                .centered()
                .render(heading, buf);

            //
            let items: Vec<_> = recipe
                .ingredients
                .iter()
                .map(|(volume, product)| {
                    let name = &product;
                    ListItem::new(Line::from(Span::from(format!("* {volume} {name}"))))
                })
                .collect();
            let list = List::new(items);
            Widget::render(list, mid, buf);

            if let Some(desc) = recipe.description.as_deref() {
                let [heading, bottom] = Layout::default()
                    .constraints([Constraint::Length(2), Constraint::Min(0)])
                    .areas(bottom);
                Line::from("Description")
                    .italic()
                    .centered()
                    .render(heading, buf);
                Paragraph::new(desc)
                    .wrap(ratatui::widgets::Wrap { trim: true })
                    .render(bottom, buf);
            }
        } else {
            Text::from("No Recipe Selected").render(area, buf);
        }
    }
}
