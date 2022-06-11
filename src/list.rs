use skia_safe::{Canvas, Color};

use crate::{Context, StyledWidget, Widget};

pub struct List {
    children: Vec<Box<dyn Widget>>,
}

pub struct ListBuilder {
    children: Vec<Box<dyn Widget>>,
}

impl ListBuilder {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl From<ListBuilder> for List {
    fn from(list_builder: ListBuilder) -> Self {
        Self {
            children: list_builder.children,
        }
    }
}

impl Widget for List {
    fn draw(&self, canvas: &mut Canvas, ctx: Context) {
        for (i, child) in self.children.iter().enumerate() {
            child.draw(
                canvas,
                Context {
                    x: ctx.x,
                    y: ctx.y + ((i as f32) * 60.0),
                    width: ctx.width,
                    height: 20.0,
                },
            )
        }
    }
}

impl StyledWidget for ListBuilder {
    fn background(mut self, color: Color) -> Self {
        self
    }

    fn color(mut self, color: Color) -> Self {
        self
    }
}
