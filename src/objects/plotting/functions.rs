use crate::{objects::vector_object::VectorFeatures, utils::{line_as_cubic_bezier, start_new_path}};

pub fn parametric_function(
    f: impl Fn(f64) -> (f64, f64),
    t_min: f64,
    t_max: f64,
    t_step: f64,
    color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
    background_image: Option<web_sys::HtmlImageElement>
) -> VectorFeatures {
    let mut func_points = Vec::new();
    let mut t = t_min;
    while t <= t_max {
        let (x, y) = f(t);
        func_points.push((x, y));
        t += t_step;
    }
    let mut points = Vec::new();
    points = start_new_path(&mut points, func_points[0]);
    let mut last_point = points[0];
    for point in func_points[1..].iter() {
        points.extend(line_as_cubic_bezier(last_point, *point));
        last_point = *point;
    }
    return VectorFeatures {
        points,
        fill_color: (0.0, 0.0, 0.0, 0.0),
        stroke_color: color.unwrap_or((1.0, 1.0, 1.0, 1.0)),
        stroke_width: stroke_width.unwrap_or(4.0),
        line_cap: line_cap.unwrap_or("butt"),
        line_join: line_join.unwrap_or("miter"),
        index: index.unwrap_or(0),
        subobjects: vec![],
        background_image: background_image,
    };
}


pub fn function(
    f: impl Fn(f64) -> f64,
    x_min: f64,
    x_max: f64,
    x_step: f64,
    color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
    background_image: Option<web_sys::HtmlImageElement>
) -> VectorFeatures {
    return parametric_function(
        |t| (t, f(t)),
        x_min,
        x_max,
        x_step,
        color,
        stroke_width,
        line_cap,
        line_join,
        index,
        background_image
    );
}
