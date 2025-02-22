use wasm_bindgen::prelude::*;

use crate::{objects::vector_object::VectorObjectBuilder, utils::point2d::Point2D};

use super::tipable::Tipable;

/// A @type {Line} is a straight one-dimensional figure that extends infinitely in both directions.
/// It is defined by two points.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    /// The starting point of the line as @type {Point2D}.
    pub start: Point2D,
    /// The ending point of the line as @type {Point2D}.
    pub end: Point2D,
}

#[wasm_bindgen]
impl Line {
    /// Creates a new @type {Line} object from two points.
    #[wasm_bindgen(constructor, return_description = "A line connecting the start and end points.")]
    pub fn new(start: Point2D, end: Point2D) -> Line {
        Line { start, end }
    }

    /// Gets a @type {VectorObjectBuilder} with the line's points.
    #[wasm_bindgen(getter, return_description = "A vector object builder with the line's points.")]
    pub fn vector_object_builder(
        &self,
    ) -> VectorObjectBuilder {
        VectorObjectBuilder::default()
            .move_point(self.start)
            .line_to(self.end)
    }

    /// Returns the length of the line.
    #[wasm_bindgen(getter, return_description = "The length of the line.")]
    pub fn length(&self) -> f32 {
        Point2D::distance(&self.start, &self.end)
    }

    /// Returns the midpoint of the line as a @type {Point2D}.
    #[wasm_bindgen(return_description = "The midpoint of the line.")]
    pub fn midpoint(&self) -> Point2D {
        (self.start + self.end) * 0.5
    }

    /// Returns the slope of the line.
    #[wasm_bindgen(return_description = "The slope of the line.")]
    pub fn slope(&self) -> f32 {
        (self.end.y - self.start.y) / (self.end.x - self.start.x)
    }

    /// Returns the y-intercept of the line.
    #[wasm_bindgen(return_description = "The y-intercept of the line.")]
    pub fn y_intercept(&self) -> f32 {
        self.start.y - self.slope() * self.start.x
    }

    /// Returns the normal slope of the line.
    #[wasm_bindgen(return_description = "The normal slope of the line.")]
    pub fn normal_slope(&self) -> f32 {
        -1.0 / self.slope()
    }

    /// Returns the perpendicular line of the line at a given @type {Point2D}.
    #[wasm_bindgen(return_description = "The perpendicular line of the line at the given point.")]
    pub fn perpendicular_line(
        &self,
        #[wasm_bindgen(param_description = "The point to create the perpendicular line at.")]
        point: Point2D
    ) -> Line {
        let slope = self.normal_slope();
        let y_intercept = point.y - slope * point.x;
        Line::new(point, Point2D::new(0.0, y_intercept))
    }

    /// Returns the intersection @type {Point2D} of the line with another line, if it exists.
    #[wasm_bindgen(return_description = "The intersection point of the two lines if it exists.")]
    pub fn intersection(
        &self,
        #[wasm_bindgen(param_description = "The other line to intersect with.")]
        other: &Line
    ) -> Option<Point2D> {
        let x = (other.y_intercept() - self.y_intercept()) / (self.slope() - other.slope());
        let y = self.slope() * x + self.y_intercept();
        if x.is_nan() || y.is_nan() {
            None
        } else {
            Some(Point2D::new(x, y))
        }
    }

    /// Returns whether the line contains a given @type {Point2D}.
    #[wasm_bindgen(return_description = "Whether the line contains the given point.")]
    pub fn contains(
        &self,
        #[wasm_bindgen(param_description = "The point to check if it is contained.")]
        point: Point2D
    ) -> bool {
        let x_min = self.start.x.min(self.end.x);
        let x_max = self.start.x.max(self.end.x);
        let y_min = self.start.y.min(self.end.y);
        let y_max = self.start.y.max(self.end.y);
        point.x >= x_min && point.x <= x_max && point.y >= y_min && point.y <= y_max
    }

    /// Returns the distance from the line to a given @type {Point2D}.
    #[wasm_bindgen(return_description = "The distance from the line to the given point.")]
    pub fn distance_to_point(
        &self,
        #[wasm_bindgen(param_description = "The point to calculate the distance to.")]
        point: Point2D
    ) -> f32 {
        let a = self.start.y - self.end.y;
        let b = self.end.x - self.start.x;
        let c = self.start.x * self.end.y - self.end.x * self.start.y;
        (a * point.x + b * point.y + c).abs() / (a * a + b * b).sqrt()
    }

    /// Creates a @type {VectorObjectBuilder} with the line's points and a tip at the start.
    #[wasm_bindgen(return_description = "A @type {VectorObjectBuilder} with the line's points and a tip at the start.")]
    pub fn with_tip_at_the_start(
        &self,
        #[wasm_bindgen(param_description = "The shape of the tip. The shape must be pointing to the right and centered to (0, 0), this function will rotate and move it to the correct angle.")]
        tip_shape: VectorObjectBuilder,
    ) -> VectorObjectBuilder {
        let mut builder = self.vector_object_builder();
        builder = builder.add_child(self.tip_at_start(tip_shape));
        builder
    }

    /// Creates a @type {VectorObjectBuilder} with the line's points and a tip at the end.
    #[wasm_bindgen(return_description = "A @type {VectorObjectBuilder} with the line's points and a tip at the end.")]
    pub fn with_tip_at_the_end(
        &self,
        #[wasm_bindgen(param_description = "The shape of the tip. The shape must be pointing to the right and centered to (0, 0), this function will rotate and move it to the correct angle.")]
        tip_shape: VectorObjectBuilder,
    ) -> VectorObjectBuilder {
        let mut builder = self.vector_object_builder();
        builder = builder.add_child(self.tip_at_end(tip_shape));
        builder
    }

    /// Creates a @type {VectorObjectBuilder} with the line's points and tips at both ends.
    #[wasm_bindgen(return_description = "A @type {VectorObjectBuilder} with the line's points and tips at both ends.")]
    pub fn with_tips_at_both_ends(
        &self,
        #[wasm_bindgen(param_description = "The shape of the tip. The shape must be pointing to the right, this function will rotate and move it to the correct angle.")]
        tip_shape: VectorObjectBuilder,
    ) -> VectorObjectBuilder {
        let mut builder = self.vector_object_builder();
        builder = builder.add_child(self.tip_at_start(tip_shape.clone()));
        builder = builder.add_child(self.tip_at_end(tip_shape));
        builder
    }
}

impl Tipable for Line {
    fn angle_at_end(&self) -> f32 {
        self.slope().atan()
    }
    fn angle_at_start(&self) -> f32 {
        self.slope().atan() + std::f32::consts::PI
    }
    fn end(&self) -> Point2D {
        self.end
    }
    fn start(&self) -> Point2D {
        self.start
    }
}