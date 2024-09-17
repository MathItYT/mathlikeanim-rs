use crate::objects::vector_object::VectorObject;

pub fn create(vec_obj: VectorObject, t: f64) -> VectorObject {
    let new_vec_obj = vec_obj.clone()
        .get_partial_copy(0.0, t, true);
    return new_vec_obj;
}