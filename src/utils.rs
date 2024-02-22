//! # Utility functions
//! 
//! Provides a set of utility functions for working with vector objects.
//! 
//! The last functions are easing functions that can be used to interpolate differently between two values.

use std::{f64::consts::PI, vec};

use crate::objects::vector_object::{generate_cubic_bezier_tuples, generate_subpaths, partial_bezier_points, VectorFeatures, VectorObject};
use wasm_bindgen::prelude::*;

/// Log utilities for console when using WebAssembly
#[wasm_bindgen]
extern "C" {
    /// Log an `&str` to the console
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    /// Log a `u32` to the console
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    /// Log two `&str` to the console
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
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


/// A function that returns `n` factorial
pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    return (1..n + 1).product();
}


/// A function that returns a `(f64, f64, f64, f64)` tuple representing the color from a hex string.
pub fn hex_to_color(hex: &str, a: f64) -> (f64, f64, f64, f64) {
    let hex = hex.trim_start_matches("#");
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap() as f64 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap() as f64 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap() as f64 / 255.0;
    return (r, g, b, a);
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


/// A function that returns a `f64` representing a one-dimensional Bezier polynomial at `t`.
fn bezier_f64(numbers: Vec<f64>, t: f64) -> f64 {
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


pub fn start_new_path(points: &mut Vec<(f64, f64)>, point: (f64, f64)) -> Vec<(f64, f64)> {
    if points.len() % 4 != 0 {
        let anchors = get_start_anchors(points);
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


pub fn insert_n_curves_to_point_list(
    n: usize,
    points: &mut Vec<(f64, f64)>
) -> Vec<(f64, f64)> {
    if points.len() == 1 {
        for _ in 0..4*n {
            points.extend(points.clone());
        }
        return points.clone();
    }
    let bezier_quads = generate_cubic_bezier_tuples(&points.clone());
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
    for (quad, split_factor) in bezier_quads.iter().zip(split_factors) {
        let mut alphas = Vec::new();
        for i in 0..split_factor+1 {
            alphas.push(i as f64 / (split_factor as f64));
        }
        for (a1, a2) in alphas.iter().zip(alphas[1..].iter()) {
            let bezier_points = partial_bezier_points(&vec![quad.0, quad.1, quad.2, quad.3], *a1, *a2);
            new_points.extend(bezier_points);
        }
    }
    return new_points;
}


pub fn null_point_align(
    vec_obj1: VectorFeatures,
    vec_obj2: VectorFeatures
) -> (VectorFeatures, VectorFeatures) {
    let mut new_vec_obj1 = vec_obj1.clone();
    let mut new_vec_obj2 = vec_obj2.clone();
    if vec_obj1.points.len() == 0 && vec_obj2.points.len() > 0 {
        new_vec_obj2.subobjects.push(vec_obj2.clone().set_subobjects(vec![]));
        new_vec_obj2.points = vec![];
    }
    if vec_obj2.points.len() == 0 && vec_obj1.points.len() > 0 {
        new_vec_obj1.subobjects.push(vec_obj1.clone().set_subobjects(vec![]));
        new_vec_obj1.points = vec![];
    }
    return (new_vec_obj1, new_vec_obj2);
}


pub fn align_points(points1: Vec<(f64, f64)>, points2: Vec<(f64, f64)>) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    if points1.len() == points2.len() {
        return (points1.clone(), points2.clone());
    }
    let mut points1 = points1;
    let mut points2 = points2;
    if points1.len() == 0 {
        points1 = start_new_path(&mut points1, (0.0, 0.0));
    }
    if has_new_path_begun(&points1) {
        points1.extend(line_as_cubic_bezier(points1[points1.len() - 1], points1[points1.len() - 1]));
    }
    if points2.len() == 0 {
        points2 = start_new_path(&mut points2, (0.0, 0.0));
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
        let path1 = insert_n_curves_to_point_list(diff1, &mut path1.clone());
        let path2 = insert_n_curves_to_point_list(diff2, &mut path2.clone());
        new_points1.extend(path1);
        new_points2.extend(path2);
    }
    return (new_points1, new_points2);
}


pub fn add_n_more_subobjects(
    subobjects: &Vec<VectorFeatures>,
    n: usize
) -> Vec<VectorFeatures> {
    if n == 0 {
        return subobjects.clone();
    }
    if subobjects.len() == 0 {
        return vec![VectorFeatures {
            points: vec![(0.0, 0.0)],
            subobjects: vec![],
            index: 0,
            stroke_color: (0.0, 0.0, 0.0, 0.0),
            fill_color: (0.0, 0.0, 0.0, 0.0),
            stroke_width: 0.0,
            line_cap: "butt",
            line_join: "miter",
            background_image: None,
            image_position: (0.0, 0.0),
        }; n];
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
            new_subobjects.push(subobject.clone()
                .set_fill_opacity(0.0, true)
                .set_stroke_opacity(0.0, true));
        }
    }
    return new_subobjects;
}


pub fn align_subobjects(
    subobjects1: Vec<VectorFeatures>,
    subobjects2: Vec<VectorFeatures>
) -> (Vec<VectorFeatures>, Vec<VectorFeatures>) {
    let mut new_subobjects1 = subobjects1.clone();
    let mut new_subobjects2 = subobjects2.clone();
    new_subobjects1 = add_n_more_subobjects(&new_subobjects1, (new_subobjects2.len() as i32 - new_subobjects1.len() as i32).max(0) as usize);
    new_subobjects2 = add_n_more_subobjects(&new_subobjects2, (new_subobjects1.len() as i32 - new_subobjects2.len() as i32).max(0) as usize);
    return (new_subobjects1, new_subobjects2);
}


pub fn align_data(
    vec_obj1: VectorFeatures,
    vec_obj2: VectorFeatures,
    skip_point_align: bool
) -> (VectorFeatures, VectorFeatures) {
    let (
        mut vec_obj1,
        mut vec_obj2
    ) = null_point_align(vec_obj1, vec_obj2);
    let (
        mut new_subobjects1,
        mut new_subobjects2
    ) = align_subobjects(vec_obj1.subobjects.clone(), vec_obj2.subobjects.clone());
    let mut new_points1 = vec_obj1.points.clone();
    let mut new_points2 = vec_obj2.points.clone();
    if !skip_point_align {
        (new_points1, new_points2) = align_points(new_points1, new_points2);
    }
    for i in 0..new_subobjects1.len() {
        let (
            sub_vec_obj1,
            sub_vec_obj2
        ) = align_data(new_subobjects1[i].clone(), new_subobjects2[i].clone(), false);
        new_subobjects1[i] = sub_vec_obj1;
        new_subobjects2[i] = sub_vec_obj2;
    }
    vec_obj1.points = new_points1;
    vec_obj2.points = new_points2;
    vec_obj1.subobjects = new_subobjects1;
    vec_obj2.subobjects = new_subobjects2;
    return (vec_obj1, vec_obj2);
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
    return distance_squared(p1, p2) < f64::EPSILON;
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
    return 2.0 * smooth(t / 2.0, inflection);
}


pub fn rush_from(t: f64, inflection: f64) -> f64 {
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
    function: fn(f64) -> f64,
    t: f64,
    proportion: f64
) -> f64 {
    return function(t) * proportion;
}


pub fn wiggle(t: f64, wiggles: f64) -> f64 {
    return (t * PI * wiggles).sin() * there_and_back(t, 10.0);
}


pub fn squish_rate_func(
    function: fn(f64) -> f64,
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
