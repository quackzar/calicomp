use ratatui::{
    prelude::*,
    widgets::{
        canvas::{self, Line, Shape},
        Block, Widget,
    },
};

pub struct Martini {}

pub fn draw_cocktail(area: Rect, buf: &mut Buffer) {
    canvas::Canvas::default()
        .block(Block::bordered().title("Canvas"))
        .x_bounds([-80.0, 80.0])
        .y_bounds([-40.0, 40.0])
        .paint(|ctx| {
            ctx.layer();
            ctx.draw(&Line {
                x1: -2.0,
                y1: 0.0,
                x2: -2.0,
                y2: 20.0,
                color: Color::White,
            });
            ctx.draw(&Line {
                x1: 2.0,
                y1: 0.0,
                x2: 2.0,
                y2: 20.0,
                color: Color::White,
            });
            ctx.draw(&Line {
                x1: 2.0,
                y1: 20.0,
                x2: 65.0,
                y2: 30.0,
                color: Color::White,
            });
            ctx.draw(&Line {
                x1: -2.0,
                y1: 20.0,
                x2: -65.0,
                y2: 30.0,
                color: Color::White,
            });
            ctx.draw(&Line {
                x1: 65.0,
                y1: 30.0,
                x2: -65.0,
                y2: 30.0,
                color: Color::White,
            });
            ctx.draw(&Line {
                x1: -2.0,
                y1: 0.0,
                x2: -30.0,
                y2: -2.0,
                color: Color::White,
            });
            ctx.draw(&Line {
                x1: 2.0,
                y1: 0.0,
                x2: 30.0,
                y2: -2.0,
                color: Color::White,
            });
            ctx.draw(&Line {
                x1: -30.0,
                y1: -2.0,
                x2: 30.0,
                y2: -2.0,
                color: Color::White,
            });
        })
        .render(area, buf);
}
