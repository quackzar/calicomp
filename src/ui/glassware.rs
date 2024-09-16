use ratatui::{
    prelude::*,
    widgets::{
        canvas::{self, Context, Line, Rectangle, Shape},
        Block, Widget,
    },
};

use crate::sys::glass::Glassware;

#[derive(Clone, Copy)]
pub struct Glass {
    pub kind: Glassware,
    pub filled: bool,
}

impl From<Glassware> for Glass {
    fn from(value: Glassware) -> Self {
        Glass {
            kind: value,
            filled: false,
        }
    }
}

impl Glass {
    pub fn new() -> Self {
        Glass {
            kind: Glassware::Martini,
            filled: true,
        }
    }
}

impl Default for Glass {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Glass {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        //let tumbler = Tumbler { width: 80.0, height: 40.0 };

        canvas::Canvas::default()
            .x_bounds([-80.0, 80.0])
            .y_bounds([-80.0, 80.0])
            .paint(|ctx| match self.kind {
                Glassware::Martini => {
                    let martini = Martini::new();
                    martini.draw(ctx, Color::White);
                }
                Glassware::Lowball => {
                    let tumbler = Martini::new();
                    tumbler.draw(ctx, Color::White);
                }
                _ => {
                    ctx.print(0., 0., "Unknown");
                }
            })
            .render(area, buf)
    }
}

pub struct Martini {
    head_height: f64,
    head_width: f64,
    stilk_height: f64,
    stilk_width: f64,
    foot_height: f64,
    foot_width: f64,
}

impl Martini {
    pub fn new() -> Self {
        Martini {
            head_width: 40.0,
            head_height: 35.0,
            stilk_height: 20.0,
            foot_height: 4.0,
            foot_width: 30.0,
            stilk_width: 2.0,
        }
    }
}

impl Default for Martini {
    fn default() -> Self {
        Self::new()
    }
}

impl Martini {
    pub fn draw(&self, ctx: &mut Context<'_>, color: Color) {
        ctx.draw(&Line {
            x1: -self.stilk_width / 2.0,
            y1: 0.0,
            x2: -self.stilk_width / 2.0,
            y2: self.stilk_height,
            color,
        });
        ctx.draw(&Line {
            x1: self.stilk_width / 2.0,
            y1: 0.0,
            x2: self.stilk_width / 2.0,
            y2: self.stilk_height,
            color,
        });
        ctx.draw(&Line {
            x1: self.stilk_width / 2.0,
            y1: self.stilk_height,
            x2: self.head_width,
            y2: self.head_height,
            color,
        });
        ctx.draw(&Line {
            x1: -self.stilk_width / 2.0,
            y1: self.stilk_height,
            x2: -self.head_width,
            y2: self.head_height,
            color,
        });
        ctx.draw(&Line {
            x1: self.head_width,
            y1: self.head_height,
            x2: -self.head_width,
            y2: self.head_height,
            color,
        });
        ctx.draw(&Line {
            x1: -self.stilk_width / 2.0,
            y1: 0.0,
            x2: -self.foot_width,
            y2: -self.foot_height / 2.0,
            color,
        });
        ctx.draw(&Line {
            x1: self.stilk_width / 2.0,
            y1: 0.0,
            x2: self.foot_width,
            y2: -self.foot_height / 2.0,
            color,
        });
        ctx.draw(&Line {
            x1: -self.foot_width,
            y1: -self.foot_height / 2.0,
            x2: self.foot_width,
            y2: -self.foot_height / 2.0,
            color,
        });
    }
}

struct Tumbler {
    width: f64,
    height: f64,
}

impl Tumbler {
    pub fn draw(&self, ctx: &mut Context<'_>, color: Color) {
        ctx.draw(&Line {
            x1: -self.width / 2.0,
            y1: 0.0,
            x2: -self.width / 2.0,
            y2: self.height,
            color,
        });
        ctx.draw(&Line {
            x1: self.width / 2.0,
            y1: 0.0,
            x2: self.width / 2.0,
            y2: self.height,
            color,
        });
        ctx.draw(&Line {
            x1: -self.width / 2.0,
            y1: 0.0,
            x2: self.width / 2.0,
            y2: 0.0,
            color,
        });
        ctx.draw(&Line {
            x1: -self.width / 2.0,
            y1: self.height,
            x2: self.width / 2.0,
            y2: self.height,
            color,
        });

        for x in (1..7).map(|i| (self.width / 7.0) * i as f64) {
            ctx.draw(&Line {
                x1: -self.width / 2.0 + x,
                y1: 0.0,
                x2: -self.width / 2.0 + x,
                y2: self.height * 0.2,
                color,
            });
        }
    }
}
