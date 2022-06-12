use std::borrow::Borrow;

use skia_safe::{Canvas, Color};

use crate::{padding::Padding, Context, StyledWidget, Widget};

pub struct Center {
    child: Padding,
}

pub struct CenterBuilder {
    child: Padding,
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
        }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.child.child = Some(Box::new(child));
        self
    }
}

impl From<CenterBuilder> for Center {
    fn from(center_builder: CenterBuilder) -> Self {
        Self {
            child: center_builder.child,
        }
    }
}

impl Widget for Center {
    fn get_size(&self, ctx: Context) -> (f32, f32) {
        self.child.get_size(ctx)
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: Context) {
        self.child.left = ctx.width / 2.0;
        self.child.right = ctx.width / 2.0;
        self.child.bottom = ctx.height / 2.0;
        self.child.top = ctx.height / 2.0;

        self.child.draw(canvas, ctx)
    }
}

impl StyledWidget for CenterBuilder {
    fn background(mut self, color: Color) -> Self {
        self
    }

    fn color(mut self, color: Color) -> Self {
        self
    }
}
