use crate::objects::vector_object::{VectorFeatures, VectorObject};
use crate::utils::{integer_interpolate, interpolate};

use crate::animations::animation_group::animation_group;


pub fn draw_stroke_then_fill(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let mut new_vec_obj = vec_obj.clone();
    let (index, subalpha) = integer_interpolate(0.0, 2.0, t);
    if index == 0 {
        let stroke_width = if vec_obj.stroke_width == 0.0 {
            2.0
        } else {
            vec_obj.stroke_width
        };
        new_vec_obj = vec_obj
            .set_stroke_width(stroke_width, true)
            .get_partial_copy(0.0, subalpha, true)
            .set_fill_opacity(0.0, true);
    } else if index == 1 {
        let vec_obj = vec_obj.clone();
        let stroke_width = if vec_obj.stroke_width == 0.0 {
            interpolate(2.0, 0.0, subalpha)
        } else {
            vec_obj.stroke_width
        };
        new_vec_obj = vec_obj
            .set_stroke_width(stroke_width, true)
            .set_fill_opacity(interpolate(0.0, 1.0, subalpha), true);
    }
    return new_vec_obj;
}


pub fn write(number_of_objects: usize, lag_ratio: f64) -> Vec<impl Fn(VectorFeatures, f64) -> VectorFeatures> {
    let mut anim_funcs: Vec<fn(VectorFeatures, f64) -> VectorFeatures> = Vec::new();
    for _ in 0..number_of_objects {
        anim_funcs.push(draw_stroke_then_fill);
    }
    let anim_funcs: &'static Vec<fn(VectorFeatures, f64) -> VectorFeatures> = Box::leak(Box::new(anim_funcs));
    return animation_group(anim_funcs, lag_ratio);
}
