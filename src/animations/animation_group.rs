use crate::objects::vector_object::{VectorFeatures, VectorObject};


pub fn make_timings(
    num_anim_funcs: usize,
    lag_ratio: f64
) -> Vec<f64> {
    let mut result = Vec::new();
    let total_lag = (num_anim_funcs - 1) as f64;
    for i in 0..num_anim_funcs {
        result.push(i as f64 * lag_ratio / total_lag);
    }
    return result;
}


// value: ((lag_ratio + 1.0) * t - timing).clamp(0.0, 1.0)
pub fn animation_group(
    anim_funcs: Vec<Box<dyn Fn(VectorFeatures, f64) -> VectorFeatures>>,
    lag_ratio: f64
) -> Box<dyn Fn(VectorFeatures, f64) -> VectorFeatures> {
    let timings = make_timings(anim_funcs.len(), lag_ratio);
    let result = Box::new(move |vec_obj: VectorFeatures, t: f64| -> VectorFeatures {
        let new_vec_obj = vec_obj.clone();
        let mut new_subobjects = Vec::new();
        for i in 0..anim_funcs.len() {
            let timing = timings[i];
            let value = ((lag_ratio + 1.0) * t - timing).clamp(0.0, 1.0);
            new_subobjects.push(anim_funcs[i](vec_obj.subobjects[i].clone(), value));
        }
        return new_vec_obj.set_subobjects(new_subobjects);
    });
    return result;
}