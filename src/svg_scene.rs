use std::collections::HashMap;

use crate::{colors::{GradientImageOrColor, Color}, objects::vector_object::VectorFeatures, renderer::render_all_vectors_svg, utils::sleep};

#[derive(Clone)]
pub struct SVGScene {
    pub objects: Vec<VectorFeatures>,
    pub width: u64,
    pub height: u64,
    pub fps: u64,
    pub div_container: Option<web_sys::HtmlDivElement>,
    pub background: GradientImageOrColor,
    top_left_corner: (f64, f64),
    bottom_right_corner: (f64, f64),
    states: HashMap<usize, (Vec<VectorFeatures>, GradientImageOrColor, (f64, f64), (f64, f64))>
}


impl SVGScene {
    pub fn new(width: u64, height: u64, fps: u64) -> SVGScene {
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
            states: HashMap::new()
        };
    }
    pub fn init_div_container(&mut self, div_container: web_sys::HtmlDivElement) {
        self.div_container = Some(div_container);
    }
    pub fn clear(&mut self) {
        self.objects = Vec::new();
    }
    pub fn restore(&mut self, n: usize) {
        let (objects, background, top_left_corner, bottom_right_corner) = self.states.get(&n).unwrap().clone();
        self.objects = objects;
        self.background = background;
        self.top_left_corner = top_left_corner;
        self.bottom_right_corner = bottom_right_corner;
    }
    pub fn save_state(&mut self, n: usize) {
        self.states.insert(n, (self.objects.clone(), self.background.clone(), self.top_left_corner, self.bottom_right_corner));
    }
    pub fn set_corners(&mut self, top_left_corner: (f64, f64), bottom_right_corner: (f64, f64)) {
        self.top_left_corner = top_left_corner;
        self.bottom_right_corner = bottom_right_corner;
    }
    pub fn get_top_left_corner(&self) -> (f64, f64) {
        return self.top_left_corner;
    }
    pub fn get_bottom_right_corner(&self) -> (f64, f64) {
        return self.bottom_right_corner;
    }
    pub fn set_background(&mut self, background: GradientImageOrColor) {
        self.background = background;
    }
    pub fn add(&mut self, vec_obj: VectorFeatures) {
        self.remove(vec_obj.index);
        self.objects.push(vec_obj);
    }
    pub fn remove(&mut self, index: usize) {
        self.objects = self.objects.clone().into_iter().filter(|obj| obj.index != index).collect();
    }
    pub async fn play(
        &mut self,
        animation_func: impl Fn(Vec<VectorFeatures>, f64) -> Vec<VectorFeatures>,
        object_indices: Vec<usize>,
        duration_in_frames: u64,
        rate_func: impl Fn(f64) -> f64
    ) {
        let fps = self.fps.clone();
        let height = self.height.clone();
        let width = self.width.clone();
        let objects = self.get_objects_from_indices(object_indices.clone());
        let mut make_frame_func = move |frame, _, _, total_frames| {
            let t = rate_func(frame as f64 / total_frames as f64);
            let new_objects = animation_func(objects.values().cloned().collect(), t);
            for obj in new_objects {
                self.add(obj);
            }
            render_all_vectors_svg(&self.objects.clone(), self.width, self.height, self.background.clone(), self.top_left_corner, self.bottom_right_corner, &self.div_container.as_mut().unwrap());
        };
        for frame in 0..duration_in_frames {
            make_frame_func(frame, width, height, duration_in_frames);
            sleep(1000 / fps as i32).await;
        }
        make_frame_func(duration_in_frames, width, height, duration_in_frames);
    }
    pub async fn wait(&mut self, duration_in_frames: u64) {
        self.play(|_, _| vec![], vec![], duration_in_frames, |t| t).await;
    }
    pub fn update(&mut self) {
        render_all_vectors_svg(&self.objects.clone(), self.width, self.height, self.background.clone(), self.top_left_corner, self.bottom_right_corner, &self.div_container.as_mut().unwrap());
    }
    pub fn get_objects_from_indices(&self, object_indices: Vec<usize>) -> HashMap<usize, VectorFeatures> {
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