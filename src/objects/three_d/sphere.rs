use std::f64::consts::PI;

use crate::colors::Color;

use super::three_d_object::ThreeDObject;

pub fn sphere(
    center: (f64, f64, f64),
    radius: f64,
    u_segments: usize,
    v_segments: usize,
    fill_colors: Vec<Color>,
    stroke_colors: Vec<Color>,
    stroke_width: f64,
    index: Option<usize>
) -> ThreeDObject {
    return ThreeDObject::from_uv_function(
        &|u, v| {
            let x = radius * u.cos() * v.sin() + center.0;
            let y = radius * u.sin() * v.sin() + center.1;
            let z = -radius * v.cos() + center.2;
            return (x, y, z);
        },
        (0.0, 2.0 * PI),
        (0.0, PI),
        u_segments,
        v_segments,
        fill_colors,
        stroke_colors,
        stroke_width,
        index
    )
}