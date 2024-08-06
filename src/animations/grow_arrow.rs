use crate::objects::vector_object::VectorFeatures;


pub fn grow_arrow_with_final_tip(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let mut result = vec_obj.clone();
    let tip = result.subobjects.pop().unwrap();
    result = result.get_partial_copy(0.0, t, true);
    let new_tip = tip.scale(t, true)
        .move_to(result.points[result.points.len() - 1], true);
    result.subobjects.push(new_tip);
    return result;
}


pub fn grow_arrow_with_initial_tip(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let mut result = vec_obj.clone();
    let tip = result.subobjects.pop().unwrap();
    result = result.get_partial_copy(1.0 - t, 1.0, true);
    let new_tip = tip.scale(t, true)
        .move_to(result.points[0], true);
    result.subobjects.push(new_tip);
    return result;
}


pub fn grow_arrow_with_tips_at_both_ends(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let mut result = vec_obj.clone();
    let final_tip = result.subobjects.pop().unwrap();
    let initial_tip = result.subobjects.pop().unwrap();
    result = result.get_partial_copy(0.5 * (1.0 - t), 0.5 * (1.0 + t), true);
    let new_final_tip = final_tip.scale(t, true)
        .move_to(result.points[result.points.len() - 1], true);
    let new_initial_tip = initial_tip.scale(t, true)
        .move_to(result.points[0], true);
    result.subobjects.push(new_initial_tip);
    result.subobjects.push(new_final_tip);
    return result;
}
