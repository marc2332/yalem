use std::borrow::Borrow;

use skia_safe::{Canvas, Color};

use crate::{padding::Padding, Context, Widget};

pub enum Direction {
    Horizontal,
    Vertical,
    Both,
}

pub struct Center {
    child: Padding,
    direction: Direction,
}

pub struct CenterBuilder {
    child: Padding,
    direction: Direction,
}

impl CenterBuilder {
    pub fn new() -> Self {
        Self {
            child: Padding {
                left: 0.0,
                right: 0.0,
                bottom: 0.0,
                top: 0.0,
                child: None,
            },
            direction: Direction::Horizontal,
        }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.child.child = Some(Box::new(child));
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
}

impl From<CenterBuilder> for Center {
    fn from(center_builder: CenterBuilder) -> Self {
        Self {
            child: center_builder.child,
            direction: center_builder.direction,
        }
    }
}

impl Widget for Center {
    fn get_size(&self, ctx: Context) -> (f32, f32) {
        self.child.get_size(ctx)
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: Context) {
        match self.direction {
            Direction::Horizontal => {
                self.child.left = ctx.width / 2.0;
                self.child.right = ctx.width / 2.0;
            }
            Direction::Vertical => {
                self.child.bottom = ctx.height / 2.0;
                self.child.top = ctx.height / 2.0;
            }
            Direction::Both => {
                self.child.left = ctx.width / 2.0;
                self.child.right = ctx.width / 2.0;
                self.child.bottom = ctx.height / 2.0;
                self.child.top = ctx.height / 2.0;
            }
        }

        self.child.draw(canvas, ctx)
    }
}
