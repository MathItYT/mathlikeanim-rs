use crate::{objects::vector_object::{VectorFeatures, VectorObject}, utils::interpolate_color};

pub fn set_fill_animation(target_fill: (f64, f64, f64, f64)) -> impl Fn(VectorFeatures, f64) -> VectorFeatures {
    move |vec_obj: VectorFeatures, t: f64| {
        let mut vec_obj = vec_obj;
        vec_obj = vec_obj.set_fill_color(interpolate_color(vec_obj.get_fill_color(), target_fill, t), true);
        return vec_obj;
    }
}