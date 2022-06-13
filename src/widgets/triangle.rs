use std::cmp::min;

use skia_safe::{Canvas, Color, Paint, PaintJoin, PaintStyle, Path};

use crate::{Context, Widget};

const PI: f32 = std::f32::consts::PI;
const DEGREES_IN_RADIANS: f32 = PI / 180.0;

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
            background_color: Color::BLACK,
        }
    }

    #[allow(dead_code)]
    fn background(mut self, color: Color) -> Self {
        self.background_color = color;
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
    // TODO(marc2332) implement get_size

    fn draw(&mut self, canvas: &mut Canvas, ctx: Context) {
        fn point_in_triangle(center: (f32, f32), radius: f32, radians: f32) -> (f32, f32) {
            (
                center.0 + radius * radians.cos(),
                center.1 - radius * radians.sin(),
            )
        }

        let size = min(ctx.width as i32, (ctx.height as i32) - 10) as i32;

        let center = (ctx.x, ctx.y + 10.0);
        let radius = size / 2 * 53 / 100;

        let c = (center.0 as f32, center.1 as f32);
        let r = radius as f32;
        let delta = 120.0 * DEGREES_IN_RADIANS;

        let mut alpha = 90.0 * DEGREES_IN_RADIANS;
        let mut path = Path::new();
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_stroke_width(1.0);
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
