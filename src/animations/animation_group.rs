use crate::objects::vector_object::VectorFeatures;


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


pub fn animation_group(
    anim_funcs: &'static Vec<impl Fn(VectorFeatures, f64) -> VectorFeatures>,
    lag_ratio: f64,
) -> Vec<impl Fn(VectorFeatures, f64) -> VectorFeatures> {
    let mut result = Vec::new();
    let timings = make_timings(anim_funcs.len(), lag_ratio);
    for (i, anim_func) in anim_funcs.iter().enumerate() {
        let timing = timings[i];
        result.push(move |vec_obj: VectorFeatures, t: f64| -> VectorFeatures {
            return anim_func(vec_obj.clone(), ((lag_ratio + 1.0) * t - timing).clamp(0.0, 1.0));
        });
    }
    return result;
}