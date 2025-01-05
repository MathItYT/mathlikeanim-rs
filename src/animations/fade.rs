use crate::{objects::{three_d::three_d_object::ThreeDObject, vector_object::VectorObject}, utils::{interpolate, interpolate_tuple, interpolate_tuple_3d}};


pub fn fade_in(vec_obj: VectorObject, scale_factor: f64, shift: (f64, f64), t: f64) -> VectorObject {
    let mut result = vec_obj.clone()
        .set_fill_opacity(interpolate(0.0, vec_obj.get_fill_opacity(), t), false)
        .set_stroke_opacity(interpolate(0.0, vec_obj.get_stroke_opacity(), t), false);
    result.subobjects = result.subobjects.iter().map(|subobj| {
        return fade_in(subobj.clone(), 1.0, (0.0, 0.0), t);
    }).collect();
    result = result
        .scale(interpolate(scale_factor, 1.0, t), true)
        .move_to(interpolate_tuple((vec_obj.get_center().0 + shift.0, vec_obj.get_center().1 + shift.1), vec_obj.get_center(), t), true);
    return result;
}


pub fn fade_out(vec_obj: VectorObject, scale_factor: f64, shift: (f64, f64), t: f64) -> VectorObject {
    let mut result = vec_obj.clone()
        .set_fill_opacity(interpolate(vec_obj.get_fill_opacity(), 0.0, t), false)
        .set_stroke_opacity(interpolate(vec_obj.get_stroke_opacity(), 0.0, t), false);
    result.subobjects = result.subobjects.iter().map(|subobj| {
        return fade_out(subobj.clone(), 1.0, (0.0, 0.0), t);
    }).collect();
    result = result
        .scale(interpolate(1.0, scale_factor, t), true)
        .move_to(interpolate_tuple(vec_obj.get_center(), (vec_obj.get_center().0 + shift.0, vec_obj.get_center().1 + shift.1), t), true);
    return result;
}


pub fn fade_in_3d(obj_3d: ThreeDObject, scale_factor: f64, shift: (f64, f64, f64), t: f64) -> ThreeDObject {
    let mut result = obj_3d.clone()
        .set_fill_opacity(interpolate(0.0, obj_3d.get_fill_opacity(), t), false)
        .set_stroke_opacity(interpolate(0.0, obj_3d.get_stroke_opacity(), t), false);
    result.subobjects = result.subobjects.iter().map(|subobj| {
        return fade_in_3d(subobj.clone(), 1.0, (0.0, 0.0, 0.0), t);
    }).collect();
    result = result
        .scale(interpolate(scale_factor, 1.0, t), true)
        .move_to(interpolate_tuple_3d((obj_3d.get_center().0 + shift.0, obj_3d.get_center().1 + shift.1, obj_3d.get_center().2 + shift.2), obj_3d.get_center(), t), true);
    return result;
}


pub fn fade_out_3d(obj_3d: ThreeDObject, scale_factor: f64, shift: (f64, f64, f64), t: f64) -> ThreeDObject {
    let mut result = obj_3d.clone()
        .set_fill_opacity(interpolate(obj_3d.get_fill_opacity(), 0.0, t), false)
        .set_stroke_opacity(interpolate(obj_3d.get_stroke_opacity(), 0.0, t), false);
    result.subobjects = result.subobjects.iter().map(|subobj| {
        return fade_out_3d(subobj.clone(), 1.0, (0.0, 0.0, 0.0), t);
    }).collect();
    result = result
        .scale(interpolate(1.0, scale_factor, t), true)
        .move_to(interpolate_tuple_3d(obj_3d.get_center(), (obj_3d.get_center().0 + shift.0, obj_3d.get_center().1 + shift.1, obj_3d.get_center().2 + shift.2), t), true);
    return result;
}
