use std::iter;

use ratatui::prelude::*;

use crate::recipe::Recipe;

pub fn cocktail(cocktail: &Recipe) -> Text<'static> {
    let layout = Layout::horizontal([Constraint::Fill(1); 2]);
    let ingredients: Vec<Line<'static>> = cocktail
        .ingredients
        .iter()
        .map(|(vol, prod)| {
            Line::from(vec![
                Span::from(format!("{vol}")),
                Span::from(format!(" {}", prod.name)),
            ])
            .alignment(Alignment::Right)
        })
        .collect();
    Text::from_iter(iter::once(Line::from(cocktail.name.clone()).bold().blue()).chain(ingredients))
}
