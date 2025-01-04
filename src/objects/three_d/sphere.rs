use std::f64::consts::PI;

use js_sys::{Array, Promise};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::future_to_promise;

use crate::colors::Color;

use super::three_d_object::ThreeDObject;

pub async fn sphere(
    center: (f64, f64, f64),
    radius: f64,
    u_segments: usize,
    v_segments: usize,
    fill_colors: Vec<Color>,
    stroke_colors: Vec<Color>,
    stroke_width: f64,
    index: Option<usize>
) -> ThreeDObject {
    return ThreeDObject::from_uv_function(
        Box::leak(Box::new(Closure::wrap(Box::new(move |u: f64, v: f64| {
            future_to_promise(async move {
                let x = center.0 + radius * u.cos() * v.sin();
                let y = center.1 + radius * u.sin() * v.sin();
                let z = center.2 + radius * v.cos();
                return Ok(JsValue::from(Array::of3(&JsValue::from_f64(x), &JsValue::from_f64(y), &JsValue::from_f64(z))));
            })
        }) as Box<dyn Fn(f64, f64) -> Promise>).into_js_value().dyn_into().unwrap())),
        (0.001, PI - 0.001),
        (0.0, 2.0 * PI),
        u_segments,
        v_segments,
        fill_colors,
        stroke_colors,
        stroke_width,
        index
    ).await;
}