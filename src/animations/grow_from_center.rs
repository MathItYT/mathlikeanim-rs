use crate::objects::vector_object::VectorObject;

pub fn grow_from_center(vec_obj: VectorObject, t: f64) -> VectorObject {
    return vec_obj.scale(t, true).move_to(vec_obj.get_center(), true);
}