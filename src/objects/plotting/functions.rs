use contour_isobands::ContourBuilder;

use crate::{colors::{Color, GradientImageOrColor}, objects::vector_object::VectorObject, utils::line_as_cubic_bezier};

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
) -> VectorObject {
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
    return VectorObject {
        points,
        fill: GradientImageOrColor::Color(Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0
        }),
        fill_rule: "nonzero",
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
) -> VectorObject {
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

pub fn contour_plot(
    f: impl Fn(f64, f64) -> f64,
    x_min: f64,
    x_max: f64,
    x_step: f64,
    y_min: f64,
    y_max: f64,
    y_step: f64,
    color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
    intervals: &[f64],
) -> VectorObject {
    let color = color.unwrap_or((1.0, 1.0, 1.0, 1.0));
    let color = Color {
        red: color.0,
        green: color.1,
        blue: color.2,
        alpha: color.3,
    };
    let color = GradientImageOrColor::Color(color);
    let x_grids_number = ((x_max - x_min) / x_step).ceil() as usize;
    let y_grids_number = ((y_max - y_min) / y_step).ceil() as usize;
    let mut grid = Vec::new();
    for j in 0..y_grids_number {
        for i in 0..x_grids_number {
            let x = x_min + i as f64 * x_step;
            let y = y_min + j as f64 * y_step;
            grid.push(f(x, y));
        }
    }
    let res = ContourBuilder::new(x_grids_number, y_grids_number)
        .x_origin(x_min)
        .y_origin(y_min)
        .x_step(x_step)
        .y_step(y_step)
        .use_quad_tree(true)
        .contours(&grid, intervals)
        .unwrap();
    let mut result = VectorObject::new();
    for contour in res {
        for poly in contour.geometry() {
            let interiors = poly.interiors().iter().map(|interior| {
                interior.0.iter().map(|point| (point.x, point.y)).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
            for interior in interiors {
                let mut points = Vec::new();
                for (point1, point2) in interior[0..interior.len()-1].iter().zip(interior[1..].iter()) {
                    points.extend(line_as_cubic_bezier(*point1, *point2));
                }
                result.subobjects.push(VectorObject {
                    points,
                    fill_rule: "nonzero",
                    stroke: color.clone(),
                    fill: GradientImageOrColor::Color(Color {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                        alpha: 0.0
                    }),
                    stroke_width: stroke_width.unwrap_or(4.0),
                    line_cap: line_cap.unwrap_or("butt"),
                    line_join: line_join.unwrap_or("miter"),
                    index: index.unwrap_or(0),
                    subobjects: vec![],
                });
            }
        }
    }
    return result.set_index(index.unwrap_or(0));
}
