use js_sys::{Array, Function};
use wasm_bindgen::prelude::*;

use crate::objects::{vector_object::VectorFeatures, wasm_interface::{WasmColor, WasmVectorObject}};
use crate::{scene::Scene, svg_scene::SVGScene, animations::{move_camera_svg::move_camera_svg, move_camera::move_camera}};

use super::{animation_group::{animation_group, make_timings}, create::create, draw_stroke_then_fill::{draw_stroke_then_fill, write}, fade::{fade_in, fade_out}, grow_arrow::{grow_arrow_with_final_tip, grow_arrow_with_initial_tip, grow_arrow_with_tips_at_both_ends}, grow_from_center::grow_from_center, morph_shape::morph_shape, rotate_animation::rotate_animation, scale_in_place::scale_in_place, set_fill_animation::set_fill_animation, set_stroke_animation::set_stroke_animation, shift_animation::shift_animation, shift_image_position::shift_image_position, show_temporarily::show_temporarily, spinning_grow::spinning_grow};

#[wasm_bindgen(js_name = makeTimings)]
pub fn make_timings_js(
    num_anim_funcs: usize,
    lag_ratio: f64,
) -> Vec<f64> {
    make_timings(num_anim_funcs, lag_ratio)
}

#[wasm_bindgen(js_name = animationGroup)]
pub fn animation_group_js(
    anim_funcs: Vec<Function>,
    lag_ratio: f64,
) -> Function {
    let result = Closure::wrap(
        Box::new(move |vec_obj: WasmVectorObject, t: f64| -> WasmVectorObject {
            let vec_obj = vec_obj.native_vec_features;
            let anim_funcs = anim_funcs.iter().map(|func| {
                let func = func.clone();
                let result = Box::new(move |vec_obj: VectorFeatures, t: f64| -> VectorFeatures {
                    let vec_obj = WasmVectorObject { native_vec_features: vec_obj };
                    let vec_obj = func.call2(&JsValue::NULL, &JsValue::from(vec_obj), &JsValue::from(t)).unwrap();
                    let vec_obj = vec_obj.unchecked_into::<WasmVectorObject>();
                    return vec_obj.native_vec_features;
                }) as Box<dyn Fn(VectorFeatures, f64) -> VectorFeatures>;
                return result;
            }).collect::<Vec<Box<dyn Fn(VectorFeatures, f64) -> VectorFeatures>>>();
            let new_vec_obj = animation_group(anim_funcs, lag_ratio)(vec_obj, t);
            return WasmVectorObject { native_vec_features: new_vec_obj };
        }) as Box<dyn Fn(WasmVectorObject, f64) -> WasmVectorObject>
    );
    let func = Function::from(result.as_ref().clone());
    result.forget();
    return func;
}


#[wasm_bindgen(js_name = create)]
pub fn create_js(
    vec_obj: WasmVectorObject,
    t: f64,
) -> WasmVectorObject {
    let vec_obj = vec_obj.native_vec_features;
    let new_vec_obj = create(vec_obj, t);
    return WasmVectorObject { native_vec_features: new_vec_obj };
}


#[wasm_bindgen(js_name = drawStrokeThenFill)]
pub fn draw_stroke_then_fill_js(
    vec_obj: WasmVectorObject,
    t: f64,
) -> WasmVectorObject {
    let vec_obj = vec_obj.native_vec_features;
    let new_vec_obj = draw_stroke_then_fill(vec_obj, t);
    return WasmVectorObject { native_vec_features: new_vec_obj };
}


#[wasm_bindgen(js_name = write)]
pub fn write_js(
    number_of_objects: usize,
    lag_ratio: f64,
) -> Function {
    let result = write(number_of_objects, lag_ratio);
    let result = Closure::wrap(
        Box::new(move |vec_obj: WasmVectorObject, t: f64| -> WasmVectorObject {
            return WasmVectorObject { native_vec_features: result(vec_obj.native_vec_features, t) };
        }) as Box<dyn Fn(WasmVectorObject, f64) -> WasmVectorObject>
    );
    let func = Function::from(result.as_ref().clone());
    result.forget();
    return func;
}


#[wasm_bindgen(js_name = fadeIn)]
pub fn fade_in_js(
    scale_factor: f64,
    shift: Array
) -> Function {
    let result = fade_in(scale_factor, (shift.get(0).as_f64().unwrap(), shift.get(1).as_f64().unwrap()));
    let result = Closure::wrap(
        Box::new(move |vec_obj: WasmVectorObject, t: f64| -> WasmVectorObject {
            return WasmVectorObject { native_vec_features: result(vec_obj.native_vec_features, t) };
        }) as Box<dyn Fn(WasmVectorObject, f64) -> WasmVectorObject>
    );
    let func = Function::from(result.as_ref().clone());
    result.forget();
    return func;
}


#[wasm_bindgen(js_name = fadeOut)]
pub fn fade_out_js(
    scale_factor: f64,
    shift: Array
) -> Function {
    let result = fade_out(scale_factor, (shift.get(0).as_f64().unwrap(), shift.get(1).as_f64().unwrap()));
    let result = Closure::wrap(
        Box::new(move |vec_obj: WasmVectorObject, t: f64| -> WasmVectorObject {
            return WasmVectorObject { native_vec_features: result(vec_obj.native_vec_features, t) };
        }) as Box<dyn Fn(WasmVectorObject, f64) -> WasmVectorObject>
    );
    let func = Function::from(result.as_ref().clone());
    result.forget();
    return func;
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


#[wasm_bindgen(js_name = morphShape)]
pub fn morph_shape_js(
    target: WasmVectorObject,
) -> Function {
    let target = target.native_vec_features;
    let result = move |vec_obj: WasmVectorObject, t: f64| -> WasmVectorObject {
        let vec_obj = vec_obj.native_vec_features;
        let new_vec_obj = morph_shape(target.clone())(vec_obj, t);
        return WasmVectorObject { native_vec_features: new_vec_obj };
    };
    let result = Closure::wrap(
        Box::new(result) as Box<dyn Fn(WasmVectorObject, f64) -> WasmVectorObject>
    );
    let func = Function::from(result.as_ref().clone());
    result.forget();
    return func;
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
    angle: f64,
) -> Function {
    let result = rotate_animation(angle);
    let result = Closure::wrap(
        Box::new(move |vec_obj: WasmVectorObject, t: f64| -> WasmVectorObject {
            return WasmVectorObject { native_vec_features: result(vec_obj.native_vec_features, t) };
        }) as Box<dyn Fn(WasmVectorObject, f64) -> WasmVectorObject>
    );
    let func = Function::from(result.as_ref().clone());
    result.forget();
    return func;
}


#[wasm_bindgen(js_name = scaleInPlace)]
pub fn scale_in_place_js(
    scale_factor: f64
) -> Function {
    let result = scale_in_place(scale_factor);
    let result = Closure::wrap(
        Box::new(move |vec_obj: WasmVectorObject, t: f64| -> WasmVectorObject {
            return WasmVectorObject { native_vec_features: result(vec_obj.native_vec_features, t) };
        }) as Box<dyn Fn(WasmVectorObject, f64) -> WasmVectorObject>
    );
    let func = Function::from(result.as_ref().clone());
    result.forget();
    return func;
}


#[wasm_bindgen(js_name = setFillAnimation)]
pub fn set_fill_animation_js(
    target_fill: WasmColor
) -> Function {
    let target_fill = (target_fill.color.red, target_fill.color.green, target_fill.color.blue, target_fill.color.alpha);
    let result = set_fill_animation(target_fill);
    let result = Closure::wrap(
        Box::new(move |vec_obj: WasmVectorObject, t: f64| -> WasmVectorObject {
            return WasmVectorObject { native_vec_features: result(vec_obj.native_vec_features, t) };
        }) as Box<dyn Fn(WasmVectorObject, f64) -> WasmVectorObject>
    );
    let func = Function::from(result.as_ref().clone());
    result.forget();
    return func;
}


#[wasm_bindgen(js_name = setStrokeAnimation)]
pub fn set_stroke_animation_js(
    target_stroke: WasmColor
) -> Function {
    let target_stroke = (target_stroke.color.red, target_stroke.color.green, target_stroke.color.blue, target_stroke.color.alpha);
    let result = set_stroke_animation(target_stroke);
    let result = Closure::wrap(
        Box::new(move |vec_obj: WasmVectorObject, t: f64| -> WasmVectorObject {
            return WasmVectorObject { native_vec_features: result(vec_obj.native_vec_features, t) };
        }) as Box<dyn Fn(WasmVectorObject, f64) -> WasmVectorObject>
    );
    let func = Function::from(result.as_ref().clone());
    result.forget();
    return func;
}


#[wasm_bindgen(js_name = shiftAnimation)]
pub fn shift_animation_js(
    shift: Array
) -> Function {
    let shift = (shift.get(0).as_f64().unwrap(), shift.get(1).as_f64().unwrap());
    let result = shift_animation(shift);
    let result = Closure::wrap(
        Box::new(move |vec_obj: WasmVectorObject, t: f64| -> WasmVectorObject {
            return WasmVectorObject { native_vec_features: result(vec_obj.native_vec_features, t) };
        }) as Box<dyn Fn(WasmVectorObject, f64) -> WasmVectorObject>
    );
    let func = Function::from(result.as_ref().clone());
    result.forget();
    return func;
}


#[wasm_bindgen(js_name = shiftImagePosition)]
pub fn shift_image_position_js(
    new_position: Array
) -> Function {
    let new_position = (new_position.get(0).as_f64().unwrap(), new_position.get(1).as_f64().unwrap());
    let result = shift_image_position(new_position);
    let result = Closure::wrap(
        Box::new(move |vec_obj: WasmVectorObject, t: f64| -> WasmVectorObject {
            return WasmVectorObject { native_vec_features: result(vec_obj.native_vec_features, t) };
        }) as Box<dyn Fn(WasmVectorObject, f64) -> WasmVectorObject>
    );
    let func = Function::from(result.as_ref().clone());
    result.forget();
    return func;
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
    angle: f64
) -> Function {
    let result = spinning_grow(angle);
    let result = Closure::wrap(
        Box::new(move |vec_obj: WasmVectorObject, t: f64| -> WasmVectorObject {
            return WasmVectorObject { native_vec_features: result(vec_obj.native_vec_features, t) };
        }) as Box<dyn Fn(WasmVectorObject, f64) -> WasmVectorObject>
    );
    let func = Function::from(result.as_ref().clone());
    result.forget();
    return func;
}