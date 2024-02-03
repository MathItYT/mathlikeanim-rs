use crate::{utils::interpolate, objects::vector_object::{VectorFeatures, VectorObject}};


pub fn scale_in_place(scale_factor: f64) -> impl Fn(VectorFeatures, f64) -> VectorFeatures {
    let animation_func = move |vec_obj: VectorFeatures, t: f64| -> VectorFeatures {
        let result = vec_obj.clone()
                .scale(interpolate(1.0, scale_factor, t), true)
                .move_to(vec_obj.get_center(), true);
        return result;
    };
    return animation_func;
}
