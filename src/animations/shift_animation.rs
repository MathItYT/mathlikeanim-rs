use crate::{objects::vector_object::{VectorFeatures, VectorObject}, utils::interpolate_tuple};

pub fn shift_animation(vec_obj: VectorFeatures, shift: (f64, f64), t: f64) -> VectorFeatures {
    return vec_obj.shift(interpolate_tuple((0.0, 0.0), shift, t), true);
}