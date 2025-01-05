use crate::{objects::{three_d::three_d_object::ThreeDObject, vector_object::VectorObject}, utils::interpolate};


pub fn scale_in_place(vec_obj: VectorObject, scale_factor: f64, t: f64) -> VectorObject {
    return vec_obj.clone()
        .scale(interpolate(1.0, scale_factor, t), true)
        .move_to(vec_obj.get_center(), true);
}


pub fn scale_in_place_3d(obj_3d: ThreeDObject, scale_factor: f64, t: f64) -> ThreeDObject {
    return obj_3d.clone()
        .scale(interpolate(1.0, scale_factor, t), true)
        .move_to(obj_3d.get_center(), true);
}
