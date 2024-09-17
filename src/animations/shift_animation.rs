use crate::{objects::vector_object::VectorObject, utils::interpolate_tuple};

pub fn shift_animation(vec_obj: VectorObject, shift: (f64, f64), t: f64) -> VectorObject {
    return vec_obj.shift(interpolate_tuple((0.0, 0.0), shift, t), true);
}