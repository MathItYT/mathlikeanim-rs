use std::f64::consts::PI;

use crate::{objects::vector_object::VectorFeatures, utils::line_as_cubic_bezier};

pub fn polygon(
    points: Vec<(f64, f64)>,
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
    background_image: Option<web_sys::HtmlImageElement>
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
        stroke_color: match stroke_color {
            Some(color) => color,
            None => (1.0, 1.0, 1.0, 1.0)
        },
        fill_color: match fill_color {
            Some(color) => color,
            None => (0.0, 0.0, 0.0, 0.0)
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
        background_image
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
    background_image: Option<web_sys::HtmlImageElement>
) -> VectorFeatures {
    let mut points = Vec::new();
    let angle = 2.0 * PI / num_sides as f64;
    for i in 0..num_sides {
        let x = center.0 + side_length * (angle * i as f64).cos();
        let y = center.1 + side_length * (angle * i as f64).sin();
        points.push((x, y));
    }
    return polygon(
        points,
        stroke_color,
        fill_color,
        stroke_width,
        line_cap,
        line_join,
        index,
        background_image
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
    index: Option<usize>,
    background_image: Option<web_sys::HtmlImageElement>
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
        background_image
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
    background_image: Option<web_sys::HtmlImageElement>
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
        background_image
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
    background_image: Option<web_sys::HtmlImageElement>
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
        background_image
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
    background_image: Option<web_sys::HtmlImageElement>
) -> VectorFeatures {
    return polygon(
        vec![point1, point2, point3],
        stroke_color,
        fill_color,
        stroke_width,
        line_cap,
        line_join,
        index,
        background_image
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
    background_image: Option<web_sys::HtmlImageElement>
) -> VectorFeatures {
    return polygon(
        vec![point1, point2, (point1.0, point2.1)],
        stroke_color,
        fill_color,
        stroke_width,
        line_cap,
        line_join,
        index,
        background_image
    );
}
