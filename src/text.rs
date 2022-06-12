use skia_safe::{
    utils::text_utils::Align, Canvas, Color, Font, FontStyle, Paint, PaintStyle, Path, TextBlob,
};

use crate::{Context, StyledWidget, Widget};

pub struct Text {
    color: Color,
    text: String,
    align: Align,
}

pub struct TextBuilder {
    color: Color,
    text: String,
    align: Align,
}

impl TextBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            color: Color::BLACK,
            text: text.into(),
            align: Align::Left,
        }
    }

    pub fn set_align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }
}

impl From<TextBuilder> for Text {
    fn from(text_builder: TextBuilder) -> Self {
        Self {
            color: text_builder.color,
            text: text_builder.text,
            align: text_builder.align,
        }
    }
}

impl Widget for Text {
    fn get_size(&self, ctx: Context) -> (f32, f32) {
        let size_char = (5.5, 10.0);
        let mut width = (self.text.len() as f32) * size_char.0;
        let mut height = size_char.1;

        // break 1 line for now
        if width > ctx.width {
            width = ctx.width;
            height *= 2.0;
        }

        (width, height)
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: Context) {
        let font = Font::default();

        let mut paint = Paint::default();

        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::StrokeAndFill);
        paint.set_color(self.color);

        let x = ctx.x;
        let y = ctx.y + 9.0;

        canvas.draw_str_align(&self.text, (x, y), &font, &paint, self.align);
    }
}

impl StyledWidget for TextBuilder {
    fn background(mut self, color: Color) -> Self {
        self
    }

    fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}
