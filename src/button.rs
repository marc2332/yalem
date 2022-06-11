use std::cmp::min;

use skia_safe::{Canvas, Color, Paint, PaintJoin, PaintStyle, Path, Rect};

use crate::{Context, StyledWidget, Widget};

pub struct Button {
    background_color: Color,
    padding: f32,
    children: Vec<Box<dyn Widget>>,
    width: f32,
    height: f32,
}

pub struct ButtonBuilder {
    background_color: Color,
    padding: f32,
    children: Vec<Box<dyn Widget>>,
    width: f32,
    height: f32,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        Self {
            background_color: Color::TRANSPARENT,
            padding: 5.0,
            children: vec![],
            width: 5.0,
            height: 5.0,
        }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

impl From<ButtonBuilder> for Button {
    fn from(button_builder: ButtonBuilder) -> Self {
        Self {
            background_color: button_builder.background_color,
            padding: button_builder.padding,
            children: button_builder.children,
            width: button_builder.width,
            height: button_builder.height,
        }
    }
}

impl Widget for Button {
    fn draw(&self, canvas: &mut Canvas, ctx: Context) {
        let mut path = Path::new();
        let mut paint = Paint::default();

        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_color(self.background_color);

        let x = ctx.x;
        let y = ctx.y;
        let mut width = ctx.x + self.width; // This could cause overflow
        let mut height = ctx.y + self.height; // This could cause overflow

        width += self.padding;
        height += self.padding;

        println!("{x}-{y}-{width}-{height}");

        path.move_to((x, y));
        path.line_to((width, y));
        path.line_to((width, height));
        path.line_to((x, height));

        path.close();
        canvas.draw_path(&path, &paint);

        let inner_x = x + (self.padding / 2.0);
        let inner_y = y + (self.padding / 2.0);

        for child in &self.children {
            child.draw(
                canvas,
                Context {
                    x: inner_x,
                    y: inner_y,
                    width: 100.0,
                    height: 25.0,
                },
            );
        }
    }
}

impl StyledWidget for ButtonBuilder {
    fn background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    fn color(mut self, color: Color) -> Self {
        self
    }
}
