use skia_safe::{Canvas, Color};

use crate::{Context, StyledWidget, Widget};

pub struct Expand {
    child: Option<Box<dyn Widget>>,
}

pub struct ExpandBuilder {
    child: Option<Box<dyn Widget>>,
}

impl ExpandBuilder {
    pub fn new() -> Self {
        Self { child: None }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }
}

impl From<ExpandBuilder> for Expand {
    fn from(expand_builder: ExpandBuilder) -> Self {
        Self {
            child: expand_builder.child,
        }
    }
}

impl Widget for Expand {
    fn get_size(&self, ctx: Context) -> (f32, f32) {
        (ctx.width, ctx.height)
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: Context) {
        if let Some(child) = &mut self.child {
            child.draw(
                canvas,
                Context {
                    height: ctx.height - ctx.y,
                    width: ctx.width - ctx.x,
                    ..ctx
                },
            )
        }
    }
}

impl StyledWidget for ExpandBuilder {
    fn background(mut self, color: Color) -> Self {
        self
    }

    fn color(mut self, color: Color) -> Self {
        self
    }
}
