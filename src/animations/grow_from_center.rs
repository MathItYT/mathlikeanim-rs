use crate::objects::vector_object::VectorFeatures;

pub fn grow_from_center(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    return vec_obj.scale(t, true).move_to(vec_obj.get_center(), true);
}