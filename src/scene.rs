use cairo::ImageSurfaceDataOwned;

use crate::{objects::vector_object::VectorFeatures, renderer::{concat_videos, render_all_vectors, render_video}};

pub struct Scene {
    pub objects: Vec<VectorFeatures>,
    pub width: u64,
    pub height: u64,
    pub fps: u64,
    pub file_name: String,
    pub current_frame: u64,
    n_plays: u64
}


impl Scene {
    pub fn new(width: u64, height: u64, fps: u64, file_name: String) -> Scene {
        return Scene {
            objects: Vec::new(),
            width,
            height,
            fps,
            file_name,
            current_frame: 0,
            n_plays: 0
        };
    }
    pub fn add(&mut self, vec_obj: VectorFeatures) {
        let max_index = self.objects.iter().map(|obj| obj.index).max();
        let objects_with_obj_index = self.objects.iter().filter(|obj| obj.index == vec_obj.index).collect::<Vec<&VectorFeatures>>();
        let mut vec_obj = vec_obj.clone();
        if objects_with_obj_index.len() > 0 {
            vec_obj.index = max_index.unwrap() + 1;
        }
        self.objects.push(vec_obj);
    }
    pub fn remove(&mut self, index: usize) {
        self.objects = self.objects.clone().into_iter().filter(|obj| obj.index != index).collect();
    }
    pub fn play(
        &mut self,
        animation_funcs: Vec<impl Fn(VectorFeatures, f64) -> VectorFeatures>,
        object_indices: Vec<usize>,
        duration_in_frames: u64,
        rate_func: impl Fn(f64) -> f64
    ) {
        let objects = self.get_objects_from_indices(object_indices);
        let mut make_frame_func = |frame, _, _, total_frames| -> ImageSurfaceDataOwned {
            let t = rate_func(frame as f64 / total_frames as f64);
            let mut new_objects = Vec::new();
            for (i, animation_func) in animation_funcs.iter().enumerate() {
                let new_object_from_func = animation_func(objects.clone()[i].clone(), t);
                new_objects.push(new_object_from_func);
            }
            self.objects = self.objects.clone().into_iter().map(
                |obj| {
                    for new_obj in &new_objects {
                        if obj.index == new_obj.index {
                            return new_obj.clone();
                        }
                    }
                    return obj;
                }
            ).collect();
            self.current_frame += 1;
            return render_all_vectors(&self.objects.clone(), self.width, self.height);
        };
        let partial_movie_file_name = format!("{}_{}.mp4", self.file_name[..self.file_name.len() - 4].to_string(), self.n_plays);
        render_video(&mut make_frame_func, self.width, self.height, self.fps, duration_in_frames, &partial_movie_file_name);
        self.n_plays += 1;
        make_frame_func(duration_in_frames, self.width, self.height, duration_in_frames);
    }
    pub fn wait(&mut self, duration_in_frames: u64) {
        let anim_funcs: Vec<fn(VectorFeatures, f64) -> VectorFeatures> = vec![];
        self.play(anim_funcs.clone(), vec![], duration_in_frames, |t| t);
    }
    pub fn get_objects_from_indices(&self, object_indices: Vec<usize>) -> Vec<VectorFeatures> {
        let mut objects = Vec::new();
        for index in object_indices {
            for obj in &self.objects {
                if obj.index == index {
                    objects.push(obj.clone());
                }
            }
        }
        return objects;
    }
    pub fn finish(&self) {
        let files = (0..self.n_plays).map(|i| format!("{}_{}.mp4", self.file_name[..self.file_name.len() - 4].to_string(), i)).collect::<Vec<String>>();
        concat_videos(files, &self.file_name);
    }
}