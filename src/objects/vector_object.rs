use crate::utils::{integer_interpolate, consider_points_equals};
use crate::utils::bezier;

use crate::colors::{Color, GradientImageOrColor, GradientStop, Image, LinearGradient, RadialGradient};


pub fn partial_bezier_points(
    points: &Vec<(f64, f64)>,
    start: f64,
    end: f64
) -> Vec<(f64, f64)> {
    if start == 1.0 {
        let mut result = Vec::new();
        for _ in 0..points.len() {
            result.push(points[points.len() - 1]);
        }
        return result;
    }

    let mut a_to_1 = Vec::new();
    for i in 0..points.len() {
        a_to_1.push(bezier(&points[i..points.len()].to_vec(), start));
    }
    let end_prop = (end - start) / (1.0 - start);
    let mut result = Vec::new();
    for i in 0..points.len() {
        result.push(bezier(&a_to_1[0..i + 1].to_vec(), end_prop));
    }
    return result;
}


pub fn get_partial_points(
    vector_features: &VectorObject,
    start: f64,
    end: f64,
    recursive: bool
) -> VectorObject {
    let points = vector_features.get_points();
    let fill = vector_features.fill.clone();
    let stroke = vector_features.stroke.clone();
    let stroke_width = vector_features.get_stroke_width();
    let line_cap = vector_features.get_line_cap();
    let line_join = vector_features.get_line_join();
    let mut subobjects = (&vector_features.subobjects).to_vec();
    if start <= 0.0 && end >= 1.0 {
        return VectorObject {
            points: points.clone(),
            fill_rule: vector_features.fill_rule,
            fill: fill,
            stroke: stroke,
            stroke_width: stroke_width,
            line_cap: line_cap,
            line_join: line_join,
            subobjects: subobjects,
            index: vector_features.index
        };
    }
    let bezier_quads = vector_features.get_cubic_bezier_tuples();
    if bezier_quads.len() == 0 {
        return VectorObject {
            points: points.clone(),
            fill: fill,
            fill_rule: vector_features.fill_rule,
            stroke: stroke,
            stroke_width: stroke_width,
            line_cap: line_cap,
            line_join: line_join,
            subobjects: subobjects.iter().map(|subobject| get_partial_points(subobject, start, end, recursive)).collect(),
            index: vector_features.index,
        };
    }
    let (lower_index, lower_residue) = integer_interpolate(0.0, bezier_quads.len() as f64, start);
    let (upper_index, upper_residue) = integer_interpolate(0.0, bezier_quads.len() as f64, end);
    if lower_index == upper_index {
        return VectorObject {
            points: partial_bezier_points(
                &vec![
                    bezier_quads[lower_index as usize].0,
                    bezier_quads[lower_index as usize].1,
                    bezier_quads[lower_index as usize].2,
                    bezier_quads[lower_index as usize].3
                ],
                lower_residue,
                upper_residue
            ),
            fill_rule: vector_features.fill_rule,
            fill: fill,
            stroke: stroke,
            stroke_width: stroke_width,
            line_cap: line_cap,
            line_join: line_join,
            subobjects: subobjects.iter().map(|subobject| get_partial_points(subobject, start, end, true)).collect(),
            index: vector_features.index,
        };
    }
    let mut new_points = Vec::new();
    new_points.extend(partial_bezier_points(
        &vec![
            bezier_quads[lower_index as usize].0,
            bezier_quads[lower_index as usize].1,
            bezier_quads[lower_index as usize].2,
            bezier_quads[lower_index as usize].3
        ],
        lower_residue,
        1.0
    ));
    for quad in bezier_quads[lower_index as usize + 1..upper_index as usize].to_vec() {
        new_points.extend(vec![quad.0, quad.1, quad.2, quad.3]);
    }
    new_points.extend(partial_bezier_points(
        &vec![
            bezier_quads[upper_index as usize].0,
            bezier_quads[upper_index as usize].1,
            bezier_quads[upper_index as usize].2,
            bezier_quads[upper_index as usize].3
        ],
        0.0,
        upper_residue
    ));
    if recursive {
        subobjects = subobjects.iter().map(|subobject| get_partial_points(subobject, start, end, true)).collect();
    }
    return VectorObject {
        points: new_points,
        fill: fill,
        fill_rule: vector_features.fill_rule,
        stroke: stroke,
        stroke_width: stroke_width,
        line_cap: line_cap,
        line_join: line_join,
        subobjects: subobjects,
        index: vector_features.index,
    };   
}

pub fn generate_subpaths(
    points: &Vec<(f64, f64)>
) -> Vec<Vec<(f64, f64)>> {
    let mut subpaths = Vec::new();
    let range = (4..points.len()).step_by(4);
    let filtered = range.filter(|i| {
        let p1 = points[i - 1];
        let p2 = points[*i];
        return !consider_points_equals(p1, p2);
    });
    let split_indices = [0].iter()
        .chain(filtered.collect::<Vec<usize>>().iter())
        .chain([points.len()].iter())
        .map(|i| *i)
        .collect::<Vec<usize>>();
    for i in 0..split_indices.len() - 1 {
        let start = split_indices[i];
        let end = split_indices[i + 1];
        let subpath = points[start..end].to_vec();
        subpaths.push(subpath);
    }
    return subpaths;
}


pub fn generate_cubic_bezier_tuples(
    points: &Vec<(f64, f64)>
) -> Vec<((f64, f64), (f64, f64), (f64, f64), (f64, f64))> {
    let remainder = points.len() % 4;
    let points = points.clone()[..points.len() - remainder].to_vec();
    let mut tuples = Vec::new();
    for i in (0..points.len()).step_by(4) {
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = points[i + 2];
        let p4 = points[i + 3];
        tuples.push((p1, p2, p3, p4));
    }
    return tuples;
}

pub fn get_subobjects_recursively(vec_features: &VectorObject) -> Vec<VectorObject> {
    let mut subobjects = Vec::new();
    for subobject in &vec_features.subobjects {
        subobjects.push(subobject.clone());
        subobjects.extend(get_subobjects_recursively(subobject));
    }
    return subobjects;
}

pub fn scale_points(points: &Vec<(f64, f64)>, scale: f64) -> Vec<(f64, f64)> {
    let mut new_points = Vec::new();
    for point in points {
        new_points.push((point.0 * scale, point.1 * scale));
    }
    return new_points;
}


pub fn stretch_points(points: &Vec<(f64, f64)>, stretch: (f64, f64)) -> Vec<(f64, f64)> {
    let mut new_points = Vec::new();
    for point in points {
        new_points.push((point.0 * stretch.0, point.1 * stretch.1));
    }
    return new_points;
}


pub fn shift_points(points: &Vec<(f64, f64)>, shift: (f64, f64)) -> Vec<(f64, f64)> {
    let mut new_points = Vec::new();
    for point in points {
        new_points.push((point.0 + shift.0, point.1 + shift.1));
    }
    return new_points;
}


#[derive(Clone, Debug)]
pub struct VectorObject {
    pub points: Vec<(f64, f64)>,
    pub fill: GradientImageOrColor,
    pub fill_rule: &'static str,
    pub stroke: GradientImageOrColor,
    pub stroke_width: f64,
    pub line_cap: &'static str,
    pub line_join: &'static str,
    pub subobjects: Vec<VectorObject>,
    pub index: usize
}


impl VectorObject {
    pub fn new() -> VectorObject {
        return VectorObject {
            points: Vec::new(),
            fill: GradientImageOrColor::Color(Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }),
            fill_rule: "nonzero",
            stroke: GradientImageOrColor::Color(Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }),
            stroke_width: 1.0,
            line_cap: "butt",
            line_join: "miter",
            subobjects: Vec::new(),
            index: 0,
        };
    }
    pub fn get_critical_point(&self, key: (f64, f64)) -> (f64, f64) {
        let bounding_box = self.get_bounding_box();
        let center_x = (bounding_box.0.0 + bounding_box.1.0) / 2.0;
        let center_y = (bounding_box.0.1 + bounding_box.1.1) / 2.0;
        let min_x = bounding_box.0.0;
        let min_y = bounding_box.0.1;
        let max_x = bounding_box.1.0;
        let max_y = bounding_box.1.1;
        let key_x = key.0;
        let key_y = key.1;
        let x_coord = if key_x > 0.0 {
            max_x
        } else if key_x < 0.0 {
            min_x
        } else {
            center_x
        };
        let y_coord = if key_y > 0.0 {
            max_y
        } else if key_y < 0.0 {
            min_y
        } else {
            center_y
        };
        return (x_coord, y_coord);
    }
    pub fn get_fill_opacity(&self) -> f64 {
        match &self.get_fill() {
            GradientImageOrColor::Color(color) => return color.alpha,
            GradientImageOrColor::LinearGradient(gradient) => return gradient.alpha,
            GradientImageOrColor::RadialGradient(gradient) => return gradient.alpha,
            GradientImageOrColor::Image(image) => return image.alpha,
        }
    }
    pub fn get_stroke_opacity(&self) -> f64 {
        match &self.get_stroke() {
            GradientImageOrColor::Color(color) => return color.alpha,
            GradientImageOrColor::LinearGradient(gradient) => return gradient.alpha,
            GradientImageOrColor::RadialGradient(gradient) => return gradient.alpha,
            GradientImageOrColor::Image(image) => return image.alpha,
        }
    }
    pub fn get_subpaths(&self) -> Vec<Vec<(f64, f64)>> {
        return generate_subpaths(self.get_points());
    }
    pub fn get_bounding_box(&self) -> ((f64, f64), (f64, f64)) {
        let mut min_x = std::f64::INFINITY;
        let mut min_y = std::f64::INFINITY;
        let mut max_x = std::f64::NEG_INFINITY;
        let mut max_y = std::f64::NEG_INFINITY;
        for point in self.merged_points() {
            if point.0 < min_x {
                min_x = point.0;
            }
            if point.0 > max_x {
                max_x = point.0;
            }
            if point.1 < min_y {
                min_y = point.1;
            }
            if point.1 > max_y {
                max_y = point.1;
            }
        }
        return ((min_x, min_y), (max_x, max_y));
    }
    pub fn get_center(&self) -> (f64, f64) {
        let ((min_x, min_y), (max_x, max_y)) = self.get_bounding_box();
        return ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);
    }
    pub fn get_center_of_mass(&self) -> (f64, f64) {
        let points = self.get_points();
        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        for point in points {
            x_sum += point.0;
            y_sum += point.1;
        }
        return (x_sum / points.len() as f64, y_sum / points.len() as f64);
    }
    pub fn get_height(&self) -> f64 {
        let ((_, min_y), (_, max_y)) = self.get_bounding_box();
        return max_y - min_y;
    }
    pub fn get_width(&self) -> f64 {
        let ((min_x, _), (max_x, _)) = self.get_bounding_box();
        return max_x - min_x;
    }
    pub fn get_cubic_bezier_tuples(&self) -> Vec<((f64, f64), (f64, f64), (f64, f64), (f64, f64))> {
        return generate_cubic_bezier_tuples(self.get_points());
    }
    pub fn get_index(&self) -> usize {
        return self.index;
    }
    pub fn set_index(&self, index: usize) -> Self {
        return VectorObject {
            points: self.points.clone(),
            fill_rule: self.fill_rule,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: index,
        };
    }
    pub fn merged_points(&self) -> Vec<(f64, f64)> {
        let mut merged_points = self.points.clone();
        merged_points.extend(self.get_subobjects().iter().map(|subobject| subobject.merged_points()).flatten());
        return merged_points;
    }
    pub fn increment_index(&self, increment: usize, recursive: bool) -> Self {
        if recursive {
            return VectorObject {
                points: self.points.clone(),
                fill_rule: self.fill_rule,
                fill: self.fill.clone(),
                stroke: self.stroke.clone(),
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.increment_index(increment, true)).collect(),
                index: self.index + increment,
            };
        }
        return VectorObject {
            points: self.points.clone(),
            fill_rule: self.fill_rule,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index + increment,
        };
    }
    pub fn apply_function(&self, f: &impl Fn(f64, f64) -> (f64, f64), recursive: bool, about_point: Option<(f64, f64)>, about_edge: Option<(f64, f64)>) -> Self {
        let edge = match about_edge {
            Some(edge) => edge,
            None => self.get_critical_point((0.0, 0.0)),
        };
        let point = match about_point {
            Some(point) => point,
            None => edge,
        };
        let points = self.points.iter().map(|(x, y)| {
            let (x, y) = f(*x - point.0, *y - point.1);
            return (x + point.0, y + point.1);
        }).collect::<Vec<(f64, f64)>>();
        let result = self.set_points(points);
        if recursive {
            return result.set_subobjects(
                self.get_subobjects().iter().map(|subobject| subobject.apply_function(f, true, about_point, about_edge)).collect()
            );
        }
        return result;
    }
    pub fn get_points(&self) -> &Vec<(f64, f64)> {
        return &self.points;
    }
    pub fn get_partial_copy(&self, start: f64, end: f64, recursive: bool) -> VectorObject {
        return get_partial_points(self, start, end, recursive);
    }
    pub fn get_fill(&self) -> GradientImageOrColor {
        return self.fill.clone();
    }
    pub fn get_stroke(&self) -> GradientImageOrColor {
        return self.stroke.clone();
    }
    pub fn get_stroke_width(&self) -> f64 {
        return self.stroke_width;
    }
    pub fn get_line_cap(&self) -> &'static str {
        return &self.line_cap;
    }
    pub fn get_line_join(&self) -> &'static str {
        return &self.line_join;
    }
    pub fn get_subobjects(&self) -> Vec<VectorObject> {
        return self.subobjects.clone();
    }
    pub fn scale(&self, scale_factor: f64, recursive: bool) -> VectorObject {
        if !recursive {
            return VectorObject {
                points: scale_points(&self.points, scale_factor),
                fill_rule: self.fill_rule,
                fill: self.fill.clone(),
                stroke: self.stroke.clone(),
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.clone(),
                index: self.index,
            };
        }
        return VectorObject {
            points: scale_points(&self.points, scale_factor),
            fill_rule: self.fill_rule,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.iter().map(|subobject| subobject.scale(scale_factor, true)).collect(),
            index: self.index,
        };
    }
    pub fn set_subobject(
        &self,
        index: usize,
        subobject: VectorObject
    ) -> Self {
        let mut new_subobjects = self.subobjects.clone();
        new_subobjects[index] = subobject;
        return VectorObject {
            points: self.points.clone(),
            fill_rule: self.fill_rule,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: new_subobjects,
            index: self.index,
        };
    }
    pub fn set_slice_subobjects(
        &self,
        start: usize,
        end: usize,
        subobjects: Vec<VectorObject>
    ) -> Self {
        let mut new_subobjects = self.subobjects.clone();
        new_subobjects.splice(start..end, subobjects.iter().cloned());
        return VectorObject {
            points: self.points.clone(),
            fill_rule: self.fill_rule,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: new_subobjects,
            index: self.index,
        };
    }
    pub fn stretch(&self, stretch: (f64, f64), recursive: bool) -> Self {
        if !recursive {
            return VectorObject {
                points: stretch_points(&self.points, stretch),
                fill: self.fill.clone(),
                fill_rule: self.fill_rule,
                stroke: self.stroke.clone(),
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.clone(),
                index: self.index,
            };
        }
        return VectorObject {
            points: stretch_points(&self.points, stretch),
            fill: self.fill.clone(),
            fill_rule: self.fill_rule,
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.iter().map(|subobject| subobject.stretch(stretch, true)).collect(),
            index: self.index,
        };
    }
    pub fn get_subobject(
        &self,
        index: usize
    ) -> VectorObject {
        return self.subobjects[index].clone();
    }
    pub fn slice_subobjects(
        &self,
        start: usize,
        end: usize
    ) -> Vec<Self> {
        return self.subobjects[start..end].to_vec();
    }
    pub fn shift(&self, shift: (f64, f64), recursive: bool) -> VectorObject {
        if !recursive {
            return VectorObject {
                points: shift_points(&self.points, shift),
                fill_rule: self.fill_rule,
                fill: self.fill.clone(),
                stroke: self.stroke.clone(),
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.clone(),
                index: self.index,
            };
        }
        return VectorObject {
            points: shift_points(&self.points, shift),
            fill_rule: self.fill_rule,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.iter().map(|subobject| subobject.shift(shift, true)).collect(),
            index: self.index,
        };
    }
    pub fn move_to(&self, position: (f64, f64), recursive: bool) -> VectorObject {
        let center = self.get_center();
        let shift = (position.0 - center.0, position.1 - center.1);
        return self.shift(shift, recursive);
    }
    pub fn set_fill(&self, fill: GradientImageOrColor, recursive: bool) -> VectorObject {
        if recursive {
            return VectorObject {
                points: self.points.clone(),
                fill_rule: self.fill_rule,
                fill: fill.clone(),
                stroke: self.stroke.clone(),
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_fill(fill.clone(), true)).collect(),
                index: self.index,
            };
        }
        return VectorObject {
            points: self.points.clone(),
            fill_rule: self.fill_rule,
            fill: fill,
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
        };
    }
    pub fn set_fill_opacity(&self, opacity: f64, recursive: bool) -> VectorObject {
        let new_fill = match &self.fill {
            GradientImageOrColor::Color(color) => GradientImageOrColor::Color(Color {
                red: color.red,
                green: color.green,
                blue: color.blue,
                alpha: opacity
            }),
            GradientImageOrColor::LinearGradient(gradient) => GradientImageOrColor::LinearGradient(LinearGradient {
                x1: gradient.x1,
                x2: gradient.x2,
                y1: gradient.y1,
                y2: gradient.y2,
                stops: gradient.stops.clone(),
                alpha: opacity
            }),
            GradientImageOrColor::RadialGradient(gradient) => GradientImageOrColor::RadialGradient(RadialGradient {
                cx: gradient.cx,
                cy: gradient.cy,
                r: gradient.r,
                fx: gradient.fx,
                fy: gradient.fy,
                stops: gradient.stops.clone(),
                alpha: opacity
            }),
            GradientImageOrColor::Image(image) => GradientImageOrColor::Image(Image {
                image_base64: image.image_base64.clone(),
                mime_type: image.mime_type.clone(),
                top_left_corner: image.top_left_corner,
                bottom_right_corner: image.bottom_right_corner,
                alpha: opacity
            }),
        };
        if recursive {
            return VectorObject {
                points: self.points.clone(),
                fill_rule: self.fill_rule,
                fill: new_fill,
                stroke: self.stroke.clone(),
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_fill_opacity(opacity, true)).collect(),
                index: self.index,
            };
        }
        return VectorObject {
            points: self.points.clone(),
            fill_rule: self.fill_rule,
            fill: new_fill,
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
        };
    }
    pub fn set_stroke(&self, stroke: GradientImageOrColor, recursive: bool) -> VectorObject {
        if recursive {
            return VectorObject {
                points: self.points.clone(),
                fill_rule: self.fill_rule,
                fill: self.fill.clone(),
                stroke: stroke.clone(),
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_stroke(stroke.clone(), true)).collect(),
                index: self.index,
            };
        }
        return VectorObject {
            points: self.points.clone(),
            fill: self.fill.clone(),
            fill_rule: self.fill_rule,
            stroke: stroke,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
        };
    }
    pub fn add(&self, other: &VectorObject) -> Self {
        let mut new_subobjects = self.subobjects.clone();
        new_subobjects.push(other.clone());
        return self.set_subobjects(new_subobjects);
    }
    pub fn remove(&self, index: usize) -> Self {
        let mut new_subobjects = self.subobjects.clone();
        new_subobjects.remove(index);
        return self.set_subobjects(new_subobjects);
    }
    pub fn set_stroke_opacity(&self, opacity: f64, recursive: bool) -> VectorObject {
        let new_stroke = match &self.stroke {
            GradientImageOrColor::Color(color) => GradientImageOrColor::Color(Color {
                red: color.red,
                green: color.green,
                blue: color.blue,
                alpha: opacity
            }),
            GradientImageOrColor::LinearGradient(gradient) => GradientImageOrColor::LinearGradient(LinearGradient {
                x1: gradient.x1,
                x2: gradient.x2,
                y1: gradient.y1,
                y2: gradient.y2,
                stops: gradient.stops.iter().map(|stop| GradientStop {
                    offset: stop.offset,
                    color: Color {
                        red: stop.color.red,
                        green: stop.color.green,
                        blue: stop.color.blue,
                        alpha: stop.color.alpha
                    }
                }).collect(),
                alpha: opacity
            }),
            GradientImageOrColor::RadialGradient(gradient) => GradientImageOrColor::RadialGradient(RadialGradient {
                cx: gradient.cx,
                cy: gradient.cy,
                r: gradient.r,
                fx: gradient.fx,
                fy: gradient.fy,
                stops: gradient.stops.clone(),
                alpha: opacity
            }),
            GradientImageOrColor::Image(image) => GradientImageOrColor::Image(Image {
                image_base64: image.image_base64.clone(),
                mime_type: image.mime_type.clone(),
                top_left_corner: image.top_left_corner,
                bottom_right_corner: image.bottom_right_corner,
                alpha: opacity
            }),
        };
        if recursive {
            return VectorObject {
                points: self.points.clone(),
                fill_rule: self.fill_rule,
                fill: self.fill.clone(),
                stroke: new_stroke,
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_stroke_opacity(opacity, true)).collect(),
                index: self.index,
            };
        }
        return VectorObject {
            points: self.points.clone(),
            fill: self.fill.clone(),
            fill_rule: self.fill_rule,
            stroke: new_stroke,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
        };
    }
    pub fn set_stroke_width(&self, stroke_width: f64, recursive: bool) -> VectorObject {
        if recursive {
            return VectorObject {
                points: self.points.clone(),
                fill_rule: self.fill_rule,
                fill: self.fill.clone(),
                stroke: self.stroke.clone(),
                stroke_width: stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_stroke_width(stroke_width, true)).collect(),
                index: self.index,
            };
        }
        return VectorObject {
            points: self.points.clone(),
            fill_rule: self.fill_rule,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
        };
    }
    pub fn set_line_cap(&self, line_cap: &'static str, recursive: bool) -> VectorObject {
        if recursive {
            return VectorObject {
                points: self.points.clone(),
                fill_rule: self.fill_rule,
                fill: self.fill.clone(),
                stroke: self.stroke.clone(),
                stroke_width: self.stroke_width,
                line_cap: line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_line_cap(line_cap, true)).collect(),
                index: self.index,
            };
        }
        return VectorObject {
            points: self.points.clone(),
            fill_rule: self.fill_rule,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
        };
    }
    pub fn set_line_join(&self, line_join: &'static str, recursive: bool) -> Self {
        if recursive {
            return VectorObject {
                fill_rule: self.fill_rule,
                points: self.points.clone(),
                fill: self.fill.clone(),
                stroke: self.stroke.clone(),
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_line_join(line_join, true)).collect(),
                index: self.index,
            };
        }
        return VectorObject {
            points: self.points.clone(),
            fill_rule: self.fill_rule,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
        };
    }
    pub fn set_points(&self, points: Vec<(f64, f64)>) -> Self {
        return VectorObject {
            points: points,
            fill_rule: self.fill_rule,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
        };
    }
    pub fn set_subobjects(&self, subobjects: Vec<VectorObject>) -> Self {
        return VectorObject {
            points: self.points.clone(),
            fill_rule: self.fill_rule,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: subobjects,
            index: self.index,
        };
    }
    pub fn rotate(&self, angle: f64, recursive: bool) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let new_points = self.get_points().iter().map(|point| {
            let x = point.0;
            let y = point.1;
            return (x * cos - y * sin, x * sin + y * cos);
        }).collect();
        if !recursive {
            return VectorObject {
                points: new_points,
                fill_rule: self.fill_rule,
                fill: self.fill.clone(),
                stroke: self.stroke.clone(),
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.clone(),
                index: self.index,
            };
        }
        return VectorObject {
            points: new_points,
            fill_rule: self.fill_rule,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.iter().map(|subobject| subobject.rotate(angle, true)).collect(),
            index: self.index,
        };
    }
    pub fn next_to_other(
        &self,
        other: &VectorObject,
        direction: (f64, f64),
        buff: f64,
        aligned_edge: (f64, f64),
        recursive: bool
    ) -> Self {
        let key1 = (direction.0 + aligned_edge.0, direction.1 + aligned_edge.1);
        let key2 = (-direction.0 + aligned_edge.0, -direction.1 + aligned_edge.1);
        let target_point = other.get_critical_point(key1);
        let point_to_align = self.get_critical_point(key2);
        let shift = (target_point.0 - point_to_align.0 + buff * direction.0, target_point.1 - point_to_align.1 + buff * direction.1);
        let result = self.shift(shift, recursive);
        return result;
    }
    pub fn next_to_point(
        &self,
        point: (f64, f64),
        direction: (f64, f64),
        buff: f64,
        aligned_edge: (f64, f64),
        recursive: bool
    ) -> VectorObject {
        let key2 = (-direction.0 + aligned_edge.0, -direction.1 + aligned_edge.1);
        let point_to_align = self.get_critical_point(key2);
        let shift = (point.0 - point_to_align.0 + buff * direction.0, point.1 - point_to_align.1 + buff * direction.1);
        let result = self.shift(shift, recursive);
        return result;
    }
    pub fn arrange_subobjects(
        &self,
        direction: (f64, f64),
        buff: f64,
        aligned_edge: (f64, f64),
        recursive: bool
    ) -> Self {
        if self.subobjects.len() == 0 {
            return self.clone();
        }
        let mut result = self.clone();
        let first_subobject = self.subobjects[0].clone();
        let mut new_subobjects: Vec<VectorObject> = vec![first_subobject.clone()];
        for i in 1..self.subobjects.len() {
            let subobject = self.subobjects[i].clone();
            let previous_subobject = new_subobjects[i - 1].clone();
            let next_subobject = subobject.next_to_other(&previous_subobject, direction, buff, aligned_edge, recursive);
            new_subobjects.push(next_subobject);
        }
        result.subobjects = new_subobjects;
        return result;
    }
    pub fn set_background_image(
        &self,
        image_base64: String,
        mime_type: String,
        width: usize,
        height: usize,
        recursive: bool
    ) -> Self {
        if recursive {
            return VectorObject {
                points: self.points.clone(),
                fill_rule: self.fill_rule,
                fill: GradientImageOrColor::Image(Image {
                    image_base64: image_base64.clone(),
                    mime_type: mime_type.clone(),
                    top_left_corner: (0.0, 0.0),
                    bottom_right_corner: (width as f64, height as f64),
                    alpha: 1.0
                }),
                stroke: GradientImageOrColor::Image(Image {
                    image_base64: image_base64.clone(),
                    mime_type: mime_type.clone(),
                    top_left_corner: (0.0, 0.0),
                    bottom_right_corner: (width as f64, height as f64),
                    alpha: 1.0
                }),
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_background_image(image_base64.clone(), mime_type.clone(), width, height, true)).collect(),
                index: self.index,
            };
        }
        return VectorObject {
            points: self.points.clone(),
            fill_rule: self.fill_rule,
            fill: GradientImageOrColor::Image(Image {
                image_base64: image_base64.clone(),
                mime_type: mime_type.clone(),
                top_left_corner: (0.0, 0.0),
                bottom_right_corner: (width as f64, height as f64),
                alpha: 1.0
            }),
            stroke: GradientImageOrColor::Image(Image {
                image_base64: image_base64.clone(),
                mime_type: mime_type.clone(),
                top_left_corner: (0.0, 0.0),
                bottom_right_corner: (width as f64, height as f64),
                alpha: 1.0
            }),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
        };
    }
    pub fn set_fill_image_corners(&self, top_left_corner: (f64, f64), bottom_right_corner: (f64, f64), recursive: bool) -> Self {
        let new_stroke = match &self.stroke {
            GradientImageOrColor::Image(image) => GradientImageOrColor::Image(Image {
                image_base64: image.image_base64.clone(),
                mime_type: image.mime_type.clone(),
                top_left_corner: top_left_corner,
                bottom_right_corner: bottom_right_corner,
                alpha: image.alpha
            }),
            _ => self.stroke.clone()
        };
        if recursive {
            return VectorObject {
                points: self.points.clone(),
                fill: self.fill.clone(),
                fill_rule: self.fill_rule,
                stroke: new_stroke,
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_stroke_image_corners(top_left_corner, bottom_right_corner, true)).collect(),
                index: self.index,
            };
        }
        return VectorObject {
            points: self.points.clone(),
            fill: self.fill.clone(),
            fill_rule: self.fill_rule,
            stroke: new_stroke,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
        };
    }
    pub fn set_fill_rule(&self, fill_rule: &'static str, recursive: bool) -> Self {
        if recursive {
            return VectorObject {
                points: self.points.clone(),
                fill_rule: fill_rule,
                fill: self.fill.clone(),
                stroke: self.stroke.clone(),
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_fill_rule(fill_rule, true)).collect(),
                index: self.index,
            };
        }
        return VectorObject {
            points: self.points.clone(),
            fill_rule: fill_rule,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
        };
    }
    pub fn get_fill_rule(&self) -> &'static str {
        return self.fill_rule;
    }
    pub fn set_stroke_image_corners(&self, top_left_corner: (f64, f64), bottom_right_corner: (f64, f64), recursive: bool) -> Self {
        let new_fill = match &self.fill {
            GradientImageOrColor::Image(image) => GradientImageOrColor::Image(Image {
                image_base64: image.image_base64.clone(),
                mime_type: image.mime_type.clone(),
                top_left_corner: top_left_corner,
                bottom_right_corner: bottom_right_corner,
                alpha: image.alpha
            }),
            _ => self.fill.clone()
        };
        if recursive {
            return VectorObject {
                points: self.points.clone(),
                fill: new_fill,
                fill_rule: self.fill_rule,
                stroke: self.stroke.clone(),
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_fill_image_corners(top_left_corner, bottom_right_corner, true)).collect(),
                index: self.index,
            };
        }
        return VectorObject {
            points: self.points.clone(),
            fill: new_fill,
            fill_rule: self.fill_rule,
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
        };
    }
    pub fn get_fill_image_corners(&self) -> ((f64, f64), (f64, f64)) {
        match &self.fill {
            GradientImageOrColor::Image(image) => return (image.top_left_corner, image.bottom_right_corner),
            _ => return ((0.0, 0.0), (0.0, 0.0))
        }
    }
    pub fn get_stroke_image_corners(&self) -> ((f64, f64), (f64, f64)) {
        match &self.stroke {
            GradientImageOrColor::Image(image) => return (image.top_left_corner, image.bottom_right_corner),
            _ => return ((0.0, 0.0), (0.0, 0.0))
        }
    }
    pub fn get_pieces(&self, n_pieces: usize) -> VectorObject {
        let template = self.set_subobjects(Vec::new());
        let alphas = (0..n_pieces + 1).map(|i| i as f64 / n_pieces as f64).collect::<Vec<f64>>();
        let mut pieces = Vec::new();
        for i in 0..n_pieces {
            let start = alphas[i];
            let end = alphas[i + 1];
            let piece = template.get_partial_copy(start, end, true);
            pieces.push(piece);
        }
        return template.set_subobjects(pieces).set_points(Vec::new());
    }
}
