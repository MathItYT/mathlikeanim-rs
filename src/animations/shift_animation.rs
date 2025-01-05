use crate::{objects::{three_d::three_d_object::ThreeDObject, vector_object::VectorObject}, utils::{interpolate_tuple, interpolate_tuple_3d}};

pub fn shift_animation(vec_obj: VectorObject, shift: (f64, f64), t: f64) -> VectorObject {
    return vec_obj.shift(interpolate_tuple((0.0, 0.0), shift, t), true);
}

pub fn shift_animation_3d(obj_3d: ThreeDObject, shift: (f64, f64, f64), t: f64) -> ThreeDObject {
    return obj_3d.shift(interpolate_tuple_3d((0.0, 0.0, 0.0), shift, t), true);
}