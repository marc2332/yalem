use skia_safe::{Canvas, Color, Font, FontStyle, Paint, PaintStyle, Path, TextBlob};

use crate::{Context, StyledWidget, Widget};

#[derive(Clone)]
pub struct Text {
    color: Color,
    padding: f32,
    text: String,
}

pub struct TextBuilder {
    color: Color,
    padding: f32,
    text: String,
}

impl TextBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            color: Color::BLACK,
            padding: 5.0,
            text: text.into(),
        }
    }
}

impl From<TextBuilder> for Text {
    fn from(text_builder: TextBuilder) -> Self {
        Self {
            color: text_builder.color,
            padding: text_builder.padding,
            text: text_builder.text,
        }
    }
}

impl Widget for Text {
    fn draw(&self, canvas: &mut Canvas, ctx: Context) {
        let font = Font::default();
        let text_blob = TextBlob::new(&self.text, &font);

        let mut paint = Paint::default();

        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::StrokeAndFill);
        paint.set_color(self.color);

        let x = ctx.x;
        let y = ctx.y + 3.0;

        canvas.draw_text_blob(&text_blob.unwrap(), (x, y), &paint);

        println!("DONE");
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
