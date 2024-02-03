use std::f64::consts::PI;

use crate::objects::{geometry::poly::equilateral_triangle, vector_object::{VectorFeatures, VectorObject}};

pub fn add_final_tip(
    shape: VectorFeatures,
    tip_side_length: f64,
    tip_color: (f64, f64, f64, f64)
) -> VectorFeatures {
    let last_point = shape.points[shape.points.len() - 1];
    let second_to_last_point = shape.points[shape.points.len() - 2];
    let angle = (last_point.1 - second_to_last_point.1).atan2(last_point.0 - second_to_last_point.0);
    let new_tip = equilateral_triangle(
        last_point,
        tip_side_length,
        None,
        Some(tip_color),
        Some(0.0),
        None,
        None,
        None
    ).rotate(angle, false);
    let mut subobjects = shape.subobjects.clone();
    subobjects.push(new_tip);
    let new_vec_obj = shape.clone().set_subobjects(subobjects);
    return new_vec_obj;
}


pub fn add_initial_tip(
    shape: VectorFeatures,
    tip_side_length: f64,
    tip_color: (f64, f64, f64, f64)
) -> VectorFeatures {
    let first_point = shape.points[0];
    let second_point = shape.points[1];
    let angle = (second_point.1 - first_point.1).atan2(second_point.0 - first_point.0) + PI;
    let new_tip = equilateral_triangle(
        first_point,
        tip_side_length,
        None,
        Some(tip_color),
        Some(0.0),
        None,
        None,
        None
    ).rotate(angle, false);
    let mut subobjects = shape.subobjects.clone();
    subobjects.push(new_tip);
    let new_vec_obj = shape.clone().set_subobjects(subobjects);
    return new_vec_obj;
}


pub fn add_both_sides_tips(
    shape: VectorFeatures,
    tip_side_length: f64,
    tip_color: (f64, f64, f64, f64)
) -> VectorFeatures {
    return add_final_tip(
        add_initial_tip(
            shape,
            tip_side_length,
            tip_color
        ),
        tip_side_length,
        tip_color
    );
}
