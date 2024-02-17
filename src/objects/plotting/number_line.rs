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
    tick_size: Option<f64>,
    angle: Option<f64>,
    background_image: Option<web_sys::HtmlImageElement>
) -> VectorFeatures {
    let mut result = line(
        (center.unwrap().0 - length.unwrap_or(1000.0) / 2.0, center.unwrap().1),
        (center.unwrap().0 + length.unwrap_or(1000.0) / 2.0, center.unwrap().1),
        color,
        stroke_width,
        line_cap,
        line_join,
        index,
        None
    ).rotate(angle.unwrap_or(0.0), false);
    if add_ticks.unwrap_or(true) {
        let mut x = x_min;
        while x <= x_max {
            let mut tick = line(
                (number_to_point(&result, x, x_min, x_max).0, center.unwrap().1 - tick_size.unwrap_or(20.0) / 2.0),
                (number_to_point(&result, x, x_min, x_max).0, center.unwrap().1 + tick_size.unwrap_or(20.0) / 2.0),
                color,
                stroke_width,
                line_cap,
                line_join,
                None,
                None
            );
            tick = tick.rotate(angle.unwrap_or(0.0), false).move_to(number_to_point(&result, x, x_min, x_max), false);
            result.subobjects.push(tick);
            x += x_step;
        }
    }
    if add_tip.unwrap_or(true) {
        result = add_final_tip(result, 50.0, (1.0, 1.0, 1.0, 1.0));
    }
    if background_image.is_some() {
        result = result.set_background_image(background_image.unwrap(), true);
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
    numbers: Vec<f64>,
    vector_objects: Vec<VectorFeatures>,
    x_min: f64,
    x_max: f64,
    height: f64,
    direction: Option<(f64, f64)>,
    buff: Option<f64>,
    index: Option<usize>,
    background_image: Option<web_sys::HtmlImageElement>
) -> VectorFeatures {
    assert_eq!(numbers.len(), vector_objects.len());
    let mut result_subobjects = Vec::new();
    for (vec_obj, number) in vector_objects.iter().zip(numbers) {
        let point = number_to_point(&number_line, number, x_min, x_max);
        let mut vec_obj = vec_obj.clone();
        vec_obj = vec_obj.scale(height / vec_obj.get_height(), true);
        vec_obj = vec_obj.next_to_point(point, direction.unwrap_or((0.0, 1.0)), buff.unwrap_or(20.0), (0.0, 0.0), true);
        result_subobjects.push(vec_obj);
    }
    return VectorFeatures {
        index: index.unwrap_or(0),
        subobjects: result_subobjects,
        stroke_width: 0.0,
        stroke_color: (1.0, 1.0, 1.0, 1.0),
        fill_color: (1.0, 1.0, 1.0, 1.0),
        line_cap: "butt",
        line_join: "miter",
        points: vec![],
        background_image: background_image,
        image_position: (0.0, 0.0),
    };
}