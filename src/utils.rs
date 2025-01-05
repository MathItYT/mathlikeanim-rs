//! # Utility functions
//! 
//! Provides a set of utility functions for working with vector objects.
//! 
//! The last functions are easing functions that can be used to interpolate differently between two values.

use std::{f64::consts::PI, vec};

use crate::{colors::{Color, GradientImageOrColor}, objects::{geometry::arc::elliptical_arc, three_d::three_d_object::{generate_cubic_bezier_tuples_3d, generate_subpaths_3d, line_as_cubic_bezier_3d, partial_bezier_points_3d, ThreeDObject}, vector_object::{generate_cubic_bezier_tuples, generate_subpaths, partial_bezier_points, VectorObject}}};
use wasm_bindgen::prelude::*;

/// Log utilities for console when using WebAssembly
#[wasm_bindgen]
extern "C" {
    /// Log an `&str` to the console
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(err: JsError);
}


/// An asynchronous sleep function for WebAssembly
pub async fn sleep(delay: i32) {
    let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| -> () {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, delay).unwrap();};

    let p = js_sys::Promise::new(&mut cb);

    wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}


pub fn radian(ux: f64, uy: f64, vx: f64, vy: f64) -> f64 {
    let dot = ux * vx + uy * vy;
    let u_magnitude = (ux * ux + uy * uy).sqrt();
    let v_magnitude = (vx * vx + vy * vy).sqrt();
    let value = dot / (u_magnitude * v_magnitude);
    let mut angle = value.clamp(-1.0, 1.0).acos();
    if (ux * vy - uy * vx).is_sign_negative() {
        angle = -angle;
    }
    return angle;
}


pub fn elliptical_arc_path(
    last_move: (f64, f64),
    rx: f64,
    ry: f64,
    rotation: f64,
    large_arc: bool,
    sweep: bool,
    x: f64,
    y: f64
) -> Vec<(f64, f64)> {
    if last_move == (x, y) {
        return vec![];
    }
    if rx == 0.0 || ry == 0.0 {
        return line_as_cubic_bezier(last_move, (x, y));
    }
    let rotation = rotation * PI / 180.0;
    let mut rx = rx.abs();
    let mut ry = ry.abs();
    let (x1, y1) = last_move;
    let (x2, y2) = (x, y);
    let x1_ = (x1 - x2) / 2.0 * rotation.cos() + (y1 - y2) / 2.0 * rotation.sin();
    let y1_ = -(x1 - x2) / 2.0 * rotation.sin() + (y1 - y2) / 2.0 * rotation.cos();
    let lambda = (x1_ * x1_) / (rx * rx) + (y1_ * y1_) / (ry * ry);
    if lambda > 1.0 {
        rx *= lambda.sqrt();
        ry *= lambda.sqrt();
    }
    let mut t = ((rx * rx * ry * ry - rx * rx * y1_ * y1_ - ry * ry * x1_ * x1_) / (rx * rx * y1_ * y1_ + ry * ry * x1_ * x1_)).max(0.0).sqrt();
    t = if large_arc == sweep { -t } else { t };
    let cx_ = t * rx * y1_ / ry;
    let cy_ = -t * ry * x1_ / rx;
    let cx = cx_ * rotation.cos() - cy_ * rotation.sin() + (x1 + x2) / 2.0;
    let cy = cx_ * rotation.sin() + cy_ * rotation.cos() + (y1 + y2) / 2.0;
    let theta1 = radian(1.0, 0.0, (x1_ - cx_) / rx, (y1_ - cy_) / ry);
    let mut delta_theta = radian((x1_ - cx_) / rx, (y1_ - cy_) / ry, (-x1_ - cx_) / rx, (-y1_ - cy_) / ry).rem_euclid(2.0 * PI);
    if !sweep {
        delta_theta = delta_theta - 2.0 * PI;
    }
    return elliptical_arc(
        (0.0, 0.0),
        rx,
        ry,
        theta1,
        theta1 + delta_theta,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).rotate(rotation, false).shift((cx, cy), false).points;
}


/// A function that returns `n` factorial
pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    return (1..n + 1).product();
}


/// A function that returns a `(f64, f64, f64, f64)` tuple representing the color from a hex string.
pub fn hex_to_color(hex: &str, a: f64) -> Color {
    let hex = hex.trim_start_matches("#");
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap() as f64 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap() as f64 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap() as f64 / 255.0;
    return Color {
        red: r,
        green: g,
        blue: b,
        alpha: a,
    };
}


/// A function that returns a `(f64, f64)` tuple representing the point on a Bezier curve at `t`.
/// 
/// ## Example
/// 
/// ```
/// let points = vec![(0.0, 0.0), (1920.0, 540.0), (960.0, 810.0), (0.0, 1080.0)]; // Points for a cubic Bezier curve
/// let t = 0.5;
/// let point = bezier(&points, t);
/// ```
pub fn bezier(points: &Vec<(f64, f64)>, t: f64) -> (f64, f64) {
    let n = points.len() - 1;
    if n == 0 {
        return points[0];
    }
    if n == 1 {
        let p1 = points[0];
        let p2 = points[1];
        let x = (1.0 - t) * p1.0 + t * p2.0;
        let y = (1.0 - t) * p1.1 + t * p2.1;
        return (x, y);
    }
    if n == 2 {
        let p1 = points[0];
        let p2 = points[1];
        let p3 = points[2];
        let x = (1.0 - t) * (1.0 - t) * p1.0 + 2.0 * (1.0 - t) * t * p2.0 + t * t * p3.0;
        let y = (1.0 - t) * (1.0 - t) * p1.1 + 2.0 * (1.0 - t) * t * p2.1 + t * t * p3.1;
        return (x, y);
    }
    if n == 3 {
        let p1 = points[0];
        let p2 = points[1];
        let p3 = points[2];
        let p4 = points[3];
        let x = (1.0 - t) * (1.0 - t) * (1.0 - t) * p1.0 + 3.0 * (1.0 - t) * (1.0 - t) * t * p2.0 + 3.0 * (1.0 - t) * t * t * p3.0 + t * t * t * p4.0;
        let y = (1.0 - t) * (1.0 - t) * (1.0 - t) * p1.1 + 3.0 * (1.0 - t) * (1.0 - t) * t * p2.1 + 3.0 * (1.0 - t) * t * t * p3.1 + t * t * t * p4.1;
        return (x, y);
    }
    let mut x = 0.0;
    let mut y = 0.0;
    for i in 0..n + 1 {
        let p = points[i];
        let b = choose(n as u64, i as u64) as f64 * (1.0 - t).powi((n - i) as i32) * t.powi(i as i32);
        x += b * p.0;
        y += b * p.1;
    }
    return (x, y);
}


pub fn bezier_3d(points: &Vec<(f64, f64, f64)>, t: f64) -> (f64, f64, f64) {
    let n = points.len() - 1;
    if n == 0 {
        return points[0];
    }
    if n == 1 {
        let p1 = points[0];
        let p2 = points[1];
        let x = (1.0 - t) * p1.0 + t * p2.0;
        let y = (1.0 - t) * p1.1 + t * p2.1;
        let z = (1.0 - t) * p1.2 + t * p2.2;
        return (x, y, z);
    }
    if n == 2 {
        let p1 = points[0];
        let p2 = points[1];
        let p3 = points[2];
        let x = (1.0 - t) * (1.0 - t) * p1.0 + 2.0 * (1.0 - t) * t * p2.0 + t * t * p3.0;
        let y = (1.0 - t) * (1.0 - t) * p1.1 + 2.0 * (1.0 - t) * t * p2.1 + t * t * p3.1;
        let z = (1.0 - t) * (1.0 - t) * p1.2 + 2.0 * (1.0 - t) * t * p2.2 + t * t * p3.2;
        return (x, y, z);
    }
    if n == 3 {
        let p1 = points[0];
        let p2 = points[1];
        let p3 = points[2];
        let p4 = points[3];
        let x = (1.0 - t) * (1.0 - t) * (1.0 - t) * p1.0 + 3.0 * (1.0 - t) * (1.0 - t) * t * p2.0 + 3.0 * (1.0 - t) * t * t * p3.0 + t * t * t * p4.0;
        let y = (1.0 - t) * (1.0 - t) * (1.0 - t) * p1.1 + 3.0 * (1.0 - t) * (1.0 - t) * t * p2.1 + 3.0 * (1.0 - t) * t * t * p3.1 + t * t * t * p4.1;
        let z = (1.0 - t) * (1.0 - t) * (1.0 - t) * p1.2 + 3.0 * (1.0 - t) * (1.0 - t) * t * p2.2 + 3.0 * (1.0 - t) * t * t * p3.2 + t * t * t * p4.2;
        return (x, y, z);
    }
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    for i in 0..n + 1 {
        let p = points[i];
        let b = choose(n as u64, i as u64) as f64 * (1.0 - t).powi((n - i) as i32) * t.powi(i as i32);
        x += b * p.0;
        y += b * p.1;
        z += b * p.2;
    }
    return (x, y, z);
}


/// A function that returns a `f64` representing a one-dimensional Bezier polynomial at `t`.
pub fn bezier_f64(numbers: Vec<f64>, t: f64) -> f64 {
    let n = numbers.len() - 1;
    if n == 0 {
        return numbers[0];
    }
    if n == 1 {
        return (1.0 - t) * numbers[0] + t * numbers[1];
    }
    if n == 2 {
        return (1.0 - t) * (1.0 - t) * numbers[0] + 2.0 * (1.0 - t) * t * numbers[1] + t * t * numbers[2];
    }
    if n == 3 {
        return (1.0 - t) * (1.0 - t) * (1.0 - t) * numbers[0] + 3.0 * (1.0 - t) * (1.0 - t) * t * numbers[1] + 3.0 * (1.0 - t) * t * t * numbers[2] + t * t * t * numbers[3];
    }
    let mut result = 0.0;
    for i in 0..n + 1 {
        let b = choose(n as u64, i as u64) as f64 * (1.0 - t).powi((n - i) as i32) * t.powi(i as i32);
        result += b * numbers[i];
    }
    return result;
}


/// A function that returns the permutation of `n` objects taken `r`.
pub fn permutation(n: u64, r: u64) -> u64 {
    if n < r {
        return 0;
    }
    return (n - r + 1..n + 1).product();
}


/// A function that returns the combination of `n` objects taken `r`.
pub fn choose(n: u64, r: u64) -> u64 {
    if n < r {
        return 0;
    }
    return permutation(n, r) / factorial(r);
}


pub fn interp(x: f64, xp: &Vec<f64>, fp: &Vec<f64>) -> f64 {
    // Check length
    if xp.len() != fp.len() {
        error(JsError::new("The length of `xp` and `fp` must be the same."));
        return 0.0;
    }
    if x <= xp[0] {
        return fp[0];
    }
    if x >= xp[xp.len() - 1] {
        return fp[fp.len() - 1];
    }
    for i in 0..xp.len() - 1 {
        if x >= xp[i] && x <= xp[i + 1] {
            let t = (x - xp[i]) / (xp[i + 1] - xp[i]);
            return (1.0 - t) * fp[i] + t * fp[i + 1];
        }
    }
    return 0.0;
}


/// A distance function that corresponds to the Euclidean distance squared, just for better performance.
pub fn distance_squared(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    return dx * dx + dy * dy; // for faster computation, we don't use sqrt
}


/// A function that returns the interpolation of two `f64` values at `t`.
pub fn interpolate(x: f64, y: f64, t: f64) -> f64 {
    return (1.0 - t) * x + t * y;
}


/// A function that returns the interpolation of two `(f64, f64)` tuples at `t`.
pub fn interpolate_tuple(p1: (f64, f64), p2: (f64, f64), t: f64) -> (f64, f64) {
    return (interpolate(p1.0, p2.0, t), interpolate(p1.1, p2.1, t));
}


/// A function that returns the interpolation of two `(f64, f64, f64)` tuples at `t`.
pub fn interpolate_tuple_3d(p1: (f64, f64, f64), p2: (f64, f64, f64), t: f64) -> (f64, f64, f64) {
    return (
        interpolate(p1.0, p2.0, t),
        interpolate(p1.1, p2.1, t),
        interpolate(p1.2, p2.2, t)
    );
}


/// A function that returns the interpolation of two `(f64, f64, f64, f64)` (RGBA) tuples at `t`.
pub fn interpolate_color(color1: (f64, f64, f64, f64), color2: (f64, f64, f64, f64), t: f64) -> (f64, f64, f64, f64) {
    let (r1, g1, b1, a1) = color1;
    let (r2, g2, b2, a2) = color2;
    return (
        interpolate(r1, r2, t),
        interpolate(g1, g2, t),
        interpolate(b1, b2, t),
        interpolate(a1, a2, t)
    );
}


pub fn points_from_anchors_and_handles(
    anchors1: Vec<(f64, f64)>,
    handles1: Vec<(f64, f64)>,
    handles2: Vec<(f64, f64)>,
    anchors2: Vec<(f64, f64)>
) -> Vec<(f64, f64)> {
    let n = anchors1.len() * 4;
    let mut points = Vec::new();
    for _ in 0..n {
        points.push((0.0, 0.0));
    }
    let arrays = vec![anchors1, handles1, handles2, anchors2];
    for (i, array) in arrays.iter().enumerate() {
        for (j, point) in array.iter().enumerate() {
            points[i + j * 4] = *point;
        }
    }
    return points;
}


pub fn get_start_anchors(points: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let mut start_anchors = Vec::new();
    for i in (0..points.len()).step_by(4) {
        start_anchors.push(points[i]);
    }
    return start_anchors;
}


pub fn start_new_path(points: &Vec<(f64, f64)>, point: (f64, f64)) -> Vec<(f64, f64)> {
    let mut points = points.clone();
    if points.len() % 4 != 0 {
        let anchors = get_start_anchors(&points);
        let last_anchor = anchors[anchors.len() - 1];
        for _ in 0..(4 - points.len() % 4) {
            points.push(last_anchor);
        }
    }
    points.push(point);
    return points.clone();
}


pub fn get_start_anchors_3d(points: &Vec<(f64, f64, f64)>) -> Vec<(f64, f64, f64)> {
    let mut start_anchors = Vec::new();
    for i in (0..points.len()).step_by(4) {
        start_anchors.push(points[i]);
    }
    return start_anchors;
}


pub fn start_new_path_3d(points: &Vec<(f64, f64, f64)>, point: (f64, f64, f64)) -> Vec<(f64, f64, f64)> {
    let mut points = points.clone();
    if points.len() % 4 != 0 {
        let anchors = get_start_anchors_3d(&points);
        let last_anchor = anchors[anchors.len() - 1];
        for _ in 0..(4 - points.len() % 4) {
            points.push(last_anchor);
        }
    }
    points.push(point);
    return points.clone();
}


pub fn has_new_path_begun(points: &Vec<(f64, f64)>) -> bool {
    return points.len() % 4 == 1;
}


pub fn get_nth_subpath(subpaths: &Vec<Vec<(f64, f64)>>, n: usize) -> Vec<(f64, f64)> {
    if n >= subpaths.len() {
        let point = subpaths[subpaths.len() - 1][subpaths[subpaths.len() - 1].len() - 1];
        return vec![point; 4];
    }
    let mut path = subpaths[n].clone();
    while path.len() > 4 {
        let mut path_equals = true;
        for i in 0..4 {
            if !consider_points_equals(path[path.len() - 4 + i], path[path.len() - 5]) {
                path_equals = false;
                break;
            }
        }
        if path_equals {
            path = path[..path.len() - 4].to_vec();
        } else {
            break;
        }
    }
    return path;
}


pub fn get_nth_subpath_3d(subpaths: &Vec<Vec<(f64, f64, f64)>>, n: usize) -> Vec<(f64, f64, f64)> {
    if n >= subpaths.len() {
        let point = subpaths[subpaths.len() - 1][subpaths[subpaths.len() - 1].len() - 1];
        return vec![point; 4];
    }
    let mut path = subpaths[n].clone();
    while path.len() > 4 {
        let mut path_equals = true;
        for i in 0..4 {
            if !consider_points_equals_3d(path[path.len() - 4 + i], path[path.len() - 5]) {
                path_equals = false;
                break;
            }
        }
        if path_equals {
            path = path[..path.len() - 4].to_vec();
        } else {
            break;
        }
    }
    return path;
}


pub fn insert_n_curves_to_point_list_3d(
    n: usize,
    points: &Vec<(f64, f64, f64)>
) -> Vec<(f64, f64, f64)> {
    if points.len() == 1 {
        return vec![points[0]; 4 * (n + 1)];
    }
    let bezier_quads = generate_cubic_bezier_tuples_3d(&points);
    let target_num = bezier_quads.len() + n;
    let mut repeat_indices = Vec::new();
    for i in 0..target_num {
        let index = i * bezier_quads.len() / target_num;
        repeat_indices.push(index);
    }
    let mut split_factors = Vec::new();
    for _ in 0..bezier_quads.len() {
        let split_factor = 0;
        split_factors.push(split_factor);
    }
    for val in repeat_indices {
        split_factors[val] += 1;
    }
    let mut new_points = Vec::new();
    for (bezier_quad, split_factor) in bezier_quads.iter().zip(split_factors) {
        let mut alphas = Vec::new();
        for i in 0..split_factor + 1 {
            let alpha = i as f64 / (split_factor as f64);
            alphas.push(alpha);
        }
        for (a1, a2) in alphas.iter().zip(alphas.iter().skip(1)) {
            new_points.extend(partial_bezier_points_3d(&vec![bezier_quad.0, bezier_quad.1, bezier_quad.2, bezier_quad.3], *a1, *a2));
        }
    }
    return new_points;
}


pub fn insert_n_curves_to_point_list(
    n: usize,
    points: &Vec<(f64, f64)>
) -> Vec<(f64, f64)> {
    if points.len() == 1 {
        return vec![points[0]; 4 * (n + 1)];
    }
    let bezier_quads = generate_cubic_bezier_tuples(&points);
    let target_num = bezier_quads.len() + n;
    let mut repeat_indices = Vec::new();
    for i in 0..target_num {
        let index = i * bezier_quads.len() / target_num;
        repeat_indices.push(index);
    }
    let mut split_factors = Vec::new();
    for _ in 0..bezier_quads.len() {
        let split_factor = 0;
        split_factors.push(split_factor);
    }
    for val in repeat_indices {
        split_factors[val] += 1;
    }
    let mut new_points = Vec::new();
    for (bezier_quad, split_factor) in bezier_quads.iter().zip(split_factors) {
        let mut alphas = Vec::new();
        for i in 0..split_factor + 1 {
            let alpha = i as f64 / (split_factor as f64);
            alphas.push(alpha);
        }
        for (a1, a2) in alphas.iter().zip(alphas.iter().skip(1)) {
            new_points.extend(partial_bezier_points(&vec![bezier_quad.0, bezier_quad.1, bezier_quad.2, bezier_quad.3], *a1, *a2));
        }
    }
    return new_points;
}


pub fn null_point_align_3d(
    obj1: ThreeDObject,
    obj2: ThreeDObject
) -> (ThreeDObject, ThreeDObject) {
    let mut obj1 = obj1;
    let mut obj2 = obj2;
    if obj1.points.len() == 0 && obj2.points.len() > 0 {
        obj2.subobjects.push(obj2.set_subobjects(vec![]));
        obj2.points.clear();
    }
    if obj2.points.len() == 0 && obj1.points.len() > 0 {
        obj1.subobjects.push(obj1.set_subobjects(vec![]));
        obj1.points.clear();
    }
    return (obj1, obj2);
}


pub fn null_point_align(
    vec_obj1: VectorObject,
    vec_obj2: VectorObject
) -> (VectorObject, VectorObject) {
    let mut vec_obj1 = vec_obj1;
    let mut vec_obj2 = vec_obj2;
    if vec_obj1.points.len() == 0 && vec_obj2.points.len() > 0 {
        vec_obj2.subobjects.push(vec_obj2.set_subobjects(vec![]));
        vec_obj2.points.clear();
    }
    if vec_obj2.points.len() == 0 && vec_obj1.points.len() > 0 {
        vec_obj1.subobjects.push(vec_obj1.set_subobjects(vec![]));
        vec_obj1.points.clear();
    }
    return (vec_obj1, vec_obj2);
}


pub fn get_bbox(points: &Vec<(f64, f64)>) -> ((f64, f64), (f64, f64)) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for point in points {
        min_x = min_x.min(point.0);
        min_y = min_y.min(point.1);
        max_x = max_x.max(point.0);
        max_y = max_y.max(point.1);
    }
    return ((min_x, min_y), (max_x, max_y));
}


pub fn center(points: &Vec<(f64, f64)>, center_if_no_points: (f64, f64)) -> (f64, f64) {
    if points.len() == 0 {
        return center_if_no_points;
    }
    let ((min_x, min_y), (max_x, max_y)) = get_bbox(points);
    return ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);
}


pub fn center_3d(points: &Vec<(f64, f64, f64)>, center_if_no_points: (f64, f64, f64)) -> (f64, f64, f64) {
    if points.len() == 0 {
        return center_if_no_points;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut min_z = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut max_z = f64::NEG_INFINITY;
    for point in points {
        min_x = min_x.min(point.0);
        min_y = min_y.min(point.1);
        min_z = min_z.min(point.2);
        max_x = max_x.max(point.0);
        max_y = max_y.max(point.1);
        max_z = max_z.max(point.2);
    }
    return ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0, (min_z + max_z) / 2.0);
}


pub fn align_points(points1: &Vec<(f64, f64)>, points2: &Vec<(f64, f64)>, center_if_no_points: (f64, f64)) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let mut points1 = points1.clone();
    let mut points2 = points2.clone();
    if points1.len() == 0 {
        let c = center(&points1, center_if_no_points);
        points1 = start_new_path(&points1, c);
    }
    if has_new_path_begun(&points1) {
        points1.extend(line_as_cubic_bezier(points1[points1.len() - 1], points1[points1.len() - 1]));
    }
    if points2.len() == 0 {
        let c = center(&points2, center_if_no_points);
        points2 = start_new_path(&points2, c);
    }
    if has_new_path_begun(&points2) {
        points2.extend(line_as_cubic_bezier(points2[points2.len() - 1], points2[points2.len() - 1]));
    }
    let subpaths1 = generate_subpaths(&points1);
    let subpaths2 = generate_subpaths(&points2);
    let n_subpaths = subpaths1.len().max(subpaths2.len());
    let mut new_points1 = Vec::new();
    let mut new_points2 = Vec::new();

    for i in 0..n_subpaths {
        let path1 = get_nth_subpath(&subpaths1, i);
        let path2 = get_nth_subpath(&subpaths2, i);
        let diff1 = ((path2.len() as i32 - path1.len() as i32) / 4).max(0) as usize;
        let diff2 = ((path1.len() as i32 - path2.len() as i32) / 4).max(0) as usize;
        let path1 = insert_n_curves_to_point_list(diff1, &path1);
        let path2 = insert_n_curves_to_point_list(diff2, &path2);
        new_points1.extend(path1);
        new_points2.extend(path2);
    }
    return (new_points1, new_points2);
}


pub fn has_new_path_begun_3d(points: &Vec<(f64, f64, f64)>) -> bool {
    return points.len() % 4 == 1;
}


pub fn align_points_3d(points1: &Vec<(f64, f64, f64)>, points2: &Vec<(f64, f64, f64)>, center_if_no_points: (f64, f64, f64)) -> (Vec<(f64, f64, f64)>, Vec<(f64, f64, f64)>) {
    let mut points1 = points1.clone();
    let mut points2 = points2.clone();
    if points1.len() == 0 {
        let c = center_3d(&points1, center_if_no_points);
        points1 = start_new_path_3d(&points1, c);
    }
    if has_new_path_begun_3d(&points1) {
        points1.extend(line_as_cubic_bezier_3d(points1[points1.len() - 1], points1[points1.len() - 1]));
    }
    if points2.len() == 0 {
        let c = center_3d(&points2, center_if_no_points);
        points2 = start_new_path_3d(&points2, c);
    }
    if has_new_path_begun_3d(&points2) {
        points2.extend(line_as_cubic_bezier_3d(points2[points2.len() - 1], points2[points2.len() - 1]));
    }
    let subpaths1 = generate_subpaths_3d(&points1);
    let subpaths2 = generate_subpaths_3d(&points2);
    let n_subpaths = subpaths1.len().max(subpaths2.len());
    let mut new_points1 = Vec::new();
    let mut new_points2 = Vec::new();

    for i in 0..n_subpaths {
        let path1 = get_nth_subpath_3d(&subpaths1, i);
        let path2 = get_nth_subpath_3d(&subpaths2, i);
        let diff1 = ((path2.len() as i32 - path1.len() as i32) / 4).max(0) as usize;
        let diff2 = ((path1.len() as i32 - path2.len() as i32) / 4).max(0) as usize;
        let path1 = insert_n_curves_to_point_list_3d(diff1, &path1);
        let path2 = insert_n_curves_to_point_list_3d(diff2, &path2);
        new_points1.extend(path1);
        new_points2.extend(path2);
    }
    return (new_points1, new_points2);
}


pub fn add_n_more_subobjects(
    vec_obj: VectorObject,
    n: usize,
    center_if_no_points: (f64, f64)
) -> VectorObject {
    let subobjects = vec_obj.subobjects.clone();
    if n == 0 {
        return vec_obj;
    }
    if subobjects.len() == 0 {
        let subobjects = vec![VectorObject {
            points: vec![center(&vec_obj.points, center_if_no_points)],
            subobjects: vec![],
            index: 0,
            fill_rule: "nonzero",
            fill: GradientImageOrColor::Color(Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
            }),
            stroke: GradientImageOrColor::Color(Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
            }),
            stroke_width: 0.0,
            line_cap: "butt",
            line_join: "miter",
        }; n];
        return vec_obj.set_subobjects(subobjects);
    }
    let target = subobjects.len() + n;
    let mut repeat_indices = Vec::new();
    for i in 0..target {
        let index = i * subobjects.len() / target;
        repeat_indices.push(index);
    }
    let mut split_factors = Vec::new();
    for _ in 0..subobjects.len() {
        let split_factor = 0;
        split_factors.push(split_factor);
    }
    for val in repeat_indices {
        split_factors[val] += 1;
    }
    let mut new_subobjects = Vec::new();
    for (subobject, split_factor) in subobjects.iter().zip(split_factors) {
        new_subobjects.push(subobject.clone());
        for _ in 1..split_factor {
            new_subobjects.push(subobject
                .set_fill_opacity(0.0, true)
                .set_stroke_opacity(0.0, true));
        }
    }
    return vec_obj.set_subobjects(new_subobjects);
}


pub fn add_n_more_subobjects_3d(
    obj: ThreeDObject,
    n: usize,
    center_if_no_points: (f64, f64, f64)
) -> ThreeDObject {
    let subobjects = obj.subobjects.clone();
    if n == 0 {
        return obj;
    }
    if subobjects.len() == 0 {
        let subobjects = vec![ThreeDObject {
            points: vec![center_3d(&obj.points, center_if_no_points)],
            subobjects: vec![],
            index: 0,
            fill: GradientImageOrColor::Color(Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
            }),
            stroke: GradientImageOrColor::Color(Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
            }),
            stroke_width: 0.0,
        }; n];
        return obj.set_subobjects(subobjects);
    }
    let target = subobjects.len() + n;
    let mut repeat_indices = Vec::new();
    for i in 0..target {
        let index = i * subobjects.len() / target;
        repeat_indices.push(index);
    }
    let mut split_factors = Vec::new();
    for _ in 0..subobjects.len() {
        let split_factor = 0;
        split_factors.push(split_factor);
    }
    for val in repeat_indices {
        split_factors[val] += 1;
    }
    let mut new_subobjects = Vec::new();
    for (subobject, split_factor) in subobjects.iter().zip(split_factors) {
        new_subobjects.push(subobject.clone());
        for _ in 1..split_factor {
            new_subobjects.push(subobject
                .set_fill_opacity(0.0, true)
                .set_stroke_opacity(0.0, true));
        }
    }
    return obj.set_subobjects(new_subobjects);
}


pub fn align_subobjects(
    vec_obj1: VectorObject,
    vec_obj2: VectorObject,
    center_if_no_points: (f64, f64)
) -> (VectorObject, VectorObject) {
    return (
        add_n_more_subobjects(vec_obj1.clone(), (vec_obj2.subobjects.len() as i32 - vec_obj1.subobjects.len() as i32).max(0) as usize, center_if_no_points),
        add_n_more_subobjects(vec_obj2.clone(), (vec_obj1.subobjects.len() as i32 - vec_obj2.subobjects.len() as i32).max(0) as usize, center_if_no_points)
    );
}


pub fn align_subobjects_3d(
    obj1: ThreeDObject,
    obj2: ThreeDObject,
    center_if_no_points: (f64, f64, f64)
) -> (ThreeDObject, ThreeDObject) {
    return (
        add_n_more_subobjects_3d(obj1.clone(), (obj2.subobjects.len() as i32 - obj1.subobjects.len() as i32).max(0) as usize, center_if_no_points),
        add_n_more_subobjects_3d(obj2.clone(), (obj1.subobjects.len() as i32 - obj2.subobjects.len() as i32).max(0) as usize, center_if_no_points)
    );
}


pub fn align_data(
    vec_obj1: VectorObject,
    vec_obj2: VectorObject,
    skip_point_align: bool,
    center_if_no_points: (f64, f64)
) -> (VectorObject, VectorObject) {
    let (
        mut vec_obj1,
        mut vec_obj2
    ) = null_point_align(vec_obj1, vec_obj2);
    (vec_obj1, vec_obj2) = align_subobjects(vec_obj1, vec_obj2, center_if_no_points);
    if !skip_point_align {
        let (new_points1, new_points2) = align_points(&vec_obj1.points, &vec_obj2.points, center_if_no_points);
        vec_obj1.points = new_points1;
        vec_obj2.points = new_points2;
    }
    for i in 0..vec_obj1.subobjects.len() {
        let (subobject1, subobject2) = align_data(vec_obj1.subobjects[i].clone(), vec_obj2.subobjects[i].clone(), false, center_if_no_points);
        vec_obj1.subobjects[i] = subobject1;
        vec_obj2.subobjects[i] = subobject2;
    }
    return (vec_obj1, vec_obj2);
}


pub fn align_data_3d(
    obj1: ThreeDObject,
    obj2: ThreeDObject,
    skip_point_align: bool,
    center_if_no_points: (f64, f64, f64)
) -> (ThreeDObject, ThreeDObject) {
    let (
        mut obj1,
        mut obj2
    ) = null_point_align_3d(obj1, obj2);
    (obj1, obj2) = align_subobjects_3d(obj1, obj2, center_if_no_points);
    if !skip_point_align {
        let (new_points1, new_points2) = align_points_3d(&obj1.points, &obj2.points, center_if_no_points);
        obj1.points = new_points1;
        obj2.points = new_points2;
    }
    for i in 0..obj1.subobjects.len() {
        let (subobject1, subobject2) = align_data_3d(obj1.subobjects[i].clone(), obj2.subobjects[i].clone(), false, center_if_no_points);
        obj1.subobjects[i] = subobject1;
        obj2.subobjects[i] = subobject2;
    }
    return (obj1, obj2);
}


/// Returns a pair `(i64, f64)` representing the integer interpolation of two `f64` values at `t`.
pub fn integer_interpolate(x: f64, y: f64, t: f64) -> (i64, f64) {
    if t >= 1.0 {
        return (((y - 1.0) as f64).floor() as i64, 1.0)
    }
    if t <= 0.0 {
        return ((x as f64).floor() as i64, 0.0)
    }
    let value = interpolate(x as f64, y as f64, t).floor();
    let residue = ((y - x) * t) % 1.0;
    return (value as i64, residue);
}


/// Returns the points that represent a line as a cubic Bezier curve.
pub fn line_as_cubic_bezier(p1: (f64, f64), p2: (f64, f64)) -> Vec<(f64, f64)> {
    let mut result = Vec::new();
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    result.push((x1, y1));
    result.push(((2.0 * x1 + x2) / 3.0, (2.0 * y1 + y2) / 3.0));
    result.push(((x1 + 2.0 * x2) / 3.0, (y1 + 2.0 * y2) / 3.0));
    result.push((x2, y2));
    return result;
}


/// Returns the points that represent a quadratic Bezier curve as a cubic Bezier curve.
pub fn quadratic_bezier_as_cubic_bezier(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> Vec<(f64, f64)> {
    let mut result = Vec::new();
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;
    result.push((x1, y1));
    result.push(((x1 + 2.0 * x2) / 3.0, (y1 + 2.0 * y2) / 3.0));
    result.push(((2.0 * x2 + x3) / 3.0, (2.0 * y2 + y3) / 3.0));
    result.push((x3, y3));
    return result;
}


/// Checks if two points `(f64, f64)` are equal given a tolerance.
pub fn consider_points_equals(p1: (f64, f64), p2: (f64, f64)) -> bool {
    return distance_squared(p1, p2) < 0.001;
}


pub fn distance_squared_3d(p1: (f64, f64, f64), p2: (f64, f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    let dz = p1.2 - p2.2;
    return dx * dx + dy * dy + dz * dz; // for faster computation, we don't use sqrt
}


pub fn consider_points_equals_3d(p1: (f64, f64, f64), p2: (f64, f64, f64)) -> bool {
    return distance_squared_3d(p1, p2) < 0.001;
}


pub fn sigmoid(t: f64) -> f64 {
    return 1.0 / (1.0 + (-t).exp());
}


pub fn linear(t: f64) -> f64 {
    return t;
}


pub fn smooth(t: f64, inflection: f64) -> f64 {
    if t == 1.0 {
        return 1.0;
    }
    let error = sigmoid(-inflection / 2.0);
    return (sigmoid(inflection * (t - 0.5)) - error) / (1.0 - 2.0 * error).max(0.0).min(1.0);
}


pub fn smoothstep(t: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }
    return t * t * (3.0 - 2.0 * t);
}


pub fn smootherstep(t: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}


pub fn smoothererstep(t: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }
    return t * t * t * t * (t * (t * (t * 35.0 - 84.0) + 70.0) - 20.0);
}


pub fn rush_into(t: f64, inflection: f64) -> f64 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    return 2.0 * smooth(t / 2.0, inflection);
}


pub fn rush_from(t: f64, inflection: f64) -> f64 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    return 2.0 * smooth(t / 2.0 + 0.5, inflection) - 1.0;
}


pub fn slow_into(t: f64) -> f64 {
    return (1.0 - (1.0 - t) * (1.0 - t)).sqrt();
}


pub fn double_smooth(t: f64) -> f64 {
    if t < 0.5 {
        return 0.5 * smooth(t * 2.0, 10.0);
    }
    return 0.5 * (1.0 + smooth(t * 2.0 - 1.0, 10.0));
}


pub fn there_and_back(t: f64, inflection: f64) -> f64 {
    let new_t = if t < 0.5 {
        t * 2.0
    } else {
        (1.0 - t) * 2.0
    };
    return smooth(new_t, inflection);
}


pub fn there_and_back_with_pause(t: f64, pause_ratio: f64) -> f64 {
    let a = 1.0 / pause_ratio;
    if t < 0.5 - pause_ratio / 2.0 {
        return smooth(t * a, 10.0);
    }
    if t < 0.5 + pause_ratio / 2.0 {
        return 1.0;
    }
    return smooth((t - pause_ratio) * a, 10.0);
}


pub fn running_start(t: f64, pull_factor: f64) -> f64 {
    return bezier_f64(vec![0.0, 0.0, pull_factor, pull_factor, 1.0, 1.0, 1.0], t);
}


pub fn not_quite_there(
    function: impl Fn(f64) -> f64,
    t: f64,
    proportion: f64
) -> f64 {
    return function(t) * proportion;
}


pub fn wiggle(t: f64, wiggles: f64) -> f64 {
    return (t * PI * wiggles).sin() * there_and_back(t, 10.0);
}


pub fn squish_rate_func(
    function: impl Fn(f64) -> f64,
    t: f64,
    a: f64,
    b: f64
) -> f64 {
    if a == b {
        return a;
    }
    if t < a {
        return function(0.0);
    }
    if t > b {
        return function(1.0);
    }
    return function((t - a) / (b - a));
}


pub fn lingering(t: f64) -> f64 {
    return squish_rate_func(linear, t, 0.0, 0.8);
}


pub fn exponential_decay(t: f64, half_life: f64) -> f64 {
    return 1.0 - 2.0f64.powf(-t / half_life);
}


pub fn ease_in_sine(t: f64) -> f64 {
    return 1.0 - (t * PI / 2.0).cos();
}


pub fn ease_out_sine(t: f64) -> f64 {
    return (t * PI / 2.0).sin();
}


pub fn ease_in_out_sine(t: f64) -> f64 {
    return (-(PI * t).cos() - 1.0) / 2.0;
}


pub fn ease_in_quad(t: f64) -> f64 {
    return t * t;
}


pub fn ease_out_quad(t: f64) -> f64 {
    return 1.0 - (1.0 - t) * (1.0 - t);
}


pub fn ease_in_out_quad(t: f64) -> f64 {
    if t < 0.5 {
        return 2.0 * t * t;
    }
    return 1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0) / 2.0;
}


pub fn ease_in_cubic(t: f64) -> f64 {
    return t * t * t;
}


pub fn ease_out_cubic(t: f64) -> f64 {
    return 1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t);
}


pub fn ease_in_out_cubic(t: f64) -> f64 {
    if t < 0.5 {
        return 4.0 * t * t * t;
    }
    return 1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) / 2.0;
}


pub fn ease_in_quart(t: f64) -> f64 {
    return t * t * t * t;
}


pub fn ease_out_quart(t: f64) -> f64 {
    return 1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t);
}


pub fn ease_in_out_quart(t: f64) -> f64 {
    if t < 0.5 {
        return 8.0 * t * t * t * t;
    }
    return 1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) / 2.0;
}


pub fn ease_in_quint(t: f64) -> f64 {
    return t * t * t * t * t;
}


pub fn ease_out_quint(t: f64) -> f64 {
    return 1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t);
}


pub fn ease_in_out_quint(t: f64) -> f64 {
    if t < 0.5 {
        return 16.0 * t * t * t * t * t;
    }
    return 1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) / 2.0;
}


pub fn ease_in_expo(t: f64) -> f64 {
    if t == 0.0 {
        return 0.0;
    }
    return 2.0f64.powf(10.0 * (t - 1.0));
}


pub fn ease_out_expo(t: f64) -> f64 {
    if t == 1.0 {
        return 1.0;
    }
    return 1.0 - 2.0f64.powf(-10.0 * t);
}


pub fn ease_in_out_expo(t: f64) -> f64 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    if t < 0.5 {
        return 2.0f64.powf(20.0 * t - 10.0) / 2.0;
    }
    return (2.0 - 2.0f64.powf(-20.0 * t + 10.0)) / 2.0;
}


pub fn ease_in_circ(t: f64) -> f64 {
    return 1.0 - (1.0 - t * t).sqrt();
}


pub fn ease_out_circ(t: f64) -> f64 {
    return (1.0 - (1.0 - t) * (1.0 - t)).sqrt();
}


pub fn ease_in_out_circ(t: f64) -> f64 {
    if t < 0.5 {
        return (1.0 - (1.0 - 2.0 * t) * (1.0 - 2.0 * t)).sqrt() / 2.0;
    }
    return (1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0)).sqrt() / 2.0;
}


pub fn ease_in_back(t: f64) -> f64 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    return c3 * t * t * t - c1 * t * t;
}


pub fn ease_out_back(t: f64) -> f64 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    return 1.0 + c3 * (t - 1.0) * (t - 1.0) * (t - 1.0) + c1 * (t - 1.0) * (t - 1.0);
}


pub fn ease_in_out_back(t: f64) -> f64 {
    let c1 = 1.70158;
    let c2 = c1 * 1.525;
    if t < 0.5 {
        return (2.0 * t * t * (c2 + 1.0) * t - c2) / 2.0;
    }
    return (2.0 * (t - 1.0) * (t - 1.0) * (c2 + 1.0) * (t - 1.0) + c2 + 2.0) / 2.0;
}


pub fn ease_in_elastic(t: f64) -> f64 {
    let c4 = (2.0 * PI) / 3.0;
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    return -2.0f64.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin();
}


pub fn ease_out_elastic(t: f64) -> f64 {
    let c4 = (2.0 * PI) / 3.0;
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    return 2.0f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0;
}


pub fn ease_in_out_elastic(t: f64) -> f64 {
    let c5 = (2.0 * PI) / 4.5;
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    if t < 0.5 {
        return -(2.0f64.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0;
    }
    return (2.0f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0 + 1.0;
}


pub fn ease_out_bounce(t: f64) -> f64 {
    if t < 1.0 / 2.75 {
        return 7.5625 * t * t;
    } else if t < 2.0 / 2.75 {
        let t = t - 1.5 / 2.75;
        return 7.5625 * t * t + 0.75;
    } else if t < 2.5 / 2.75 {
        let t = t - 2.25 / 2.75;
        return 7.5625 * t * t + 0.9375;
    } else {
        let t = t - 2.625 / 2.75;
        return 7.5625 * t * t + 0.984375;
    }
}


pub fn ease_in_bounce(t: f64) -> f64 {
    return 1.0 - ease_out_bounce(1.0 - t);
}


pub fn ease_in_out_bounce(t: f64) -> f64 {
    if t < 0.5 {
        return (1.0 - ease_out_bounce(1.0 - 2.0 * t)) / 2.0;
    }
    return (1.0 + ease_out_bounce(2.0 * t - 1.0)) / 2.0;
}
