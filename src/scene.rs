use std::collections::HashMap;

use crate::{objects::vector_object::VectorFeatures, renderer::render_all_vectors, utils::sleep};
#[cfg(not(target_arch = "wasm32"))]
use crate::renderer::{render_video, concat_videos};


pub struct Scene {
    pub objects: Vec<VectorFeatures>,
    pub width: u64,
    pub height: u64,
    pub fps: u64,
    pub file_name: &'static str,
    pub current_frame: u64,
    pub background_color: (f64, f64, f64, f64),
    n_plays: u64,
    context: Option<web_sys::CanvasRenderingContext2d>,
    top_left_corner: (f64, f64),
    bottom_right_corner: (f64, f64)
}


impl Scene {
    pub fn new(width: u64, height: u64, fps: u64, file_name: &'static str) -> Scene {
        return Scene {
            objects: Vec::new(),
            width,
            height,
            fps,
            file_name: file_name,
            current_frame: 0,
            n_plays: 0,
            context: None,
            background_color: (0.0, 0.0, 0.0, 1.0),
            top_left_corner: (0.0, 0.0),
            bottom_right_corner: (width as f64, height as f64)
        };
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
    pub fn init_context(&mut self, context: web_sys::CanvasRenderingContext2d) {
        self.context = Some(context);
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
        let mut make_frame_func = |frame, _, _, total_frames| -> Option<Vec<u8>> {
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
            return render_all_vectors(&self.objects.clone(), self.width, self.height, self.context.clone(), self.background_color, self.top_left_corner, self.bottom_right_corner);
        };
        #[cfg(not(target_arch = "wasm32"))]
        if self.file_name != "" {
            let partial_movie_file_name = format!("{}_{}.mp4", self.file_name[..self.file_name.len() - 4].to_string(), self.n_plays);
            render_video(&mut make_frame_func, self.width, self.height, self.fps, duration_in_frames, &partial_movie_file_name);
            self.n_plays += 1;
            self.current_frame += duration_in_frames;
            make_frame_func(duration_in_frames, self.width, self.height, duration_in_frames);
            return;
        }

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
        render_all_vectors(&self.objects.clone(), self.width, self.height, self.context.clone(), self.background_color, self.top_left_corner, self.bottom_right_corner);
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
    #[cfg(not(target_arch = "wasm32"))]
    pub fn finish(&self) {
        let files = (0..self.n_plays).map(|i| format!("{}_{}.mp4", self.file_name[..self.file_name.len() - 4].to_string(), i)).collect::<Vec<String>>();
        concat_videos(files, &self.file_name);
    }
}