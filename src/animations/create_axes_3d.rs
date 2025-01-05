use crate::{colors::{Color, GradientImageOrColor}, objects::three_d::three_d_object::ThreeDObject, utils::integer_interpolate};

use super::draw_stroke_then_fill::draw_stroke_then_fill_3d;

pub fn create_axes_3d(obj_3d: ThreeDObject, t: f64, default_stroke_width: Option<f64>) -> ThreeDObject {
    if t == 0.0 {
        return ThreeDObject::new(
            Vec::new(),
            Vec::new(),
            GradientImageOrColor::Color(Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }),
            GradientImageOrColor::Color(Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }),
            0.0,
            obj_3d.get_index()
        )
    }
    if t == 1.0 {
        return obj_3d.clone();
    }
    let mut x_axis = obj_3d.subobjects[0].clone();
    let mut y_axis = obj_3d.subobjects[1].clone();
    let mut z_axis = obj_3d.subobjects[2].clone();
    let mut x_axis_pieces = x_axis.subobjects[0].clone();
    let mut y_axis_pieces = y_axis.subobjects[0].clone();
    let mut z_axis_pieces = z_axis.subobjects[0].clone();
    let n_pieces = x_axis_pieces.subobjects.len();
    let (index, residue) = integer_interpolate(0.0, n_pieces as f64, t);
    x_axis_pieces.subobjects = x_axis_pieces.subobjects.iter().enumerate().map(|(i, subobj)| {
        if i as i64 == index {
            return subobj.get_partial_copy(0.0, residue, true);
        }
        if i as i64 > index {
            return subobj.set_points(Vec::new());
        }
        return subobj.clone();
    }).collect();
    y_axis_pieces.subobjects = y_axis_pieces.subobjects.iter().enumerate().map(|(i, subobj)| {
        if i as i64 == index {
            return subobj.get_partial_copy(0.0, residue, true);
        }
        if i as i64 > index {
            return subobj.set_points(Vec::new());
        }
        return subobj.clone();
    }).collect();
    z_axis_pieces.subobjects = z_axis_pieces.subobjects.iter().enumerate().map(|(i, subobj)| {
        if i as i64 == index {
            return subobj.get_partial_copy(0.0, residue, true);
        }
        if i as i64 > index {
            return subobj.set_points(Vec::new());
        }
        return subobj.clone();
    }).collect();
    x_axis.subobjects[0] = x_axis_pieces;
    y_axis.subobjects[0] = y_axis_pieces;
    z_axis.subobjects[0] = z_axis_pieces;
    x_axis.subobjects[1] = draw_stroke_then_fill_3d(x_axis.subobjects[1].clone(), t, default_stroke_width);
    y_axis.subobjects[1] = draw_stroke_then_fill_3d(y_axis.subobjects[1].clone(), t, default_stroke_width);
    z_axis.subobjects[1] = draw_stroke_then_fill_3d(z_axis.subobjects[1].clone(), t, default_stroke_width);
    return ThreeDObject::new(
        Vec::new(),
        vec![x_axis, y_axis, z_axis],
        GradientImageOrColor::Color(Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }),
        GradientImageOrColor::Color(Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }),
        0.0,
        obj_3d.get_index()
    );
}