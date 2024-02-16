use crate::objects::vector_object::{VectorFeatures, VectorObject};

pub fn rotate_animation(angle: f64) -> impl Fn(VectorFeatures, f64) -> VectorFeatures {
    return move |vec_obj: VectorFeatures, t: f64| {
        let mut new_vec_obj = vec_obj.rotate(angle * t, true);
        new_vec_obj = new_vec_obj.move_to(vec_obj.get_center(), true);
        return new_vec_obj;
    };
}