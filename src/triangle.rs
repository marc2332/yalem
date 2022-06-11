use std::cmp::min;

use skia_safe::{Canvas, Color, Paint, PaintJoin, PaintStyle, Path};

use crate::{Context, StyledWidget, Widget};

const PI: f32 = std::f32::consts::PI;
const DEGREES_IN_RADIANS: f32 = PI / 180.0;
const PEN_SIZE: f32 = 1.0;

#[derive(Clone)]
pub struct Triangle {
    background_color: Color,
}

pub struct TriangleBuilder {
    background_color: Color,
}

impl TriangleBuilder {
    pub fn new() -> Self {
        Self {
            background_color: Color::TRANSPARENT,
        }
    }

    fn child(&mut self, child: impl Widget) -> &mut Self {
        self
    }
}

impl From<TriangleBuilder> for Triangle {
    fn from(triangle_builder: TriangleBuilder) -> Self {
        Self {
            background_color: triangle_builder.background_color,
        }
    }
}

impl Widget for Triangle {
    fn draw(&self, canvas: &mut Canvas, ctx: Context) {
        fn point_in_triangle(center: (f32, f32), radius: f32, radians: f32) -> (f32, f32) {
            (
                center.0 + radius * radians.cos(),
                center.1 - radius * radians.sin(),
            )
        }

        let size = {
            let dim = canvas.image_info().dimensions();
            min(dim.width, dim.height) as i32
        };

        let center = (size / 2, size / 2);
        let radius = size / 2 * 53 / 100;

        let c = (center.0 as f32, center.1 as f32);
        let r = radius as f32;
        let delta = 120.0 * DEGREES_IN_RADIANS;

        let mut alpha = 90.0 * DEGREES_IN_RADIANS;
        let mut path = Path::new();
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_stroke_width(PEN_SIZE.max(canvas.image_info().dimensions().width as f32 / 360.0));
        paint.set_style(PaintStyle::Stroke);
        paint.set_stroke_join(PaintJoin::Bevel);

        for i in 0..3 {
            let v = point_in_triangle(c, r, alpha);
            if i == 0 {
                path.move_to(v);
            } else {
                path.line_to(v);
            }
            alpha += delta;
        }
        paint.set_color(self.background_color);
        path.close();
        canvas.draw_path(&path, &paint);
    }
}

impl StyledWidget for TriangleBuilder {
    fn background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }
    fn color(mut self, color: Color) -> Self {
        self
    }
}
