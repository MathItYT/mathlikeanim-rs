use crate::{utils::{interpolate, interpolate_tuple}, objects::vector_object::VectorObject};


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
