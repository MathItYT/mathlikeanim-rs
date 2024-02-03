use crate::{utils::{interpolate, interpolate_tuple}, objects::vector_object::{VectorFeatures, VectorObject}};


pub fn fade_in(scale_factor: f64, shift: (f64, f64)) -> impl Fn(VectorFeatures, f64) -> VectorFeatures {
    let animation_func = move |vec_obj: VectorFeatures, t: f64| -> VectorFeatures {
        let result = vec_obj.clone()
            .scale(interpolate(scale_factor, 1.0, t), true)
            .move_to(interpolate_tuple((vec_obj.get_center().0 + shift.0, vec_obj.get_center().1 + shift.1), vec_obj.get_center(), t), true)
            .set_fill_opacity(interpolate(0.0, 1.0, t), true);
        return result;
    };
    return animation_func;
}


pub fn fade_out(scale_factor: f64, shift: (f64, f64)) -> impl Fn(VectorFeatures, f64) -> VectorFeatures {
    let animation_func = move |vec_obj: VectorFeatures, t: f64| -> VectorFeatures {
        let result = vec_obj.clone()
            .scale(interpolate(1.0, scale_factor, t), true)
            .move_to(interpolate_tuple(vec_obj.get_center(), (vec_obj.get_center().0 + shift.0, vec_obj.get_center().1 + shift.1), t), true)
            .set_fill_opacity(interpolate(1.0, 0.0, t), true);
        return result;
    };
    return animation_func;
}
