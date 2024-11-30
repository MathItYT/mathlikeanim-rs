use std::{collections::HashMap, future::Future};

use js_sys::{Array, Function, Map, Promise};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::{colors::GradientImageOrColor, objects::{vector_object::VectorObject, wasm_interface::WasmVectorObject}};

pub trait SceneAPI {
    fn new(width: u32, height: u32, fps: u32) -> Self;
    fn clear(&mut self);
    fn restore(&mut self, n: usize);
    fn save_state(&mut self, n: usize);
    fn set_corners(&mut self, top_left_corner: (f64, f64), bottom_right_corner: (f64, f64));
    fn get_top_left_corner(&self) -> (f64, f64);
    fn get_bottom_right_corner(&self) -> (f64, f64);
    fn set_background(&mut self, background: GradientImageOrColor);
    fn add(&mut self, vec_obj: VectorObject);
    fn remove(&mut self, index: usize);
    fn get_fps(&self) -> &u32;
    fn get_height(&self) -> &u32;
    fn get_width(&self) -> &u32;
    fn play(
        &mut self,
        animation_func: Function,
        object_indices: Vec<usize>,
        duration_in_frames: u32,
        rate_func: impl Fn(f64) -> f64
    ) -> impl Future<Output = ()> {
        async move {
            let objects = self.get_objects_from_indices(&object_indices);
            let objects_map = Map::new();
            for (index, object) in objects.iter() {
                objects_map.set(&JsValue::from_f64(*index as f64), &JsValue::from(WasmVectorObject { native_vec_features: object.clone() }));
            }
            for frame in 0..=duration_in_frames {
                let progress = rate_func(frame as f64 / duration_in_frames as f64);
                let promise = animation_func.call2(&JsValue::NULL, &objects_map, &JsValue::from_f64(progress)).unwrap();
                let new_objects_map = JsFuture::from(Promise::resolve(&promise)).await.unwrap().dyn_into::<Map>().unwrap();
                for index in object_indices.iter() {
                    let wasm_object = new_objects_map.get(&JsValue::from_f64(*index as f64)).dyn_into::<WasmVectorObject>().unwrap();
                    self.add(wasm_object.native_vec_features);
                }
                if frame < duration_in_frames {
                    self.render_frame().await;
                    self.sleep((1000 / self.get_fps()) as i32).await;
                }
            }
        }
    }
    fn make_frame(
        &mut self,
        animation_func: &Function,
        objects: &Array,
        t: f64
    ) -> impl Future<Output = ()> {
        async move {
            let promise = animation_func.call2(
                &JsValue::NULL,
                &JsValue::from(objects),
                &JsValue::from_f64(t)
            ).unwrap().dyn_into::<Promise>().unwrap();
            let result = JsFuture::from(promise).await.unwrap();
            let new_objects = Array::from(&result).iter().map(|obj| {
                let obj = obj.dyn_into::<WasmVectorObject>().unwrap();
                obj.native_vec_features
            }).collect::<Vec<VectorObject>>();
            for obj in new_objects {
                self.add(obj);
            }
        }
    }
    fn wait(&mut self, duration_in_frames: u32) -> impl Future<Output = ()> {
        async move {
            for _ in 0..duration_in_frames {
                self.render_frame().await;
                self.sleep((1000 / self.get_fps()) as i32).await;
            }
        }
    }
    fn sleep(&mut self, duration_in_ms: i32) -> impl Future<Output = ()>;
    fn render_frame(&mut self) -> impl Future<Output = ()>;
    fn get_objects_from_indices(&self, object_indices: &Vec<usize>) -> HashMap<usize, VectorObject>;
}
