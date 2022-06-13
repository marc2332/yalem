use glutin::event::{ElementState, MouseButton, WindowEvent};
use skia_safe::{Canvas, Color, Paint, PaintStyle, Path};

use crate::{Context, Widget, YalemEvent, YalemMouse};

pub struct Button {
    background_color: Color,
    child: Option<Box<dyn Widget>>,
    width: Option<f32>,
    height: Option<f32>,
    callback: Option<Box<dyn FnMut() -> ()>>,
    positions: (f64, f64, f64, f64),
}

pub struct ButtonBuilder {
    background_color: Color,
    child: Option<Box<dyn Widget>>,
    width: Option<f32>,
    height: Option<f32>,
    callback: Option<Box<dyn FnMut() -> ()>>,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        Self {
            background_color: Color::TRANSPARENT,
            child: None,
            width: None,
            height: None,
            callback: None,
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

    pub fn on_click<T>(mut self, callback: T) -> Self
    where
        T: FnMut() -> () + 'static,
    {
        self.callback = Some(Box::new(callback));
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
            callback: button_builder.callback,
            positions: (0.0, 0.0, 0.0, 0.0),
        }
    }
}

impl Widget for Button {
    fn send_event(&mut self, event: &YalemEvent) {
        if let Some(child) = &mut self.child {
            child.send_event(&event)
        }

        match event {
            YalemEvent::YalemMouse(YalemMouse::Pressed { button, position }) => {
                if let Some(callback) = &mut self.callback {
                    if &MouseButton::Left == button {
                        if position.0 >= self.positions.0
                            && position.1 >= self.positions.1
                            && position.0 <= self.positions.2
                            && position.1 <= self.positions.3
                        {
                            callback()
                        }
                    }
                }
            }
            _ => {}
        }
    }

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

        self.positions = (x as f64, y as f64, width as f64, height as f64);

        path.close();
        canvas.draw_path(&path, &paint);

        if let Some(child) = &mut self.child {
            child.draw(canvas, ctx);
        }
    }
}
