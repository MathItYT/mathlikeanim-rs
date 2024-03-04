use crate::{objects::vector_object::{VectorFeatures, VectorObject}, utils::{align_data, interpolate, interpolate_tuple}};

pub fn morph_shape(target: VectorFeatures) -> impl Fn(VectorFeatures, f64) -> VectorFeatures {
    let animation_func = move |vec_obj: VectorFeatures, t: f64| -> VectorFeatures {
        if t == 0.0 {
            return vec_obj.clone();
        }
        if t == 1.0 {
            return target.clone();
        }
        let (
            original_obj,
            target_obj
        ) = align_data(vec_obj.clone(), target.clone(), false);
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
            new_subobjects.push(morph_shape(target_subobjects[i].clone())(original_subobjects[i].clone(), t));
        }
        let result = original_obj.clone()
            .set_points(new_points)
            .set_subobjects(new_subobjects)
            .set_stroke_width(interpolate(original_obj.stroke_width, target_obj.stroke_width, t), false);
        return result;
    };
    return animation_func;
}
