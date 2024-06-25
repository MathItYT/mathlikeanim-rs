use crate::objects::wasm_interface::{WasmGradientImageOrColor, WasmVectorObject};
#[cfg(feature = "browser")]
use crate::{scene::Scene, svg_scene::SVGScene, utils::error};
#[cfg(feature = "node")]
use crate::video_scene::VideoScene;
use js_sys::{Array, Function, Map};
use wasm_bindgen::prelude::*;

enum SceneEnum {
    #[cfg(feature = "browser")]
    Scene(Scene),
    #[cfg(feature = "browser")]
    SVGScene(SVGScene),
    #[cfg(feature = "node")]
    VideoScene(VideoScene)
}


#[wasm_bindgen]
pub struct GenericScene {
    width: u64,
    height: u64,
    fps: u64,
    scene: SceneEnum
}


#[wasm_bindgen]
impl GenericScene {
    #[cfg(feature = "browser")]
    #[wasm_bindgen(js_name = fromScene)]
    pub fn from_scene(scene: Scene) -> GenericScene {
        return GenericScene {
            width: scene.width,
            height: scene.height,
            fps: scene.fps,
            scene: SceneEnum::Scene(scene)
        }
    }
    #[cfg(feature = "browser")]
    #[wasm_bindgen(js_name = fromSVGScene)]
    pub fn from_svg_scene(scene: SVGScene) -> GenericScene {
         return GenericScene {
              width: scene.width,
              height: scene.height,
              fps: scene.fps,
              scene: SceneEnum::SVGScene(scene)
         }
    }
    #[cfg(feature = "node")]
    #[wasm_bindgen(js_name = fromVideoScene)]
    pub fn from_video_scene(scene: VideoScene) -> GenericScene {
        return GenericScene {
            width: scene.width,
            height: scene.height,
            fps: scene.fps,
            scene: SceneEnum::VideoScene(scene)
        }
    }
    #[wasm_bindgen(js_name = isScene)]
    pub fn is_scene(&self) -> bool {
        match &self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(_) => {
                return true;
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(_) => {
                return false;
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(_) => {
                return false;
            }
        }
    }
    #[wasm_bindgen(js_name = isSVGScene)]
    pub fn is_svg_scene(&self) -> bool {
        match &self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(_) => {
                return false;
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(_) => {
                return true;
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(_) => {
                return false;
            }
        }
    }
    #[wasm_bindgen(js_name = isVideoScene)]
    pub fn is_video_scene(&self) -> bool {
        match &self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(_) => {
                return false;
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(_) => {
                return false;
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(_) => {
                return true;
            }
        }
    }
    #[wasm_bindgen(js_name = getFps)]
    pub fn get_fps(&self) -> u64 {
        return self.fps;
    }
    #[wasm_bindgen(js_name = getHeight)]
    pub fn get_height(&self) -> u64 {
        return self.height;
    }
    #[wasm_bindgen(js_name = getWidth)]
    pub fn get_width(&self) -> u64 {
        return self.width;
    }
    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.render_frame_js();
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.render_frame_js();
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.render_frame_js();
            }
        }
    }
    #[wasm_bindgen(js_name = clear)]
    pub fn clear(&mut self) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.clear_js();
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.clear_js();
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.clear_js();
            }
        }
    }
    #[wasm_bindgen(js_name = restore)]
    pub fn restore(&mut self, n: usize) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.restore_js(n);
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.restore_js(n);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.restore_js(n);
            }
        }
    }
    #[wasm_bindgen(js_name = saveState)]
    pub fn save_state(&mut self, n: usize) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.save_state_js(n);
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.save_state_js(n);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.save_state_js(n);
            }
        }
    }
    #[wasm_bindgen(js_name = setTopLeftCorner)]
    pub fn set_top_left_corner(&mut self, x: f64, y: f64) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.set_top_left_corner_js(x, y);
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.set_top_left_corner_js(x, y);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.set_top_left_corner_js(x, y);
            }
        }
    }
    #[wasm_bindgen(js_name = setBottomRightCorner)]
    pub fn set_bottom_right_corner(&mut self, x: f64, y: f64) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.set_bottom_right_corner_js(x, y);
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.set_bottom_right_corner_js(x, y);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.set_bottom_right_corner_js(x, y);
            }
        }
    }
    #[wasm_bindgen(js_name = getTopLeftCorner)]
    pub fn get_top_left_corner(&self) -> Array {
        match &self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                return scene.get_top_left_corner_js();
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                return scene.get_top_left_corner_js();
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                return scene.get_top_left_corner_js();
            }
        }
    }
    #[wasm_bindgen(js_name = getBottomRightCorner)]
    pub fn get_bottom_right_corner(&self) -> Array {
        match &self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                return scene.get_bottom_right_corner_js();
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                return scene.get_bottom_right_corner_js();
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                return scene.get_bottom_right_corner_js();
            }
        }
    }
    #[wasm_bindgen(js_name = setBackground)]
    pub fn set_background(&mut self, color: WasmGradientImageOrColor) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.set_background_js(color);
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.set_background_js(color);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.set_background_js(color);
            }
        }
    }
    #[wasm_bindgen(js_name = add)]
    pub fn add(&mut self, object: WasmVectorObject) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.add_js(object);
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.add_js(object);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.add_js(object);
            }
        }
    }
    #[wasm_bindgen(js_name = insert)]
    pub fn insert(&mut self, index: usize, object: WasmVectorObject) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.insert_js(index, object);
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.insert_js(index, object);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.insert_js(index, object);
            }
        }
    }
    #[wasm_bindgen(js_name = remove)]
    pub fn remove(&mut self, index: usize) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.remove_js(index);
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.remove_js(index);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.remove_js(index);
            }
        }
    }
    #[wasm_bindgen(js_name = getObjects)]
    pub fn get_objects(&self) -> Array {
        match &self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                return scene.get_objects_js();
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                return scene.get_objects_js();
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                return scene.get_objects_js();
            }
        }
    }
    #[wasm_bindgen(js_name = getObjectsFromIndices)]
    pub fn get_objects_from_indices(&self, object_indices: Array) -> Map {
        match &self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                return scene.get_objects_from_indices_js(object_indices);
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                return scene.get_objects_from_indices_js(object_indices);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                return scene.get_objects_from_indices_js(object_indices);
            }
        }
    }
    #[cfg(feature = "browser")]
    #[wasm_bindgen(js_name = setCanvasContext)]
    pub fn set_canvas_context(&mut self, context: JsValue) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.set_canvas_context_js(context.dyn_into().unwrap());
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(_) => {
                error(JsError::new("SVGScene does not have a canvas context"));
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.set_canvas_context_js(context.dyn_into().unwrap());
            }
        }
    }
    #[cfg(feature = "browser")]
    #[wasm_bindgen(js_name = setDivContainer)]
    pub fn set_div_container(&mut self, div_container: web_sys::HtmlDivElement) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(_) => {
                error(JsError::new("Scene does not have a div container"));
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.set_div_container_js(div_container);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(_) => {
                error(JsError::new("VideoScene does not have a div container"));
            }
        }
    }
    #[wasm_bindgen(js_name = sleep)]
    pub async fn sleep(&mut self, duration_in_ms: i32) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.sleep_js(duration_in_ms).await;
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.sleep_js(duration_in_ms).await;
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.sleep_js(duration_in_ms).await;
            }
        }
    }
    #[wasm_bindgen(js_name = setObjects)]
    pub fn set_objects(&mut self, objects: Array) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.set_objects_js(objects);
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.set_objects_js(objects);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.set_objects_js(objects);
            }
        }
    }
    #[wasm_bindgen(js_name = play)]
    pub async fn play(
        &mut self,
        animation_func: Function,
        object_indices: Vec<usize>,
        duration_in_frames: u64,
        rate_func: Function
    ) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.play_js(animation_func, object_indices, duration_in_frames, rate_func).await;
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.play_js(animation_func, object_indices, duration_in_frames, rate_func).await;
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.play_js(animation_func, object_indices, duration_in_frames, rate_func).await;
            }
        }
    }
    #[wasm_bindgen(js_name = makeFrame)]
    pub async fn make_frame(
        &mut self,
        animation_func: &Function,
        objects: &Array,
        t: f64
    ) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.make_frame_js(animation_func, objects, t).await;
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.make_frame_js(animation_func, objects, t).await;
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.make_frame_js(animation_func, objects, t).await;
            }
        }
    }
    #[wasm_bindgen(js_name = wait)]
    pub async fn wait(&mut self, duration_in_frames: u64) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.wait_js(duration_in_frames).await;
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.wait_js(duration_in_frames).await;
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.wait_js(duration_in_frames).await;
            }
        }
    }
    #[wasm_bindgen(js_name = setCallback)]
    pub fn set_callback(&mut self, callback: Function) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.set_callback_js(callback);
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.set_callback_js(callback);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.set_callback_js(callback);
            }
        }
    }
    #[wasm_bindgen(js_name = callCallback)]
    pub async fn call_callback(&self) {
        match &self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(scene) => {
                scene.call_callback_js().await;
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.call_callback_js().await;
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(scene) => {
                scene.call_callback_js().await;
            }
        }
    }
    #[cfg(feature = "browser")]
    #[wasm_bindgen(js_name = setClass)]
    pub fn set_class(&mut self, index: usize, class: String) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(_) => {
                error(JsError::new("Can't assign class to an object in a Scene"));
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.set_class_js(index, class);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(_) => {
                error(JsError::new("Can't assign class to an object in a VideoScene"));
            }
        }
    }
    #[cfg(feature = "browser")]
    #[wasm_bindgen(js_name = setStyle)]
    pub fn remove_class(&mut self, index: usize) {
        match &mut self.scene {
            #[cfg(feature = "browser")]
            SceneEnum::Scene(_) => {
                error(JsError::new("Can't remove class from an object in a Scene"));
            }
            #[cfg(feature = "browser")]
            SceneEnum::SVGScene(scene) => {
                scene.remove_class_js(index);
            }
            #[cfg(feature = "node")]
            SceneEnum::VideoScene(_) => {
                error(JsError::new("Can't remove class from an object in a VideoScene"));
            }
        }
    }
}