use crate::{objects::vector_object::{VectorFeatures, VectorObject}, utils::interpolate_tuple};

pub fn shift_image_position(new_position: (f64, f64)) -> impl Fn(VectorFeatures, f64) -> VectorFeatures {
    let anim_function = move |vec_obj: VectorFeatures, t: f64| -> VectorFeatures {
        let img_pos = vec_obj.get_image_position();
        return vec_obj.set_image_position(interpolate_tuple(img_pos, new_position, t), true);
    };
    return anim_function;
}