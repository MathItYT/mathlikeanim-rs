use std::{collections::HashMap, future::Future};

use crate::{colors::GradientImageOrColor, objects::vector_object::VectorFeatures};

pub trait SceneAPI {
    fn new(width: u64, height: u64, fps: u64) -> Self;
    fn clear(&mut self);
    fn restore(&mut self, n: usize);
    fn save_state(&mut self, n: usize);
    fn set_corners(&mut self, top_left_corner: (f64, f64), bottom_right_corner: (f64, f64));
    fn get_top_left_corner(&self) -> (f64, f64);
    fn get_bottom_right_corner(&self) -> (f64, f64);
    fn set_background(&mut self, background: GradientImageOrColor);
    fn add(&mut self, vec_obj: VectorFeatures);
    fn remove(&mut self, index: usize);
    fn get_fps(&self) -> &u64;
    fn get_height(&self) -> &u64;
    fn get_width(&self) -> &u64;
    fn play(
        &mut self,
        animation_func: impl Fn(Vec<VectorFeatures>, f64) -> Vec<VectorFeatures>,
        object_indices: Vec<usize>,
        duration_in_frames: u64,
        rate_func: impl Fn(f64) -> f64
    ) -> impl Future<Output = ()> {
        async move {
            let fps = self.get_fps().clone();
            let objects = self.get_objects_from_indices(object_indices.clone());
            for frame in 0..duration_in_frames {
                self.make_frame(
                    &animation_func,
                    objects.values().cloned().collect(),
                    rate_func(frame as f64 / duration_in_frames as f64)
                );
                self.render_frame();
                self.sleep((1000 / fps) as i32).await;
            }
            self.make_frame(
                &animation_func,
                objects.values().cloned().collect(),
                rate_func(1.0)
            );
        }
    }
    fn make_frame(
        &mut self,
        animation_func: &impl Fn(Vec<VectorFeatures>, f64) -> Vec<VectorFeatures>,
        objects: Vec<VectorFeatures>,
        t: f64
    ) {
        let new_objects = animation_func(objects, t);
        for obj in new_objects {
            self.add(obj);
        }
    }
    fn wait(&mut self, duration_in_frames: u64) -> impl Future<Output = ()> {
        async move {
            self.play(|_, _| vec![], vec![], duration_in_frames, |t| t).await;
        }
    }
    fn sleep(&mut self, duration_in_ms: i32) -> impl Future<Output = ()>;
    fn render_frame(&mut self);
    fn update(&mut self) {
        self.render_frame();
    }
    fn get_objects_from_indices(&self, object_indices: Vec<usize>) -> HashMap<usize, VectorFeatures>;
}
