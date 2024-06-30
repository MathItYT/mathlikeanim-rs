use std::collections::HashMap;

use js_sys::{Array, Function, Map, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::{colors::GradientImageOrColor, node_renderer::{create_write_stream, render_all_vectors, spawn, CanvasRenderingContext2D, ChildProcess}, objects::{vector_object::VectorFeatures, wasm_interface::{WasmGradientImageOrColor, WasmVectorObject}}, scene_api::SceneAPI, utils::log};


#[wasm_bindgen]
pub struct VideoScene {
    #[wasm_bindgen(skip)]
    pub objects: Vec<VectorFeatures>,
    #[wasm_bindgen(skip)]
    pub width: u64,
    #[wasm_bindgen(skip)]
    pub height: u64,
    #[wasm_bindgen(skip)]
    pub fps: u64,
    #[wasm_bindgen(skip)]
    pub save_frames: bool,
    #[wasm_bindgen(skip)]
    pub file_name_prefix: String,
    #[wasm_bindgen(skip)]
    pub background: GradientImageOrColor,
    #[wasm_bindgen(skip)]
    pub top_left_corner: (f64, f64),
    #[wasm_bindgen(skip)]
    pub bottom_right_corner: (f64, f64),
    #[wasm_bindgen(skip)]
    pub context: Option<CanvasRenderingContext2D>,
    #[wasm_bindgen(skip)]
    pub states: HashMap<usize, (Vec<VectorFeatures>, GradientImageOrColor, (f64, f64), (f64, f64))>,
    #[wasm_bindgen(skip)]
    pub callback: Function,
    #[wasm_bindgen(skip)]
    pub frame_number: u32,
}


impl SceneAPI for VideoScene {
    fn new(width: u64, height: u64, fps: u64) -> VideoScene {
        return VideoScene {
            objects: Vec::new(),
            width,
            height,
            fps,
            save_frames: false,
            context: None,
            background: GradientImageOrColor::Color(crate::colors::Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }),
            file_name_prefix: "frame".to_string(),
            top_left_corner: (0.0, 0.0),
            bottom_right_corner: (width as f64, height as f64),
            states: HashMap::new(),
            callback: Closure::wrap(Box::new(|| Promise::resolve(&JsValue::NULL)) as Box<dyn Fn() -> Promise>).into_js_value().dyn_into().unwrap(),
            frame_number: 0,
        };
    }
    async fn on_rendered(&mut self) {
        if self.save_frames {
            self.frame_number += 1;
            let canvas = self.context.as_ref().unwrap().canvas();
            let options = Map::new();
            options.set(&JsValue::from_str("compressionLevel"), &JsValue::from_f64(0.0));
            let png_stream = canvas.create_png_stream(&options);
            let stream = create_write_stream(&format!("{}-{}.png", self.file_name_prefix, self.frame_number));
            let ok = stream.write(&png_stream.read());
            if !ok {
                log("Warning: Frame is too big to save");
            }
            stream.end();
            let promise = Promise::new(&mut |resolve, _| {
                stream.on("close", &resolve);
            });
            JsFuture::from(promise).await.unwrap(); 
        }
        let promise = self.callback.call0(&JsValue::NULL).unwrap().dyn_into::<Promise>().unwrap();
        wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    }
    fn get_fps(&self) -> &u64 {
        return &self.fps;
    }
    fn get_width(&self) -> &u64 {
        return &self.width;
    }
    fn get_height(&self) -> &u64 {
        return &self.height;
    }
    fn get_top_left_corner(&self) -> (f64, f64) {
        return self.top_left_corner;
    }
    fn get_bottom_right_corner(&self) -> (f64, f64) {
        return self.bottom_right_corner;
    }
    fn add(&mut self, vec_obj: VectorFeatures) {
        self.remove(vec_obj.index);
        self.objects.push(vec_obj);
    }
    fn remove(&mut self, index: usize) {
        self.objects.retain(|x| x.index != index);
    }
    fn get_objects_from_indices(&self, object_indices: Vec<usize>) -> HashMap<usize, VectorFeatures> {
        let mut objects = HashMap::new();
        for index in object_indices {
            for object in &self.objects {
                if object.index == index {
                    objects.insert(index, object.clone());
                }
            }
        }
        return objects;
    }
    fn clear(&mut self) {
        self.objects.clear();
    }
    fn render_frame(&mut self) {
        render_all_vectors(&self.objects, self.width, self.height, self.context.as_ref().unwrap(), self.background.clone(), self.top_left_corner, self.bottom_right_corner);
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
    fn set_background(&mut self, background: GradientImageOrColor) {
        self.background = background;
    }
    async fn sleep(&mut self, _: i32) {
        // Do nothing
    }
}


#[wasm_bindgen]
impl VideoScene {
    #[wasm_bindgen(constructor)]
    pub fn new_js(width: u64, height: u64, fps: u64) -> VideoScene {
        return VideoScene::new(width, height, fps);
    }
    #[wasm_bindgen(js_name = getFps)]
    pub fn get_fps_js(&self) -> u64 {
        return self.fps;
    }
    #[wasm_bindgen(js_name = toggleSaveFrames)]
    pub async fn toggle_save_frames_js(&mut self) {
        self.save_frames = !self.save_frames;
    }
    #[wasm_bindgen(js_name = setFileNamePrefix)]
    pub fn set_file_name_prefix_js(&mut self, file_name_prefix: String) {
        self.file_name_prefix = file_name_prefix;
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
        let original = self.get_objects_from_indices(object_indices.iter().map(|x| x.as_f64().unwrap() as usize).collect());
        let js_map = js_sys::Map::new();
        for (key, value) in original {
            js_map.set(&JsValue::from_f64(key as f64), &JsValue::from(WasmVectorObject {
                native_vec_features: value
            }));
        }
        return js_map;
    }
    #[wasm_bindgen(js_name = setCanvasContext)]
    pub fn set_canvas_context_js(&mut self, context: CanvasRenderingContext2D) {
        self.context = Some(context);
    }
    #[wasm_bindgen(js_name = renderVideo)]
    pub fn render_video_js(&mut self, file_name: String, codec: Option<String>, pix_fmt: Option<String>, qp: Option<String>) -> ChildProcess {
        let command = "ffmpeg";
        let args = vec![
            "-y".to_string(),
            "-s".to_string(),
            format!("{}x{}", self.width, self.height),
            "-pix_fmt".to_string(),
            "bgra".to_string(),
            "-r".to_string(),
            format!("{}", self.fps),
            "-i".to_string(),
            format!("{}-%d.png", self.file_name_prefix),
            "-an".to_string(),
            "-vcodec".to_string(),
            codec.unwrap_or("libx264".to_string()).to_string(),
            "-pix_fmt".to_string(),
            pix_fmt.unwrap_or("yuv420p".to_string()).to_string(),
            "-qp".to_string(),
            qp.unwrap_or("0".to_string()).to_string(),
            file_name.to_string()
        ];
        self.frame_number = 0;
        return spawn(command, args);
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
        duration_in_frames: u64,
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
    pub async fn wait_js(&mut self, duration_in_frames: u64) {
        self.wait(duration_in_frames).await;
    }
    #[wasm_bindgen(js_name = setCallback)]
    pub fn set_callback_js(&mut self, callback: js_sys::Function) {
        self.callback = callback;
    }
    #[wasm_bindgen(js_name = callCallback)]
    pub async fn call_callback_js(&mut self) {
        self.on_rendered().await;
    }
}