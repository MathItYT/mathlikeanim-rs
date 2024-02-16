use crate::{objects::vector_object::{VectorFeatures, VectorObject}, utils::interpolate};

pub fn show_temporarily(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let upper = interpolate(0.0, 1.1, t);
    let lower = upper - 0.1;
    let upper = if upper > 1.0 { 1.0 } else { upper };
    let lower = if lower < 0.0 { 0.0 } else { lower };
    return vec_obj.get_partial_copy(lower, upper, true);
}