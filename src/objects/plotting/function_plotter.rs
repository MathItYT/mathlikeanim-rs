use exmex::{parse, ExError, Express, FlatEx, FloatOpsFactory};
use wasm_bindgen::prelude::*;
use std::rc::Rc;

use crate::{objects::vector_object::VectorObjectBuilder, utils::{bezier::CubicBezierTuple, console::error, interval::ClosedInterval, point2d::{Path2D, Point2D}}};

/// A ParametricFunctionPlot represents a plot of a parametric function (x(t), y(t)).
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct ParametricFunctionPlot {
    expression_x: Rc<String>,
    expression_y: Rc<String>,
    domain: ClosedInterval,
    x_range: ClosedInterval,
    y_range: ClosedInterval,
    discontinuities: Rc<Vec<f32>>,
    min_depth: u32,
    max_depth: u32,
    threshold: f32,
}

#[wasm_bindgen]
impl ParametricFunctionPlot {
    /// Creates a new ParametricFunctionPlot from an expression, domain, x-range, y-range, and other optional parameters.
    #[wasm_bindgen(constructor, return_description = "A new parametric function plot.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The x expression of the parametric function.")]
        expression_x: String,
        #[wasm_bindgen(param_description = "The y expression of the parametric function.")]
        expression_y: String,
        #[wasm_bindgen(param_description = "The domain of the parametric function.")]
        domain: ClosedInterval,
        #[wasm_bindgen(param_description = "The x-range of the plot.")]
        x_range: ClosedInterval,
        #[wasm_bindgen(param_description = "The y-range of the plot.")]
        y_range: ClosedInterval,
        #[wasm_bindgen(param_description = "The discontinuities of the plot.")]
        discontinuities: Option<Vec<f32>>,
        #[wasm_bindgen(param_description = "The minimum depth of the plot.")]
        min_depth: Option<u32>,
        #[wasm_bindgen(param_description = "The maximum depth of the plot.")]
        max_depth: Option<u32>,
        #[wasm_bindgen(param_description = "The threshold of the plot.")]
        threshold: Option<f32>,
    ) -> ParametricFunctionPlot {
        let discontinuities = discontinuities.unwrap_or_default();
        let min_depth = min_depth.unwrap_or(8);
        let max_depth = max_depth.unwrap_or(14);
        let threshold = threshold.unwrap_or(0.01);
        ParametricFunctionPlot {
            expression_x: Rc::new(expression_x),
            expression_y: Rc::new(expression_y),
            domain,
            x_range,
            y_range,
            discontinuities: Rc::new(discontinuities),
            min_depth,
            max_depth,
            threshold,
        }
    }

    /// Returns the x expression of the parametric function.
    #[wasm_bindgen(getter, return_description = "The x expression of the parametric function.")]
    pub fn expression_x(&self) -> String {
        self.expression_x.to_string()
    }

    /// Returns the y expression of the parametric function.
    #[wasm_bindgen(getter, return_description = "The y expression of the parametric function.")]
    pub fn expression_y(&self) -> String {
        self.expression_y.to_string()
    }

    /// Returns the expression of the parametric function.
    #[wasm_bindgen(getter, return_description = "The expression of the parametric function.")]
    pub fn expression(&self) -> String {
        format!("({}, {})", self.expression_x, self.expression_y)
    }

    /// Returns the domain of the parametric function.
    #[wasm_bindgen(getter, return_description = "The domain of the parametric function.")]
    pub fn domain(&self) -> ClosedInterval {
        self.domain.clone()
    }

    /// Returns the x-range of the plot.
    #[wasm_bindgen(getter, return_description = "The x-range of the plot.")]
    pub fn x_range(&self) -> ClosedInterval {
        self.x_range.clone()
    }

    /// Returns the y-range of the plot.
    #[wasm_bindgen(getter, return_description = "The y-range of the plot.")]
    pub fn y_range(&self) -> ClosedInterval {
        self.y_range.clone()
    }

    /// Returns the discontinuities of the plot.
    #[wasm_bindgen(getter, return_description = "The discontinuities of the plot.")]
    pub fn discontinuities(&self) -> Vec<f32> {
        self.discontinuities.to_vec()
    }

    /// Returns the minimum depth of the plot.
    #[wasm_bindgen(getter, return_description = "The minimum depth of the plot.")]
    pub fn min_depth(&self) -> u32 {
        self.min_depth
    }

    /// Returns the maximum depth of the plot.
    #[wasm_bindgen(getter, return_description = "The maximum depth of the plot.")]
    pub fn max_depth(&self) -> u32 {
        self.max_depth
    }

    /// Returns the threshold of the plot.
    #[wasm_bindgen(getter, return_description = "The threshold of the plot.")]
    pub fn threshold(&self) -> f32 {
        self.threshold
    }

    /// Checks if a number is a discontinuity of the plot.
    #[wasm_bindgen(return_description = "A boolean indicating if the number is a discontinuity of the plot.")]
    pub fn is_discontinuity(
        &self,
        #[wasm_bindgen(param_description = "The number to check.")]
        number: f32,
    ) -> bool {
        self.discontinuities.iter().any(|&discontinuity| (number - discontinuity).abs() < self.threshold)
    }

    /// Returns the error of both Point2Ds.
    #[wasm_bindgen(js_name = error, return_description = "The error of both Point2Ds.")]
    pub fn error(
        &self,
        #[wasm_bindgen(param_description = "The first Point2D.")]
        point1: Point2D,
        #[wasm_bindgen(param_description = "The second Point2D.")]
        point2: Point2D,
    ) -> f32 {
        point1.distance_squared(&point2)
    }

    /// Cheap hash function for the plot. Reference: https://github.com/stevenpetryk/mafs/blob/85c954ee649bebe65963bc8e7ad5708797c394d6/src/display/Plot/PlotUtils.tsx#L26
    #[wasm_bindgen(js_name = hash, return_description = "The hash of the plot.")]
    pub fn hash(
        &self,
        #[wasm_bindgen(param_description = "The minimum value of the object to hash.")]
        min: f32,
        #[wasm_bindgen(param_description = "The maximum value of the object to hash.")]
        max: f32,
    ) -> f32 {
        let result = (min * 12.9898 + max * 78.233).sin() * 43758.5453;
        0.4 + 0.2 * (result - result.floor())
    }

    /// Checks if point is within the plot.
    #[wasm_bindgen(return_description = "A boolean indicating if the point is within the plot.")]
    pub fn contains(
        &self,
        #[wasm_bindgen(param_description = "The point to check.")]
        point: Point2D,
    ) -> bool {
        self.x_range.contains(point.x) && self.y_range.contains(point.y)
    }

    /// Gets a VectorObjectBuilder with the plot's points.
    #[wasm_bindgen(getter, return_description = "A VectorObjectBuilder with the plot's points.")]
    pub fn vector_object_builder(&self) -> Result<VectorObjectBuilder, JsError> {
        let mut builder = VectorObjectBuilder::default();
        let mut path = Path2D::default();
        let mut previous_was_discontinuity = false;
        let t_min = self.domain.start();
        let t_max = self.domain.end();
        let p_min = self.evaluate(t_min).ok_or_else(|| JsError::new("Failed to evaluate parametric function."))?;
        let p_max = self.evaluate(t_max).ok_or_else(|| JsError::new("Failed to evaluate parametric function."))?;
        self.on_point(&mut path, t_min, &p_min, &mut previous_was_discontinuity);
        self.subdivide(&mut path, &mut previous_was_discontinuity, t_min, t_max, 0, p_min, p_max).ok_or_else(|| JsError::new("Failed to subdivide plot."))?;
        self.on_point(&mut path, t_max, &p_max, &mut previous_was_discontinuity);
        builder = builder.set_path(path);
        Ok(builder)
    }

    /// Evaluates the parametric function at a given value.
    #[wasm_bindgen(return_description = "The evaluated point.")]
    pub fn evaluate(
        &self,
        #[wasm_bindgen(param_description = "The value to evaluate the parametric function at.")]
        t: f32,
    ) -> Option<Point2D> {
        let x_expr: Result<FlatEx<_, FloatOpsFactory<f32>>, ExError> = parse(&self.expression_x);
        let y_expr: Result<FlatEx<_, FloatOpsFactory<f32>>, ExError> = parse(&self.expression_y);
        match (x_expr, y_expr) {
            (Ok(x_expr), Ok(y_expr)) => {
                let x = x_expr.eval(&[t.into()]);
                let y = y_expr.eval(&[t.into()]);
                match (x, y) {
                    (Ok(x), Ok(y)) => Some(Point2D::new(x, y)),
                    _ => {
                        error("Failed to evaluate parametric function.");
                        None
                    },
                }
            }
            _ => {
                error("Failed to parse parametric function.");
                None
            }
        }
    }
}

impl ParametricFunctionPlot {
    pub fn subdivide(
        &self,
        path: &mut Path2D,
        previous_was_discontinuity: &mut bool,
        min: f32,
        max: f32,
        depth: u32,
        p_min: Point2D,
        p_max: Point2D,
    ) -> Option<()> {
        let t = self.hash(min, max);
        let mid = min + (max - min) * t;
        let p_mid = self.evaluate(mid);
        if p_mid.is_none() {
            return None;
        }
        let p_mid = p_mid.unwrap();
        let mut deepen = || {
            let result = self.subdivide(path, previous_was_discontinuity, min, mid, depth + 1, p_min, p_mid);
            if result.is_none() {
                return None;
            }
            if self.is_discontinuity(mid) {
                self.on_discontinuity(previous_was_discontinuity);
            } else {
                self.on_point(path, mid, &p_mid, previous_was_discontinuity);
            }
            let result = self.subdivide(path, previous_was_discontinuity, mid, max, depth + 1, p_mid, p_max);
            if result.is_none() {
                return None;
            }
            Some(())
        };
        if depth < self.min_depth {
            let result = deepen();
            if result.is_none() {
                return None;
            }
        } else if depth < self.max_depth {
            let fn_midpoint = Point2D::lerp(&p_min, &p_max, t);
            let error = self.error(p_mid, fn_midpoint);
            if error > self.threshold * self.threshold {
                let result = deepen();
                if result.is_none() {
                    return None;
                }
            }
        }
        Some(())
    }

    pub fn on_point(&self, path: &mut Path2D, t: f32, p: &Point2D, previous_was_discontinuity: &mut bool) {
        if self.contains(*p) && p.is_finite() && !self.is_discontinuity(t) {
            if path.is_empty() || *previous_was_discontinuity {
                path.push(*p);
                *previous_was_discontinuity = false;
            } else {
                path.push_bezier(CubicBezierTuple::from_line(path.last().unwrap(), *p));
            }
        } else {
            *previous_was_discontinuity = true;
        }
    }

    pub fn on_discontinuity(&self, previous_was_discontinuity: &mut bool) {
        *previous_was_discontinuity = true;
    }
}