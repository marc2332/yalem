use skia_safe::{Canvas, Color, Paint, PaintStyle, Path};

use crate::{Context, Widget};

pub struct Button {
    background_color: Color,
    child: Option<Box<dyn Widget>>,
    width: Option<f32>,
    height: Option<f32>,
}

pub struct ButtonBuilder {
    background_color: Color,
    child: Option<Box<dyn Widget>>,
    width: Option<f32>,
    height: Option<f32>,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        Self {
            background_color: Color::TRANSPARENT,
            child: None,
            width: None,
            height: None,
        }
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background_color = color;
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
        let mut height = ctx.height;
        let mut width = ctx.width;

        if let Some(child) = &self.child {
            let size = child.get_size(ctx.clone());
            width = size.0;
            height = size.1;
        }

        if let Some(w) = self.width {
            width = ctx.x + w;
        }

        if let Some(h) = self.height {
            height = ctx.y + h;
        }

        (width, height)
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: Context) {
        let mut path = Path::new();
        let mut paint = Paint::default();

        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_color(self.background_color);

        let x = ctx.x;
        let y = ctx.y;
        let mut width = ctx.width;
        let mut height = ctx.height;

        if let Some(child) = &self.child {
            let size = child.get_size(ctx.clone());
            width = ctx.x + size.0;
            height = ctx.y + size.1;
        }

        if let Some(w) = self.width {
            width = ctx.x + w;
        }

        if let Some(h) = self.height {
            height = ctx.y + h;
        }

        path.move_to((x, y));
        path.line_to((width, y));
        path.line_to((width, height));
        path.line_to((x, height));

        path.close();
        canvas.draw_path(&path, &paint);

        if let Some(child) = &mut self.child {
            child.draw(canvas, ctx);
        }
    }
}
