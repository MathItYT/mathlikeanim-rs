use std::collections::HashMap;

use js_sys::{Array, Function, Map, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::{colors::GradientImageOrColor, node_renderer::{create_canvas, create_canvas_with_type, create_write_stream, render_all_vectors, spawn, CanvasRenderingContext2D, ChildProcess}, objects::{vector_object::VectorFeatures, wasm_interface::{WasmGradientImageOrColor, WasmVectorObject}}, scene_api::SceneAPI, utils::log};


#[wasm_bindgen]
pub struct NodeScene {
    #[wasm_bindgen(skip)]
    pub objects: Vec<VectorFeatures>,
    #[wasm_bindgen(skip)]
    pub width: u32,
    #[wasm_bindgen(skip)]
    pub height: u32,
    #[wasm_bindgen(skip)]
    pub fps: u32,
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
    pub animation_number: usize,
    #[wasm_bindgen(skip)]
    pub current_ffmpeg: Option<ChildProcess>,
    #[wasm_bindgen(skip)]
    pub svg: Option<bool>
}


impl SceneAPI for NodeScene {
    fn new(width: u32, height: u32, fps: u32) -> NodeScene {
        return NodeScene {
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
            animation_number: 0,
            current_ffmpeg: None,
            svg: None
        };
    }
    fn get_fps(&self) -> &u32 {
        return &self.fps;
    }
    fn get_width(&self) -> &u32 {
        return &self.width;
    }
    fn get_height(&self) -> &u32 {
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
    async fn render_frame(&mut self) {
        render_all_vectors(&self.objects, self.width, self.height, self.context.as_ref().unwrap(), &self.background, self.top_left_corner, self.bottom_right_corner, &self.callback).await;
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
impl NodeScene {
    #[wasm_bindgen(constructor)]
    pub fn new_js(width: u32, height: u32, fps: u32) -> NodeScene {
        return NodeScene::new(width, height, fps);
    }
    #[wasm_bindgen(js_name = getContext)]
    pub fn get_context_js(&self) -> CanvasRenderingContext2D {
        return self.context.as_ref().unwrap().clone();
    }
    #[wasm_bindgen(js_name = initContext)]
    pub fn init_context_js(&mut self, svg: Option<bool>) {
        self.context = if svg.unwrap_or(false) {
            Some(create_canvas_with_type(self.width, self.height, "svg").get_context("2d"))
        } else {
            Some(create_canvas(self.width, self.height).get_context("2d"))
        };
        self.svg = Some(svg.unwrap_or(false));
    }
    #[wasm_bindgen(js_name = isSVG)]
    pub fn is_svg_js(&self) -> Option<bool> {
        return self.svg;
    }
    #[wasm_bindgen(js_name = getFps)]
    pub fn get_fps_js(&self) -> u32 {
        return self.fps;
    }
    #[wasm_bindgen(js_name = toggleSaveFrames)]
    pub fn toggle_save_frames_js(&mut self) {
        self.save_frames = !self.save_frames;
    }
    #[wasm_bindgen(js_name = initFFmpegPartialMovie)]
    pub fn init_ffmpeg_partial_movie_js(&mut self, codec: Option<String>, pix_fmt: Option<String>, qp: Option<String>) {
        let command = "ffmpeg";
        let args = vec![
            "-y".to_string(),
            "-f".to_string(),
            "rawvideo".to_string(),
            "-s".to_string(),
            format!("{}x{}", self.width, self.height),
            "-pix_fmt".to_string(),
            "bgra".to_string(),
            "-r".to_string(),
            format!("{}", self.fps),
            "-i".to_string(),
            "-".to_string(),
            "-an".to_string(),
            "-vcodec".to_string(),
            codec.unwrap_or("libx264".to_string()).to_string(),
            "-pix_fmt".to_string(),
            pix_fmt.unwrap_or("yuv420p".to_string()).to_string(),
            "-qp".to_string(),
            qp.unwrap_or("0".to_string()).to_string(),
            "-r".to_string(),
            format!("{}", self.fps),
            format!("{}-{}.mp4", self.file_name_prefix, self.animation_number)
        ];
        self.current_ffmpeg = Some(spawn(command, args));
    }
    #[wasm_bindgen(js_name = closeFFmpegPartialMovie)]
    pub async fn close_ffmpeg_partial_movie_js(&mut self) {
        let ffmpeg = self.current_ffmpeg.as_ref().unwrap();
        ffmpeg.stdin().end();
        let promise = Promise::new(&mut |resolve, _| {
            ffmpeg.on("close", resolve);
        });
        JsFuture::from(promise).await.unwrap();
        self.current_ffmpeg = None;
        self.animation_number += 1;
    }
    #[wasm_bindgen(js_name = setFileNamePrefix)]
    pub fn set_file_name_prefix_js(&mut self, file_name_prefix: String) {
        self.file_name_prefix = file_name_prefix;
    }
    #[wasm_bindgen(js_name = getHeight)]
    pub fn get_height_js(&self) -> u32 {
        return self.height;
    }
    #[wasm_bindgen(js_name = getWidth)]
    pub fn get_width_js(&self) -> u32 {
        return self.width;
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
    #[wasm_bindgen(js_name = saveFramePNG)]
    pub async fn save_frame_png_js(&mut self, file_name: String) {
        let canvas = self.context.as_ref().unwrap().canvas();
        let options = Map::new();
        options.set(&JsValue::from_str("compressionLevel"), &JsValue::from_f64(0.0));
        let png_stream = canvas.create_png_stream(&options);
        let stream = create_write_stream(&file_name);
        let ok = stream.write(&png_stream.read());
        if !ok {
            log("Frame is too big");
        }
        stream.end();
        let promise = Promise::new(&mut |resolve, _| {
            stream.on("close", resolve);
        });
        JsFuture::from(promise).await.unwrap();
    }
    #[wasm_bindgen(js_name = saveFrameSVG)]
    pub async fn save_frame_svg_js(&mut self, file_name: String) {
        let canvas = self.context.as_ref().unwrap().canvas();
        let svg_buffer = canvas.to_buffer();
        let stream = create_write_stream(&file_name);
        let ok = stream.write(&svg_buffer);
        if !ok {
            log("Frame is too big");
        }
        stream.end();
        let promise = Promise::new(&mut |resolve, _| {
            stream.on("close", resolve);
        });
        JsFuture::from(promise).await.unwrap();
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
    pub async fn wait_js(&mut self, duration_in_frames: u32) {
        self.wait(duration_in_frames).await;
    }
    #[wasm_bindgen(js_name = setOnRendered)]
    pub fn set_on_rendered_js(&mut self, callback: js_sys::Function) {
        self.callback = callback;
    }
    #[wasm_bindgen(js_name = onRendered)]
    pub async fn on_rendered_js(&mut self) {
        if self.save_frames && self.current_ffmpeg.is_some() {
            let canvas = self.context.as_ref().unwrap().canvas();
            let ffmpeg = self.current_ffmpeg.as_ref().unwrap();
            let options = Map::new();
            options.set(&JsValue::from_str("compressionLevel"), &JsValue::from_f64(0.0));
            let buffer = canvas.to_buffer_with_mime_type("raw");
            let ok = ffmpeg.stdin().write(&buffer);
            if !ok {
                log("Frame is too big");
            }
        }
        let promise = self.callback.call0(&JsValue::NULL).unwrap().dyn_into::<Promise>().unwrap();
        wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    }
}