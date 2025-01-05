use crate::objects::{three_d::three_d_object::ThreeDObject, vector_object::VectorObject};

pub fn grow_from_center(vec_obj: VectorObject, t: f64) -> VectorObject {
    return vec_obj.scale(t, true).move_to(vec_obj.get_center(), true);
}

pub fn grow_from_center_3d(obj_3d: ThreeDObject, t: f64) -> ThreeDObject {
    return obj_3d.scale(t, true).move_to(obj_3d.get_center(), true);
}
