use std::collections::HashMap;

use crate::{colors::{Color, GradientImageOrColor}, objects::vector_object::VectorFeatures, renderer::render_all_vectors, scene_api::SceneAPI, utils::sleep};


#[derive(Clone)]
pub struct Scene {
    pub objects: Vec<VectorFeatures>,
    pub width: u64,
    pub height: u64,
    pub fps: u64,
    pub background: GradientImageOrColor,
    pub top_left_corner: (f64, f64),
    pub bottom_right_corner: (f64, f64),
    pub context: Option<web_sys::CanvasRenderingContext2d>,
    states: HashMap<usize, (Vec<VectorFeatures>, GradientImageOrColor, (f64, f64), (f64, f64))>
}


impl SceneAPI for Scene {
    fn new(width: u64, height: u64, fps: u64) -> Scene {
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
            states: HashMap::new()
        };
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
    fn render_frame(&mut self) {
        render_all_vectors(&self.objects.clone(), self.width, self.height, self.context.clone(), self.background.clone(), self.top_left_corner, self.bottom_right_corner);
    }
    async fn sleep(&mut self, duration_in_ms: i32) {
        sleep(duration_in_ms).await;
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
    fn add(&mut self, vec_obj: VectorFeatures) {
        self.remove(vec_obj.index);
        self.objects.push(vec_obj);
    }
    fn remove(&mut self, index: usize) {
        self.objects = self.objects.clone().into_iter().filter(|obj| obj.index != index).collect();
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
}