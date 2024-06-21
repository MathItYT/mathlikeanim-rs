use std::f64::consts::PI;

use crate::colors::{Color, GradientImageOrColor};
use crate::objects::geometry::line::line;
use crate::objects::geometry::poly::rectangle;
use crate::objects::vector_object::{VectorFeatures, VectorObject};

use crate::objects::plotting::number_line::{number_line, number_to_point};
use crate::utils::{distance_squared, line_as_cubic_bezier};

use super::functions::{contour_plot, parametric_function};
use super::number_line::point_to_number;

pub fn axes(
    x_min: f64,
    x_max: f64,
    x_step: f64,
    y_min: f64,
    y_max: f64,
    y_step: f64,
    center: (f64, f64),
    x_length: Option<f64>,
    y_length: Option<f64>,
    color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
    add_x_ticks: Option<bool>,
    add_y_ticks: Option<bool>,
    x_tick_size: Option<f64>,
    y_tick_size: Option<f64>,
    add_x_tip: Option<bool>,
    add_y_tip: Option<bool>,
) -> VectorFeatures {
    let mut subobjects = Vec::new();
    let mut x_axis = number_line(
        x_min,
        x_max,
        x_step,
        color,
        stroke_width,
        line_cap.clone(),
        line_join.clone(),
        index,
        Some(center),
        x_length,
        add_x_tip,
        add_x_ticks,
        x_tick_size,
        Some(0.0),
    );
    let y_axis = number_line(
        y_min,
        y_max,
        y_step,
        color,
        stroke_width,
        line_cap,
        line_join,
        index,
        Some(center),
        y_length,
        add_y_tip,
        add_y_ticks,
        y_tick_size,
        Some(-PI / 2.0),
    );
    let origin_x = number_to_point(&x_axis, 0.0, x_min, x_max);
    let origin_y = number_to_point(&y_axis, 0.0, y_min, y_max);
    x_axis = x_axis.shift((origin_y.0 - origin_x.0, origin_y.1 - origin_x.1), true);
    subobjects.push(x_axis);
    subobjects.push(y_axis);
    let mut axes = VectorFeatures {
        subobjects,
        points: Vec::new(),
        fill: GradientImageOrColor::Color(
            Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0
            }
        ),
        stroke: GradientImageOrColor::Color(
            Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0
            }
        ),
        stroke_width: 0.0,
        line_cap: "butt",
        line_join: "miter",
        index: index.unwrap_or(0),
    };
    let axes_current_center = axes.get_center();
    axes = axes.shift((center.0 - axes_current_center.0, center.1 - axes_current_center.1), true);
    return axes;
}


pub fn coords_to_point(
    axes: &VectorFeatures,
    x: f64,
    y: f64,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64
) -> (f64, f64) {
    let x_point = number_to_point(&axes.subobjects[0], x, x_min, x_max);
    let y_point = number_to_point(&axes.subobjects[1], y, y_min, y_max);
    return (x_point.0, y_point.1);
}


pub fn point_to_coords(
    axes: &VectorFeatures,
    point: (f64, f64),
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64
) -> (f64, f64) {
    let x = point_to_number(&axes.subobjects[0], (point.0, axes.subobjects[0].points[0].1), x_min, x_max);
    let y = point_to_number(&axes.subobjects[1], (axes.subobjects[1].points[0].0, point.1), y_min, y_max);
    return (x, y);
}


pub fn parametric_plot_in_axes(
    f: impl Fn(f64) -> (f64, f64),
    t_min: f64,
    t_max: f64,
    t_step: f64,
    axes: &VectorFeatures,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
) -> VectorFeatures {
    let new_f = |t: f64| {
        let (x, y) = f(t);
        let point = coords_to_point(axes, x, y, x_min, x_max, y_min, y_max);
        return point;
    };
    return parametric_function(
        new_f,
        t_min,
        t_max,
        t_step,
        color,
        stroke_width,
        line_cap,
        line_join,
        index,
    );
}


pub fn plot_in_axes(
    f: impl Fn(f64) -> f64,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    x_1: f64,
    x_2: f64,
    x_step: f64,
    axes: &VectorFeatures,
    color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
) -> VectorFeatures {
    let new_f = |t: f64| {
        let x = t;
        let y = f(t);
        let point = coords_to_point(axes, x, y, x_min, x_max, y_min, y_max);
        return point;
    };
    return parametric_function(
        new_f,
        x_1,
        x_2,
        x_step,
        color,
        stroke_width,
        line_cap,
        line_join,
        index,
    );
}


pub fn contour_plot_in_axes(
    f: impl Fn(f64, f64) -> f64,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    x_1: f64,
    x_2: f64,
    x_step: f64,
    y_1: f64,
    y_2: f64,
    y_step: f64,
    axes: &VectorFeatures,
    color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
    intervals: &[f64],
) -> VectorFeatures {
    let mut func = contour_plot(
        f,
        x_1,
        x_2,
        x_step,
        y_1,
        y_2,
        y_step,
        color,
        stroke_width,
        line_cap,
        line_join,
        index,
        intervals
    );
    let x_unit_size = coords_to_point(axes, 1.0, 0.0, x_min, x_max, y_min, y_max).0 - coords_to_point(axes, 0.0, 0.0, x_min, x_max, y_min, y_max).0;
    let y_unit_size = coords_to_point(axes, 0.0, 1.0, x_min, x_max, y_min, y_max).1 - coords_to_point(axes, 0.0, 0.0, x_min, x_max, y_min, y_max).1;
    func = func.stretch((x_unit_size, y_unit_size), true);
    func = func.shift(coords_to_point(axes, 0.0, 0.0, x_min, x_max, y_min, y_max), true);
    return func;
}


pub fn area_under_curve(
    axes: &VectorFeatures,
    plot: &VectorFeatures,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    x1: f64,
    x2: f64,
    color: Option<(f64, f64, f64, f64)>,
    index: Option<usize>,
) -> VectorFeatures {
    let mut x1_index = 0;
    let mut x2_index = 0;
    for i in 0..plot.points.len() {
        let x_coord = point_to_coords(axes, plot.points[i], x_min, x_max, y_min, y_max).0;
        if x_coord >= x1 {
            x1_index = i;
            break;
        }
    }
    for i in 0..plot.points.len() {
        let x_coord = point_to_coords(axes, plot.points[i], x_min, x_max, y_min, y_max).0;
        if x_coord >= x2 {
            x2_index = i;
            break;
        }
    }
    let mut area_points = Vec::new();
    for (point1, point2) in plot.points[x1_index..x2_index].iter().zip(plot.points[x1_index + 1..x2_index + 1].iter()) {
        area_points.push(*point1);
        area_points.push(*point2);
    }
    area_points.extend(line_as_cubic_bezier(area_points[area_points.len() - 1], (area_points[area_points.len() - 1].0, number_to_point(&axes.subobjects[axes.subobjects.len() - 1], 0.0, y_min, y_max).1)));
    area_points.extend(line_as_cubic_bezier(area_points[area_points.len() - 1], (area_points[0].0, number_to_point(&axes.subobjects[axes.subobjects.len() - 1], 0.0, y_min, y_max).1)));
    area_points.extend(line_as_cubic_bezier(area_points[area_points.len() - 1], area_points[0]));
    let (red, green, blue, alpha) = color.unwrap_or((1.0, 1.0, 1.0, 1.0));
    let area = VectorFeatures {
        points: area_points,
        fill: GradientImageOrColor::Color(
            Color {
                red,
                green,
                blue,
                alpha
            }
        ),
        stroke: GradientImageOrColor::Color(
            Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }
        ),
        stroke_width: 0.0,
        line_cap: "butt",
        line_join: "miter",
        index: index.unwrap_or(0),
        subobjects: vec![],
    };
    return area;
}


pub fn riemann_rectangles_for_plot(
    f: impl Fn(f64) -> f64,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    direction: f64,
    x_1: f64,
    x_2: f64,
    n_rects: usize,
    axes: &VectorFeatures,
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
) -> VectorFeatures {
    let mut subobjects = Vec::new();
    let dx = (x_2 - x_1) / n_rects as f64;
    if direction < 0.0 {
        for i in 0..n_rects {
            let x = x_1 + i as f64 * dx;
            let y = f(x);
            let width = number_to_point(&axes.subobjects[0], x + dx, x_min, x_max).0 - number_to_point(&axes.subobjects[0], x, x_min, x_max).0;
            let y_origin = number_to_point(&axes.subobjects[1], 0.0, y_min, y_max);
            let y_point = number_to_point(&axes.subobjects[1], y, y_min, y_max);
            let mut rect = rectangle(
                (0.0, 0.0),
                width,
                (y_point.1 - y_origin.1).abs(),
                stroke_color,
                fill_color,
                stroke_width,
                line_cap,
                line_join,
                index,
            );
            if y > 0.0 {
                rect = rect.next_to_point(
                    (number_to_point(&axes.subobjects[0], x + dx, x_min, x_max).0, y_origin.1),
                    (0.0, -1.0),
                    0.0,
                    (1.0, 0.0),
                    true
                );
            } else {
                rect = rect.next_to_point(
                    (number_to_point(&axes.subobjects[0], x + dx, x_min, x_max).0, y_origin.1),
                    (0.0, 1.0),
                    0.0,
                    (1.0, 0.0),
                    true
                )
            }
            subobjects.push(rect);
        }
        return VectorFeatures {
            points: Vec::new(),
            fill: GradientImageOrColor::Color(
                Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.0
                }
            ),
            stroke: GradientImageOrColor::Color(
                Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.0
                }
            ),
            stroke_width: 0.0,
            line_cap: "butt",
            line_join: "miter",
            index: index.unwrap_or(0),
            subobjects,
        };
    } else if direction == 0.0 {
        for i in 0..n_rects {
            let x = x_1 + i as f64 * dx;
            let y = f(x + dx / 2.0);
            let width = number_to_point(&axes.subobjects[0], x + dx, x_min, x_max).0 - number_to_point(&axes.subobjects[0], x, x_min, x_max).0;
            let y_origin = number_to_point(&axes.subobjects[1], 0.0, y_min, y_max);
            let y_point = number_to_point(&axes.subobjects[1], y, y_min, y_max);
            let mut rect = rectangle(
                (0.0, 0.0),
                width,
                (y_point.1 - y_origin.1).abs(),
                stroke_color,
                fill_color,
                stroke_width,
                line_cap,
                line_join,
                index,
            );
            if y > 0.0 {
                rect = rect.next_to_point(
                    (number_to_point(&axes.subobjects[0], x + dx, x_min, x_max).0, y_origin.1),
                    (0.0, -1.0),
                    0.0,
                    (1.0, 0.0),
                    true
                );
            } else {
                rect = rect.next_to_point(
                    (number_to_point(&axes.subobjects[0], x + dx, x_min, x_max).0, y_origin.1),
                    (0.0, 1.0),
                    0.0,
                    (1.0, 0.0),
                    true
                )
            }
            subobjects.push(rect);
        }
        return VectorFeatures {
            points: Vec::new(),
            fill: GradientImageOrColor::Color(
                Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.0
                }
            ),
            stroke: GradientImageOrColor::Color(
                Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.0
                }
            ),
            stroke_width: 0.0,
            line_cap: "butt",
            line_join: "miter",
            index: index.unwrap_or(0),
            subobjects,
        };
    }
    for i in 0..n_rects {
        let x = x_1 + i as f64 * dx;
        let y = f(x + dx);
        let width = number_to_point(&axes.subobjects[0], x + dx, x_min, x_max).0 - number_to_point(&axes.subobjects[0], x, x_min, x_max).0;
        let y_origin = number_to_point(&axes.subobjects[1], 0.0, y_min, y_max);
        let y_point = number_to_point(&axes.subobjects[1], y, y_min, y_max);
        let mut rect = rectangle(
            (0.0, 0.0),
            width,
            (y_point.1 - y_origin.1).abs(),
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join,
            index,
        );
        if y > 0.0 {
            rect = rect.next_to_point(
                (number_to_point(&axes.subobjects[0], x + dx, x_min, x_max).0, y_origin.1),
                (0.0, -1.0),
                0.0,
                (1.0, 0.0),
                true
            );
        } else {
            rect = rect.next_to_point(
                (number_to_point(&axes.subobjects[0], x + dx, x_min, x_max).0, y_origin.1),
                (0.0, 1.0),
                0.0,
                (1.0, 0.0),
                true
            )
        }
        subobjects.push(rect);
    }
    return VectorFeatures {
        points: Vec::new(),
        fill: GradientImageOrColor::Color(
            Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }
        ),
        stroke: GradientImageOrColor::Color(
            Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }
        ),
        stroke_width: 0.0,
        line_cap: "butt",
        line_join: "miter",
        index: index.unwrap_or(0),
        subobjects,
    };
}


pub fn secant_line_for_plot(
    f: impl Fn(f64) -> f64,
    x_1: f64,
    x_2: f64,
    length: f64,
    axes: &VectorFeatures,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
) -> VectorFeatures {
    let y_1 = f(x_1);
    let y_2 = f(x_2);
    let point_1 = coords_to_point(axes, x_1, y_1, x_min, x_max, y_min, y_max);
    let point_2 = coords_to_point(axes, x_2, y_2, x_min, x_max, y_min, y_max);
    let mut line = line(
        point_1,
        point_2,
        color,
        stroke_width,
        line_cap,
        line_join,
        index,
    );
    let old_length = distance_squared(point_1, point_2).sqrt();
    line =  line.scale(length / old_length, true).move_to(line.get_center(), true);
    return line;
}