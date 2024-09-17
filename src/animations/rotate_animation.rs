use crate::objects::vector_object::VectorObject;

pub fn rotate_animation(vec_obj: VectorObject, angle: f64, t: f64) -> VectorObject {
    let mut new_vec_obj = vec_obj.rotate(angle * t, true);
    new_vec_obj = new_vec_obj.move_to(vec_obj.get_center(), true);
    return new_vec_obj;
}