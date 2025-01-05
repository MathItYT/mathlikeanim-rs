use crate::objects::{three_d::three_d_object::ThreeDObject, vector_object::VectorObject};

pub fn create(vec_obj: VectorObject, t: f64) -> VectorObject {
    let new_vec_obj = vec_obj.clone()
        .get_partial_copy(0.0, t, true);
    return new_vec_obj;
}

pub fn create_3d(obj_3d: ThreeDObject, t: f64) -> ThreeDObject {
    let new_obj_3d = obj_3d.get_partial_copy(0.0, t, true);
    return new_obj_3d;
}