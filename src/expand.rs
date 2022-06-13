use skia_safe::{Canvas, Color};

use crate::{center::Direction, Context, Widget};

pub struct Expand {
    child: Option<Box<dyn Widget>>,
    direction: Direction,
}

pub struct ExpandBuilder {
    child: Option<Box<dyn Widget>>,
    direction: Direction,
}

impl ExpandBuilder {
    pub fn new() -> Self {
        Self {
            child: None,
            direction: Direction::Horizontal,
        }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
}

impl From<ExpandBuilder> for Expand {
    fn from(expand_builder: ExpandBuilder) -> Self {
        Self {
            child: expand_builder.child,
            direction: expand_builder.direction,
        }
    }
}

impl Widget for Expand {
    fn get_size(&self, ctx: Context) -> (f32, f32) {
        if let Some(child) = &self.child {
            let child_size = child.get_size(ctx.clone());

            let mut width = ctx.width;
            let mut height = ctx.height;

            match self.direction {
                Direction::Horizontal => {
                    width = ctx.width - ctx.x;
                    height = child_size.1;
                }
                Direction::Vertical => {
                    width = child_size.0;
                    height = ctx.height - ctx.y;
                }
                Direction::Both => {}
            }

            (width, height)
        } else {
            (ctx.width, ctx.height)
        }
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: Context) {
        if let Some(child) = &mut self.child {
            let mut width = ctx.width;
            let mut height = ctx.height;
            let child_size = child.get_size(ctx.clone());

            match self.direction {
                Direction::Horizontal => {
                    width = ctx.width - ctx.x;
                    height = child_size.1;
                }
                Direction::Vertical => {
                    width = child_size.0;
                    height = ctx.height - ctx.y;
                }
                Direction::Both => {}
            }

            child.draw(
                canvas,
                Context {
                    height,
                    width,
                    ..ctx
                },
            )
        }
    }
}
