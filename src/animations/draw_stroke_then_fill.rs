use crate::objects::three_d::three_d_object::ThreeDObject;
use crate::objects::vector_object::VectorObject;
use crate::utils::{integer_interpolate, interpolate};


pub fn draw_stroke_then_fill(vec_obj: VectorObject, t: f64, default_stroke_width: Option<f64>) -> VectorObject {
    let mut new_vec_obj = vec_obj.clone();
    let (index, subalpha) = integer_interpolate(0.0, 2.0, t);
    if index == 0 {
        let stroke_width = if vec_obj.stroke_width == 0.0 {
            default_stroke_width.unwrap_or(2.0)
        } else {
            vec_obj.stroke_width
        };
        new_vec_obj = vec_obj
            .set_stroke_width(stroke_width, false)
            .get_partial_copy(0.0, subalpha, false)
            .set_fill_opacity(0.0, false)
            .set_subobjects(vec_obj.subobjects.iter().map(|subobj| {
                draw_stroke_then_fill(subobj.clone(), t, default_stroke_width)
            }).collect());
        return new_vec_obj;
    } else if index == 1 {
        let vec_obj = vec_obj.clone();
        let stroke_width = if vec_obj.stroke_width == 0.0 {
            interpolate(default_stroke_width.unwrap_or(2.0), 0.0, subalpha)
        } else {
            vec_obj.stroke_width
        };
        new_vec_obj = vec_obj
            .set_stroke_width(stroke_width, false)
            .set_subobjects(vec_obj.subobjects.iter().map(|subobj| {
                draw_stroke_then_fill(subobj.clone(), t, default_stroke_width)
            }).collect())
            .set_fill_opacity(interpolate(0.0, vec_obj.get_fill_opacity(), subalpha), false);
        return new_vec_obj;
    }
    return new_vec_obj;
}


pub fn draw_stroke_then_fill_3d(obj_3d: ThreeDObject, t: f64, default_stroke_width: Option<f64>) -> ThreeDObject {
    let mut new_obj = obj_3d.clone();
    let (index, subalpha) = integer_interpolate(0.0, 2.0, t);
    if index == 0 {
        let stroke_width = if obj_3d.stroke_width == 0.0 {
            default_stroke_width.unwrap_or(0.5)
        } else {
            obj_3d.stroke_width
        };
        new_obj = obj_3d
            .set_stroke_width(stroke_width, false)
            .get_partial_copy(0.0, subalpha, false)
            .set_fill_opacity(0.0, false)
            .set_subobjects(obj_3d.subobjects.iter().map(|subobj| {
                draw_stroke_then_fill_3d(subobj.clone(), t, default_stroke_width)
            }).collect());
        return new_obj;
    } else if index == 1 {
        let obj_3d = obj_3d.clone();
        let stroke_width = if obj_3d.stroke_width == 0.0 {
            interpolate(default_stroke_width.unwrap_or(0.5), 0.0, subalpha)
        } else {
            obj_3d.stroke_width
        };
        new_obj = obj_3d
            .set_stroke_width(stroke_width, false)
            .set_subobjects(obj_3d.subobjects.iter().map(|subobj| {
                draw_stroke_then_fill_3d(subobj.clone(), t, default_stroke_width)
            }).collect())
            .set_fill_opacity(interpolate(0.0, obj_3d.get_fill_opacity(), subalpha), false);
        return new_obj;
    }
    return new_obj;
}
