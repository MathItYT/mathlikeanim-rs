use crate::{objects::wasm_interface::{WasmGradientImageOrColor, WasmVectorObject}, scene::Scene, svg_scene::SVGScene, utils::error};
use js_sys::{Array, Function, Map};
use wasm_bindgen::prelude::*;

enum SceneEnum {
    Scene(Scene),
    SVGScene(SVGScene)
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
    #[wasm_bindgen(js_name = fromScene)]
    pub fn from_scene(scene: Scene) -> GenericScene {
        return GenericScene {
            width: scene.width,
            height: scene.height,
            fps: scene.fps,
            scene: SceneEnum::Scene(scene)
        }
    }
    #[wasm_bindgen(js_name = fromSVGScene)]
    pub fn from_svg_scene(scene: SVGScene) -> GenericScene {
         return GenericScene {
              width: scene.width,
              height: scene.height,
              fps: scene.fps,
              scene: SceneEnum::SVGScene(scene)
         }
    }
    #[wasm_bindgen(js_name = isScene)]
    pub fn is_scene(&self) -> bool {
        match &self.scene {
            SceneEnum::Scene(_) => {
                return true;
            }
            SceneEnum::SVGScene(_) => {
                return false;
            }
        }
    }
    #[wasm_bindgen(js_name = isSVGScene)]
    pub fn is_svg_scene(&self) -> bool {
        match &self.scene {
            SceneEnum::Scene(_) => {
                return false;
            }
            SceneEnum::SVGScene(_) => {
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
            SceneEnum::Scene(scene) => {
                scene.render_frame_js();
            }
            SceneEnum::SVGScene(scene) => {
                scene.render_frame_js();
            }
        }
    }
    #[wasm_bindgen(js_name = clear)]
    pub fn clear(&mut self) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.clear_js();
            }
            SceneEnum::SVGScene(scene) => {
                scene.clear_js();
            }
        }
    }
    #[wasm_bindgen(js_name = restore)]
    pub fn restore(&mut self, n: usize) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.restore_js(n);
            }
            SceneEnum::SVGScene(scene) => {
                scene.restore_js(n);
            }
        }
    }
    #[wasm_bindgen(js_name = saveState)]
    pub fn save_state(&mut self, n: usize) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.save_state_js(n);
            }
            SceneEnum::SVGScene(scene) => {
                scene.save_state_js(n);
            }
        }
    }
    #[wasm_bindgen(js_name = setTopLeftCorner)]
    pub fn set_top_left_corner(&mut self, x: f64, y: f64) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.set_top_left_corner_js(x, y);
            }
            SceneEnum::SVGScene(scene) => {
                scene.set_top_left_corner_js(x, y);
            }
        }
    }
    #[wasm_bindgen(js_name = setBottomRightCorner)]
    pub fn set_bottom_right_corner(&mut self, x: f64, y: f64) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.set_bottom_right_corner_js(x, y);
            }
            SceneEnum::SVGScene(scene) => {
                scene.set_bottom_right_corner_js(x, y);
            }
        }
    }
    #[wasm_bindgen(js_name = getTopLeftCorner)]
    pub fn get_top_left_corner(&self) -> Array {
        match &self.scene {
            SceneEnum::Scene(scene) => {
                return scene.get_top_left_corner_js();
            }
            SceneEnum::SVGScene(scene) => {
                return scene.get_top_left_corner_js();
            }
        }
    }
    #[wasm_bindgen(js_name = getBottomRightCorner)]
    pub fn get_bottom_right_corner(&self) -> Array {
        match &self.scene {
            SceneEnum::Scene(scene) => {
                return scene.get_bottom_right_corner_js();
            }
            SceneEnum::SVGScene(scene) => {
                return scene.get_bottom_right_corner_js();
            }
        }
    }
    #[wasm_bindgen(js_name = setBackground)]
    pub fn set_background(&mut self, color: WasmGradientImageOrColor) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.set_background_js(color);
            }
            SceneEnum::SVGScene(scene) => {
                scene.set_background_js(color);
            }
        }
    }
    #[wasm_bindgen(js_name = add)]
    pub fn add(&mut self, object: WasmVectorObject) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.add_js(object);
            }
            SceneEnum::SVGScene(scene) => {
                scene.add_js(object);
            }
        }
    }
    #[wasm_bindgen(js_name = insert)]
    pub fn insert(&mut self, index: usize, object: WasmVectorObject) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.insert_js(index, object);
            }
            SceneEnum::SVGScene(scene) => {
                scene.insert_js(index, object);
            }
        }
    }
    #[wasm_bindgen(js_name = remove)]
    pub fn remove(&mut self, index: usize) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.remove_js(index);
            }
            SceneEnum::SVGScene(scene) => {
                scene.remove_js(index);
            }
        }
    }
    #[wasm_bindgen(js_name = getObjects)]
    pub fn get_objects(&self) -> Array {
        match &self.scene {
            SceneEnum::Scene(scene) => {
                return scene.get_objects_js();
            }
            SceneEnum::SVGScene(scene) => {
                return scene.get_objects_js();
            }
        }
    }
    #[wasm_bindgen(js_name = getObjectsFromIndices)]
    pub fn get_objects_from_indices(&self, object_indices: Array) -> Map {
        match &self.scene {
            SceneEnum::Scene(scene) => {
                return scene.get_objects_from_indices_js(object_indices);
            }
            SceneEnum::SVGScene(scene) => {
                return scene.get_objects_from_indices_js(object_indices);
            }
        }
    }
    #[wasm_bindgen(js_name = setCanvasContext)]
    pub fn set_canvas_context(&mut self, context: web_sys::CanvasRenderingContext2d) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.set_canvas_context_js(context);
            }
            SceneEnum::SVGScene(_) => {
                error(JsError::new("SVGScene does not have a canvas context"));
            }
        }
    }
    #[wasm_bindgen(js_name = setDivContainer)]
    pub fn set_div_container(&mut self, div_container: web_sys::HtmlDivElement) {
        match &mut self.scene {
            SceneEnum::Scene(_) => {
                error(JsError::new("Scene does not have a div container"));
            }
            SceneEnum::SVGScene(scene) => {
                scene.set_div_container_js(div_container);
            }
        }
    }
    #[wasm_bindgen(js_name = sleep)]
    pub async fn sleep(&mut self, duration_in_ms: i32) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.sleep_js(duration_in_ms).await;
            }
            SceneEnum::SVGScene(scene) => {
                scene.sleep_js(duration_in_ms).await;
            }
        }
    }
    #[wasm_bindgen(js_name = setObjects)]
    pub fn set_objects(&mut self, objects: Array) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.set_objects_js(objects);
            }
            SceneEnum::SVGScene(scene) => {
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
            SceneEnum::Scene(scene) => {
                scene.play_js(animation_func, object_indices, duration_in_frames, rate_func).await;
            }
            SceneEnum::SVGScene(scene) => {
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
            SceneEnum::Scene(scene) => {
                scene.make_frame_js(animation_func, objects, t).await;
            }
            SceneEnum::SVGScene(scene) => {
                scene.make_frame_js(animation_func, objects, t).await;
            }
        }
    }
    #[wasm_bindgen(js_name = wait)]
    pub async fn wait(&mut self, duration_in_frames: u64) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.wait_js(duration_in_frames).await;
            }
            SceneEnum::SVGScene(scene) => {
                scene.wait_js(duration_in_frames).await;
            }
        }
    }
    #[wasm_bindgen(js_name = setCallback)]
    pub fn set_callback(&mut self, callback: Function) {
        match &mut self.scene {
            SceneEnum::Scene(scene) => {
                scene.set_callback_js(callback);
            }
            SceneEnum::SVGScene(scene) => {
                scene.set_callback_js(callback);
            }
        }
    }
    #[wasm_bindgen(js_name = callCallback)]
    pub async fn call_callback(&self) {
        match &self.scene {
            SceneEnum::Scene(scene) => {
                scene.call_callback_js().await;
            }
            SceneEnum::SVGScene(scene) => {
                scene.call_callback_js().await;
            }
        }
    }
    #[wasm_bindgen(js_name = setClass)]
    pub fn set_class(&mut self, index: usize, class: String) {
        match &mut self.scene {
            SceneEnum::Scene(_) => {
                error(JsError::new("Can't assign class to an object in a Scene"));
            }
            SceneEnum::SVGScene(scene) => {
                scene.set_class_js(index, class);
            }
        }
    }
    #[wasm_bindgen(js_name = setStyle)]
    pub fn remove_class(&mut self, index: usize) {
        match &mut self.scene {
            SceneEnum::Scene(_) => {
                error(JsError::new("Can't remove class from an object in a Scene"));
            }
            SceneEnum::SVGScene(scene) => {
                scene.remove_class_js(index);
            }
        }
    }
}