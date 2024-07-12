use std::f64::consts::PI;

use crate::colors::{Color, GradientImageOrColor};
use crate::objects::three_d::three_d_object::ThreeDObject;

use crate::objects::plotting::number_line::number_line;
use crate::objects::vector_object::VectorObject;
use crate::utils::{interpolate, interpolate_tuple_3d};

use super::three_d_object::line_as_cubic_bezier_3d;


pub fn three_d_axes(
    x_min: f64,
    x_max: f64,
    x_step: f64,
    y_min: f64,
    y_max: f64,
    y_step: f64,
    z_min: f64,
    z_max: f64,
    z_step: f64,
    center: (f64, f64, f64),
    x_length: Option<f64>,
    y_length: Option<f64>,
    z_length: Option<f64>,
    color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    add_x_ticks: Option<bool>,
    add_y_ticks: Option<bool>,
    add_z_ticks: Option<bool>,
    x_tick_size: Option<f64>,
    y_tick_size: Option<f64>,
    z_tick_size: Option<f64>,
    add_x_tip: Option<bool>,
    add_y_tip: Option<bool>,
    add_z_tip: Option<bool>,
    n_pieces: Option<usize>,
) -> ThreeDObject {
    let mut subobjects = Vec::new();
    let mut x_axis = number_line(
        x_min,
        x_max,
        x_step,
        color,
        stroke_width,
        None,
        None,
        None,
        Some((0.0, 0.0)),
        x_length,
        add_x_tip,
        add_x_ticks,
        x_tick_size,
        Some(0.0),
    );
    let mut x_axis_pieces = x_axis.get_pieces(n_pieces.unwrap_or(20));
    x_axis_pieces.subobjects.extend(x_axis.subobjects);
    x_axis = x_axis_pieces;
    let mut y_axis = number_line(
        y_min,
        y_max,
        y_step,
        color,
        stroke_width,
        None,
        None,
        None,
        Some((0.0, 0.0)),
        y_length,
        add_y_tip,
        add_y_ticks,
        y_tick_size,
        Some(-PI / 2.0),
    );
    let mut y_axis_pieces = y_axis.get_pieces(n_pieces.unwrap_or(20));
    y_axis_pieces.subobjects.extend(y_axis.subobjects);
    y_axis = y_axis_pieces;
    let mut z_axis = number_line(
        z_min,
        z_max,
        z_step,
        color,
        stroke_width,
        None,
        None,
        None,
        Some((0.0, 0.0)),
        z_length,
        add_z_tip,
        add_z_ticks,
        z_tick_size,
        Some(PI / 2.0),
    );
    let mut z_axis_pieces = z_axis.get_pieces(n_pieces.unwrap_or(20));
    z_axis_pieces.subobjects.extend(z_axis.subobjects);
    z_axis = z_axis_pieces;
    let x_axis_3d = ThreeDObject::from_vector_object(&x_axis);
    let y_axis_3d = ThreeDObject::from_vector_object(&y_axis);
    let z_axis_3d = ThreeDObject::from_vector_object(&z_axis).rotate_x(PI / 2.0, true);
    subobjects.push(x_axis_3d);
    subobjects.push(y_axis_3d);
    subobjects.push(z_axis_3d);
    let axes = ThreeDObject {
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
    };
    axes.shift((center.0, center.1, center.2), true)
}


pub fn coords_to_point_3d(
    axes: &ThreeDObject,
    coords: (f64, f64, f64),
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
) -> (f64, f64, f64) {
    let x_t = (coords.0 - x_min) / (x_max - x_min);
    let y_t = (coords.1 - y_min) / (y_max - y_min);
    let z_t = (coords.2 - z_min) / (z_max - z_min);
    let x_point = interpolate_tuple_3d(
        axes.subobjects[0].points[0],
        axes.subobjects[0].points[axes.subobjects[0].points.len() - 1],
        x_t
    );
    let y_point = interpolate_tuple_3d(
        axes.subobjects[1].points[0],
        axes.subobjects[1].points[axes.subobjects[1].points.len() - 1],
        y_t
    );
    let z_point = interpolate_tuple_3d(
        axes.subobjects[2].points[0],
        axes.subobjects[2].points[axes.subobjects[2].points.len() - 1],
        z_t
    );
    return (x_point.0, y_point.1, z_point.1);
}


pub fn point_to_coords_3d(
    axes: &ThreeDObject,
    point: (f64, f64, f64),
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
) -> (f64, f64, f64) {
    let x_t = (point.0 - axes.subobjects[0].points[0].0) / (axes.subobjects[0].points[axes.subobjects[0].points.len() - 1].0 - axes.subobjects[0].points[0].0);
    let y_t = (point.1 - axes.subobjects[1].points[0].1) / (axes.subobjects[1].points[axes.subobjects[1].points.len() - 1].1 - axes.subobjects[1].points[0].1);
    let z_t = (point.2 - axes.subobjects[2].points[0].1) / (axes.subobjects[2].points[axes.subobjects[2].points.len() - 1].1 - axes.subobjects[2].points[0].1);
    let x = interpolate(x_min, x_max, x_t);
    let y = interpolate(y_min, y_max, y_t);
    let z = interpolate(z_min, z_max, z_t);
    return (x, y, z);
}


pub fn parametric_plot_in_axes_3d(
    axes: &ThreeDObject,
    f: &dyn Fn(f64, f64) -> (f64, f64, f64),
    u_min: f64,
    u_max: f64,
    v_min: f64,
    v_max: f64,
    u_steps: usize,
    v_steps: usize,
    fills: Vec<Color>,
    strokes: Vec<Color>,
    stroke_width: f64
) -> ThreeDObject {
    let new_f = Box::new(|u, v| {
        let coords = f(u, v);
        return coords_to_point_3d(axes, coords, u_min, u_max, v_min, v_max, -1.0, 1.0);
    }) as Box<dyn Fn(f64, f64) -> (f64, f64, f64)>;
    ThreeDObject::from_uv_function(
        Box::leak(new_f),
        (u_min, u_max),
        (v_min, v_max),
        u_steps,
        v_steps,
        fills,
        strokes,
        stroke_width
    )
}


pub fn plot_in_axes_3d(
    axes: &ThreeDObject,
    f: &dyn Fn(f64, f64) -> f64,
    u_min: f64,
    u_max: f64,
    v_min: f64,
    v_max: f64,
    u_steps: usize,
    v_steps: usize,
    fills: Vec<Color>,
    strokes: Vec<Color>,
    stroke_width: f64
) -> ThreeDObject {
    let new_f = Box::new(|u, v| {
        return (u, v, f(u, v));
    }) as Box<dyn Fn(f64, f64) -> (f64, f64, f64)>;
    parametric_plot_in_axes_3d(
        axes,
        Box::leak(new_f),
        u_min,
        u_max,
        v_min,
        v_max,
        u_steps,
        v_steps,
        fills,
        strokes,
        stroke_width
    )
}


pub fn parametric_line_plot_in_axes_3d(
    axes: &ThreeDObject,
    f: &dyn Fn(f64) -> (f64, f64, f64),
    t_min: f64,
    t_max: f64,
    t_steps: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
    color: Color,
    stroke_width: f64
) -> ThreeDObject {
    let vertices_coords = (0..t_steps + 1).map(|i| {
        let t = interpolate(t_min, t_max, i as f64 / t_steps as f64);
        f(t)
    }).collect::<Vec<(f64, f64, f64)>>();
    let mut points = Vec::new();
    for i in 0..vertices_coords.len() - 1 {
        let p1 = vertices_coords[i];
        let p2 = vertices_coords[i + 1];
        points.extend(line_as_cubic_bezier_3d(
            coords_to_point_3d(
                axes,
                p1,
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max
            ),
            coords_to_point_3d(
                axes,
                p2,
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max
            ),
        ));
    }
    ThreeDObject {
        subobjects: Vec::new(),
        points,
        fill: GradientImageOrColor::Color(color.clone()),
        stroke: GradientImageOrColor::Color(color),
        stroke_width,
    }
}
