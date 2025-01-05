use crate::{objects::vector_object::VectorObject, utils::interpolate};

pub fn spinning_grow(vec_obj: VectorObject, angle: f64, t: f64) -> VectorObject {
    let mut new_vec_obj = vec_obj.rotate(interpolate(angle, 0.0, t), true)
        .scale(interpolate(0.0, 1.0, t), true);
    let new_vec_obj_center = new_vec_obj.get_center();
    let vec_obj_center = vec_obj.get_center();
    new_vec_obj = new_vec_obj.shift(
        (vec_obj_center.0 - new_vec_obj_center.0, vec_obj_center.1 - new_vec_obj_center.1),
        true
    );
    return new_vec_obj;
}
