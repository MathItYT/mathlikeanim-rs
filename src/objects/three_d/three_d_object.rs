use core::f64;
use std::{f64::consts::PI, future::Future, pin::Pin, vec};

use js_sys::{Function, Promise};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{future_to_promise, JsFuture};

use crate::{colors::{Color, GradientImageOrColor, GradientStop, LinearGradient}, objects::vector_object::VectorObject, utils::{interpolate, interpolate_tuple_3d}};

pub fn rot_matrix(angle: f64, axis: usize) -> [[f64; 3]; 3] {
    let mut matrix = [[0.0; 3]; 3];
    let (s, c) = angle.sin_cos();
    match axis {
        0 => {
            matrix[0][0] = 1.0;
            matrix[1][1] = c;
            matrix[1][2] = -s;
            matrix[2][1] = s;
            matrix[2][2] = c;
        },
        1 => {
            matrix[0][0] = c;
            matrix[0][2] = s;
            matrix[1][1] = 1.0;
            matrix[2][0] = -s;
            matrix[2][2] = c;
        },
        2 => {
            matrix[0][0] = c;
            matrix[0][1] = -s;
            matrix[1][0] = s;
            matrix[1][1] = c;
            matrix[2][2] = 1.0;
        },
        _ => ()
    }
    matrix
}

pub fn matrix_product(matrix1: [[f64; 3]; 3], matrix2: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut new_matrix = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            new_matrix[i][j] = matrix1[i][0] * matrix2[0][j] + matrix1[i][1] * matrix2[1][j] + matrix1[i][2] * matrix2[2][j];
        }
    }
    new_matrix
}

pub fn rot_matrix_euler(phi: f64, theta: f64, gamma: f64) -> [[f64; 3]; 3] {
    let matrix1 = rot_matrix(-theta - PI / 2.0, 2);
    let matrix2 = rot_matrix(-phi, 0);
    let matrix3 = rot_matrix(-gamma, 2);
    let result = matrix1;
    let result = matrix_product(matrix2, result);
    let result = matrix_product(matrix3, result);
    result
}

pub fn transpose_matrix(matrix: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut new_matrix = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            new_matrix[i][j] = matrix[j][i];
        }
    }
    new_matrix
}


pub fn matrix_times_points(matrix: [[f64; 3]; 3], points: Vec<(f64, f64, f64)>) -> Vec<(f64, f64, f64)> {
    points.iter().map(|point| {
        (
            matrix[0][0] * point.0 + matrix[0][1] * point.1 + matrix[0][2] * point.2,
            matrix[1][0] * point.0 + matrix[1][1] * point.1 + matrix[1][2] * point.2,
            matrix[2][0] * point.0 + matrix[2][1] * point.1 + matrix[2][2] * point.2
        )
    }).collect()
}


pub fn shift_points_3d(points: &Vec<(f64, f64, f64)>, shift: (f64, f64, f64)) -> Vec<(f64, f64, f64)> {
    points.iter().map(|point| {
        (point.0 + shift.0, point.1 + shift.1, point.2 + shift.2)
    }).collect()
}

pub fn ensure_valid_three_d_color(color: GradientImageOrColor) -> GradientImageOrColor {
    match color {
        GradientImageOrColor::Color(c) => GradientImageOrColor::Color(c),
        GradientImageOrColor::LinearGradient(g) => {
            if g.stops.len() == 0 {
                return GradientImageOrColor::Color(Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 });
            }
            if g.stops.len() == 1 {
                let stop = g.stops[0].clone();
                return GradientImageOrColor::Color(stop.color);
            }
            let new_stops = g.stops[0..2].to_vec();
            GradientImageOrColor::LinearGradient(LinearGradient {
                stops: new_stops,
                x1: g.x1,
                y1: g.y1,
                x2: g.x2,
                y2: g.y2,
                alpha: g.alpha
            })
        },
        GradientImageOrColor::RadialGradient(_) => {
            GradientImageOrColor::Color(Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 })
        },
        GradientImageOrColor::Image(_) => {
            GradientImageOrColor::Color(Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 })
        }
    }
}

pub fn get_shaded_rgb(
    color: &Color,
    point: (f64, f64, f64),
    unit_normal: (f64, f64, f64),
    light_source: &LightSource
) -> Color {
    let (lx, ly, lz) = light_source.position;
    let (x, y, z) = point;
    let red = color.red;
    let green = color.green;
    let blue = color.blue;
    let alpha = color.alpha;
    let mut to_sun = (lx - x, ly - y, lz - z);
    let to_sun_magnitude = (to_sun.0.powi(2) + to_sun.1.powi(2) + to_sun.2.powi(2)).sqrt();
    if to_sun_magnitude == 0.0 {
        to_sun = (0.0, 0.0, 0.0);
    } else {
        to_sun = (to_sun.0 / to_sun_magnitude, to_sun.1 / to_sun_magnitude, to_sun.2 / to_sun_magnitude);
    }
    let dot_product = to_sun.0 * unit_normal.0 + to_sun.1 * unit_normal.1 + to_sun.2 * unit_normal.2;
    let mut factor = 0.5 * dot_product.powi(3);
    if factor < 0.0 {
        factor *= 0.5;
    }
    let red = (red + factor).clamp(0.0, 1.0);
    let green = (green + factor).clamp(0.0, 1.0);
    let blue = (blue + factor).clamp(0.0, 1.0);
    Color { red, green, blue, alpha }
}

pub fn get_start_corner(points: &Vec<(f64, f64, f64)>) -> (f64, f64, f64) {
    points[0]
}

pub fn get_end_corner(points: &Vec<(f64, f64, f64)>) -> (f64, f64, f64) {
    points[((points.len() - 1) / 6) * 3]
}

pub fn cross_product(v1: (f64, f64, f64), v2: (f64, f64, f64)) -> (f64, f64, f64) {
    let x = v1.1 * v2.2 - v1.2 * v2.1;
    let y = v1.2 * v2.0 - v1.0 * v2.2;
    let z = v1.0 * v2.1 - v1.1 * v2.0;
    (x, y, z)
}

pub fn get_unit_normal(v1: (f64, f64, f64), v2: (f64, f64, f64)) -> (f64, f64, f64) {
    let cross = cross_product(v1, v2);
    let magnitude = (cross.0.powi(2) + cross.1.powi(2) + cross.2.powi(2)).sqrt();
    if magnitude == 0.0 {
        return (0.0, 0.0, 0.0);
    }
    (cross.0 / magnitude, cross.1 / magnitude, cross.2 / magnitude)
}

pub fn get_start_anchors(points: &Vec<(f64, f64, f64)>) -> Vec<(f64, f64, f64)> {
    let mut anchors = vec![];
    for i in (0..points.len()).step_by(4) {
        anchors.push(points[i]);
    }
    anchors
}

pub fn get_end_anchors(points: &Vec<(f64, f64, f64)>) -> Vec<(f64, f64, f64)> {
    let mut anchors = vec![];
    for i in (3..points.len()).step_by(4) {
        anchors.push(points[i]);
    }
    anchors
}

pub fn get_anchors(points: &Vec<(f64, f64, f64)>) -> Vec<(f64, f64, f64)> {
    let mut anchors = get_start_anchors(points);
    anchors.extend(get_end_anchors(points));
    anchors
}

pub fn get_corner_unit_normal(points: &Vec<(f64, f64, f64)>, index: usize) -> (f64, f64, f64) {
    if get_anchors(points).len() <= 2 {
        return (0.0, 1.0, 0.0);
    }
    let n_points = points.len();
    let im3 = if index >= 3 {
        index - 3
    } else {
        n_points - 4
    };
    let ip3 = if index + 3 < n_points {
        index + 3
    } else {
        3
    };
    let unit_normal = get_unit_normal(
        (points[ip3].0 - points[index].0, points[ip3].1 - points[index].1, points[ip3].2 - points[index].2),
        (points[im3].0 - points[index].0, points[im3].1 - points[index].1, points[im3].2 - points[index].2)
    );
    if unit_normal == (0.0, 0.0, 0.0) {
        return (0.0, 1.0, 0.0);
    }
    unit_normal
}

pub fn get_start_corner_unit_normal(points: &Vec<(f64, f64, f64)>) -> (f64, f64, f64) {
    get_corner_unit_normal(points, 0)
}

pub fn get_end_corner_unit_normal(points: &Vec<(f64, f64, f64)>) -> (f64, f64, f64) {
    get_corner_unit_normal(points, ((points.len() - 1) / 6) * 3)
}

pub fn get_shaded_color(
    color: &GradientImageOrColor,
    points: &Vec<(f64, f64, f64)>,
    light_source: &LightSource,
    camera: &Camera
) -> GradientImageOrColor {
    if points.len() == 0 {
        return color.clone();
    }
    let point1 = get_start_corner(points);
    let normal1 = get_start_corner_unit_normal(points);
    let point2 = get_end_corner(points);
    let normal2 = get_end_corner_unit_normal(points);
    let projected = project_points(&vec![point1, point2], camera);
    let point1_projected = projected[0];
    let point2_projected = projected[1];
    match color {
        GradientImageOrColor::Color(color) => {
            let color1 = get_shaded_rgb(color, point1, normal1, light_source);
            let color2 = get_shaded_rgb(color, point2, normal2, light_source);
            GradientImageOrColor::LinearGradient(LinearGradient {
                x1: point1_projected.0,
                y1: point1_projected.1,
                x2: point2_projected.0,
                y2: point2_projected.1,
                stops: vec![
                    GradientStop {
                        offset: 0.0,
                        color: color1,
                    },
                    GradientStop {
                        offset: 1.0,
                        color: color2,
                    },
                ],
                alpha: 1.0,
            })
        },
        GradientImageOrColor::LinearGradient(gradient) => {
            if gradient.stops.len() == 0 {
                return GradientImageOrColor::LinearGradient(LinearGradient {
                    x1: point1_projected.0,
                    y1: point1_projected.1,
                    x2: point2_projected.0,
                    y2: point2_projected.1,
                    stops: vec![],
                    alpha: 1.0
                });
            }
            if gradient.stops.len() == 1 {
                let stop = gradient.stops[0].clone();
                let color1 = get_shaded_rgb(&stop.color, point1, normal1, light_source);
                let color2 = get_shaded_rgb(&stop.color, point2, normal2, light_source);
                return GradientImageOrColor::LinearGradient(LinearGradient {
                    x1: point1_projected.0,
                    y1: point1_projected.1,
                    x2: point2_projected.0,
                    y2: point2_projected.1,
                    stops: vec![
                        GradientStop {
                            offset: 0.0,
                            color: color1,
                        },
                        GradientStop {
                            offset: 1.0,
                            color: color2,
                        },
                    ],
                    alpha: 1.0
                });
            }
            let stops = gradient.stops.iter().take(2).collect::<Vec<&GradientStop>>();
            let stop1 = stops[0];
            let stop2 = stops[1];
            let color1 = get_shaded_rgb(&stop1.color, point1, normal1, light_source);
            let color2 = get_shaded_rgb(&stop2.color, point2, normal2, light_source);
            GradientImageOrColor::LinearGradient(LinearGradient {
                x1: point1_projected.0,
                y1: point1_projected.1,
                x2: point2_projected.0,
                y2: point2_projected.1,
                stops: vec![
                    GradientStop {
                        offset: 0.0,
                        color: color1,
                    },
                    GradientStop {
                        offset: 1.0,
                        color: color2,
                    },
                ],
                alpha: 1.0
            })
        },
        GradientImageOrColor::RadialGradient(gradient) => {
            return GradientImageOrColor::RadialGradient(gradient.clone());
        }
        GradientImageOrColor::Image(image) => {
            return GradientImageOrColor::Image(image.clone());
        }
    }
}

#[derive(Clone, Debug)]
pub struct Camera {
    pub position: (f64, f64, f64),
    pub rotation: (f64, f64, f64),
    pub focal_distance: f64,
    pub zoom: f64,
}

#[derive(Clone, Debug)]
pub struct LightSource {
    pub position: (f64, f64, f64)
}

pub fn project_points(points: &Vec<(f64, f64, f64)>, camera: &Camera) -> Vec<(f64, f64)> {
    let mut points = shift_points_3d(points, (-camera.position.0, -camera.position.1, -camera.position.2));
    let rot_matrix = rot_matrix_euler(camera.rotation.0, camera.rotation.1, camera.rotation.2);
    points = matrix_times_points(rot_matrix, points);
    points = points.iter().map(|point| {
        let z = point.2;
        let factor = camera.focal_distance / (camera.focal_distance - z);
        let x = point.0 * factor * camera.zoom;
        let y = point.1 * factor * camera.zoom;
        (x, y, z)
    }).collect::<Vec<(f64, f64, f64)>>();
    points = shift_points_3d(&points, (camera.position.0, camera.position.1, camera.position.2));
    points.iter().map(|point| (point.0, point.1)).collect()
}

#[derive(Clone, Debug)]
pub struct ThreeDObject {
    pub points: Vec<(f64, f64, f64)>,
    pub subobjects: Vec<ThreeDObject>,
    pub fill: GradientImageOrColor,
    pub stroke: GradientImageOrColor,
    pub stroke_width: f64,
    pub index: usize,
}

pub fn line_as_cubic_bezier_3d(start: (f64, f64, f64), end: (f64, f64, f64)) -> Vec<(f64, f64, f64)> {
    return vec![start, interpolate_tuple_3d(start, end, 1.0 / 3.0), interpolate_tuple_3d(start, end, 2.0 / 3.0), end];
}

impl ThreeDObject {
    pub fn new(
        points: Vec<(f64, f64, f64)>,
        subobjects: Vec<ThreeDObject>,
        fill: GradientImageOrColor,
        stroke: GradientImageOrColor,
        stroke_width: f64,
        index: usize
    ) -> ThreeDObject {
        ThreeDObject {
            points,
            subobjects,
            fill: ensure_valid_three_d_color(fill),
            stroke: ensure_valid_three_d_color(stroke),
            stroke_width,
            index
        }
    }
    pub fn set_points(&self, points: Vec<(f64, f64, f64)>) -> ThreeDObject {
        ThreeDObject {
            points,
            subobjects: self.subobjects.clone(),
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            index: self.index
        }
    }
    pub fn set_subobjects(&self, subobjects: Vec<ThreeDObject>) -> ThreeDObject {
        ThreeDObject {
            points: self.points.clone(),
            subobjects,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            index: self.index
        }
    }
    pub fn set_fill(&self, color: GradientImageOrColor) -> ThreeDObject {
        ThreeDObject {
            points: self.points.clone(),
            subobjects: self.subobjects.clone(),
            fill: ensure_valid_three_d_color(color),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            index: self.index
        }
    }
    pub fn set_stroke(&self, color: GradientImageOrColor) -> ThreeDObject {
        ThreeDObject {
            points: self.points.clone(),
            subobjects: self.subobjects.clone(),
            fill: self.fill.clone(),
            stroke: ensure_valid_three_d_color(color),
            stroke_width: self.stroke_width,
            index: self.index
        }
    }

    pub fn set_stroke_width(&self, width: f64) -> ThreeDObject {
        ThreeDObject {
            points: self.points.clone(),
            subobjects: self.subobjects.clone(),
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: width,
            index: self.index
        }
    }
    pub fn get_points(&self) -> Vec<(f64, f64, f64)> {
        self.points.clone()
    }
    pub fn get_subobjects(&self) -> Vec<ThreeDObject> {
        self.subobjects.clone()
    }
    pub fn get_fill(&self) -> GradientImageOrColor {
        self.fill.clone()
    }
    pub fn get_stroke(&self) -> GradientImageOrColor {
        self.stroke.clone()
    }
    pub fn get_stroke_width(&self) -> f64 {
        self.stroke_width
    }
    pub fn scale(&self, factor: f64, recursive: bool) -> ThreeDObject {
        let new_points = self.points.iter().map(|point| {
            (point.0 * factor, point.1 * factor, point.2 * factor)
        }).collect();
        let new_subobjects = if recursive {
            self.subobjects.iter().map(|subobject| {
                subobject.scale(factor, true)
            }).collect()
        } else {
            self.subobjects.clone()
        };
        ThreeDObject {
            points: new_points,
            subobjects: new_subobjects,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            index: self.index
        }
    }
    pub fn stretch(&self, factor: (f64, f64, f64), recursive: bool) -> ThreeDObject {
        let new_points = self.points.iter().map(|point| {
            (point.0 * factor.0, point.1 * factor.1, point.2 * factor.2)
        }).collect();
        let new_subobjects = if recursive {
            self.subobjects.iter().map(|subobject| {
                subobject.stretch(factor, true)
            }).collect()
        } else {
            self.subobjects.clone()
        };
        ThreeDObject {
            points: new_points,
            subobjects: new_subobjects,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            index: self.index
        }
    }
    pub fn shift(&self, shift: (f64, f64, f64), recursive: bool) -> ThreeDObject {
        let new_points = shift_points_3d(&self.points, shift);
        let new_subobjects = if recursive {
            self.subobjects.iter().map(|subobject| {
                subobject.shift(shift, true)
            }).collect()
        } else {
            self.subobjects.clone()
        };
        ThreeDObject {
            points: new_points,
            subobjects: new_subobjects,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            index: self.index
        }
    }
    pub fn rotate_x(&self, angle: f64, recursive: bool) -> ThreeDObject {
        let new_points = self.points.iter().map(|point| {
            let matrix = rot_matrix(angle, 0);
            (
                matrix[0][0] * point.0 + matrix[0][1] * point.1 + matrix[0][2] * point.2,
                matrix[1][0] * point.0 + matrix[1][1] * point.1 + matrix[1][2] * point.2,
                matrix[2][0] * point.0 + matrix[2][1] * point.1 + matrix[2][2] * point.2
            )
        }).collect();
        let new_subobjects = if recursive {
            self.subobjects.iter().map(|subobject| {
                subobject.rotate_x(angle, true)
            }).collect()
        } else {
            self.subobjects.clone()
        };
        ThreeDObject {
            points: new_points,
            subobjects: new_subobjects,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            index: self.index
        }
    }
    pub fn rotate_y(&self, angle: f64, recursive: bool) -> ThreeDObject {
        let new_points = self.points.iter().map(|point| {
            let matrix = rot_matrix(angle, 1);
            (
                matrix[0][0] * point.0 + matrix[0][1] * point.1 + matrix[0][2] * point.2,
                matrix[1][0] * point.0 + matrix[1][1] * point.1 + matrix[1][2] * point.2,
                matrix[2][0] * point.0 + matrix[2][1] * point.1 + matrix[2][2] * point.2
            )
        }).collect();
        let new_subobjects = if recursive {
            self.subobjects.iter().map(|subobject| {
                subobject.rotate_y(angle, true)
            }).collect()
        } else {
            self.subobjects.clone()
        };
        ThreeDObject {
            points: new_points,
            subobjects: new_subobjects,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            index: self.index
        }
    }
    pub fn rotate_z(&self, angle: f64, recursive: bool) -> ThreeDObject {
        let new_points = self.points.iter().map(|point| {
            let matrix = rot_matrix(angle, 2);
            (
                matrix[0][0] * point.0 + matrix[0][1] * point.1 + matrix[0][2] * point.2,
                matrix[1][0] * point.0 + matrix[1][1] * point.1 + matrix[1][2] * point.2,
                matrix[2][0] * point.0 + matrix[2][1] * point.1 + matrix[2][2] * point.2
            )
        }).collect();
        let new_subobjects = if recursive {
            self.subobjects.iter().map(|subobject| {
                subobject.rotate_z(angle, true)
            }).collect()
        } else {
            self.subobjects.clone()
        };
        ThreeDObject {
            points: new_points,
            subobjects: new_subobjects,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            index: self.index
        }
    }
    pub fn get_subobjects_recursively(&self) -> Vec<ThreeDObject> {
        let mut subobjects = self.subobjects.clone();
        for subobject in self.subobjects.iter() {
            subobjects.extend(subobject.get_subobjects_recursively());
        }
        subobjects
    }
    pub fn project_and_shade(&self, camera: &Camera, light_source: &LightSource) -> VectorObject {
        let mut subobjects_3d = self.get_subobjects_recursively();
        subobjects_3d.push(self.clone());
        let rot_matrix = rot_matrix_euler(camera.rotation.0, camera.rotation.1, camera.rotation.2);
        subobjects_3d.sort_by(
            |a, b| {
                let a_center = a.set_subobjects(vec![]).get_center();
                let b_center = b.set_subobjects(vec![]).get_center();
                let a_z_rotated = matrix_times_points(rot_matrix, vec![a_center]).pop().unwrap().2;
                let b_z_rotated = matrix_times_points(rot_matrix, vec![b_center]).pop().unwrap().2;
                a_z_rotated.partial_cmp(&b_z_rotated).unwrap()
            }
        );
        let mut vec_obj = VectorObject::new();
        for subobject in subobjects_3d.iter() {
            let mut subobject_2d = VectorObject::new();
            subobject_2d.points = project_points(&subobject.points, camera); 
            subobject_2d.fill = get_shaded_color(&subobject.fill, &subobject.points, light_source, camera);
            subobject_2d.stroke = get_shaded_color(&subobject.stroke, &subobject.points, light_source, camera);
            subobject_2d.stroke_width = subobject.stroke_width;
            subobject_2d.index = subobject.index;
            vec_obj.subobjects.push(subobject_2d);
        }
        return vec_obj;
    }
    pub fn scale_handle_to_anchor_distances(&self, factor: f64, recursive: bool) -> ThreeDObject {
        let mut result = self.clone();
        if result.points.len() > 0 {
            let (a1, h1, h2, a2) = result.get_anchors_and_handles();
            let a1_to_h1 = a1.iter().zip(h1.iter()).map(|(a, h)| {
                (h.0 - a.0, h.1 - a.1, h.2 - a.2)
            }).collect::<Vec<(f64, f64, f64)>>();
            let a2_to_h2 = a2.iter().zip(h2.iter()).map(|(a, h)| {
                (h.0 - a.0, h.1 - a.1, h.2 - a.2)
            }).collect::<Vec<(f64, f64, f64)>>();
            let new_h1 = a1.iter().zip(a1_to_h1.iter()).map(|(a, h)| {
                (a.0 + h.0 * factor, a.1 + h.1 * factor, a.2 + h.2 * factor)
            }).collect::<Vec<(f64, f64, f64)>>();
            let new_h2 = a2.iter().zip(a2_to_h2.iter()).map(|(a, h)| {
                (a.0 + h.0 * factor, a.1 + h.1 * factor, a.2 + h.2 * factor)
            }).collect::<Vec<(f64, f64, f64)>>();
            result = result.set_anchors_and_handles((a1, new_h1, new_h2, a2));
        }
        if recursive {
            result = result.set_subobjects(
                result.subobjects.iter().map(|subobject| {
                    subobject.scale_handle_to_anchor_distances(factor, true)
                }).collect()
            );
        }
        result
    }
    pub fn get_anchors_and_handles(&self) -> (Vec<(f64, f64, f64)>, Vec<(f64, f64, f64)>, Vec<(f64, f64, f64)>, Vec<(f64, f64, f64)>) {
        (
            self.points.iter().step_by(4).map(|point| *point).collect(),
            self.points.iter().skip(1).step_by(4).map(|point| *point).collect(),
            self.points.iter().skip(2).step_by(4).map(|point| *point).collect(),
            self.points.iter().skip(3).step_by(4).map(|point| *point).collect()
        )
    }
    pub fn set_anchors_and_handles(&self, anchors_and_handles: (Vec<(f64, f64, f64)>, Vec<(f64, f64, f64)>, Vec<(f64, f64, f64)>, Vec<(f64, f64, f64)>)) -> ThreeDObject {
        let (anchors, start_handles, end_handles, end_anchors) = anchors_and_handles;
        let mut new_points = Vec::new();
        for (((a, h1), h2), b) in anchors.iter().zip(start_handles.iter()).zip(end_handles.iter()).zip(end_anchors.iter()) {
            new_points.push(*a);
            new_points.push(*h1);
            new_points.push(*h2);
            new_points.push(*b);
        }
        self.set_points(new_points)
    }
    pub fn apply_function(
        &self,
        function: Function,
        recursive: bool
    ) -> Pin<Box<dyn Future<Output = ThreeDObject> + '_>> {
        Box::pin(async move {
            let factor = 0.00001;
            let mut result = self.scale_handle_to_anchor_distances(factor, false);
            let mut new_points = Vec::new();
            for point in result.points.iter() {
                let (x, y, z) = point;
                let promise = function.call3(&JsValue::NULL, &JsValue::from_f64(*x), &JsValue::from_f64(*y), &JsValue::from_f64(*z)).unwrap().dyn_into::<Promise>().unwrap();
                let new_point = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap().dyn_into::<js_sys::Array>().unwrap();
                let new_x = new_point.get(0).as_f64().unwrap();
                let new_y = new_point.get(1).as_f64().unwrap();
                let new_z = new_point.get(2).as_f64().unwrap();
                new_points.push((new_x, new_y, new_z));
            }
            result = result.set_points(new_points);
            result = result.scale_handle_to_anchor_distances(1.0 / factor, false);
            if recursive {
                let mut new_subobjects = Vec::new();
                for subobject in result.subobjects.iter() {
                    new_subobjects.push(subobject.apply_function(function.clone(), true).await);
                }
            }
            result
        })
    }
    pub async fn from_uv_function(
        uv_function: &'static Function,
        u_range: (f64, f64),
        v_range: (f64, f64),
        u_samples: usize,
        v_samples: usize,
        fills: Vec<Color>,
        strokes: Vec<Color>,
        stroke_width: f64,
        index: Option<usize>
    ) -> Self {
        let mut faces = Vec::new();
        for i in 0..u_samples {
            for j in 0..v_samples {
                let u1 = interpolate(u_range.0, u_range.1, i as f64 / u_samples as f64);
                let u2 = interpolate(u_range.0, u_range.1, (i + 1) as f64 / u_samples as f64);
                let v1 = interpolate(v_range.0, v_range.1, j as f64 / v_samples as f64);
                let v2 = interpolate(v_range.0, v_range.1, (j + 1) as f64 / v_samples as f64);
                let vertices = vec![
                    (u1, v1, 0.0),
                    (u2, v1, 0.0),
                    (u2, v2, 0.0),
                    (u1, v2, 0.0),
                    (u1, v1, 0.0)
                ];
                let mut points = Vec::new();
                for (v1, v2) in vertices[0..4].iter().zip(vertices[1..5].iter()) {
                    points.extend(line_as_cubic_bezier_3d(*v1, *v2));
                }
                let face = ThreeDObject::new(
                    points,
                    vec![],
                    GradientImageOrColor::Color(fills[(i + j) % fills.len()].clone()),
                    GradientImageOrColor::Color(strokes[(i + j) % strokes.len()].clone()),
                    stroke_width,
                    i * v_samples + j
                );
                faces.push(face);
            }
        }
        let new_function = Closure::wrap(
            Box::new(move |x: f64, y: f64, _: f64| {
                future_to_promise(async move {
                    let u = JsValue::from_f64(x);
                    let v = JsValue::from_f64(y);
                    let promise = uv_function.call2(&JsValue::NULL, &u, &v).unwrap().dyn_into::<Promise>().unwrap();
                    JsFuture::from(promise).await.unwrap().dyn_into::<js_sys::Array>().map(|point| {
                        let x = point.get(0).as_f64().unwrap();
                        let y = point.get(1).as_f64().unwrap();
                        let z = point.get(2).as_f64().unwrap();
                        Ok(JsValue::from(js_sys::Array::of3(&JsValue::from_f64(x), &JsValue::from_f64(y), &JsValue::from_f64(z))))
                    }).unwrap()
                })
            }) as Box<dyn Fn(f64, f64, f64) -> Promise>
        );
        return ThreeDObject::new(
            vec![],
            faces,
            GradientImageOrColor::Color(Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }),
            GradientImageOrColor::Color(Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }),
            0.0,
            index.unwrap_or(0)
        ).apply_function(new_function.into_js_value().dyn_into().unwrap(), true).await;
    }
    pub fn get_bounding_box(
        &self
    ) -> ((f64, f64, f64), (f64, f64, f64)) {
        let mut min_x = self.points.first().map(|point| point.0).unwrap_or(0.0);
        let mut min_y = self.points.first().map(|point| point.1).unwrap_or(0.0);
        let mut min_z = self.points.first().map(|point| point.2).unwrap_or(0.0);
        let mut max_x = self.points.first().map(|point| point.0).unwrap_or(0.0);
        let mut max_y = self.points.first().map(|point| point.1).unwrap_or(0.0);
        let mut max_z = self.points.first().map(|point| point.2).unwrap_or(0.0);
        for point in self.points.iter() {
            if point.0 < min_x {
                min_x = point.0;
            }
            if point.1 < min_y {
                min_y = point.1;
            }
            if point.2 < min_z {
                min_z = point.2;
            }
            if point.0 > max_x {
                max_x = point.0;
            }
            if point.1 > max_y {
                max_y = point.1;
            }
            if point.2 > max_z {
                max_z = point.2;
            }
        }
        for subobject in self.subobjects.iter() {
            let (sub_min, sub_max) = subobject.get_bounding_box();
            if sub_min.0 < min_x {
                min_x = sub_min.0;
            }
            if sub_min.1 < min_y {
                min_y = sub_min.1;
            }
            if sub_min.2 < min_z {
                min_z = sub_min.2;
            }
            if sub_max.0 > max_x {
                max_x = sub_max.0;
            }
            if sub_max.1 > max_y {
                max_y = sub_max.1;
            }
            if sub_max.2 > max_z {
                max_z = sub_max.2;
            }
        }
        ((min_x, min_y, min_z), (max_x, max_y, max_z))
    }
    pub fn get_center(&self) -> (f64, f64, f64) {
        let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = self.get_bounding_box();
        ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0, (min_z + max_z) / 2.0)
    }
    pub fn move_to(&self, point: (f64, f64, f64), recursive: bool) -> ThreeDObject {
        let center = self.get_center();
        let shift = (point.0 - center.0, point.1 - center.1, point.2 - center.2);
        self.shift(shift, recursive)
    }
    pub fn from_vector_object(vector_object: &VectorObject) -> ThreeDObject {
        ThreeDObject::new(
            vector_object.points.iter().map(|point| {
                (point.0, point.1, 0.0)
            }).collect(),
            vector_object.subobjects.iter().map(|subobject| {
                ThreeDObject::from_vector_object(subobject)
            }).collect(),
            ensure_valid_three_d_color(vector_object.fill.clone()),
            ensure_valid_three_d_color(vector_object.stroke.clone()),
            vector_object.stroke_width,
            vector_object.index
        )
    }
    pub fn set_index(&self, index: usize) -> ThreeDObject {
        ThreeDObject {
            points: self.points.clone(),
            subobjects: self.subobjects.clone(),
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            index
        }
    }
    pub fn get_index(&self) -> usize {
        self.index
    }
}
