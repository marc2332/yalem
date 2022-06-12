use skia_safe::{Canvas, Color};

use crate::{Context, StyledWidget, Widget};

pub struct Padding {
    pub(crate) left: f32,
    pub(crate) right: f32,
    pub(crate) bottom: f32,
    pub(crate) top: f32,
    pub(crate) child: Option<Box<dyn Widget>>,
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
    fn get_size(&self, ctx: Context) -> (f32, f32) {
        let child = self.child.as_ref().unwrap();
        let width = ctx.width + self.right + self.left;
        let height = ctx.height + self.top + self.bottom;

        let child_size = child.get_size(Context {
            x: ctx.x,
            y: ctx.y,
            width,
            height,
        });

        (
            child_size.0 + self.right + self.left,
            child_size.1 + self.top + self.bottom,
        )
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: Context) {
        let x = ctx.x + self.left;
        let y = ctx.y + self.top;
        let width = ctx.width + self.right + self.left;
        let height = ctx.height + self.top + self.bottom;

        if let Some(child) = &mut self.child {
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
