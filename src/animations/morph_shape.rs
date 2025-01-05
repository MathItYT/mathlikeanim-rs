use wasm_bindgen::JsError;

use crate::{colors::{Color, GradientImageOrColor}, objects::{three_d::three_d_object::ThreeDObject, vector_object::VectorObject}, utils::{error, interpolate, interpolate_color, interpolate_tuple, interpolate_tuple_3d}};

pub fn morph_shape(original_obj: VectorObject, target_obj: VectorObject, t: f64) -> VectorObject {
    if t == 0.0 {
        return original_obj.clone();
    }
    if t == 1.0 {
        return target_obj.set_index(original_obj.index);
    }
    if original_obj.points.len() != target_obj.points.len() {
        error(JsError::new("Original and target objects have different number of points"));
    }
    if original_obj.subobjects.len() != target_obj.subobjects.len() {
        error(JsError::new("Original and target objects have different number of subobjects"));
    }
    let original_points = original_obj.clone().points;
    let target_points = target_obj.clone().points;
    let mut new_points = Vec::new();
    for i in 0..original_points.len() {
        new_points.push(interpolate_tuple(original_points[i], target_points[i], t));
    }
    let original_subobjects = original_obj.clone().subobjects;
    let target_subobjects = target_obj.subobjects;
    let mut new_subobjects = Vec::new();
    for i in 0..original_subobjects.len() {
        new_subobjects.push(morph_shape(original_subobjects[i].clone(), target_subobjects[i].clone(), t));
    }
    let mut result = original_obj.clone()
        .set_points(new_points)
        .set_subobjects(new_subobjects)
        .set_stroke_width(interpolate(original_obj.stroke_width, target_obj.stroke_width, t), false);
    match result.fill.clone() {
        GradientImageOrColor::Color(color_original) => {
            match target_obj.fill.clone() {
                GradientImageOrColor::Color(color_target) => {
                    let color = interpolate_color((color_original.red, color_original.green, color_original.blue, color_original.alpha), (color_target.red, color_target.green, color_target.blue, color_target.alpha), t);
                    result.fill = GradientImageOrColor::Color(Color {
                        red: color.0,
                        green: color.1,
                        blue: color.2,
                        alpha: color.3,
                    });
                }
                _ => {}
            }
        }
        _ => {}
    }
    match result.stroke.clone() {
        GradientImageOrColor::Color(color_original) => {
            match target_obj.stroke.clone() {
                GradientImageOrColor::Color(color_target) => {
                    let color = interpolate_color((color_original.red, color_original.green, color_original.blue, color_original.alpha), (color_target.red, color_target.green, color_target.blue, color_target.alpha), t);
                    result.stroke = GradientImageOrColor::Color(Color {
                        red: color.0,
                        green: color.1,
                        blue: color.2,
                        alpha: color.3,
                    });
                }
                _ => {}
            }
        }
        _ => {}
    }
    return result;
}


pub fn morph_shape_3d(original_obj: ThreeDObject, target_obj: ThreeDObject, t: f64) -> ThreeDObject {
    if t == 0.0 {
        return original_obj.clone();
    }
    if t == 1.0 {
        return target_obj.set_index(original_obj.index);
    }
    if original_obj.points.len() != target_obj.points.len() {
        error(JsError::new("Original and target objects have different number of points"));
    }
    if original_obj.subobjects.len() != target_obj.subobjects.len() {
        error(JsError::new("Original and target objects have different number of subobjects"));
    }
    let original_points = original_obj.clone().points;
    let target_points = target_obj.clone().points;
    let mut new_points = Vec::new();
    for i in 0..original_points.len() {
        new_points.push(interpolate_tuple_3d(original_points[i], target_points[i], t));
    }
    let original_subobjects = original_obj.clone().subobjects;
    let target_subobjects = target_obj.subobjects;
    let mut new_subobjects = Vec::new();
    for i in 0..original_subobjects.len() {
        new_subobjects.push(morph_shape_3d(original_subobjects[i].clone(), target_subobjects[i].clone(), t));
    }
    let mut result = original_obj.clone()
        .set_points(new_points)
        .set_subobjects(new_subobjects)
        .set_stroke_width(interpolate(original_obj.stroke_width, target_obj.stroke_width, t), false);
    match result.fill.clone() {
        GradientImageOrColor::Color(color_original) => {
            match target_obj.fill.clone() {
                GradientImageOrColor::Color(color_target) => {
                    let color = interpolate_color((color_original.red, color_original.green, color_original.blue, color_original.alpha), (color_target.red, color_target.green, color_target.blue, color_target.alpha), t);
                    result.fill = GradientImageOrColor::Color(Color {
                        red: color.0,
                        green: color.1,
                        blue: color.2,
                        alpha: color.3,
                    });
                }
                _ => {}
            }
        }
        _ => {}
    }
    match result.stroke.clone() {
        GradientImageOrColor::Color(color_original) => {
            match target_obj.stroke.clone() {
                GradientImageOrColor::Color(color_target) => {
                    let color = interpolate_color((color_original.red, color_original.green, color_original.blue, color_original.alpha), (color_target.red, color_target.green, color_target.blue, color_target.alpha), t);
                    result.stroke = GradientImageOrColor::Color(Color {
                        red: color.0,
                        green: color.1,
                        blue: color.2,
                        alpha: color.3,
                    });
                }
                _ => {}
            }
        }
        _ => {}
    }
    return result;
}
