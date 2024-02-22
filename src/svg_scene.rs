use std::collections::HashMap;

use crate::{objects::vector_object::VectorFeatures, renderer::render_all_vectors_svg, utils::sleep};

#[derive(Clone)]
pub struct SVGScene {
    pub objects: Vec<VectorFeatures>,
    pub width: u64,
    pub height: u64,
    pub fps: u64,
    pub div_container_id: &'static str,
    pub current_frame: u64,
    pub background_color: (f64, f64, f64, f64),
    n_plays: u64,
    top_left_corner: (f64, f64),
    bottom_right_corner: (f64, f64),
    states: HashMap<usize, (Vec<VectorFeatures>, u64, u64, (f64, f64, f64, f64), (f64, f64), (f64, f64))>
}


impl SVGScene {
    pub fn new(width: u64, height: u64, fps: u64, div_container_id: &'static str) -> SVGScene {
        return SVGScene {
            objects: Vec::new(),
            width,
            height,
            fps,
            div_container_id,
            current_frame: 0,
            n_plays: 0,
            background_color: (0.0, 0.0, 0.0, 1.0),
            top_left_corner: (0.0, 0.0),
            bottom_right_corner: (width as f64, height as f64),
            states: HashMap::new()
        };
    }
    pub fn clear(&mut self) {
        self.objects = Vec::new();
    }
    pub fn restore(&mut self, n: usize) {
        let (objects, current_frame, n_plays, background_color, top_left_corner, bottom_right_corner) = self.states.get(&n).unwrap().clone();
        self.objects = objects;
        self.current_frame = current_frame;
        self.n_plays = n_plays;
        self.background_color = background_color;
        self.top_left_corner = top_left_corner;
        self.bottom_right_corner = bottom_right_corner;
    }
    pub fn save_state(&mut self, n: usize) {
        self.states.insert(n, (self.objects.clone(), self.current_frame, self.n_plays, self.background_color, self.top_left_corner, self.bottom_right_corner));
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
    pub fn set_background_color(&mut self, color: (f64, f64, f64, f64)) {
        self.background_color = color;
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
        animation_funcs: Vec<impl Fn(VectorFeatures, f64) -> VectorFeatures>,
        object_indices: Vec<usize>,
        duration_in_frames: u64,
        rate_func: impl Fn(f64) -> f64
    ) {
        let objects = self.get_objects_from_indices(object_indices.clone());
        let mut make_frame_func = |frame, _, _, total_frames| {
            let t = rate_func(frame as f64 / total_frames as f64);
            let mut new_objects = HashMap::new();
            for (i, animation_func) in animation_funcs.iter().enumerate() {
                let new_object_from_func = animation_func(new_objects.get(&object_indices[i]).unwrap_or(&objects[&object_indices[i]]).clone(), t);
                new_objects.insert(object_indices[i], new_object_from_func);
            }
            self.objects = self.objects.clone().into_iter().map(
                |obj| {
                    if objects.contains_key(&obj.index) {
                        return new_objects[&obj.index].clone();
                    }
                    return obj;
                }
            ).collect();
            render_all_vectors_svg(&self.objects.clone(), self.width, self.height, self.background_color, self.top_left_corner, self.bottom_right_corner, self.div_container_id);
        };
        for frame in 0..duration_in_frames {
            make_frame_func(frame, self.width, self.height, duration_in_frames);
            sleep(1000 / self.fps as i32).await;
        }
        self.n_plays += 1;
        self.current_frame += duration_in_frames;
        make_frame_func(duration_in_frames, self.width, self.height, duration_in_frames);
    }
    pub async fn wait(&mut self, duration_in_frames: u64) {
        let anim_funcs: Vec<fn(VectorFeatures, f64) -> VectorFeatures> = vec![];
        self.play(anim_funcs.clone(), vec![], duration_in_frames, |t| t).await;
    }
    pub fn update(&mut self) {
        render_all_vectors_svg(&self.objects.clone(), self.width, self.height, self.background_color, self.top_left_corner, self.bottom_right_corner, self.div_container_id);
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