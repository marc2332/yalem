use skia_safe::{Canvas, Color, Paint, PaintStyle, Path};

use crate::{Context, StyledWidget, Widget};

pub struct Button {
    background_color: Color,
    child: Option<Box<dyn Widget>>,
    width: f32,
    height: f32,
}

pub struct ButtonBuilder {
    background_color: Color,
    child: Option<Box<dyn Widget>>,
    width: f32,
    height: f32,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        Self {
            background_color: Color::TRANSPARENT,
            child: None,
            width: 5.0,
            height: 5.0,
        }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
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
            child: button_builder.child,
            width: button_builder.width,
            height: button_builder.height,
        }
    }
}

impl Widget for Button {
    fn get_size(&self, ctx: Context) -> (f32, f32) {
        let x = ctx.x;
        let y = ctx.y;
        let mut width = ctx.x;
        let mut height = ctx.y;

        let inner_x = x;
        let inner_y = y;

        if let Some(child) = &self.child {
            let size = child.get_size(Context {
                x: inner_x,
                y: inner_y,
                width: ctx.width,
                height: ctx.height,
            });

            size
        } else {
            width += self.width;
            height += self.height;

            (width, height)
        }
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: Context) {
        let mut path = Path::new();
        let mut paint = Paint::default();

        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_color(self.background_color);

        let x = ctx.x;
        let y = ctx.y;
        let mut width = ctx.x;
        let mut height = ctx.y;

        let inner_x = x;
        let inner_y = y;

        let child_size = if let Some(child) = &self.child {
            let size = child.get_size(Context {
                x: inner_x,
                y: inner_y,
                width: ctx.width,
                height: ctx.height,
            });

            width += size.0;
            height += size.1;

            Some(size)
        } else {
            width += self.width;
            height += self.height;

            None
        };

        path.move_to((x, y));
        path.line_to((width, y));
        path.line_to((width, height));
        path.line_to((x, height));

        path.close();
        canvas.draw_path(&path, &paint);

        if let Some(child) = &mut self.child {
            let size = child_size.unwrap();
            child.draw(
                canvas,
                Context {
                    x: inner_x,
                    y: inner_y,
                    width: size.0,
                    height: size.1,
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

    fn color(self, _color: Color) -> Self {
        self
    }
}
