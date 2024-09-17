use crate::objects::vector_object::VectorObject;


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
    vec_obj: VectorObject,
    anim_funcs: Vec<impl Fn(VectorObject, f64) -> VectorObject>,
    lag_ratio: f64,
    t: f64
) -> VectorObject {
    let timings = make_timings(anim_funcs.len(), lag_ratio);
    let new_vec_obj = vec_obj.clone();
    let mut new_subobjects = Vec::new();
    for i in 0..anim_funcs.len() {
        let timing = timings[i];
        let value = ((lag_ratio + 1.0) * t - timing).clamp(0.0, 1.0);
        new_subobjects.push(anim_funcs[i](vec_obj.subobjects[i].clone(), value));
    }
    return new_vec_obj.set_subobjects(new_subobjects);
}