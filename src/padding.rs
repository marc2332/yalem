use skia_safe::{Canvas, Color};

use crate::{Context, StyledWidget, Widget};

pub struct Padding {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    child: Option<Box<dyn Widget>>,
}

pub struct PaddingBuilder {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    child: Option<Box<dyn Widget>>,
}

impl PaddingBuilder {
    pub fn new((left, right, bottom, top): (f32, f32, f32, f32)) -> Self {
        Self {
            left,
            right,
            bottom,
            top,
            child: None,
        }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }
}

impl From<PaddingBuilder> for Padding {
    fn from(padding_builder: PaddingBuilder) -> Self {
        Self {
            left: padding_builder.left,
            right: padding_builder.right,
            bottom: padding_builder.bottom,
            top: padding_builder.top,
            child: padding_builder.child,
        }
    }
}

impl Widget for Padding {
    fn draw(&self, canvas: &mut Canvas, ctx: Context) {
        let x = ctx.x + self.left;
        let y = ctx.y + self.top;
        let width = ctx.width - self.right;
        let height = ctx.height - self.bottom;

        if let Some(child) = &self.child {
            child.draw(
                canvas,
                Context {
                    x,
                    y,
                    width,
                    height,
                },
            )
        }
    }
}

impl StyledWidget for PaddingBuilder {
    fn background(mut self, color: Color) -> Self {
        self
    }

    fn color(mut self, color: Color) -> Self {
        self
    }
}
