use std::collections::HashMap;
use js_sys::{Array, Function, Map, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::{colors::{Color, GradientImageOrColor}, objects::{vector_object::VectorObject, wasm_interface::{WasmGradientImageOrColor, WasmVectorObject}}, scene_api::SceneAPI, utils::sleep, web_renderer::render_all_vectors};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Scene {
    #[wasm_bindgen(skip)]
    pub objects: Vec<VectorObject>,
    #[wasm_bindgen(skip)]
    pub width: u32,
    #[wasm_bindgen(skip)]
    pub height: u32,
    #[wasm_bindgen(skip)]
    pub fps: u32,
    #[wasm_bindgen(skip)]
    pub background: GradientImageOrColor,
    #[wasm_bindgen(skip)]
    pub top_left_corner: (f64, f64),
    #[wasm_bindgen(skip)]
    pub bottom_right_corner: (f64, f64),
    #[wasm_bindgen(skip)]
    pub context: Option<&'static web_sys::CanvasRenderingContext2d>,
    #[wasm_bindgen(skip)]
    pub states: HashMap<usize, (Vec<VectorObject>, GradientImageOrColor, (f64, f64), (f64, f64))>,
    #[wasm_bindgen(skip)]
    pub loaded_images: Map,
    #[wasm_bindgen(skip)]
    pub callback: Function,
    #[wasm_bindgen(skip)]
    pub updaters: Map,
    #[wasm_bindgen(skip)]
    pub time_in_frames: usize
}


impl SceneAPI for Scene {
    fn new(width: u32, height: u32, fps: u32) -> Scene {
        return Scene {
            objects: Vec::new(),
            width,
            height,
            fps,
            context: None,
            background: GradientImageOrColor::Color(Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }),
            top_left_corner: (0.0, 0.0),
            bottom_right_corner: (width as f64, height as f64),
            states: HashMap::new(),
            loaded_images: Map::new(),
            callback: Closure::wrap(Box::new(|| Promise::resolve(&JsValue::NULL)) as Box<dyn Fn() -> Promise>).into_js_value().dyn_into().unwrap(),
            updaters: Map::new(),
            time_in_frames: 0
        };
    }
    fn set_updater(&mut self, index: usize, updater: Function) {
        self.updaters.set(&JsValue::from_f64(index as f64), &updater);
    }
    fn remove_updater(&mut self, index: usize) {
        self.updaters.delete(&JsValue::from_f64(index as f64));
    }
    async fn update(&mut self, index: usize) {
        let updater = self.updaters.get(&JsValue::from_f64(index as f64));
        if updater.is_falsy() {
            return;
        }
        let object = self.objects.iter().find(|obj| obj.index == index).unwrap();
        let updater = updater.dyn_into::<Function>().unwrap();
        let promise = updater.call1(&JsValue::NULL, &JsValue::from(WasmVectorObject {
            native_vec_features: object.clone()
        })).unwrap();
        let new_object = JsFuture::from(Promise::resolve(&promise)).await.unwrap().dyn_into::<WasmVectorObject>().unwrap().native_vec_features;
        self.add(new_object);
    }
    fn get_fps(&self) -> &u32 {
        return &self.fps;
    }
    fn get_height(&self) -> &u32 {
        return &self.height;
    }
    fn get_width(&self) -> &u32 {
        return &self.width;
    }
    async fn render_frame(&mut self) {
        render_all_vectors(&self.objects, self.width, self.height, self.context, &self.background, self.top_left_corner, self.bottom_right_corner, &self.loaded_images, &self.callback).await;
    }
    fn clear(&mut self) {
        self.objects = Vec::new();
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
    fn add(&mut self, vec_obj: VectorObject) {
        self.remove(vec_obj.index);
        self.objects.push(vec_obj);
    }
    fn remove(&mut self, index: usize) {
        self.objects = self.objects.clone().into_iter().filter(|obj| obj.index != index).collect();
    }
    fn get_objects_from_indices(&self, object_indices: &Vec<usize>) -> HashMap<usize, VectorObject> {
        let mut objects = HashMap::new();
        for index in object_indices {
            for obj in &self.objects {
                if obj.index == *index {
                    objects.insert(*index, obj.clone());
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
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new_js(width: u32, height: u32, fps: u32) -> Scene {
        return Scene::new(width, height, fps);
    }
    #[wasm_bindgen(js_name = getFps)]
    pub fn get_fps_js(&self) -> u32 {
        return self.fps;
    }
    #[wasm_bindgen(js_name = getHeight)]
    pub fn get_height_js(&self) -> u32 {
        return self.height;
    }
    #[wasm_bindgen(js_name = getWidth)]
    pub fn get_width_js(&self) -> u32 {
        return self.width;
    }
    #[wasm_bindgen(js_name = setUpdater)]
    pub fn set_updater_js(&mut self, index: usize, updater: Function) {
        self.set_updater(index, updater);
    }
    #[wasm_bindgen(js_name = update)]
    pub async fn update_js(&mut self, index: usize) {
        self.update(index).await;
    }
    #[wasm_bindgen(js_name = removeUpdater)]
    pub fn remove_updater_js(&mut self, index: usize) {
        self.remove_updater(index);
    }
    #[wasm_bindgen(js_name = renderFrame)]
    pub async fn render_frame_js(&mut self) {
        self.render_frame().await;
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
    #[wasm_bindgen(js_name = insert)]
    pub fn insert_js(&mut self, index: usize, vec_obj: WasmVectorObject) {
        self.remove(vec_obj.native_vec_features.index);
        self.objects.insert(index, vec_obj.native_vec_features);
    }
    #[wasm_bindgen(js_name = remove)]
    pub fn remove_js(&mut self, index: usize) {
        self.remove(index);
    }
    #[wasm_bindgen(js_name = getObjects)]
    pub fn get_objects_js(&self) -> js_sys::Array {
        let js_array = js_sys::Array::new();
        for obj in &self.objects {
            js_array.push(&JsValue::from(WasmVectorObject {
                native_vec_features: obj.clone()
            }));
        }
        return js_array;
    }
    #[wasm_bindgen(js_name = getObjectsFromIndices)]
    pub fn get_objects_from_indices_js(&self, object_indices: js_sys::Array) -> js_sys::Map {
        let original = self.get_objects_from_indices(&object_indices.iter().map(|x| x.as_f64().unwrap() as usize).collect());
        let js_map = js_sys::Map::new();
        for (key, value) in original {
            js_map.set(&JsValue::from_f64(key as f64), &JsValue::from(WasmVectorObject {
                native_vec_features: value
            }));
        }
        return js_map;
    }
    #[wasm_bindgen(js_name = setCanvasContext)]
    pub fn set_canvas_context_js(&mut self, context: web_sys::CanvasRenderingContext2d) {
        let context: &'static web_sys::CanvasRenderingContext2d = Box::leak(Box::new(context));
        self.context = Some(context);
    }
    #[wasm_bindgen(js_name = sleep)]
    pub async fn sleep_js(&mut self, duration_in_ms: i32) {
        self.sleep(duration_in_ms).await;
    }
    #[wasm_bindgen(js_name = setObjects)]
    pub fn set_objects_js(&mut self, objects: js_sys::Array) {
        self.objects = objects.iter().map(|x| x.dyn_into::<WasmVectorObject>().unwrap().native_vec_features).collect();
    }
    #[wasm_bindgen(js_name = play)]
    pub async fn play_js(
        &mut self,
        animation_func: js_sys::Function,
        object_indices: Vec<usize>,
        duration_in_frames: u32,
        rate_func: js_sys::Function
    ) {
        let rate_func_rs = |t: f64| -> f64 {
            rate_func.call1(&JsValue::NULL, &JsValue::from_f64(t)).unwrap().as_f64().unwrap()
        };
        self.play(animation_func, object_indices, duration_in_frames, rate_func_rs).await;
    }
    #[wasm_bindgen(js_name = makeFrame)]
    pub async fn make_frame_js(
        &mut self,
        animation_func: &js_sys::Function,
        objects: &Array,
        t: f64
    ) { 
        self.make_frame(animation_func, objects, t).await;
    }
    #[wasm_bindgen(js_name = wait)]
    pub async fn wait_js(&mut self, duration_in_frames: u32, object_indices: Vec<usize>) {
        self.wait(duration_in_frames, object_indices).await;
    }
    #[wasm_bindgen(js_name = setOnRendered)]
    pub fn set_on_rendered_js(&mut self, callback: js_sys::Function) {
        self.callback = callback;
    }
    #[wasm_bindgen(js_name = waitUntil)]
    pub async fn wait_until_js(&mut self, condition: js_sys::Function, object_indices: Vec<usize>) {
        self.wait_until(condition, object_indices).await;
    }
    #[wasm_bindgen(js_name = onRendered)]
    pub async fn on_rendered_js(&mut self) {
        let promise = self.callback.call0(&JsValue::NULL).unwrap().dyn_into::<Promise>().unwrap();
        JsFuture::from(promise).await.unwrap();
    }
    #[wasm_bindgen(js_name = getLoadedImages)]
    pub fn get_loaded_images_js(&self) -> js_sys::Map {
        return self.loaded_images.clone();
    }
}