use crate::{objects::{geometry::{add_tip::add_final_tip, line::line}, vector_object::{VectorFeatures, VectorObject}}, utils::{interpolate, interpolate_tuple}};

pub fn number_line(
    x_min: f64,
    x_max: f64,
    x_step: f64,
    color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
    center: Option<(f64, f64)>,
    length: Option<f64>,
    add_tip: Option<bool>,
    add_ticks: Option<bool>,
    tick_size: Option<f64>
) -> VectorFeatures {
    let mut result = line(
        (center.unwrap().0 - length.unwrap_or(1000.0) / 2.0, center.unwrap().1),
        (center.unwrap().0 + length.unwrap_or(1000.0) / 2.0, center.unwrap().1),
        color,
        stroke_width,
        line_cap,
        line_join,
        index,
    );
    if add_ticks.unwrap_or(true) {
        let mut x = x_min;
        while x <= x_max {
            let tick = line(
                (center.unwrap().0 + (x - x_min) / (x_max - x_min) * length.unwrap_or(1000.0) - length.unwrap_or(1000.0) / 2.0, center.unwrap().1 - tick_size.unwrap_or(20.0) / 2.0),
                (center.unwrap().0 + (x - x_min) / (x_max - x_min) * length.unwrap_or(1000.0) - length.unwrap_or(1000.0) / 2.0, center.unwrap().1 + tick_size.unwrap_or(20.0) / 2.0),
                color,
                stroke_width,
                line_cap,
                line_join,
                None,
            );
            result.subobjects.push(tick);
            x += x_step;
        }
    }
    if add_tip.unwrap_or(true) {
        result = add_final_tip(result, 0.1, (1.0, 1.0, 1.0, 1.0));
    }
    return result;
}


pub fn number_to_point(
    number_line: &VectorFeatures,
    number: f64,
    x_min: f64,
    x_max: f64
) -> (f64, f64) {
    let t = (number - x_min) / (x_max - x_min);
    let point = interpolate_tuple(
        number_line.points[0],
        number_line.points[number_line.points.len() - 1],
        t
    );
    return point;
}


pub fn point_to_number(
    number_line: &VectorFeatures,
    point: (f64, f64),
    x_min: f64,
    x_max: f64
) -> f64 {
    let t = (point.0 - number_line.points[0].0) / (number_line.points[number_line.points.len() - 1].0 - number_line.points[0].0);
    let number = interpolate(x_min, x_max, t);
    return number;
}


pub fn get_numbers_tex(
    number_line: VectorFeatures,
    x_min: f64,
    x_max: f64,
    step: f64,
    tex_to_vector: &dyn Fn(String) -> VectorFeatures,
    tex: impl Fn(f64) -> String,
    height: f64,
    shift: Option<(f64, f64)>,
    index: Option<usize>
) -> VectorFeatures {
    let mut result_subobjects = Vec::new();
    let mut x = x_min;
    while x <= x_max {
        let point = number_to_point(&number_line, x, x_min, x_max);
        let tex_str = tex(x);
        let mut tex_obj = tex_to_vector(tex_str);
        tex_obj = tex_obj.scale(height / tex_obj.get_height(), true);
        tex_obj = tex_obj.move_to(point, true).shift(shift.unwrap_or((0.0, 15.0 + tex_obj.get_height() / 2.0)), true);
        result_subobjects.push(tex_obj);
        x += step;
    }
    return VectorFeatures {
        index: index.unwrap_or(0),
        subobjects: result_subobjects,
        stroke_width: 0.0,
        stroke_color: (1.0, 1.0, 1.0, 1.0),
        fill_color: (1.0, 1.0, 1.0, 1.0),
        line_cap: "butt",
        line_join: "miter",
        points: vec![]
    };
}