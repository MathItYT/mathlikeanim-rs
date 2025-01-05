use crate::{colors::{Color, GradientImageOrColor}, objects::{three_d::three_d_object::ThreeDObject, vector_object::VectorObject}, utils::interpolate_color};

pub fn set_stroke_animation(vec_obj: VectorObject, target_stroke: (f64, f64, f64, f64), t: f64) -> VectorObject {
    let mut vec_obj = vec_obj;
    match vec_obj.get_stroke() {
        GradientImageOrColor::Color(color) => {
            let color = (color.red, color.green, color.blue, color.alpha);
            let new_color = interpolate_color(color, target_stroke, t);
            vec_obj = vec_obj.set_stroke(GradientImageOrColor::Color(Color {
                red: new_color.0,
                green: new_color.1,
                blue: new_color.2,
                alpha: new_color.3,
            }), true);
        },
        _ => {}
    }
    return vec_obj;
}


pub fn set_stroke_animation_3d(obj_3d: ThreeDObject, target_stroke: (f64, f64, f64, f64), t: f64) -> ThreeDObject {
    let mut obj_3d = obj_3d;
    match obj_3d.get_stroke() {
        GradientImageOrColor::Color(color) => {
            let color = (color.red, color.green, color.blue, color.alpha);
            let new_color = interpolate_color(color, target_stroke, t);
            obj_3d = obj_3d.set_stroke(GradientImageOrColor::Color(Color {
                red: new_color.0,
                green: new_color.1,
                blue: new_color.2,
                alpha: new_color.3,
            }), false);
        },
        _ => {}
    }
    obj_3d.subobjects = obj_3d.subobjects.iter().map(|subobj| {
        return set_stroke_animation_3d(subobj.clone(), target_stroke, t);
    }).collect();
    return obj_3d;
}