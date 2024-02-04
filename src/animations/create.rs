use crate::objects::vector_object::{VectorFeatures, VectorObject};

pub fn create(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let new_vec_obj = vec_obj.clone()
        .get_partial_copy(0.0, t, true);
    return new_vec_obj;
}