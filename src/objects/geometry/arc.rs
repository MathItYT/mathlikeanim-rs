use wasm_bindgen::prelude::*;

use crate::{objects::vector_object::VectorObjectBuilder, utils::{bezier::AnchorsAndHandles, point2d::{Path2D, Point2D}}};

use super::tipable::Tipable;

/// An @type {Arc} is a portion of the circumference of a circle.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Arc {
    /// The center point of the arc as a @type {Point2D}.
    center: Point2D,
    /// The radius of the arc.
    radius: f32,
    /// The start angle of the arc in radians.
    start_angle: f32,
    /// The end angle of the arc in radians.
    end_angle: f32,
}

#[wasm_bindgen]
impl Arc {
    /// Creates a new @type {Arc} object from a center point, radius, start angle, and end angle.
    #[wasm_bindgen(constructor, return_description = "An arc.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The center point of the arc as a @type {Point2D}.")]
        center: Point2D,
        #[wasm_bindgen(param_description = "The radius of the arc.")]
        radius: f32,
        #[wasm_bindgen(param_description = "The start angle of the arc in radians.")]
        start_angle: f32,
        #[wasm_bindgen(param_description = "The end angle of the arc in radians.")]
        end_angle: f32,
    ) -> Arc {
        Arc {
            center,
            radius,
            start_angle,
            end_angle,
        }
    }

    /// Creates a new vector object builder with the arc's points.
    #[wasm_bindgen(return_description = "A @type {VectorObjectBuilder} representing the arc.")]
    pub fn vector_object_builder(&self, samples: Option<usize>) -> VectorObjectBuilder {
        let samples = samples.unwrap_or(15);
        let anchors = (0..samples)
            .map(|i| self.start_angle + (self.end_angle - self.start_angle) * i as f32 / (samples - 1) as f32)
            .map(|angle| Point2D::new(angle.cos(), angle.sin()))
            .collect::<Vec<Point2D>>();
        let dtheta = (self.end_angle - self.start_angle) / (samples - 1) as f32;
        let tangent_vectors = anchors.iter().map(|point| {
            Point2D::new(-point.y, point.x)
        }).collect::<Vec<Point2D>>();
        let factor = 4.0 / 3.0 * (dtheta / 4.0).tan();
        let handles1 = anchors[0..samples - 1].iter().zip(tangent_vectors[0..samples - 1].iter()).map(|(point, tangent)| {
            *point + *tangent * factor
        }).collect::<Vec<Point2D>>();
        let handles2 = anchors[1..samples].iter().zip(tangent_vectors[1..samples].iter()).map(|(point, tangent)| {
            *point - *tangent * factor
        }).collect::<Vec<Point2D>>();
        let path = Path2D::from_anchors_and_handles(&AnchorsAndHandles::new(
            anchors[0..samples - 1].to_vec(),
            handles1,
            handles2,
            anchors[1..samples].to_vec(),
        ).unwrap());
        VectorObjectBuilder::default()
            .set_path(path)
            .scale(self.radius, self.radius, None, None)
            .shift(self.center.x, self.center.y, None)
    }

    /// Returns the center point of the arc.
    #[wasm_bindgen(getter, return_description = "The center point of the arc as a @type {Point2D}.")]
    pub fn center(&self) -> Point2D {
        self.center
    }

    /// Returns the radius of the arc.
    #[wasm_bindgen(getter, return_description = "The radius of the arc.")]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Returns the start angle of the arc in radians.
    #[wasm_bindgen(getter, return_description = "The start angle of the arc in radians.")]
    pub fn start_angle(&self) -> f32 {
        self.start_angle
    }

    /// Returns the end angle of the arc in radians.
    #[wasm_bindgen(getter, return_description = "The end angle of the arc in radians.")]
    pub fn end_angle(&self) -> f32 {
        self.end_angle
    }

    /// Returns a VectorObjectBuilder representing the arc and a tip at the start of the arc.
    #[wasm_bindgen(return_description = "A @type {VectorObjectBuilder} representing the arc and a tip at the start of the arc.")]
    pub fn start_tip_vector_object_builder(
        &self,
        #[wasm_bindgen(param_description = "The tip shape as a @type {VectorObjectBuilder} to add to the start of the arc. It must be pointing to the right and centered at (0, 0). This function will rotate and move it to the correct angle.")]
        tip_shape: VectorObjectBuilder,
        #[wasm_bindgen(param_description = "The number of samples to use to create the arc, by default 15.")]
        samples: Option<usize>
    ) -> VectorObjectBuilder {
        let mut builder = self.vector_object_builder(samples);
        builder = builder.add_child(self.tip_at_start(tip_shape));
        builder
    }

    /// Returns a VectorObjectBuilder representing the arc and a tip at the end of the arc.
    #[wasm_bindgen(return_description = "A @type {VectorObjectBuilder} representing the arc and a tip at the end of the arc.")]
    pub fn end_tip_vector_object_builder(
        &self,
        #[wasm_bindgen(param_description = "The tip shape as a @type {VectorObjectBuilder} to add to the end of the arc. It must be pointing to the right and centered at (0, 0). This function will rotate and move it to the correct angle.")]
        tip_shape: VectorObjectBuilder,
        #[wasm_bindgen(param_description = "The number of samples to use to create the arc.")]
        samples: Option<usize>
    ) -> VectorObjectBuilder {
        let mut builder = self.vector_object_builder(samples);
        builder = builder.add_child(self.tip_at_end(tip_shape));
        builder
    }

    /// Returns a VectorObjectBuilder representing the arc and tips at both ends of the arc.
    #[wasm_bindgen(return_description = "A @type {VectorObjectBuilder} representing the arc and tips at both ends of the arc.")]
    pub fn both_tips_vector_object_builder(
        &self,
        #[wasm_bindgen(param_description = "The tip shape as a @type {VectorObjectBuilder} to add to the start of the arc. It must be pointing to the right and centered at (0, 0). This function will rotate and move it to the correct angle.")]
        tip_shape: VectorObjectBuilder,
        #[wasm_bindgen(param_description = "The number of samples to use to create the arc.")]
        samples: Option<usize>
    ) -> VectorObjectBuilder {
        let mut builder = self.vector_object_builder(samples);
        builder = builder.add_child(self.tip_at_start(tip_shape.clone()));
        builder = builder.add_child(self.tip_at_end(tip_shape));
        builder
    }

    /// Returns the point on the arc at a given t progress value.
    #[wasm_bindgen(return_description = "The @type {Point2D} on the arc at the given t value.")]
    pub fn point_at(
        &self,
        #[wasm_bindgen(param_description = "The t value to evaluate the polynomial at. A number between 0 and 1.")]
        t: f32,
    ) -> Point2D {
        let angle = self.start_angle + (self.end_angle - self.start_angle) * t;
        let x = self.center.x + self.radius * angle.cos();
        let y = self.center.y + self.radius * angle.sin();
        Point2D::new(x, y)
    }

    /// Returns the length of the arc.
    #[wasm_bindgen(return_description = "The length of the arc.")]
    pub fn length(&self) -> f32 {
        let angle = self.end_angle - self.start_angle;
        angle.abs() * self.radius
    }
}

impl Tipable for Arc {
    fn start(&self) -> Point2D {
        self.point_at(0.0)
    }

    fn end(&self) -> Point2D {
        self.point_at(1.0)
    }

    fn angle_at_end(&self) -> f32 {
        self.end_angle + std::f32::consts::PI / 2.0
    }

    fn angle_at_start(&self) -> f32 {
        self.start_angle - std::f32::consts::PI / 2.0
    }
}

/// A @type {Circle} is a set of all points in a plane that are at a given distance from a given point, the center.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Circle {
    /// The center @type {Point2D} of the circle.
    center: Point2D,
    /// The radius of the circle.
    radius: f32,
}

#[wasm_bindgen]
impl Circle {
    /// Creates a new @type {Circle} from a center @type {Point2D} and a radius.
    #[wasm_bindgen(constructor, return_description = "A circle.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The center point of the circle as a @type {Point2D}.")]
        center: Point2D,
        #[wasm_bindgen(param_description = "The radius of the circle.")]
        radius: f32,
    ) -> Circle {
        Circle { center, radius }
    }
    /// Creates an @type {Arc} from the circle.
    #[wasm_bindgen(getter, return_description = "An @type {Arc} representing the circle.")]
    pub fn arc(&self) -> Arc {
        Arc::new(self.center, self.radius, 0.0, 2.0 * std::f32::consts::PI)
    }
    /// Creates a new @type {VectorObjectBuilder} from the circle.
    #[wasm_bindgen(return_description = "A @type {VectorObjectBuilder} representing the circle.")]
    pub fn vector_object_builder(
        &self,
        #[wasm_bindgen(param_description = "The number of samples to use to create the circle, by default 15.")]
        samples: Option<usize>
    ) -> VectorObjectBuilder {
        self.arc().vector_object_builder(samples).close()
    }
    /// Returns the center point of the circle as a @type {Point2D}.
    #[wasm_bindgen(getter, return_description = "The center point of the circle.")]
    pub fn center(&self) -> Point2D {
        self.center
    }
    /// Returns the radius of the circle.
    #[wasm_bindgen(getter, return_description = "The radius of the circle.")]
    pub fn radius(&self) -> f32 {
        self.radius
    }
    /// Returns the circumference of the circle.
    #[wasm_bindgen(return_description = "The circumference of the circle.")]
    pub fn circumference(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius
    }
    /// Returns the area of the circle.
    #[wasm_bindgen(return_description = "The area of the circle.")]
    pub fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius.powi(2)
    }
}
