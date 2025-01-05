use js_sys::Array;
use wasm_bindgen::prelude::*;

use crate::objects::three_d::wasm_interface::WasmThreeDObject;
use crate::objects::wasm_interface::{WasmColor, WasmVectorObject};
use crate::{scene::Scene, svg_scene::SVGScene};
use super::create::create_3d;
use super::create_axes_3d::create_axes_3d;
use super::draw_stroke_then_fill::draw_stroke_then_fill_3d;
use super::fade::{fade_in_3d, fade_out_3d};
use super::grow_from_center::grow_from_center_3d;
use super::morph_shape::morph_shape_3d;
use super::rotate_animation::{rotate_x_animation_3d, rotate_y_animation_3d, rotate_z_animation_3d};
use super::scale_in_place::scale_in_place_3d;
use super::set_fill_animation::set_fill_animation_3d;
use super::set_stroke_animation::set_stroke_animation_3d;
use super::shift_animation::shift_animation_3d;
use super::{move_camera::move_camera, move_camera_svg::move_camera_svg};

use super::{create::create, draw_stroke_then_fill::draw_stroke_then_fill, fade::{fade_in, fade_out}, grow_arrow::{grow_arrow_with_final_tip, grow_arrow_with_initial_tip, grow_arrow_with_tips_at_both_ends}, grow_from_center::grow_from_center, morph_shape::morph_shape, rotate_animation::rotate_animation, scale_in_place::scale_in_place, set_fill_animation::set_fill_animation, set_stroke_animation::set_stroke_animation, shift_animation::shift_animation, show_temporarily::show_temporarily, spinning_grow::spinning_grow};


#[wasm_bindgen(js_name = create)]
pub fn create_js(
    vec_obj: WasmVectorObject,
    t: f64,
) -> WasmVectorObject {
    let vec_obj = vec_obj.native_vec_features;
    let new_vec_obj = create(vec_obj, t);
    return WasmVectorObject { native_vec_features: new_vec_obj };
}


#[wasm_bindgen(js_name = create3D)]
pub fn create_3d_js(
    vec_obj: WasmThreeDObject,
    t: f64,
) -> WasmThreeDObject {
    let obj_3d = vec_obj.three_d_object;
    WasmThreeDObject { three_d_object: create_3d(obj_3d, t) }
}


#[wasm_bindgen(js_name = createAxes3D)]
pub fn create_axes_3d_js(
    axes: WasmThreeDObject,
    t: f64,
    default_stroke_width: Option<f64>,
) -> WasmThreeDObject {
    let axes = axes.three_d_object;
    WasmThreeDObject { three_d_object: create_axes_3d(axes, t, default_stroke_width) }
}


#[wasm_bindgen(js_name = drawStrokeThenFill)]
pub fn draw_stroke_then_fill_js(
    vec_obj: WasmVectorObject,
    t: f64,
    default_stroke_width: Option<f64>,
) -> WasmVectorObject {
    let vec_obj = vec_obj.native_vec_features;
    let new_vec_obj = draw_stroke_then_fill(vec_obj, t, default_stroke_width);
    return WasmVectorObject { native_vec_features: new_vec_obj };
}


#[wasm_bindgen(js_name = drawStrokeThenFill3D)]
pub fn draw_stroke_then_fill_3d_js(
    obj_3d: WasmThreeDObject,
    t: f64,
    default_stroke_width: Option<f64>,
) -> WasmThreeDObject {
    let obj_3d = obj_3d.three_d_object;
    WasmThreeDObject { three_d_object: draw_stroke_then_fill_3d(obj_3d, t, default_stroke_width) }
}


#[wasm_bindgen(js_name = fadeIn)]
pub fn fade_in_js(
    vec_obj: WasmVectorObject,
    scale_factor: f64,
    shift: Array,
    t: f64
) -> WasmVectorObject {
    return WasmVectorObject { native_vec_features: fade_in(vec_obj.native_vec_features, scale_factor, (shift.get(0).as_f64().unwrap(), shift.get(1).as_f64().unwrap()), t) };
}


#[wasm_bindgen(js_name = fadeIn3D)]
pub fn fade_in_3d_js(
    obj_3d: WasmThreeDObject,
    scale_factor: f64,
    shift: Array,
    t: f64
) -> WasmThreeDObject {
    return WasmThreeDObject { three_d_object: fade_in_3d(obj_3d.three_d_object, scale_factor, (shift.get(0).as_f64().unwrap(), shift.get(1).as_f64().unwrap(), shift.get(2).as_f64().unwrap()), t) };
}


#[wasm_bindgen(js_name = fadeOut)]
pub fn fade_out_js(
    vec_obj: WasmVectorObject,
    scale_factor: f64,
    shift: Array,
    t: f64
) -> WasmVectorObject {
    return WasmVectorObject { native_vec_features: fade_out(vec_obj.native_vec_features, scale_factor, (shift.get(0).as_f64().unwrap(), shift.get(1).as_f64().unwrap()), t) };
}


#[wasm_bindgen(js_name = fadeOut3D)]
pub fn fade_out_3d_js(
    obj_3d: WasmThreeDObject,
    scale_factor: f64,
    shift: Array,
    t: f64
) -> WasmThreeDObject {
    return WasmThreeDObject { three_d_object: fade_out_3d(obj_3d.three_d_object, scale_factor, (shift.get(0).as_f64().unwrap(), shift.get(1).as_f64().unwrap(), shift.get(2).as_f64().unwrap()), t) };
}


#[wasm_bindgen(js_name = growArrowWithFinalTip)]
pub fn grow_arrow_with_final_tip_js(
    vec_obj: WasmVectorObject,
    t: f64,
) -> WasmVectorObject {
    let vec_obj = vec_obj.native_vec_features;
    let new_vec_obj = grow_arrow_with_final_tip(vec_obj, t);
    return WasmVectorObject { native_vec_features: new_vec_obj };
}


#[wasm_bindgen(js_name = morphShape3D)]
pub fn morph_shape_3d_js(
    original: WasmThreeDObject,
    target: WasmThreeDObject,
    t: f64
) -> WasmThreeDObject {
    return WasmThreeDObject { three_d_object: morph_shape_3d(original.three_d_object, target.three_d_object, t) };
}


#[wasm_bindgen(js_name = rotateXAnimation3D)]
pub fn rotate_x_animation_3d_js(
    obj_3d: WasmThreeDObject,
    angle: f64,
    t: f64
) -> WasmThreeDObject {
    return WasmThreeDObject { three_d_object: rotate_x_animation_3d(obj_3d.three_d_object, angle, t) };
}


#[wasm_bindgen(js_name = rotateYAnimation3D)]
pub fn rotate_y_animation_3d_js(
    obj_3d: WasmThreeDObject,
    angle: f64,
    t: f64
) -> WasmThreeDObject {
    return WasmThreeDObject { three_d_object: rotate_y_animation_3d(obj_3d.three_d_object, angle, t) };
}


#[wasm_bindgen(js_name = rotateZAnimation3D)]
pub fn rotate_z_animation_3d_js(
    obj_3d: WasmThreeDObject,
    angle: f64,
    t: f64
) -> WasmThreeDObject {
    return WasmThreeDObject { three_d_object: rotate_z_animation_3d(obj_3d.three_d_object, angle, t) };
}


#[wasm_bindgen(js_name = growArrowWithInitialTip)]
pub fn grow_arrow_with_initial_tip_js(
    vec_obj: WasmVectorObject,
    t: f64,
) -> WasmVectorObject {
    let vec_obj = vec_obj.native_vec_features;
    let new_vec_obj = grow_arrow_with_initial_tip(vec_obj, t);
    return WasmVectorObject { native_vec_features: new_vec_obj };
}


#[wasm_bindgen(js_name = growArrowWithTipsAtBothEnds)]
pub fn grow_arrow_with_tips_at_both_ends_js(
    vec_obj: WasmVectorObject,
    t: f64,
) -> WasmVectorObject {
    let vec_obj = vec_obj.native_vec_features;
    let new_vec_obj = grow_arrow_with_tips_at_both_ends(vec_obj, t);
    return WasmVectorObject { native_vec_features: new_vec_obj };
}


#[wasm_bindgen(js_name = growFromCenter)]
pub fn grow_from_center_js(
    vec_obj: WasmVectorObject,
    t: f64,
) -> WasmVectorObject {
    let vec_obj = vec_obj.native_vec_features;
    let new_vec_obj = grow_from_center(vec_obj, t);
    return WasmVectorObject { native_vec_features: new_vec_obj };
}


#[wasm_bindgen(js_name = growFromCenter3D)]
pub fn grow_from_center_3d_js(
    obj_3d: WasmThreeDObject,
    t: f64,
) -> WasmThreeDObject {
    let obj_3d = obj_3d.three_d_object;
    WasmThreeDObject { three_d_object: grow_from_center_3d(obj_3d, t) }
}


#[wasm_bindgen(js_name = morphShape)]
pub fn morph_shape_js(
    original: WasmVectorObject,
    target: WasmVectorObject,
    t: f64
) -> WasmVectorObject {
    return WasmVectorObject { native_vec_features: morph_shape(original.native_vec_features, target.native_vec_features, t) };
}


#[wasm_bindgen(js_name = moveCameraSVG)]
pub fn move_camera_svg_js(
    top_left_corner: Array,
    bottom_right_corner: Array,
    scene: &mut SVGScene,
    t: f64
) {
    let top_left_corner = (top_left_corner.get(0).as_f64().unwrap(), top_left_corner.get(1).as_f64().unwrap());
    let bottom_right_corner = (bottom_right_corner.get(0).as_f64().unwrap(), bottom_right_corner.get(1).as_f64().unwrap());
    move_camera_svg(top_left_corner, bottom_right_corner, scene, t);
}


#[wasm_bindgen(js_name = moveCamera)]
pub fn move_camera_js(
    top_left_corner: Array,
    bottom_right_corner: Array,
    scene: &mut Scene,
    t: f64
) {
    let top_left_corner = (top_left_corner.get(0).as_f64().unwrap(), top_left_corner.get(1).as_f64().unwrap());
    let bottom_right_corner = (bottom_right_corner.get(0).as_f64().unwrap(), bottom_right_corner.get(1).as_f64().unwrap());
    move_camera(top_left_corner, bottom_right_corner, scene, t);
}


#[wasm_bindgen(js_name = rotateAnimation)]
pub fn rotate_animation_js(
    vec_obj: WasmVectorObject,
    angle: f64,
    t: f64
) -> WasmVectorObject {
    return WasmVectorObject { native_vec_features: rotate_animation(vec_obj.native_vec_features, angle, t) };
}


#[wasm_bindgen(js_name = scaleInPlace)]
pub fn scale_in_place_js(
    vec_obj: WasmVectorObject,
    scale_factor: f64,
    t: f64
) -> WasmVectorObject {
    return WasmVectorObject { native_vec_features: scale_in_place(vec_obj.native_vec_features, scale_factor, t) };
}


#[wasm_bindgen(js_name = scaleInPlace3D)]
pub fn scale_in_place_3d_js(
    obj_3d: WasmThreeDObject,
    scale_factor: f64,
    t: f64
) -> WasmThreeDObject {
    return WasmThreeDObject { three_d_object: scale_in_place_3d(obj_3d.three_d_object, scale_factor, t) };
}


#[wasm_bindgen(js_name = setFillAnimation)]
pub fn set_fill_animation_js(
    vec_obj: WasmVectorObject,
    target_fill: WasmColor,
    t: f64
) -> WasmVectorObject {
    let target_fill = (target_fill.color.red, target_fill.color.green, target_fill.color.blue, target_fill.color.alpha);
    return WasmVectorObject { native_vec_features: set_fill_animation(vec_obj.native_vec_features, target_fill, t) };
}


#[wasm_bindgen(js_name = setFillAnimation3D)]
pub fn set_fill_animation_3d_js(
    obj_3d: WasmThreeDObject,
    target_fill: WasmColor,
    t: f64
) -> WasmThreeDObject {
    let target_fill = (target_fill.color.red, target_fill.color.green, target_fill.color.blue, target_fill.color.alpha);
    return WasmThreeDObject { three_d_object: set_fill_animation_3d(obj_3d.three_d_object, target_fill, t) };
}


#[wasm_bindgen(js_name = setStrokeAnimation)]
pub fn set_stroke_animation_js(
    vec_obj: WasmVectorObject,
    target_stroke: WasmColor,
    t: f64
) -> WasmVectorObject {
    let target_stroke = (target_stroke.color.red, target_stroke.color.green, target_stroke.color.blue, target_stroke.color.alpha);
    return WasmVectorObject { native_vec_features: set_stroke_animation(vec_obj.native_vec_features, target_stroke, t) };
}


#[wasm_bindgen(js_name = setStrokeAnimation3D)]
pub fn set_stroke_animation_3d_js(
    obj_3d: WasmThreeDObject,
    target_stroke: WasmColor,
    t: f64
) -> WasmThreeDObject {
    let target_stroke = (target_stroke.color.red, target_stroke.color.green, target_stroke.color.blue, target_stroke.color.alpha);
    return WasmThreeDObject { three_d_object: set_stroke_animation_3d(obj_3d.three_d_object, target_stroke, t) };
}


#[wasm_bindgen(js_name = shiftAnimation)]
pub fn shift_animation_js(
    vec_obj: WasmVectorObject,
    shift: Array,
    t: f64
) -> WasmVectorObject {
    return WasmVectorObject { native_vec_features: shift_animation(vec_obj.native_vec_features, (shift.get(0).as_f64().unwrap(), shift.get(1).as_f64().unwrap()), t) };
}


#[wasm_bindgen(js_name = shiftAnimation3D)]
pub fn shift_animation_3d_js(
    obj_3d: WasmThreeDObject,
    shift: Array,
    t: f64
) -> WasmThreeDObject {
    return WasmThreeDObject { three_d_object: shift_animation_3d(obj_3d.three_d_object, (shift.get(0).as_f64().unwrap(), shift.get(1).as_f64().unwrap(), shift.get(2).as_f64().unwrap()), t) };
}


#[wasm_bindgen(js_name = showTemporaily)]
pub fn show_temporarily_js(
    vec_obj: WasmVectorObject,
    t: f64,
) -> WasmVectorObject {
    let vec_obj = vec_obj.native_vec_features;
    let new_vec_obj = show_temporarily(vec_obj, t);
    return WasmVectorObject { native_vec_features: new_vec_obj };
}


#[wasm_bindgen(js_name = spinningGrow)]
pub fn spinning_grow_js(
    vec_obj: WasmVectorObject,
    angle: f64,
    t: f64
) -> WasmVectorObject {
    return WasmVectorObject { native_vec_features: spinning_grow(vec_obj.native_vec_features, angle, t) };
}