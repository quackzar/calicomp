use ratatui::{
    prelude::*,
    widgets::{
        canvas::{self, Context, Line, Shape},
        Block, Widget,
    },
};

pub struct Martini {
    foot_width: f64,
    head_width: f64,
    head_height: f64,
    stilk_height: f64,
    foot_height: f64,
    stilk_width: f64,
}

impl Martini {
    pub fn draw(&self, ctx: &mut Context<'_>, color: Color) {
            ctx.draw(&Line {
                x1: -self.stilk_width/2.0,
                y1: 0.0,
                x2: -self.stilk_width/2.0,
                y2: self.stilk_height,
                color,
            });
            ctx.draw(&Line {
                x1: self.stilk_width/2.0,
                y1: 0.0,
                x2: self.stilk_width/2.0,
                y2: self.stilk_height,
                color,
            });
            ctx.draw(&Line {
                x1: self.stilk_width/2.0,
                y1: self.stilk_height,
                x2: self.head_width/2.0,
                y2: self.head_height - self.stilk_height,
                color,
            });
            ctx.draw(&Line {
                x1: -self.stilk_width/2.0,
                y1: self.stilk_height,
                x2: -self.head_width/2.0,
                y2: self.head_height - self.stilk_height,
                color,
            });
            ctx.draw(&Line {
                x1: -self.stilk_width/2.0,
                y1: self.head_height - self.stilk_height,
                x2: self.stilk_width/2.0,
                y2: self.head_height - self.stilk_height,
                color,
            });
            ctx.draw(&Line {
                x1: -self.stilk_width,
                y1: 0.0,
                x2: -self.foot_width,
                y2: -self.foot_height,
                color,
            });
            ctx.draw(&Line {
                x1: self.stilk_width,
                y1: 0.0,
                x2: self.foot_width,
                y2: -self.foot_height,
                color,
            });
            ctx.draw(&Line {
                x1: -self.foot_width,
                y1: self.foot_height,
                x2: self.foot_width,
                y2: -self.foot_height,
                color,
            });

    }
}

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
