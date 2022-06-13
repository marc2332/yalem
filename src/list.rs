use skia_safe::{Canvas, Color};

use crate::{Context, Widget};

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
    fn get_size(&self, ctx: Context) -> (f32, f32) {
        let mut prev_pos_y = ctx.y;
        let max_pos = ctx.y + ctx.height;

        for child in self.children.iter() {
            let height_left = max_pos - prev_pos_y;

            prev_pos_y += child
                .get_size(Context {
                    width: ctx.width,
                    height: height_left,
                    ..ctx
                })
                .1;
        }

        (ctx.width, prev_pos_y)
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: Context) {
        let mut prev_pos_y = ctx.y;
        let max_pos = ctx.y + ctx.height;

        for child in self.children.iter_mut() {
            let height_left = max_pos - prev_pos_y;

            child.draw(
                canvas,
                Context {
                    x: ctx.x,
                    y: prev_pos_y,
                    width: ctx.width,
                    height: height_left,
                },
            );
            prev_pos_y += child
                .get_size(Context {
                    x: ctx.x,
                    y: prev_pos_y,
                    width: ctx.width,
                    height: height_left,
                })
                .1;
        }
    }
}
