use crate::utils::{integer_interpolate, consider_points_equals};
use crate::utils::bezier;


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


fn get_partial_points(
    vector_features: &VectorFeatures,
    start: f64,
    end: f64,
    recursive: bool
) -> VectorFeatures {
    let points = vector_features.get_points();
    let fill_color = vector_features.get_fill_color();
    let stroke_color = vector_features.get_stroke_color();
    let stroke_width = vector_features.get_stroke_width();
    let line_cap = vector_features.get_line_cap();
    let line_join = vector_features.get_line_join();
    let mut subobjects = (&vector_features.subobjects).to_vec();
    if start <= 0.0 && end >= 1.0 {
        return VectorFeatures {
            points: points.clone(),
            fill_color: fill_color.clone(),
            stroke_color: stroke_color,
            stroke_width: stroke_width,
            line_cap: line_cap,
            line_join: line_join,
            subobjects: subobjects,
            index: vector_features.index,
            background_image: vector_features.background_image.clone(),
            image_position: vector_features.image_position
        };
    }
    let bezier_quads = vector_features.get_cubic_bezier_tuples();
    if bezier_quads.len() == 0 {
        return VectorFeatures {
            points: points.clone(),
            fill_color: fill_color.clone(),
            stroke_color: stroke_color,
            stroke_width: stroke_width,
            line_cap: line_cap,
            line_join: line_join,
            subobjects: subobjects.iter().map(|subobject| get_partial_points(subobject, start, end, recursive)).collect(),
            index: vector_features.index,
            background_image: vector_features.background_image.clone(),
            image_position: vector_features.image_position
        };
    }
    let (lower_index, lower_residue) = integer_interpolate(0.0, bezier_quads.len() as f64, start);
    let (upper_index, upper_residue) = integer_interpolate(0.0, bezier_quads.len() as f64, end);
    if lower_index == upper_index {
        return VectorFeatures {
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
            fill_color: fill_color.clone(),
            stroke_color: stroke_color.clone(),
            stroke_width: stroke_width,
            line_cap: line_cap,
            line_join: line_join,
            subobjects: subobjects.iter().map(|subobject| get_partial_points(subobject, start, end, true)).collect(),
            index: vector_features.index,
            background_image: vector_features.background_image.clone(),
            image_position: vector_features.image_position
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
    return VectorFeatures {
        points: new_points,
        fill_color: fill_color.clone(),
        stroke_color: stroke_color.clone(),
        stroke_width: stroke_width,
        line_cap: line_cap,
        line_join: line_join,
        subobjects: subobjects,
        index: vector_features.index,
        background_image: vector_features.background_image.clone(),
        image_position: vector_features.image_position
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


pub fn generate_subpaths_wasm(
    points: js_sys::Array
) -> Vec<Vec<(f64, f64)>> {
    let points = points.to_vec().iter().map(
        |point| {
            let point = js_sys::Array::from(point);
            let x = point.get(0).as_f64().unwrap();
            let y = point.get(1).as_f64().unwrap();
            return (x, y);
        }
    ).collect::<Vec<(f64, f64)>>();
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

pub fn get_subobjects_recursively(vec_features: &VectorFeatures) -> Vec<VectorFeatures> {
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


pub trait VectorObject {
    fn get_index(&self) -> usize;
    fn increment_index(&self, increment: usize, recursive: bool) -> Self;
    fn get_points(&self) -> &Vec<(f64, f64)>;
    fn get_fill_color(&self) -> (f64, f64, f64, f64);
    fn get_stroke_color(&self) -> (f64, f64, f64, f64);
    fn get_stroke_width(&self) -> f64;
    fn get_line_cap(&self) -> &'static str;
    fn get_line_join(&self) -> &'static str;
    fn get_partial_copy(&self, start: f64, end: f64, recursive: bool) -> Self;
    fn get_subpaths(&self) -> Vec<Vec<(f64, f64)>> {
        return generate_subpaths(self.get_points());
    }
    fn get_cubic_bezier_tuples(&self) -> Vec<((f64, f64), (f64, f64), (f64, f64), (f64, f64))> {
        return generate_cubic_bezier_tuples(self.get_points());
    }
    fn get_subobjects(&self) -> Vec<VectorFeatures>;
    fn scale(&self, scale_factor: f64, recursive: bool) -> Self;
    fn stretch(&self, stretch: (f64, f64), recursive: bool) -> Self;
    fn shift(&self, shift: (f64, f64), recursive: bool) -> Self;
    fn merged_points(&self) -> Vec<(f64, f64)> {
        let mut merged_points = self.get_points().clone();
        merged_points.extend(self.get_subobjects().iter().map(|subobject| subobject.merged_points()).flatten());
        return merged_points;
    }
    fn get_bounding_box(&self) -> ((f64, f64), (f64, f64)) {
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
    fn get_center(&self) -> (f64, f64) {
        let ((min_x, min_y), (max_x, max_y)) = self.get_bounding_box();
        return ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);
    }
    fn get_center_of_mass(&self) -> (f64, f64) {
        let points = self.get_points();
        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        for point in points {
            x_sum += point.0;
            y_sum += point.1;
        }
        return (x_sum / points.len() as f64, y_sum / points.len() as f64);
    }
    fn get_height(&self) -> f64 {
        let ((_, min_y), (_, max_y)) = self.get_bounding_box();
        return max_y - min_y;
    }
    fn get_width(&self) -> f64 {
        let ((min_x, _), (max_x, _)) = self.get_bounding_box();
        return max_x - min_x;
    }
    fn set_fill_color(&self, fill_color: (f64, f64, f64, f64), recursive: bool) -> Self;
    fn set_fill_opacity(&self, opacity: f64, recursive: bool) -> Self;
    fn move_to(&self, position: (f64, f64), recursive: bool) -> Self;
    fn set_stroke_color(&self, stroke_color: (f64, f64, f64, f64), recursive: bool) -> Self;
    fn set_stroke_opacity(&self, opacity: f64, recursive: bool) -> Self;
    fn set_stroke_width(&self, stroke_width: f64, recursive: bool) -> Self;
    fn set_line_cap(&self, line_cap: &'static str, recursive: bool) -> Self;
    fn set_line_join(&self, line_join: &'static str, recursive: bool) -> Self;
    fn set_points(&self, points: Vec<(f64, f64)>) -> Self;
    fn set_subobjects(&self, subobjects: Vec<VectorFeatures>) -> Self;
    fn rotate(&self, angle: f64, recursive: bool) -> Self;
    fn get_critical_point(&self, key: (f64, f64)) -> (f64, f64) {
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
    fn next_to_other(
        &self,
        other: &VectorFeatures,
        direction: (f64, f64),
        buff: f64,
        aligned_edge: (f64, f64),
        recursive: bool
    ) -> Self;
    fn arrange_subobjects(
        &self,
        direction: (f64, f64),
        buff: f64,
        aligned_edge: (f64, f64),
        recursive: bool
    ) -> Self;
    fn next_to_point(
        &self,
        point: (f64, f64),
        direction: (f64, f64),
        buff: f64,
        aligned_edge: (f64, f64),
        recursive: bool
    ) -> Self;
    fn set_background_image(&self, image: web_sys::HtmlImageElement, recursive: bool) -> Self;
    fn get_background_image(&self) -> Option<web_sys::HtmlImageElement>;
    fn set_image_position(&self, position: (f64, f64), recursive: bool) -> Self;
    fn get_image_position(&self) -> (f64, f64);
}

#[derive(Clone, Debug)]
pub struct VectorFeatures {
    pub points: Vec<(f64, f64)>,
    pub fill_color: (f64, f64, f64, f64),
    pub stroke_color: (f64, f64, f64, f64),
    pub stroke_width: f64,
    pub line_cap: &'static str,
    pub line_join: &'static str,
    pub subobjects: Vec<VectorFeatures>,
    pub index: usize,
    pub background_image: Option<web_sys::HtmlImageElement>,
    pub image_position: (f64, f64)
}

impl VectorObject for VectorFeatures {
    fn get_index(&self) -> usize {
        return self.index;
    }
    fn increment_index(&self, increment: usize, recursive: bool) -> Self {
        if recursive {
            return VectorFeatures {
                points: self.points.clone(),
                fill_color: self.fill_color,
                stroke_color: self.stroke_color,
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.increment_index(increment, true)).collect(),
                index: self.index + increment,
                background_image: self.background_image.clone(),
                image_position: self.image_position
            };
        }
        return VectorFeatures {
            points: self.points.clone(),
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index + increment,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn get_points(&self) -> &Vec<(f64, f64)> {
        return &self.points;
    }
    fn get_partial_copy(&self, start: f64, end: f64, recursive: bool) -> VectorFeatures {
        return get_partial_points(self, start, end, recursive);
    }
    fn get_fill_color(&self) -> (f64, f64, f64, f64) {
        return self.fill_color;
    }
    fn get_stroke_color(&self) -> (f64, f64, f64, f64) {
        return self.stroke_color;
    }
    fn get_stroke_width(&self) -> f64 {
        return self.stroke_width;
    }
    fn get_line_cap(&self) -> &'static str {
        return &self.line_cap;
    }
    fn get_line_join(&self) -> &'static str {
        return &self.line_join;
    }
    fn get_subobjects(&self) -> Vec<VectorFeatures> {
        return self.subobjects.clone();
    }
    fn scale(&self, scale_factor: f64, recursive: bool) -> VectorFeatures {
        if !recursive {
            return VectorFeatures {
                points: scale_points(&self.points, scale_factor),
                fill_color: self.fill_color,
                stroke_color: self.stroke_color,
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.clone(),
                index: self.index,
                background_image: self.background_image.clone(),
                image_position: self.image_position
            };
        }
        return VectorFeatures {
            points: scale_points(&self.points, scale_factor),
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.iter().map(|subobject| subobject.scale(scale_factor, true)).collect(),
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn stretch(&self, stretch: (f64, f64), recursive: bool) -> Self {
        let center = self.get_center();
        if !recursive {
            return VectorFeatures {
                points: shift_points(&stretch_points(&self.points, stretch), (center.0 - self.get_center().0, center.1 - self.get_center().1)),
                fill_color: self.fill_color,
                stroke_color: self.stroke_color,
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.clone(),
                index: self.index,
                background_image: self.background_image.clone(),
                image_position: self.image_position
            };
        }
        return VectorFeatures {
            points: shift_points(&stretch_points(&self.points, stretch), (center.0 - self.get_center().0, center.1 - self.get_center().1)),
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.iter().map(|subobject| subobject.stretch(stretch, true)).collect(),
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn shift(&self, shift: (f64, f64), recursive: bool) -> VectorFeatures {
        if !recursive {
            return VectorFeatures {
                points: shift_points(&self.points, shift),
                fill_color: self.fill_color,
                stroke_color: self.stroke_color,
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.clone(),
                index: self.index,
                background_image: self.background_image.clone(),
                image_position: self.image_position
            };
        }
        return VectorFeatures {
            points: shift_points(&self.points, shift),
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.iter().map(|subobject| subobject.shift(shift, true)).collect(),
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn move_to(&self, position: (f64, f64), recursive: bool) -> VectorFeatures {
        let center = self.get_center();
        let shift = (position.0 - center.0, position.1 - center.1);
        return self.shift(shift, recursive);
    }
    fn set_fill_color(&self, fill_color: (f64, f64, f64, f64), recursive: bool) -> VectorFeatures {
        if recursive {
            return VectorFeatures {
                points: self.points.clone(),
                fill_color: fill_color,
                stroke_color: self.stroke_color,
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_fill_color(fill_color, true)).collect(),
                index: self.index,
                background_image: self.background_image.clone(),
                image_position: self.image_position
            };
        }
        return VectorFeatures {
            points: self.points.clone(),
            fill_color: fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn set_fill_opacity(&self, opacity: f64, recursive: bool) -> VectorFeatures {
        if recursive {
            return VectorFeatures {
                points: self.points.clone(),
                fill_color: (self.fill_color.0, self.fill_color.1, self.fill_color.2, opacity),
                stroke_color: self.stroke_color,
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_fill_opacity(opacity, true)).collect(),
                index: self.index,
                background_image: self.background_image.clone(),
                image_position: self.image_position
            };
        }
        return VectorFeatures {
            points: self.points.clone(),
            fill_color: (self.fill_color.0, self.fill_color.1, self.fill_color.2, opacity),
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn set_stroke_color(&self, stroke_color: (f64, f64, f64, f64), recursive: bool) -> Self {
        if recursive {
            return VectorFeatures {
                points: self.points.clone(),
                fill_color: self.fill_color,
                stroke_color: stroke_color,
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_stroke_color(stroke_color, true)).collect(),
                index: self.index,
                background_image: self.background_image.clone(),
                image_position: self.image_position
            };
        }
        return VectorFeatures {
            points: self.points.clone(),
            fill_color: self.fill_color,
            stroke_color: stroke_color,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn set_stroke_opacity(&self, opacity: f64, recursive: bool) -> VectorFeatures {
        if recursive {
            return VectorFeatures {
                points: self.points.clone(),
                fill_color: self.fill_color,
                stroke_color: (self.stroke_color.0, self.stroke_color.1, self.stroke_color.2, opacity),
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_stroke_opacity(opacity, true)).collect(),
                index: self.index,
                background_image: self.background_image.clone(),
                image_position: self.image_position
            };
        }
        return VectorFeatures {
            points: self.points.clone(),
            fill_color: self.fill_color,
            stroke_color: (self.stroke_color.0, self.stroke_color.1, self.stroke_color.2, opacity),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn set_stroke_width(&self, stroke_width: f64, recursive: bool) -> VectorFeatures {
        if recursive {
            return VectorFeatures {
                points: self.points.clone(),
                fill_color: self.fill_color,
                stroke_color: self.stroke_color,
                stroke_width: stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_stroke_width(stroke_width, true)).collect(),
                index: self.index,
                background_image: self.background_image.clone(),
                image_position: self.image_position
            };
        }
        return VectorFeatures {
            points: self.points.clone(),
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn set_line_cap(&self, line_cap: &'static str, recursive: bool) -> VectorFeatures {
        if recursive {
            return VectorFeatures {
                points: self.points.clone(),
                fill_color: self.fill_color,
                stroke_color: self.stroke_color,
                stroke_width: self.stroke_width,
                line_cap: line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_line_cap(line_cap, true)).collect(),
                index: self.index,
                background_image: self.background_image.clone(),
                image_position: self.image_position
            };
        }
        return VectorFeatures {
            points: self.points.clone(),
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            line_cap: line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn set_line_join(&self, line_join: &'static str, recursive: bool) -> Self {
        if recursive {
            return VectorFeatures {
                points: self.points.clone(),
                fill_color: self.fill_color,
                stroke_color: self.stroke_color,
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_line_join(line_join, true)).collect(),
                index: self.index,
                background_image: self.background_image.clone(),
                image_position: self.image_position
            };
        }
        return VectorFeatures {
            points: self.points.clone(),
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn set_points(&self, points: Vec<(f64, f64)>) -> Self {
        return VectorFeatures {
            points: points,
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn set_subobjects(&self, subobjects: Vec<VectorFeatures>) -> Self {
        return VectorFeatures {
            points: self.points.clone(),
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: subobjects,
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn rotate(&self, angle: f64, recursive: bool) -> Self {
        let center = self.get_center();
        let cos = angle.cos();
        let sin = angle.sin();
        let new_points = self.get_points().iter().map(|point| {
            let x = point.0 - center.0;
            let y = point.1 - center.1;
            return (
                x * cos - y * sin + center.0,
                x * sin + y * cos + center.1
            );
        }).collect();
        if !recursive {
            return VectorFeatures {
                points: new_points,
                fill_color: self.fill_color,
                stroke_color: self.stroke_color,
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.clone(),
                index: self.index,
                background_image: self.background_image.clone(),
                image_position: self.image_position
            };
        }
        return VectorFeatures {
            points: new_points,
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.iter().map(|subobject| subobject.rotate(angle, true)).collect(),
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: self.image_position
        };
    }
    fn next_to_other(
        &self,
        other: &VectorFeatures,
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
    fn next_to_point(
        &self,
        point: (f64, f64),
        direction: (f64, f64),
        buff: f64,
        aligned_edge: (f64, f64),
        recursive: bool
    ) -> VectorFeatures {
        let key2 = (-direction.0 + aligned_edge.0, -direction.1 + aligned_edge.1);
        let point_to_align = self.get_critical_point(key2);
        let shift = (point.0 - point_to_align.0 + buff * direction.0, point.1 - point_to_align.1 + buff * direction.1);
        let result = self.shift(shift, recursive);
        return result;
    }
    fn arrange_subobjects(
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
        let mut new_subobjects: Vec<VectorFeatures> = vec![first_subobject.clone()];
        for i in 1..self.subobjects.len() {
            let subobject = self.subobjects[i].clone();
            let previous_subobject = new_subobjects[i - 1].clone();
            let next_subobject = subobject.next_to_other(&previous_subobject, direction, buff, aligned_edge, recursive);
            new_subobjects.push(next_subobject);
        }
        result.subobjects = new_subobjects;
        return result;
    }
    fn set_background_image(&self, image: web_sys::HtmlImageElement, recursive: bool) -> Self {
        if recursive {
            return VectorFeatures {
                points: self.points.clone(),
                fill_color: self.fill_color,
                stroke_color: self.stroke_color,
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_background_image(image.clone(), true)).collect(),
                index: self.index,
                background_image: Some(image),
                image_position: self.image_position
            };
        }
        return VectorFeatures {
            points: self.points.clone(),
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
            background_image: Some(image),
            image_position: self.image_position
        };
    }
    fn get_background_image(&self) -> Option<web_sys::HtmlImageElement> {
        return self.background_image.clone();
    }
    fn set_image_position(&self, position: (f64, f64), recursive: bool) -> Self {
        if recursive {
            return VectorFeatures {
                points: self.points.clone(),
                fill_color: self.fill_color,
                stroke_color: self.stroke_color,
                stroke_width: self.stroke_width,
                line_cap: self.line_cap,
                line_join: self.line_join,
                subobjects: self.subobjects.iter().map(|subobject| subobject.set_image_position(position, true)).collect(),
                index: self.index,
                background_image: self.background_image.clone(),
                image_position: position
            };
        }
        return VectorFeatures {
            points: self.points.clone(),
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            line_cap: self.line_cap,
            line_join: self.line_join,
            subobjects: self.subobjects.clone(),
            index: self.index,
            background_image: self.background_image.clone(),
            image_position: position
        };
    }
    fn get_image_position(&self) -> (f64, f64) {
        return self.image_position;
    }
}
