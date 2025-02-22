use std::rc::Rc;

use wasm_bindgen::prelude::*;

use super::point2d::{Path2D, Point2D};

/// @type {AnchorsAndHandles} is an object containing the start anchors, first control points, second control points, and end anchors of a path.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct AnchorsAndHandles {
    /// The start anchors of the path.
    start_anchors: Rc<Vec<Point2D>>,
    /// The first control points of the path.
    first_controls: Rc<Vec<Point2D>>,
    /// The second control points of the path.
    second_controls: Rc<Vec<Point2D>>,
    /// The end anchors of the path.
    end_anchors: Rc<Vec<Point2D>>,
}

#[wasm_bindgen]
impl AnchorsAndHandles {
    /// Creates a new @type {AnchorsAndHandles} object from start anchors, first control points, second control points, and end anchors.
    #[wasm_bindgen(constructor, return_description = "An object containing the start anchors, first control points, second control points, and end anchors of a path.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The start anchors of the path.")]
        start_anchors: Vec<Point2D>,
        #[wasm_bindgen(param_description = "The first control points of the path.")]
        first_controls: Vec<Point2D>,
        #[wasm_bindgen(param_description = "The second control points of the path.")]
        second_controls: Vec<Point2D>,
        #[wasm_bindgen(param_description = "The end anchors of the path.")]
        end_anchors: Vec<Point2D>
    ) -> Result<AnchorsAndHandles, JsError> {
        let lengths = vec![start_anchors.len(), first_controls.len(), second_controls.len(), end_anchors.len()];
        let min_length = *lengths.iter().min().unwrap();
        let max_length = *lengths.iter().max().unwrap();
        if min_length != max_length {
            return Err(JsError::new("The start anchors, first controls, second controls, and end anchors must have the same length."));
        }
        Ok(AnchorsAndHandles { start_anchors: Rc::new(start_anchors), first_controls: Rc::new(first_controls), second_controls: Rc::new(second_controls), end_anchors: Rc::new(end_anchors) })
    }
    /// Creates a new @type {AnchorsAndHandles} object from a path.
    #[wasm_bindgen(return_description = "An object containing the start anchors, first control points, second control points, and end anchors of a path.")]
    pub fn from_path(
        #[wasm_bindgen(param_description = "The path to extract the anchors and handles from.")]
        path: Path2D
    ) -> Result<AnchorsAndHandles, JsError> {
        if path.len() % 4 != 0 {
            return Err(JsError::new("The path length must be a multiple of 4."));
        }
        let mut start_anchors = Vec::with_capacity(path.len() / 4);
        let mut first_controls = Vec::with_capacity(path.len() / 4);
        let mut second_controls = Vec::with_capacity(path.len() / 4);
        let mut end_anchors = Vec::with_capacity(path.len() / 4);
        let cubic_beziers = path.cubic_bezier_tuples();
        for cubic_bezier in cubic_beziers {
            start_anchors.push(cubic_bezier.start_anchor);
            first_controls.push(cubic_bezier.first_control);
            second_controls.push(cubic_bezier.second_control);
            end_anchors.push(cubic_bezier.end_anchor);
        }
        Ok(AnchorsAndHandles { start_anchors: Rc::new(start_anchors), first_controls: Rc::new(first_controls), second_controls: Rc::new(second_controls), end_anchors: Rc::new(end_anchors) })
    }
    /// Returns the start anchors of the path.
    #[wasm_bindgen(getter, return_description = "The start anchors of the path.")]
    pub fn start_anchors(&self) -> Vec<Point2D> {
        self.start_anchors.to_vec()
    }
    /// Returns the first control points of the path.
    #[wasm_bindgen(getter, return_description = "The first control points of the path.")]
    pub fn first_controls(&self) -> Vec<Point2D> {
        self.first_controls.to_vec()
    }
    /// Returns the second control points of the path.
    #[wasm_bindgen(getter, return_description = "The second control points of the path.")]
    pub fn second_controls(&self) -> Vec<Point2D> {
        self.second_controls.to_vec()
    }
    /// Returns the end anchors of the path.
    #[wasm_bindgen(getter, return_description = "The end anchors of the path.")]
    pub fn end_anchors(&self) -> Vec<Point2D> {
        self.end_anchors.to_vec()
    }
    /// Returns the number of cubic bezier curves in the path.
    #[wasm_bindgen(getter, return_description = "The number of cubic bezier curves in the path.")]
    pub fn len(&self) -> usize {
        self.start_anchors.len()
    }
}

/// A @type {CubicBezierTuple} is a cubic bezier curve represented by a start anchor, first control point, second control point, and end anchor.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicBezierTuple {
    /// The start anchor of the cubic bezier.
    start_anchor: Point2D,
    /// The first control point of the cubic bezier.
    first_control: Point2D,
    /// The second control point of the cubic bezier.
    second_control: Point2D,
    /// The end anchor of the cubic bezier.
    end_anchor: Point2D,
}

#[wasm_bindgen]
impl CubicBezierTuple {
    /// Creates a new @type {CubicBezierTuple} from a start anchor, first control point, second control point, and end anchor.
    #[wasm_bindgen(constructor, return_description = "A cubic bezier curve.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The start anchor of the cubic bezier.")]
        start_anchor: Point2D,
        #[wasm_bindgen(param_description = "The first control point of the cubic bezier.")]
        first_control: Point2D,
        #[wasm_bindgen(param_description = "The second control point of the cubic bezier.")]
        second_control: Point2D,
        #[wasm_bindgen(param_description = "The end anchor of the cubic bezier.")]
        end_anchor: Point2D
    ) -> CubicBezierTuple {
        CubicBezierTuple { start_anchor, first_control, second_control, end_anchor }
    }
    /// Creates a new @type {CubicBezierTuple} from two @type {Point2D}s representing a line.
    #[wasm_bindgen(return_description = "The cubic bezier curve representing the line.")]
    pub fn from_line(
        #[wasm_bindgen(param_description = "The start point of the line.")]
        p1: Point2D,
        #[wasm_bindgen(param_description = "The end point of the line.")]
        p2: Point2D
    ) -> CubicBezierTuple {
        let handle1 = Point2D::lerp(&p1, &p2, 1.0 / 3.0);
        let handle2 = Point2D::lerp(&p1, &p2, 2.0 / 3.0);
        CubicBezierTuple::new(p1, handle1, handle2, p2)
    }
    /// Creates a new @type {CubicBezierTuple} from three @type {Point2D}s representing a quadratic bezier.
    #[wasm_bindgen(return_description = "The cubic bezier curve representing the quadratic bezier.")]
    pub fn from_quadratic(
        #[wasm_bindgen(param_description = "The first anchor point of the quadratic bezier.")]
        p1: Point2D,
        #[wasm_bindgen(param_description = "The control point of the quadratic bezier.")]
        p2: Point2D,
        #[wasm_bindgen(param_description = "The second anchor point of the quadratic bezier.")]
        p3: Point2D
    ) -> CubicBezierTuple {
        let handle1 = Point2D::lerp(&p1, &p2, 2.0 / 3.0);
        let handle2 = Point2D::lerp(&p2, &p3, 1.0 / 3.0);
        CubicBezierTuple::new(p1, handle1, handle2, p3)
    }
    /// Returns the start anchor of the @type {CubicBezierTuple}.
    #[wasm_bindgen(getter, return_description = "The start anchor of the cubic bezier.")]
    pub fn start_anchor(&self) -> Point2D {
        self.start_anchor
    }
    /// Returns the first control point of the @type {CubicBezierTuple}.
    #[wasm_bindgen(getter, return_description = "The first control point of the cubic bezier.")]
    pub fn first_control(&self) -> Point2D {
        self.first_control
    }
    /// Returns the second control point of the @type {CubicBezierTuple}.
    #[wasm_bindgen(getter, return_description = "The second control point of the cubic bezier.")]
    pub fn second_control(&self) -> Point2D {
        self.second_control
    }
    /// Returns the end anchor of the @type {CubicBezierTuple}.
    #[wasm_bindgen(getter, return_description = "The end anchor of the cubic bezier.")]
    pub fn end_anchor(&self) -> Point2D {
        self.end_anchor
    }
    /// Returns the point on the @type {CubicBezierTuple} when the polynomial is evaluated at the given t value.
    #[wasm_bindgen(return_description = "The point on the cubic bezier curve when the polynomial is evaluated at the given t value.")]
    pub fn point_at(
        &self,
        #[wasm_bindgen(param_description = "The t value to evaluate the polynomial at. A number between 0 and 1.")]
        t: f32
    ) -> Point2D {
        let one_minus_t = 1.0 - t;
        let one_minus_t_squared = one_minus_t * one_minus_t;
        let one_minus_t_cubed = one_minus_t_squared * one_minus_t;
        let t_squared = t * t;
        let t_cubed = t_squared * t;
        self.start_anchor * one_minus_t_cubed
            + self.first_control * (3.0 * one_minus_t_squared * t)
            + self.second_control * (3.0 * one_minus_t * t_squared)
            + self.end_anchor * t_cubed
    }
    /// Returns an approximation of the length of the @type {CubicBezierTuple}. Based on sampling the curve and add an optional extra length to fill in the gaps.
    pub fn length(
        &self,
        #[wasm_bindgen(param_description = "The number of samples to take along the curve.")]
        samples: Option<usize>,
        #[wasm_bindgen(param_description = "An optional extra length to add to the approximation.")]
        extra_length: Option<f32>
    ) -> f32 {
        let mut length = 0.0;
        let mut last_point = self.start_anchor;
        let samples = samples.unwrap_or(100);
        let extra_length = extra_length.unwrap_or(0.0);
        for i in 1..=samples {
            let t = i as f32 / samples as f32;
            let point = self.point_at(t);
            length += last_point.distance(&point);
            last_point = point;
        }
        length + extra_length
    }
}
