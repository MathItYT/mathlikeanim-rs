use crate::{objects::vector_object::{VectorFeatures, VectorObject}, utils::interpolate_tuple};

pub fn shift_animation(shift: (f64, f64)) -> impl Fn(VectorFeatures, f64) -> VectorFeatures {
    let anim_function = move |vec_obj: VectorFeatures, t: f64| -> VectorFeatures {
        return vec_obj.shift(interpolate_tuple((0.0, 0.0), shift, t), true);
    };
    return anim_function;
}