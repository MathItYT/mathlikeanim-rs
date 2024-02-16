use crate::{objects::vector_object::{VectorFeatures, VectorObject}, utils::interpolate_color};

pub fn set_stroke_animation(target_stroke: (f64, f64, f64, f64)) -> impl Fn(VectorFeatures, f64) -> VectorFeatures {
    move |vec_obj: VectorFeatures, t: f64| {
        let mut vec_obj = vec_obj;
        vec_obj = vec_obj.set_stroke_color(interpolate_color(vec_obj.get_stroke_color(), target_stroke, t), true);
        return vec_obj;
    }
}