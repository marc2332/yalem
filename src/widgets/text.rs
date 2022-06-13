use skia_safe::{utils::text_utils::Align, Canvas, Color, Font, Paint, PaintStyle};

use crate::{Context, Widget};

pub struct Text {
    color: Color,
    text: String,
    align: Align,
}

impl Text {
    pub fn builder(text: impl Into<String>) -> TextBuilder {
        TextBuilder::new(text)
    }
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

    pub fn build(self) -> Text {
        Text::from(self)
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
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
        let size_char = (6.0, 12.5);
        let mut width = (self.text.len() as f32) * size_char.0;
        let height = size_char.1;

        // TODO(marc2332) break lines
        if width > ctx.width {
            width = ctx.width;
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
