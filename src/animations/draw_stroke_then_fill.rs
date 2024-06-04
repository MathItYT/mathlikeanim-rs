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
            .set_stroke_width(stroke_width, false)
            .get_partial_copy(0.0, subalpha, false)
            .set_fill_opacity(0.0, false)
            .set_subobjects(vec_obj.subobjects.iter().map(|subobj| {
                draw_stroke_then_fill(subobj.clone(), t)
            }).collect());
        return new_vec_obj;
    } else if index == 1 {
        let vec_obj = vec_obj.clone();
        let stroke_width = if vec_obj.stroke_width == 0.0 {
            interpolate(2.0, 0.0, subalpha)
        } else {
            vec_obj.stroke_width
        };
        new_vec_obj = vec_obj
            .set_stroke_width(stroke_width, false)
            .set_subobjects(vec_obj.subobjects.iter().map(|subobj| {
                draw_stroke_then_fill(subobj.clone(), t)
            }).collect())
            .set_fill_opacity(interpolate(0.0, vec_obj.get_fill_opacity(), subalpha), false);
        return new_vec_obj;
    }
    return new_vec_obj;
}


pub fn write(number_of_objects: usize, lag_ratio: f64) -> Box<dyn Fn(VectorFeatures, f64) -> VectorFeatures> {
    let mut anim_funcs = Vec::new();
    for _ in 0..number_of_objects {
        anim_funcs.push(Box::new(|vec_obj, t| {
            draw_stroke_then_fill(vec_obj, t)
        }) as Box<dyn Fn(VectorFeatures, f64) -> VectorFeatures>);
    }
    let result = animation_group(anim_funcs, lag_ratio);
    return result;
}
