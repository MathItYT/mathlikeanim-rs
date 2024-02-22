use std::f64::consts::PI;

use super::three_d_object::ThreeDObject;

pub fn three_d_line(
    point1: (f64, f64, f64),
    point2: (f64, f64, f64),
    color: (f64, f64, f64, f64)
) -> ThreeDObject {
    let result = ThreeDObject::from_uv_function(
        &|u, v| {
            let x = point1.0 + (point2.0 - point1.0) * u;
            let y = point1.1 + (point2.1 - point1.1) * u;
            let z = point1.2 + (point2.2 - point1.2) * u;
            let r = 1.0;
            let theta = 2.0 * PI * v;
            (x + r * theta.cos(), y + r * theta.sin(), z)
        },
        (0.0, 1.0),
        (0.0, 1.0),
        20,
        20,
        color,
        color,
        0.0
    );
    return result;
}