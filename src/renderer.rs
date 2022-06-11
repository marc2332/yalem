// Author: Alberto González Palomo https://sentido-labs.com
// ©2019 Alberto González Palomo https://sentido-labs.com
// Released under the MIT license: https://opensource.org/licenses/MIT
#![allow(clippy::unusual_byte_groupings)]
use skia_safe::{Paint, PaintJoin, PaintStyle, Path, Rect};
use std::cmp::min;

const PI: f32 = std::f32::consts::PI;
const DEGREES_IN_RADIANS: f32 = PI / 180.0;
const PEN_SIZE: f32 = 1.0;

fn point_in_circle(center: (f32, f32), radius: f32, radians: f32) -> (f32, f32) {
    (
        center.0 + radius * radians.cos(),
        center.1 - radius * radians.sin(),
    )
}

pub fn render_frame(
    frame: usize,
    fps: usize,
    bpm: usize,
    canvas: &mut skia_safe::canvas::Canvas,
) -> usize {
    let step = 12.0 * bpm as f32 / 60.0 / fps as f32;
    let frame_count = (360.0 / step) as usize;

    let size = {
        let dim = canvas.image_info().dimensions();
        min(dim.width, dim.height) as i32
    };

    let center = (size / 2, size / 2);
    let triangle_radius = size / 2 * 53 / 100;

    let rotation = frame as f32 * step;

    let triangle_rotation = 60.0 + rotation;
    triangle(canvas, center, triangle_radius, triangle_rotation, true);

    frame_count - (frame + 1)
}

#[allow(clippy::many_single_char_names)]
fn triangle(
    canvas: &mut skia_safe::canvas::Canvas,
    center: (i32, i32),
    radius: i32,
    degrees: f32,
    wankel: bool,
) {
    let c = (center.0 as f32, center.1 as f32);
    let r = radius as f32;
    let b = r * 0.9;
    let delta = 120.0 * DEGREES_IN_RADIANS;

    let mut alpha = degrees * DEGREES_IN_RADIANS;
    let mut path = Path::new();
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_stroke_width(PEN_SIZE.max(canvas.image_info().dimensions().width as f32 / 360.0));
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_join(PaintJoin::Bevel);

    for i in 0..3 {
        let v = point_in_circle(c, r, alpha);
        if i == 0 {
            path.move_to(v);
        } else if wankel {
            path.cubic_to(
                point_in_circle(c, b, alpha - 2.0 * delta / 3.0),
                point_in_circle(c, b, alpha - delta / 3.0),
                v,
            );
        } else {
            path.line_to(v);
        }
        alpha += delta;
    }
    path.close();
    canvas.draw_path(&path, &paint);
}
