use std::f64::consts::PI;

use crate::{objects::vector_object::{VectorFeatures, VectorObject}, utils::{line_as_cubic_bezier, points_from_anchors_and_handles}};


pub fn arc(
    center: (f64, f64),
    radius: f64,
    start_angle: f64,
    end_angle: f64,
    n_samples: Option<usize>,
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
    background_image: Option<web_sys::HtmlImageElement>
) -> VectorFeatures {
    let mut anchors = Vec::new();
    let n_samples = match n_samples {
        Some(n) => n,
        None => 10
    };
    let angle_step = (end_angle - start_angle) / (n_samples as f64 - 1.0);
    for i in 0..n_samples {
        let angle = start_angle + (i as f64) * angle_step;
        let x = angle.cos();
        let y = angle.sin();
        anchors.push((x, y));
    }
    let tangent_vectors = anchors.iter().map(|point| {
        let x = -point.1;
        let y = point.0;
        return (x, y);
    }).collect::<Vec<(f64, f64)>>();
    let handles1 = anchors[..anchors.len()-1].iter().zip(tangent_vectors[..tangent_vectors.len()-1].iter()).map(|(anchor, tangent_vector)| {
        let x = anchor.0 + angle_step / 3.0 * tangent_vector.0;
        let y = anchor.1 + angle_step / 3.0 * tangent_vector.1;
        return (x, y);
    }).collect::<Vec<(f64, f64)>>();
    let handles2 = anchors[1..].iter().zip(tangent_vectors[1..].iter()).map(|(anchor, tangent_vector)| {
        let x = anchor.0 - angle_step / 3.0 * tangent_vector.0;
        let y = anchor.1 - angle_step / 3.0 * tangent_vector.1;
        return (x, y);
    }).collect::<Vec<(f64, f64)>>();
    let points = points_from_anchors_and_handles(anchors[..anchors.len()-1].to_vec(), handles1, handles2, anchors[1..].to_vec());
    return VectorFeatures {
        points,
        subobjects: vec![],
        index: match index {
            Some(i) => i,
            None => 0
        },
        stroke_color: match stroke_color {
            Some(color) => color,
            None => (1.0, 1.0, 1.0, 1.0)
        },
        fill_color: match fill_color {
            Some(color) => color,
            None => (1.0, 1.0, 1.0, 0.0)
        },
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
        background_image,
        image_position: (0.0, 0.0)
    }.scale(radius, true).move_to(center, true);
}


pub fn circle(
    center: (f64, f64),
    radius: f64,
    num_points: Option<usize>,
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
    background_image: Option<web_sys::HtmlImageElement>
) -> VectorFeatures {
    return arc(
        center,
        radius,
        0.0,
        2.0 * PI,
        num_points,
        stroke_color,
        fill_color,
        stroke_width,
        line_cap,
        line_join,
        index,
        background_image
    );
}


pub fn elliptical_arc(
    center: (f64, f64),
    x_radius: f64,
    y_radius: f64,
    start_angle: f64,
    end_angle: f64,
    num_points: Option<usize>,
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
    background_image: Option<web_sys::HtmlImageElement>
) -> VectorFeatures {
    return arc(
        center,
        x_radius,
        start_angle,
        end_angle,
        num_points,
        stroke_color,
        fill_color,
        stroke_width,
        line_cap,
        line_join,
        index,
        background_image
    ).stretch((1.0, y_radius / x_radius), true).move_to(center, true);
}


pub fn ellipse(
    center: (f64, f64),
    x_radius: f64,
    y_radius: f64,
    num_points: Option<usize>,
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
    background_image: Option<web_sys::HtmlImageElement>
) -> VectorFeatures {
    return elliptical_arc(
        center,
        x_radius,
        y_radius,
        0.0,
        2.0 * PI,
        num_points,
        stroke_color,
        fill_color,
        stroke_width,
        line_cap,
        line_join,
        index,
        background_image
    );
}


pub fn annular_sector(
    center: (f64, f64),
    inner_radius: f64,
    outer_radius: f64,
    start_angle: f64,
    end_angle: f64,
    num_points: Option<usize>,
    stroke_color: Option<(f64, f64, f64, f64)>,
    fill_color: Option<(f64, f64, f64, f64)>,
    stroke_width: Option<f64>,
    line_cap: Option<&'static str>,
    line_join: Option<&'static str>,
    index: Option<usize>,
    background_image: Option<web_sys::HtmlImageElement>
) -> VectorFeatures {
    let mut points = Vec::new();
    let inner_arc = arc(
        center,
        inner_radius,
        start_angle,
        end_angle,
        num_points,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    );
    let outer_arc = arc(
        center,
        outer_radius,
        start_angle,
        end_angle,
        num_points,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    );
    let inner_arc_points = inner_arc.points.clone();
    let mut outer_arc_points = outer_arc.points.clone();
    outer_arc_points.reverse();
    points.extend(inner_arc_points.clone());
    points.extend(line_as_cubic_bezier(
        inner_arc_points.clone()[inner_arc_points.len() - 1],
        outer_arc_points[0]
    ));
    points.extend(outer_arc_points.clone());
    points.extend(line_as_cubic_bezier(
        outer_arc_points.clone()[outer_arc_points.len() - 1],
        inner_arc_points[0]
    ));
    return VectorFeatures {
        points,
        subobjects: vec![],
        index: match index {
            Some(i) => i,
            None => 0
        },
        stroke_color: match stroke_color {
            Some(color) => color,
            None => (0.0, 0.0, 0.0, 0.0)
        },
        fill_color: match fill_color {
            Some(color) => color,
            None => (1.0, 1.0, 1.0, 1.0)
        },
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
        background_image,
        image_position: (0.0, 0.0)
    };
}
