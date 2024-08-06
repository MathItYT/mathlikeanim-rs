use crate::{utils::interpolate, objects::vector_object::VectorFeatures};


pub fn scale_in_place(vec_obj: VectorFeatures, scale_factor: f64, t: f64) -> VectorFeatures {
    return vec_obj.clone()
        .scale(interpolate(1.0, scale_factor, t), true)
        .move_to(vec_obj.get_center(), true);
}
