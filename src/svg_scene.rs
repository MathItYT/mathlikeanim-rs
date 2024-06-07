use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::{colors::{Color, GradientImageOrColor}, objects::{vector_object::VectorFeatures, wasm_interface::{WasmGradientImageOrColor, WasmVectorObject}}, renderer::render_all_vectors_svg, scene_api::SceneAPI, utils::sleep};

#[wasm_bindgen]
#[derive(Clone)]
pub struct SVGScene {
    #[wasm_bindgen(skip)]
    pub objects: Vec<VectorFeatures>,
    #[wasm_bindgen(skip)]
    pub width: u64,
    #[wasm_bindgen(skip)]
    pub height: u64,
    #[wasm_bindgen(skip)]
    pub fps: u64,
    #[wasm_bindgen(skip)]
    pub div_container: Option<web_sys::HtmlDivElement>,
    #[wasm_bindgen(skip)]
    pub background: GradientImageOrColor,
    #[wasm_bindgen(skip)]
    pub top_left_corner: (f64, f64),
    #[wasm_bindgen(skip)]
    pub bottom_right_corner: (f64, f64),
    #[wasm_bindgen(skip)]
    pub states: HashMap<usize, (Vec<VectorFeatures>, GradientImageOrColor, (f64, f64), (f64, f64))>,
    #[wasm_bindgen(skip)]
    pub callback: &'static dyn Fn()
}


impl SceneAPI for SVGScene {
    fn new(width: u64, height: u64, fps: u64) -> SVGScene {
        return SVGScene {
            objects: Vec::new(),
            width,
            height,
            fps,
            div_container: None,
            background: GradientImageOrColor::Color(Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }),
            top_left_corner: (0.0, 0.0),
            bottom_right_corner: (width as f64, height as f64),
            states: HashMap::new(),
            callback: Box::leak(Box::new(&|| {}))
        };
    }
    fn clear(&mut self) {
        self.objects = Vec::new();
    }
    fn on_rendered(&self) {
        (self.callback)();
    }
    fn restore(&mut self, n: usize) {
        let (objects, background, top_left_corner, bottom_right_corner) = self.states.get(&n).unwrap().clone();
        self.objects = objects;
        self.background = background;
        self.top_left_corner = top_left_corner;
        self.bottom_right_corner = bottom_right_corner;
    }
    fn save_state(&mut self, n: usize) {
        self.states.insert(n, (self.objects.clone(), self.background.clone(), self.top_left_corner, self.bottom_right_corner));
    }
    fn set_corners(&mut self, top_left_corner: (f64, f64), bottom_right_corner: (f64, f64)) {
        self.top_left_corner = top_left_corner;
        self.bottom_right_corner = bottom_right_corner;
    }
    fn get_top_left_corner(&self) -> (f64, f64) {
        return self.top_left_corner;
    }
    fn get_bottom_right_corner(&self) -> (f64, f64) {
        return self.bottom_right_corner;
    }
    fn set_background(&mut self, background: GradientImageOrColor) {
        self.background = background;
    }
    fn add(&mut self, vec_obj: VectorFeatures) {
        self.remove(vec_obj.index);
        self.objects.push(vec_obj);
    }
    fn remove(&mut self, index: usize) {
        self.objects = self.objects.clone().into_iter().filter(|obj| obj.index != index).collect();
    }
    fn get_fps(&self) -> &u64 {
        return &self.fps;
    }
    fn get_height(&self) -> &u64 {
        return &self.height;
    }
    fn get_width(&self) -> &u64 {
        return &self.width;
    }
    fn render_frame(&self) {
        render_all_vectors_svg(&self);
    }
    fn get_objects_from_indices(&self, object_indices: Vec<usize>) -> HashMap<usize, VectorFeatures> {
        let mut objects = HashMap::new();
        for index in object_indices {
            for obj in &self.objects {
                if obj.index == index {
                    objects.insert(index, obj.clone());
                }
            }
        }
        return objects;
    }
    async fn sleep(&mut self, duration_in_ms: i32) {
        sleep(duration_in_ms).await;
    }
}


#[wasm_bindgen]
impl SVGScene {
    #[wasm_bindgen(constructor)]
    pub fn new_js(width: u64, height: u64, fps: u64) -> SVGScene {
        return SVGScene::new(width, height, fps);
    }
    #[wasm_bindgen(js_name = getFps)]
    pub fn get_fps_js(&self) -> u64 {
        return self.fps;
    }
    #[wasm_bindgen(js_name = getHeight)]
    pub fn get_height_js(&self) -> u64 {
        return self.height;
    }
    #[wasm_bindgen(js_name = getWidth)]
    pub fn get_width_js(&self) -> u64 {
        return self.width;
    }
    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame_js(&mut self) {
        self.render_frame();
    }
    #[wasm_bindgen(js_name = clear)]
    pub fn clear_js(&mut self) {
        self.clear();
    }
    #[wasm_bindgen(js_name = restore)]
    pub fn restore_js(&mut self, n: usize) {
        self.restore(n);
    }
    #[wasm_bindgen(js_name = saveState)]
    pub fn save_state_js(&mut self, n: usize) {
        self.save_state(n);
    }
    #[wasm_bindgen(js_name = setTopLeftCorner)]
    pub fn set_top_left_corner_js(&mut self, x: f64, y: f64) {
        self.set_corners((x, y), self.bottom_right_corner);
    }
    #[wasm_bindgen(js_name = setBottomRightCorner)]
    pub fn set_bottom_right_corner_js(&mut self, x: f64, y: f64) {
        self.set_corners(self.top_left_corner, (x, y));
    }
    #[wasm_bindgen(js_name = getTopLeftCorner)]
    pub fn get_top_left_corner_js(&self) -> js_sys::Array {
        let array = js_sys::Array::new();
        array.push(&JsValue::from_f64(self.top_left_corner.0));
        array.push(&JsValue::from_f64(self.top_left_corner.1));
        return array;
    }
    #[wasm_bindgen(js_name = getBottomRightCorner)]
    pub fn get_bottom_right_corner_js(&self) -> js_sys::Array {
        let array = js_sys::Array::new();
        array.push(&JsValue::from_f64(self.bottom_right_corner.0));
        array.push(&JsValue::from_f64(self.bottom_right_corner.1));
        return array;
    }
    #[wasm_bindgen(js_name = setBackground)]
    pub fn set_background_js(&mut self, background: WasmGradientImageOrColor) {
        let background: GradientImageOrColor = background.gradient_image_or_color;
        self.set_background(background);
    }
    #[wasm_bindgen(js_name = add)]
    pub fn add_js(&mut self, vec_obj: WasmVectorObject) {
        self.add(vec_obj.native_vec_features);
    }
    #[wasm_bindgen(js_name = remove)]
    pub fn remove_js(&mut self, index: usize) {
        self.remove(index);
    }
    #[wasm_bindgen(js_name = getObjectsFromIndices)]
    pub fn get_objects_from_indices_js(&self, object_indices: js_sys::Array) -> js_sys::Map {
        let original = self.get_objects_from_indices(object_indices.iter().map(|x| x.as_f64().unwrap() as usize).collect());
        let js_map = js_sys::Map::new();
        for (key, value) in original {
            js_map.set(&JsValue::from_f64(key as f64), &JsValue::from(WasmVectorObject {
                native_vec_features: value
            }));
        }
        return js_map;
    }
    #[wasm_bindgen(js_name = setDivContainer)]
    pub fn set_div_container_js(&mut self, div_container: web_sys::HtmlDivElement) {
        self.div_container = Some(div_container);
    }
    #[wasm_bindgen(js_name = sleep)]
    pub async fn sleep_js(&mut self, duration_in_ms: i32) {
        self.sleep(duration_in_ms).await;
    }
    #[wasm_bindgen(js_name = play)]
    pub async fn play_js(
        &mut self,
        animation_func: js_sys::Function,
        object_indices: Vec<usize>,
        duration_in_frames: u64,
        rate_func: js_sys::Function
    ) {
        let animation_func_rs = |objects: Vec<VectorFeatures>, t: f64| {
            let objects_js = objects.iter().map(|obj| {
                JsValue::from(WasmVectorObject {
                    native_vec_features: obj.clone()
                })
            }).collect::<js_sys::Array>();
            let result = animation_func.call2(&JsValue::NULL, &objects_js, &JsValue::from_f64(t)).unwrap();
            let result = js_sys::Array::from(&result).iter().map(|obj| {
                let obj = obj.dyn_into::<WasmVectorObject>().unwrap();
                obj.native_vec_features
            }).collect();
            return result;
        };
        let rate_func_rs = |t: f64| -> f64 {
            rate_func.call1(&JsValue::NULL, &JsValue::from_f64(t)).unwrap().as_f64().unwrap()
        };
        self.play(animation_func_rs, object_indices, duration_in_frames, rate_func_rs).await;
    }
    #[wasm_bindgen(js_name = makeFrame)]
    pub fn make_frame_js(
        &mut self,
        animation_func: js_sys::Function,
        objects: Vec<WasmVectorObject>,
        t: f64
    ) {
        let objects_rs = objects.iter().map(|obj| obj.native_vec_features.clone()).collect();
        let animation_func_rs = |objects: Vec<VectorFeatures>, t: f64| -> Vec<VectorFeatures> {
            let objects_js = objects.iter().map(|obj| {
                JsValue::from(WasmVectorObject {
                    native_vec_features: obj.clone()
                })
            }).collect::<js_sys::Array>();
            let result = animation_func.call2(&JsValue::NULL, &objects_js, &JsValue::from_f64(t)).unwrap();
            let result = js_sys::Array::from(&result).iter().map(|obj| {
                let obj = obj.dyn_into::<WasmVectorObject>().unwrap();
                obj.native_vec_features
            }).collect();
            return result;
        };
        self.make_frame(&animation_func_rs, objects_rs, t);
    }
    #[wasm_bindgen(js_name = wait)]
    pub async fn wait_js(&mut self, duration_in_frames: u64) {
        self.wait(duration_in_frames).await;
    }
    #[wasm_bindgen(js_name = setCallback)]
    pub fn set_callback_js(&mut self, callback: js_sys::Function) {
        self.callback = Box::leak(Box::new(move || {
            callback.call0(&JsValue::NULL).unwrap();
        }));
    }
}