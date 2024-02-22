use crate::{svg_scene::SVGScene, utils::interpolate_tuple};

pub fn move_camera_svg(
    top_left_corner: (f64, f64),
    bottom_right_corner: (f64, f64),
    scene: &mut SVGScene,
    t: f64
) {
    let old_top_left_corner = scene.get_top_left_corner();
    let old_bottom_right_corner = scene.get_bottom_right_corner();
    let new_top_left_corner = interpolate_tuple(old_top_left_corner, top_left_corner, t);
    let new_bottom_right_corner = interpolate_tuple(old_bottom_right_corner, bottom_right_corner, t);
    scene.set_corners(new_top_left_corner, new_bottom_right_corner);
}