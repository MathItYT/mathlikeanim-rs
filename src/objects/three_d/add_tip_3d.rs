use std::f64::consts::PI;

use super::three_d_object::ThreeDObject;

pub fn add_tip_3d(
    object: ThreeDObject,
    tip_size: f64,
    color: (f64, f64, f64, f64),
    position: (f64, f64, f64),
    euler_angles: (f64, f64, f64)
) -> ThreeDObject {
    let mut cone = ThreeDObject::from_uv_function(
        &|u, v| {
            let r = tip_size * (1.0 - u);
            let theta = 2.0 * PI * v;
            (r * theta.cos(), r * theta.sin(), tip_size * u)
        },
        (0.0, 1.0),
        (0.0, 1.0),
        20,
        20,
        color,
        color,
        0.0
    );
    cone = cone.rotate(euler_angles, true).move_to(position, true);
    let mut result = object.clone();
    result.subobjects.push(cone);
    return result;
}