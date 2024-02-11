use crate::{objects::vector_object::VectorFeatures, utils::line_as_cubic_bezier};

pub fn line(
    point1: (f64, f64),
    point2: (f64, f64),
    stroke_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
    background_image: Option<web_sys::HtmlImageElement>
) -> VectorFeatures {
    let new_points = line_as_cubic_bezier(point1, point2);
    return VectorFeatures {
        points: new_points,
        subobjects: vec![],
        index: match index {
            Some(i) => i,
            None => 0
        },
        stroke_color: match stroke_color {
            Some(color) => color,
            None => (1.0, 1.0, 1.0, 1.0)
        },
        fill_color: (0.0, 0.0, 0.0, 0.0),
        stroke_width: match stroke_width {
            Some(width) => width,
            None => 4.0
        },
        line_cap: match line_cap {
            Some(cap) => cap,
            None => "butt"
        },
        line_join: match line_join {
            Some(join) => join,
            None => "miter"
        },
        background_image
    };
}