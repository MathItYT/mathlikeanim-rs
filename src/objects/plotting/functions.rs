use crate::{objects::vector_object::VectorFeatures, colors::{GradientImageOrColor, Color}, utils::line_as_cubic_bezier};

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
) -> VectorFeatures {
    let mut func_points = Vec::new();
    let mut t = t_min;
    while t <= t_max {
        let (x, y) = f(t);
        func_points.push((x, y));
        t += t_step;
    }
    let mut points = Vec::new();
    for (point1, point2) in func_points[0..func_points.len()-1].iter().zip(func_points[1..].iter()) {
        points.extend(line_as_cubic_bezier(*point1, *point2));
    }
    let (red, green, blue, alpha) = color.unwrap_or((1.0, 1.0, 1.0, 1.0));
    return VectorFeatures {
        points,
        fill: GradientImageOrColor::Color(Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0
        }),
        stroke: GradientImageOrColor::Color(Color {
            red,
            green,
            blue,
            alpha
        }),
        stroke_width: stroke_width.unwrap_or(4.0),
        line_cap: line_cap.unwrap_or("butt"),
        line_join: line_join.unwrap_or("miter"),
        index: index.unwrap_or(0),
        subobjects: vec![],
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
    );
}
