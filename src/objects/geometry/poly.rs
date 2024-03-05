use std::f64::consts::PI;

use crate::{colors::{Color, GradientImageOrColor}, objects::vector_object::VectorFeatures, utils::line_as_cubic_bezier};

pub fn polygon(
    points: Vec<(f64, f64)>,
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
) -> VectorFeatures {
    let mut new_points = Vec::new();
    for i in 0..points.len()-1 {
        new_points.extend(line_as_cubic_bezier(
            points[i],
            points[i + 1]
        ));
    }
    new_points.extend(line_as_cubic_bezier(
        points[points.len()-1],
        points[0]
    ));
    return VectorFeatures {
        points: new_points,
        subobjects: vec![],
        index: match index {
            Some(i) => i,
            None => 0
        },
        stroke: match stroke_color {
            Some(color) => GradientImageOrColor::Color(Color {
                red: color.0,
                green: color.1,
                blue: color.2,
                alpha: color.3
            }),
            None => GradientImageOrColor::Color(Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0
            })
        },
        fill: match fill_color {
            Some(color) => GradientImageOrColor::Color(Color {
                red: color.0,
                green: color.1,
                blue: color.2,
                alpha: color.3
            }),
            None => GradientImageOrColor::Color(Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            })
        },
        stroke_width: match stroke_width {
            Some(width) => width,
            None => 4.0
        },
        line_cap: match line_cap {
            Some(cap) => cap,
            None => "butt"
        },
        line_join: match line_join {
            Some(join) => join,
            None => "miter"
        },
    };
}


pub fn regular_polygon(
    center: (f64, f64),
    side_length: f64,
    num_sides: usize,
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
) -> VectorFeatures {
    let mut points = Vec::new();
    for i in 0..num_sides {
        let r = side_length / (2.0 * (PI / num_sides as f64).sin());
        let angle = 2.0 * PI * i as f64 / num_sides as f64;
        points.push((
            center.0 + r * angle.cos(),
            center.1 + r * angle.sin()
        ));
    }
    return polygon(
        points,
        stroke_color,
        fill_color,
        stroke_width,
        line_cap,
        line_join,
        index,
    );
}


pub fn square(
    center: (f64, f64),
    side_length: f64,
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>
) -> VectorFeatures {
    return regular_polygon(
        center,
        side_length,
        4,
        stroke_color,
        fill_color,
        stroke_width,
        line_cap,
        line_join,
        index,
    );
}


pub fn rectangle(
    center: (f64, f64),
    width: f64,
    height: f64,
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
) -> VectorFeatures {
    let half_width = width / 2.0;
    let half_height = height / 2.0;
    return polygon(
        vec![
            (center.0 - half_width, center.1 - half_height),
            (center.0 + half_width, center.1 - half_height),
            (center.0 + half_width, center.1 + half_height),
            (center.0 - half_width, center.1 + half_height)
        ],
        stroke_color,
        fill_color,
        stroke_width,
        line_cap,
        line_join,
        index,
    );
}


pub fn equilateral_triangle(
    center: (f64, f64),
    side_length: f64,
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
) -> VectorFeatures {
    return regular_polygon(
        center,
        side_length,
        3,
        stroke_color,
        fill_color,
        stroke_width,
        line_cap,
        line_join,
        index,
    );
}


pub fn triangle(
    point1: (f64, f64),
    point2: (f64, f64),
    point3: (f64, f64),
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
) -> VectorFeatures {
    return polygon(
        vec![point1, point2, point3],
        stroke_color,
        fill_color,
        stroke_width,
        line_cap,
        line_join,
        index,
    );
}


pub fn right_triangle(
    point1: (f64, f64),
    point2: (f64, f64),
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
) -> VectorFeatures {
    return polygon(
        vec![point1, point2, (point1.0, point2.1)],
        stroke_color,
        fill_color,
        stroke_width,
        line_cap,
        line_join,
        index,
    );
}
