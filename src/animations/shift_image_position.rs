use crate::{objects::vector_object::{VectorFeatures, VectorObject}, utils::interpolate_tuple};

pub fn shift_image_position(new_position: (f64, f64)) -> impl Fn(VectorFeatures, f64) -> VectorFeatures {
    let anim_function = move |vec_obj: VectorFeatures, t: f64| -> VectorFeatures {
        let img_corners = vec_obj.get_image_corners();
        let new_top_left_corner = interpolate_tuple(img_corners.0, new_position, t);
        return vec_obj.set_image_corners(
            new_top_left_corner,
            (new_top_left_corner.0 + img_corners.1.0 - img_corners.0.0, new_top_left_corner.1 + img_corners.1.1 - img_corners.0.1),
            true
        );
    };
    return anim_function;
}