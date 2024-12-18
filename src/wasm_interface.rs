use js_sys::{Array, Function};
use wasm_bindgen::prelude::*;
use crate::{colors::Color, objects::wasm_interface::{WasmColor, WasmVectorObject}, utils::{add_n_more_subobjects, align_data, align_points, align_subobjects, bezier, bezier_f64, center, choose, consider_points_equals, distance_squared, double_smooth, ease_in_back, ease_in_circ, ease_in_cubic, ease_in_elastic, ease_in_expo, ease_in_out_back, ease_in_out_bounce, ease_in_out_circ, ease_in_out_cubic, ease_in_out_elastic, ease_in_out_expo, ease_in_out_quad, ease_in_out_quart, ease_in_out_quint, ease_in_out_sine, ease_in_quad, ease_in_quart, ease_in_quint, ease_in_sine, ease_out_bounce, ease_out_circ, ease_out_cubic, ease_out_elastic, ease_out_expo, ease_out_quad, ease_out_quart, ease_out_quint, ease_out_sine, elliptical_arc_path, exponential_decay, factorial, get_bbox, get_nth_subpath, has_new_path_begun, hex_to_color, insert_n_curves_to_point_list, integer_interpolate, interp, interpolate, interpolate_color, interpolate_tuple, interpolate_tuple_3d, line_as_cubic_bezier, linear, lingering, not_quite_there, null_point_align, permutation, points_from_anchors_and_handles, quadratic_bezier_as_cubic_bezier, radian, running_start, rush_from, rush_into, sigmoid, sleep, slow_into, smooth, smoothererstep, smootherstep, smoothstep, squish_rate_func, start_new_path, there_and_back, there_and_back_with_pause, wiggle}};

#[wasm_bindgen(js_name = radian)]
pub fn radian_js(ux: f64, uy: f64, vx: f64, vy: f64) -> f64 {
    return radian(ux, uy, vx, vy);
}


#[wasm_bindgen(js_name = interp)]
pub fn interp_js(x: f64, xp: &Array, fp: &Array) -> f64 {
    return interp(x, &xp.iter().map(|x| x.as_f64().unwrap()).collect::<Vec<f64>>(), &fp.iter().map(|x| x.as_f64().unwrap()).collect::<Vec<f64>>());
}


#[wasm_bindgen(js_name = sleep)]
pub async fn sleep_js(ms: i32) {
    sleep(ms).await;
}


#[wasm_bindgen(js_name = ellipticalArcPath)]
pub fn elliptical_arc_path_js(
    last_move: Array,
    rx: f64,
    ry: f64,
    rotation: f64,
    large_arc: bool,
    sweep: bool,
    x: f64,
    y: f64
) -> Array {
    let result = elliptical_arc_path(
        (last_move.get(0).as_f64().unwrap(), last_move.get(1).as_f64().unwrap()),
        rx,
        ry,
        rotation,
        large_arc,
        sweep,
        x,
        y
    );
    return Array::from(
        &JsValue::from(result.iter().map(|point| {
            return Array::of2(&JsValue::from_f64(point.0), &JsValue::from_f64(point.1));
        }).collect::<Array>()),
    );
}


#[wasm_bindgen(js_name = getBbox)]
pub fn get_bbox_js(points: Array) -> Array {
    let points = points.iter().map(|point| {
        let point = point.dyn_into::<Array>().unwrap();
        return (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    }).collect();
    let result = get_bbox(&points);
    return Array::from(
        &JsValue::from(Array::of2(
            &Array::of2(&JsValue::from_f64(result.0.0), &JsValue::from_f64(result.0.1)),
            &Array::of2(&JsValue::from_f64(result.1.0), &JsValue::from_f64(result.1.1)),
        )),
    );
}


#[wasm_bindgen(js_name = center)]
pub fn center_js(points: Array, center_if_no_points: Array) -> Array {
    let points = points.iter().map(|point| {
        let point = point.dyn_into::<Array>().unwrap();
        return (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    }).collect();
    let result = center(&points, (center_if_no_points.get(0).as_f64().unwrap(), center_if_no_points.get(1).as_f64().unwrap()));
    return Array::of2(&JsValue::from_f64(result.0), &JsValue::from_f64(result.1));
}


#[wasm_bindgen(js_name = factorial)]
pub fn factorial_js(n: u64) -> u64 {
    return factorial(n);
}


#[wasm_bindgen(js_name = hexToColor)]
pub fn hex_to_color_js(hex: String, a: f64) -> WasmColor {
    return WasmColor { color: hex_to_color(hex.as_str(), a) }
}


#[wasm_bindgen(js_name = bezier)]
pub fn bezier_js(points: Array, t: f64) -> Array {
    let points = points.iter().map(|point| {
        let point = point.dyn_into::<Array>().unwrap();
        return (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    }).collect();
    let result = bezier(&points, t);
    return Array::of2(&JsValue::from_f64(result.0), &JsValue::from_f64(result.1));
}


#[wasm_bindgen(js_name = bezierNumber)]
pub fn bezier_number_js(numbers: Array, t: f64) -> f64 {
    let numbers = numbers.iter().map(|number| {
        return number.as_f64().unwrap();
    }).collect();
    let result = bezier_f64(numbers, t);
    return result;
}


#[wasm_bindgen(js_name = permutation)]
pub fn permutation_js(n: u64, r: u64) -> u64 {
    return permutation(n, r);
}


#[wasm_bindgen(js_name = choose)]
pub fn choose_js(n: u64, r: u64) -> u64 {
    return choose(n, r);
}


#[wasm_bindgen(js_name = distanceSquared)]
pub fn distance_squared_js(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    return distance_squared((x1, y1), (x2, y2));
}


#[wasm_bindgen(js_name = interpolate)]
pub fn interpolate_js(x: f64, y: f64, t: f64) -> f64 {
    return interpolate(x, y, t);
}


#[wasm_bindgen(js_name = interpolateTuple)]
pub fn interpolate_tuple_js(x: Array, y: Array, t: f64) -> Array {
    let x = (x.get(0).as_f64().unwrap(), x.get(1).as_f64().unwrap());
    let y = (y.get(0).as_f64().unwrap(), y.get(1).as_f64().unwrap());
    let result = interpolate_tuple(x, y, t);
    return Array::of2(&JsValue::from_f64(result.0), &JsValue::from_f64(result.1));
}


#[wasm_bindgen(js_name = interpolateTuple3D)]
pub fn interpolate_tuple_3d_js(x: Array, y: Array, t: f64) -> Array {
    let x = (x.get(0).as_f64().unwrap(), x.get(1).as_f64().unwrap(), x.get(2).as_f64().unwrap());
    let y = (y.get(0).as_f64().unwrap(), y.get(1).as_f64().unwrap(), y.get(2).as_f64().unwrap());
    let result = interpolate_tuple_3d(x, y, t);
    return Array::of3(&JsValue::from_f64(result.0), &JsValue::from_f64(result.1), &JsValue::from_f64(result.2));
}


#[wasm_bindgen(js_name = interpolateColor)]
pub fn interpolate_color_js(x: WasmColor, y: WasmColor, t: f64) -> WasmColor {
    let result = interpolate_color(
        (x.get_r(), x.get_g(), x.get_b(), x.get_a()),
        (y.get_r(), y.get_g(), y.get_b(), y.get_a()),
        t
    );
    return WasmColor { color: Color { red: result.0, blue: result.1, green: result.2, alpha: result.3 } };
}


#[wasm_bindgen(js_name = pointsFromAnchorsAndHandles)]
pub fn points_from_anchors_and_handles_js(
    anchors1: Array,
    handles1: Array,
    handles2: Array,
    anchors2: Array,
) -> Array {
    let anchors1 = anchors1.iter().map(|point| {
        let point = point.dyn_into::<Array>().unwrap();
        return (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    }).collect();
    let handles1 = handles1.iter().map(|point| {
        let point = point.dyn_into::<Array>().unwrap();
        return (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    }).collect();
    let handles2 = handles2.iter().map(|point| {
        let point = point.dyn_into::<Array>().unwrap();
        return (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    }).collect();
    let anchors2 = anchors2.iter().map(|point| {
        let point = point.dyn_into::<Array>().unwrap();
        return (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    }).collect();
    let result = points_from_anchors_and_handles(anchors1, handles1, handles2, anchors2);
    return result.iter().map(|point| {
        return Array::of2(&JsValue::from_f64(point.0), &JsValue::from_f64(point.1));
    }).collect();
}


#[wasm_bindgen(js_name = startNewPath)]
pub fn start_new_path_js(points: Array, point: Array) -> Array {
    let points = points.iter().map(|point| {
        let point = point.dyn_into::<Array>().unwrap();
        return (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    }).collect::<Vec<(f64, f64)>>();
    let point = (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    let result = start_new_path(&points, point);
    return result.iter().map(|point| {
        return Array::of2(&JsValue::from_f64(point.0), &JsValue::from_f64(point.1));
    }).collect();
}


#[wasm_bindgen(js_name = hasNewPathBegun)]
pub fn has_new_path_begun_js(points: Array) -> bool {
    let points = points.iter().map(|point| {
        let point = point.dyn_into::<Array>().unwrap();
        return (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    }).collect();
    return has_new_path_begun(&points);
}


#[wasm_bindgen(js_name = getNthSubpath)]
pub fn get_nth_subpath_js(points: Array, n: usize) -> Array {
    let points = points.iter().map(|points| {
        let points = points.dyn_into::<Array>().unwrap();
        let points = points.iter().collect::<Vec<JsValue>>();
        let points = points.iter().map(|point| {
            let point = point.clone().dyn_into::<Array>().unwrap();
            return (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
        }).collect();
        return points;
    }).collect();
    let result = get_nth_subpath(&points, n);
    return result.iter().map(|point| {
        return Array::of2(&JsValue::from_f64(point.0), &JsValue::from_f64(point.1));
    }).collect();
}


#[wasm_bindgen(js_name = insertNCurvesToPointList)]
pub fn insert_n_curves_to_point_list_js(n: usize, points: Array) -> Array {
    let points = points.iter().map(|point| {
        let point = point.dyn_into::<Array>().unwrap();
        return (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    }).collect();
    let result = insert_n_curves_to_point_list(n, &points);
    return result.iter().map(|point| {
        return Array::of2(&JsValue::from_f64(point.0), &JsValue::from_f64(point.1));
    }).collect();
}


#[wasm_bindgen(js_name = nullPointAlign)]
pub fn null_point_align_js(
    vec_obj1: WasmVectorObject,
    vec_obj2: WasmVectorObject
) -> Array {
    let vec_obj1 = vec_obj1.native_vec_features;
    let vec_obj2 = vec_obj2.native_vec_features;
    let result = null_point_align(vec_obj1, vec_obj2);
    return Array::of2(
        &JsValue::from(WasmVectorObject { native_vec_features: result.0 }),
        &JsValue::from(WasmVectorObject { native_vec_features: result.1 })
    )
}


#[wasm_bindgen(js_name = alignPoints)]
pub fn align_points_js(points1: Array, points2: Array, center_if_no_points: Array) -> Array {
    let points1 = points1.iter().map(|point| {
        let point = point.dyn_into::<Array>().unwrap();
        return (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    }).collect();
    let points2 = points2.iter().map(|point| {
        let point = point.dyn_into::<Array>().unwrap();
        return (point.get(0).as_f64().unwrap(), point.get(1).as_f64().unwrap());
    }).collect();
    let center_if_no_points = (center_if_no_points.get(0).as_f64().unwrap(), center_if_no_points.get(1).as_f64().unwrap());
    let result = align_points(&points1, &points2, center_if_no_points);
    return Array::of2(
        &JsValue::from(result.0.iter().map(|point| {
            return Array::of2(&JsValue::from_f64(point.0), &JsValue::from_f64(point.1));
        }).collect::<Array>()),
        &JsValue::from(result.1.iter().map(|point| {
            return Array::of2(&JsValue::from_f64(point.0), &JsValue::from_f64(point.1));
        }).collect::<Array>())
    )
}


#[wasm_bindgen(js_name = addNMoreSubobjects)]
pub fn add_n_more_subobjects_js(
    vec_obj: WasmVectorObject,
    n: usize,
    center_if_no_points: Array
) -> WasmVectorObject {
    let result = add_n_more_subobjects(vec_obj.native_vec_features, n, (center_if_no_points.get(0).as_f64().unwrap(), center_if_no_points.get(1).as_f64().unwrap()));
    return WasmVectorObject { native_vec_features: result };
}


#[wasm_bindgen(js_name = alignSubobjects)]
pub fn align_subobjects_js(
    vec_obj1: WasmVectorObject,
    vec_obj2: WasmVectorObject,
    center_if_no_points: Array
) -> Vec<WasmVectorObject> {
    let result = align_subobjects(vec_obj1.native_vec_features, vec_obj2.native_vec_features, (center_if_no_points.get(0).as_f64().unwrap(), center_if_no_points.get(1).as_f64().unwrap()));
    return vec![
        WasmVectorObject { native_vec_features: result.0 },
        WasmVectorObject { native_vec_features: result.1 }
    ];
}


#[wasm_bindgen(js_name = alignData)]
pub fn align_data_js(
    vec_obj1: WasmVectorObject,
    vec_obj2: WasmVectorObject,
    skip_point_align: bool,
    center_if_no_points: Array
) -> Array {
    let vec_obj1 = vec_obj1.native_vec_features;
    let vec_obj2 = vec_obj2.native_vec_features;
    let result = align_data(vec_obj1, vec_obj2, skip_point_align, (center_if_no_points.get(0).as_f64().unwrap(), center_if_no_points.get(1).as_f64().unwrap()));
    return Array::of2(
        &JsValue::from(WasmVectorObject { native_vec_features: result.0 }),
        &JsValue::from(WasmVectorObject { native_vec_features: result.1 })
    )
}


#[wasm_bindgen(js_name = integerInterpolate)]
pub fn integer_interpolate_js(x: f64, y: f64, t: f64) -> Array {
    let result = integer_interpolate(x, y, t);
    return Array::of2(&JsValue::from_f64(result.0 as f64), &JsValue::from_f64(result.1));
}


#[wasm_bindgen(js_name = lineAsCubicBezier)]
pub fn line_as_cubic_bezier_js(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64
) -> Array {
    let result = line_as_cubic_bezier((x1, y1), (x2, y2));
    return Array::from(
        &JsValue::from(result.iter().map(|point| {
            return Array::of2(&JsValue::from_f64(point.0), &JsValue::from_f64(point.1));
        }).collect::<Array>()),
    );
}


#[wasm_bindgen(js_name = quadraticBezierAsCubicBezier)]
pub fn quadratic_bezier_as_cubic_bezier_js(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64
) -> Array {
    let result = quadratic_bezier_as_cubic_bezier((x1, y1), (x2, y2), (x3, y3));
    return Array::from(
        &JsValue::from(result.iter().map(|point| {
            return Array::of2(&JsValue::from_f64(point.0), &JsValue::from_f64(point.1));
        }).collect::<Array>()),
    );
}


#[wasm_bindgen(js_name = considerPointsEquals)]
pub fn consider_points_equals_js(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64
) -> bool {
    return consider_points_equals((x1, y1), (x2, y2))
}


#[wasm_bindgen(js_name = sigmoid)]
pub fn sigmoid_js(t: f64) -> f64 {
    return sigmoid(t);
}


#[wasm_bindgen(js_name = linear)]
pub fn linear_js(t: f64) -> f64 {
    return linear(t);
}


#[wasm_bindgen(js_name = smooth)]
pub fn smooth_js(t: f64, inflection: f64) -> f64 {
    return smooth(t, inflection);
}


#[wasm_bindgen(js_name = smoothstep)]
pub fn smoothstep_js(t: f64) -> f64 {
    return smoothstep(t);
}


#[wasm_bindgen(js_name = smootherstep)]
pub fn smootherstep_js(t: f64) -> f64 {
    return smootherstep(t);
}


#[wasm_bindgen(js_name = smoothererstep)]
pub fn smoothererstep_js(t: f64) -> f64 {
    return smoothererstep(t);
}


#[wasm_bindgen(js_name = rushInto)]
pub fn rush_into_js(t: f64, inflection: f64) -> f64 {
    return rush_into(t, inflection);
}


#[wasm_bindgen(js_name = rushFrom)]
pub fn rush_from_js(t: f64, inflection: f64) -> f64 {
    return rush_from(t, inflection);
}


#[wasm_bindgen(js_name = slowInto)]
pub fn slow_into_js(t: f64) -> f64 {
    return slow_into(t);
}


#[wasm_bindgen(js_name = doubleSmooth)]
pub fn double_smooth_js(t: f64) -> f64 {
    return double_smooth(t);
}


#[wasm_bindgen(js_name = thereAndBack)]
pub fn there_and_back_js(t: f64, inflection: f64) -> f64 {
    return there_and_back(t, inflection);
}


#[wasm_bindgen(js_name = thereAndBackWithPause)]
pub fn there_and_back_with_pause_js(t: f64, pause_ratio: f64) -> f64 {
    return there_and_back_with_pause(t, pause_ratio);
}


#[wasm_bindgen(js_name = runningStart)]
pub fn running_start_js(t: f64, pull_factor: f64) -> f64 {
    return running_start(t, pull_factor);
}


#[wasm_bindgen(js_name = notQuiteThere)]
pub fn not_quite_there_js(
    func: Function,
    t: f64,
    proportion: f64
) -> f64 {
    let function = |t: f64| -> f64 {
        let this = JsValue::NULL;
        let args = Array::of1(&JsValue::from_f64(t));
        return func.call1(&this, &args).unwrap().as_f64().unwrap();
    };
    return not_quite_there(function, t, proportion);
}


#[wasm_bindgen(js_name = wiggle)]
pub fn wiggle_js(
    t: f64,
    wiggles: f64
) -> f64 {
    return wiggle(t, wiggles);
}


#[wasm_bindgen(js_name = squishRateFunc)]
pub fn squish_rate_func_js(
    func: Function,
    t: f64,
    a: f64,
    b: f64
) -> f64 {
    let function = |t: f64| -> f64 {
        let this = JsValue::NULL;
        let args = Array::of1(&JsValue::from_f64(t));
        return func.call1(&this, &args).unwrap().as_f64().unwrap();
    };
    return squish_rate_func(function, t, a, b);
}


#[wasm_bindgen(js_name = lingering)]
pub fn lingering_js(t: f64) -> f64 {
    return lingering(t);
}


#[wasm_bindgen(js_name = exponentialDecay)]
pub fn exponential_decay_js(
    t: f64,
    half_life: f64
) -> f64 {
    return exponential_decay(t, half_life);
}


#[wasm_bindgen(js_name = easeInSine)]
pub fn ease_in_sine_js(t: f64) -> f64 {
    return ease_in_sine(t);
}


#[wasm_bindgen(js_name = easeOutSine)]
pub fn ease_out_sine_js(t: f64) -> f64 {
    return ease_out_sine(t);
}


#[wasm_bindgen(js_name = easeInOutSine)]
pub fn ease_in_out_sine_js(t: f64) -> f64 {
    return ease_in_out_sine(t);
}


#[wasm_bindgen(js_name = easeInQuad)]
pub fn ease_in_quad_js(t: f64) -> f64 {
    return ease_in_quad(t);
}


#[wasm_bindgen(js_name = easeOutQuad)]
pub fn ease_out_quad_js(t: f64) -> f64 {
    return ease_out_quad(t);
}


#[wasm_bindgen(js_name = easeInOutQuad)]
pub fn ease_in_out_quad_js(t: f64) -> f64 {
    return ease_in_out_quad(t);
}


#[wasm_bindgen(js_name = easeInCubic)]
pub fn ease_in_cubic_js(t: f64) -> f64 {
    return ease_in_cubic(t);
}


#[wasm_bindgen(js_name = easeOutCubic)]
pub fn ease_out_cubic_js(t: f64) -> f64 {
    return ease_out_cubic(t);
}


#[wasm_bindgen(js_name = easeInOutCubic)]
pub fn ease_in_out_cubic_js(t: f64) -> f64 {
    return ease_in_out_cubic(t);
}


#[wasm_bindgen(js_name = easeInQuart)]
pub fn ease_in_quart_js(t: f64) -> f64 {
    return ease_in_quart(t);
}


#[wasm_bindgen(js_name = easeOutQuart)]
pub fn ease_out_quart_js(t: f64) -> f64 {
    return ease_out_quart(t);
}


#[wasm_bindgen(js_name = easeInOutQuart)]
pub fn ease_in_out_quart_js(t: f64) -> f64 {
    return ease_in_out_quart(t);
}


#[wasm_bindgen(js_name = easeInQuint)]
pub fn ease_in_quint_js(t: f64) -> f64 {
    return ease_in_quint(t);
}


#[wasm_bindgen(js_name = easeOutQuint)]
pub fn ease_out_quint_js(t: f64) -> f64 {
    return ease_out_quint(t);
}


#[wasm_bindgen(js_name = easeInOutQuint)]
pub fn ease_in_out_quint_js(t: f64) -> f64 {
    return ease_in_out_quint(t);
}


#[wasm_bindgen(js_name = easeInExpo)]
pub fn ease_in_expo_js(t: f64) -> f64 {
    return ease_in_expo(t);
}


#[wasm_bindgen(js_name = easeOutExpo)]
pub fn ease_out_expo_js(t: f64) -> f64 {
    return ease_out_expo(t);
}


#[wasm_bindgen(js_name = easeInOutExpo)]
pub fn ease_in_out_expo_js(t: f64) -> f64 {
    return ease_in_out_expo(t);
}


#[wasm_bindgen(js_name = easeInCirc)]
pub fn ease_in_circ_js(t: f64) -> f64 {
    return ease_in_circ(t);
}


#[wasm_bindgen(js_name = easeOutCirc)]
pub fn ease_out_circ_js(t: f64) -> f64 {
    return ease_out_circ(t);
}


#[wasm_bindgen(js_name = easeInOutCirc)]
pub fn ease_in_out_circ_js(t: f64) -> f64 {
    return ease_in_out_circ(t);
}


#[wasm_bindgen(js_name = easeInBack)]
pub fn ease_in_back_js(t: f64) -> f64 {
    return ease_in_back(t);
}


#[wasm_bindgen(js_name = easeOutBack)]
pub fn ease_out_back_js(t: f64) -> f64 {
    return ease_in_back(t);
}


#[wasm_bindgen(js_name = easeInOutBack)]
pub fn ease_in_out_back_js(t: f64) -> f64 {
    return ease_in_out_back(t);
}


#[wasm_bindgen(js_name = easeInElastic)]
pub fn ease_in_elastic_js(t: f64) -> f64 {
    return ease_in_elastic(t);
}


#[wasm_bindgen(js_name = easeOutElastic)]
pub fn ease_out_elastic_js(t: f64) -> f64 {
    return ease_out_elastic(t);
}


#[wasm_bindgen(js_name = easeInOutElastic)]
pub fn ease_in_out_elastic_js(t: f64) -> f64 {
    return ease_in_out_elastic(t);
}


#[wasm_bindgen(js_name = easeOutBounce)]
pub fn ease_out_bounce_js(t: f64) -> f64 {
    return ease_out_bounce(t);
}


#[wasm_bindgen(js_name = easeInBounce)]
pub fn ease_in_bounce_js(t: f64) -> f64 {
    return ease_out_bounce(t);
}


#[wasm_bindgen(js_name = easeInOutBounce)]
pub fn ease_in_out_bounce_js(t: f64) -> f64 {
    return ease_in_out_bounce(t);
}
