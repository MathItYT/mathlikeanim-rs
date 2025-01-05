use crate::objects::{three_d::three_d_object::ThreeDObject, vector_object::VectorObject};

pub fn rotate_animation(vec_obj: VectorObject, angle: f64, t: f64) -> VectorObject {
    let mut new_vec_obj = vec_obj.rotate(angle * t, true);
    new_vec_obj = new_vec_obj.move_to(vec_obj.get_center(), true);
    return new_vec_obj;
}

pub fn rotate_x_animation_3d(obj_3d: ThreeDObject, angle: f64, t: f64) -> ThreeDObject {
    let mut new_obj = obj_3d.rotate_x(angle * t, true);
    new_obj = new_obj.move_to(obj_3d.get_center(), true);
    return new_obj;
}


pub fn rotate_y_animation_3d(obj_3d: ThreeDObject, angle: f64, t: f64) -> ThreeDObject {
    let mut new_obj = obj_3d.rotate_y(angle * t, true);
    new_obj = new_obj.move_to(obj_3d.get_center(), true);
    return new_obj;
}


pub fn rotate_z_animation_3d(obj_3d: ThreeDObject, angle: f64, t: f64) -> ThreeDObject {
    let mut new_obj = obj_3d.rotate_z(angle * t, true);
    new_obj = new_obj.move_to(obj_3d.get_center(), true);
    return new_obj;
}