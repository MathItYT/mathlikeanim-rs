use crate::utils::interpolate_tuple_3d;

use super::{add_tip_3d::add_tip_3d, three_d_line::three_d_line, three_d_object::ThreeDObject};

pub fn three_d_number_line(
    x_min: f64,
    x_max: f64,
    x_step: f64,
    center: Option<(f64, f64, f64)>,
    length: Option<f64>,
    color: Option<(f64, f64, f64, f64)>,
    ticks: Option<bool>,
    tick_size: Option<f64>,
    euler_angles: Option<(f64, f64, f64)>,
    tip: Option<bool>
) -> ThreeDObject {
    let mut line = three_d_line(
        (0.0, 0.0, 0.0),
        (length.unwrap_or(1000.0), 0.0, 0.0),
        color.unwrap_or((1.0, 1.0, 1.0, 1.0))
    );
    if ticks.unwrap_or(true) {
        let mut x = x_min;
        while x <= x_max {
            let mut tick = three_d_line(
                (x, -tick_size.unwrap_or(20.0) / 2.0, 0.0),
                (x, tick_size.unwrap_or(20.0) / 2.0, 0.0),
                color.unwrap_or((1.0, 1.0, 1.0, 1.0))
            );
            tick = tick.rotate(euler_angles.unwrap_or((0.0, 0.0, 0.0)), false).move_to((0.0, 0.0, 0.0), false);
            tick = tick.move_to(
                interpolate_tuple_3d(
                    (0.0, 0.0, 0.0),
                    (length.unwrap_or(1000.0), 0.0, 0.0),
                    (x - x_min) / (x_max - x_min)
                ),
                false
            );
            line.subobjects.push(tick);
            x += x_step;
        }
    }
    if tip.unwrap_or(true) {
        line = add_tip_3d(
            line,
            50.0,
            color.unwrap_or((1.0, 1.0, 1.0, 1.0)),
            (length.unwrap_or(1000.0), 0.0, 0.0),
            euler_angles.unwrap_or((0.0, 0.0, 0.0))
        );
    }
    line = line.rotate(euler_angles.unwrap_or((0.0, 0.0, 0.0)), true).move_to(center.unwrap_or((0.0, 0.0, 0.0)), true);
    return line;
}