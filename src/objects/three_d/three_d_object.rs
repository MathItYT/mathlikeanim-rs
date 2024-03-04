use crate::{colors::{Color, GradientImageOrColor}, objects::{geometry::poly::polygon, vector_object::VectorFeatures}};


#[derive(Clone)]
pub struct Face {
    pub vertices: Vec<(f64, f64, f64)>,
    pub fill: (f64, f64, f64, f64),
    pub stroke: (f64, f64, f64, f64),
    pub stroke_width: f64
}

impl Face {
    pub fn new(
        vertices: Vec<(f64, f64, f64)>,
        fill: (f64, f64, f64, f64),
        stroke: (f64, f64, f64, f64),
        stroke_width: f64
    ) -> Face {
        Face {
            vertices,
            fill,
            stroke,
            stroke_width
        }
    }
}


#[derive(Clone)]
pub struct ThreeDObject {
    pub faces: Vec<Face>,
    pub subobjects: Vec<ThreeDObject>
}


impl ThreeDObject {
    pub fn new(faces: Vec<Face>, subobjects: Vec<ThreeDObject>) -> ThreeDObject {
        ThreeDObject {
            faces,
            subobjects
        }
    }
    pub fn rotate(&self, euler_angles: (f64, f64, f64), recursive: bool) -> ThreeDObject {
        let mut faces = Vec::new();
        for face in self.faces.clone() {
            let mut vertices = Vec::new();
            for vertex in face.vertices {
                let x = vertex.0;
                let y = vertex.1;
                let z = vertex.2;
                let x_rot = x * euler_angles.0.cos() - z * euler_angles.0.sin();
                let z_rot = x * euler_angles.0.sin() + z * euler_angles.0.cos();
                let y_rot = y * euler_angles.1.cos() - z_rot * euler_angles.1.sin();
                let z_rot = y * euler_angles.1.sin() + z_rot * euler_angles.1.cos();
                let x_proj = x_rot * euler_angles.2.cos() - y_rot * euler_angles.2.sin();
                let y_proj = x_rot * euler_angles.2.sin() + y_rot * euler_angles.2.cos();
                vertices.push((x_proj, y_proj, z_rot));
            }
            faces.push(Face::new(vertices, face.fill, face.stroke, face.stroke_width));
        }
        let mut subobjects = self.subobjects.clone();
        if recursive {
            for i in 0..subobjects.len() {
                subobjects[i] = subobjects[i].rotate(euler_angles, true);
            }
        }
        ThreeDObject::new(faces, subobjects)
    }
    pub fn scale(&self, factor: f64, recursive: bool) -> ThreeDObject {
        let mut faces = Vec::new();
        for face in self.faces.clone() {
            let mut vertices = Vec::new();
            for vertex in face.vertices {
                vertices.push((vertex.0 * factor, vertex.1 * factor, vertex.2 * factor));
            }
            faces.push(Face::new(vertices, face.fill, face.stroke, face.stroke_width));
        }
        let mut subobjects = self.subobjects.clone();
        if recursive {
            for i in 0..subobjects.len() {
                subobjects[i] = subobjects[i].scale(factor, true);
            }
        }
        ThreeDObject::new(faces, subobjects)
    }
    pub fn shift(&self, shift: (f64, f64, f64), recursive: bool) -> ThreeDObject {
        let mut faces = Vec::new();
        for face in self.faces.clone() {
            let mut vertices = Vec::new();
            for vertex in face.vertices {
                vertices.push((vertex.0 + shift.0, vertex.1 + shift.1, vertex.2 + shift.2));
            }
            faces.push(Face::new(vertices, face.fill, face.stroke, face.stroke_width));
        }
        let mut subobjects = self.subobjects.clone();
        if recursive {
            for i in 0..subobjects.len() {
                subobjects[i] = subobjects[i].shift(shift, true);
            }
        }
        ThreeDObject::new(faces, subobjects)
    }
    pub fn get_bounding_cube(&self) -> ((f64, f64), (f64, f64), (f64, f64)) {
        let mut x_min = std::f64::INFINITY;
        let mut x_max = std::f64::NEG_INFINITY;
        let mut y_min = std::f64::INFINITY;
        let mut y_max = std::f64::NEG_INFINITY;
        let mut z_min = std::f64::INFINITY;
        let mut z_max = std::f64::NEG_INFINITY;
        for face in self.faces.clone() {
            for vertex in face.vertices {
                if vertex.0 < x_min {
                    x_min = vertex.0;
                }
                if vertex.0 > x_max {
                    x_max = vertex.0;
                }
                if vertex.1 < y_min {
                    y_min = vertex.1;
                }
                if vertex.1 > y_max {
                    y_max = vertex.1;
                }
                if vertex.2 < z_min {
                    z_min = vertex.2;
                }
                if vertex.2 > z_max {
                    z_max = vertex.2;
                }
            }
        }
        return ((x_min, x_max), (y_min, y_max), (z_min, z_max));
    }
    pub fn get_center(&self) -> (f64, f64, f64) {
        let bounding_cube = self.get_bounding_cube();
        return (
            (bounding_cube.0 .0 + bounding_cube.0 .1) / 2.0,
            (bounding_cube.1 .0 + bounding_cube.1 .1) / 2.0,
            (bounding_cube.2 .0 + bounding_cube.2 .1) / 2.0
        );
    }
    pub fn move_to(&self, position: (f64, f64, f64), recursive: bool) -> ThreeDObject {
        let center = self.get_center();
        let shift = (position.0 - center.0, position.1 - center.1, position.2 - center.2);
        return self.shift(shift, recursive);
    }
    pub fn from_uv_function(
        uv_function: &dyn Fn(f64, f64) -> (f64, f64, f64),
        u_range: (f64, f64),
        v_range: (f64, f64),
        u_steps: usize,
        v_steps: usize,
        fill: (f64, f64, f64, f64),
        stroke: (f64, f64, f64, f64),
        stroke_width: f64
    ) -> ThreeDObject {
        let mut faces = Vec::new();
        for u in 0..u_steps {
            for v in 0..v_steps {
                let u1 = u as f64 / u_steps as f64;
                let u2 = (u + 1) as f64 / u_steps as f64;
                let v1 = v as f64 / v_steps as f64;
                let v2 = (v + 1) as f64 / v_steps as f64;
                let p1 = uv_function(u_range.0 + (u_range.1 - u_range.0) * u1, v_range.0 + (v_range.1 - v_range.0) * v1);
                let p2 = uv_function(u_range.0 + (u_range.1 - u_range.0) * u2, v_range.0 + (v_range.1 - v_range.0) * v1);
                let p3 = uv_function(u_range.0 + (u_range.1 - u_range.0) * u2, v_range.0 + (v_range.1 - v_range.0) * v2);
                let p4 = uv_function(u_range.0 + (u_range.1 - u_range.0) * u1, v_range.0 + (v_range.1 - v_range.0) * v2);
                faces.push(Face::new(vec![p1, p2, p3, p4], fill, stroke, stroke_width));
            }
        }
        ThreeDObject::new(faces, Vec::new())
    }
    pub fn project_and_shade(
        &self,
        camera_position: (f64, f64, f64),
        euler_angles: (f64, f64, f64),
        light_position: (f64, f64, f64)
    ) -> VectorFeatures {
        let mut faces_2d = Vec::new();
        let mut faces = self.faces.clone();
        faces.sort_by(|a, b| {
            let a = (a.vertices[0].0 - camera_position.0).powi(2) + (a.vertices[0].1 - camera_position.1).powi(2) + (a.vertices[0].2 - camera_position.2).powi(2);
            let b = (b.vertices[0].0 - camera_position.0).powi(2) + (b.vertices[0].1 - camera_position.1).powi(2) + (b.vertices[0].2 - camera_position.2).powi(2);
            a.partial_cmp(&b).unwrap()
        });
        faces.reverse();
        for face in faces {
            let mut vertices_xyz = Vec::new();
            for vertex in face.vertices {
                let x = vertex.0 - camera_position.0;
                let y = vertex.1 - camera_position.1;
                let z = vertex.2 - camera_position.2;
                let x_rot = x * euler_angles.0.cos() - z * euler_angles.0.sin();
                let z_rot = x * euler_angles.0.sin() + z * euler_angles.0.cos();
                let y_rot = y * euler_angles.1.cos() - z_rot * euler_angles.1.sin();
                let z_rot = y * euler_angles.1.sin() + z_rot * euler_angles.1.cos();
                let x_proj = x_rot * euler_angles.2.cos() - y_rot * euler_angles.2.sin();
                let y_proj = x_rot * euler_angles.2.sin() + y_rot * euler_angles.2.cos();
                vertices_xyz.push((x_proj, y_proj, z_rot));
            }
            let mut vertices_2d = Vec::new();
            for vertex in vertices_xyz.clone() {
                vertices_2d.push((vertex.0, vertex.1));
            }
            let mut normal = (
                (vertices_xyz[1].1 - vertices_xyz[0].1) * (vertices_xyz[2].2 - vertices_xyz[0].2) - (vertices_xyz[1].2 - vertices_xyz[0].2) * (vertices_xyz[2].1 - vertices_xyz[0].1),
                (vertices_xyz[1].2 - vertices_xyz[0].2) * (vertices_xyz[2].0 - vertices_xyz[0].0) - (vertices_xyz[1].0 - vertices_xyz[0].0) * (vertices_xyz[2].2 - vertices_xyz[0].2),
                (vertices_xyz[1].0 - vertices_xyz[0].0) * (vertices_xyz[2].1 - vertices_xyz[0].1) - (vertices_xyz[1].1 - vertices_xyz[0].1) * (vertices_xyz[2].0 - vertices_xyz[0].0)
            );
            let normal_magnitude = (normal.0.powi(2) + normal.1.powi(2) + normal.2.powi(2)).sqrt();
            normal = (normal.0 / normal_magnitude, normal.1 / normal_magnitude, normal.2 / normal_magnitude);
            let mut light_vector = (
                light_position.0 - vertices_xyz[0].0,
                light_position.1 - vertices_xyz[0].1,
                light_position.2 - vertices_xyz[0].2
            );
            let light_vector_magnitude = (light_vector.0.powi(2) + light_vector.1.powi(2) + light_vector.2.powi(2)).sqrt();
            light_vector = (light_vector.0 / light_vector_magnitude, light_vector.1 / light_vector_magnitude, light_vector.2 / light_vector_magnitude);
            let dot_product = normal.0 * light_vector.0 + normal.1 * light_vector.1 + normal.2 * light_vector.2;
            let mut factor = 0.5 * dot_product.powi(3);
            if factor < 0.0 {
                factor *= 0.5;
            }
            let fill = (
                face.fill.0 + factor,
                face.fill.1 + factor,
                face.fill.2 + factor,
                face.fill.3
            );
            let stroke = (
                face.stroke.0 + factor,
                face.stroke.1 + factor,
                face.stroke.2 + factor,
                face.stroke.3
            );
            faces_2d.push(polygon(
                vertices_2d,
                Some(stroke),
                Some(fill),
                Some(face.stroke_width),
                Some("butt"),
                Some("miter"),
                Some(0),
            ));
        }
        for subobject in self.subobjects.clone() {
            let subobject_2d = subobject.project_and_shade(camera_position, euler_angles, light_position);
            faces_2d.push(subobject_2d);
        }
        return VectorFeatures {
            subobjects: faces_2d,
            points: Vec::new(),
            fill: GradientImageOrColor::Color(
                Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.0
                }
            ),
            stroke: GradientImageOrColor::Color(
                Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.0
                }
            ),
            line_cap: "butt",
            line_join: "miter",
            stroke_width: 0.0,
            index: 0,
        }
    }
}