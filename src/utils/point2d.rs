use std::{ops::{Add, AddAssign, Index, Mul, MulAssign, Range, RangeFrom, RangeFull, RangeTo, Sub, SubAssign}, rc::Rc};

use usvg::tiny_skia_path;
use wasm_bindgen::prelude::*;

use crate::utils::{console::log, interpolation::lerp, linear_algebra::matrix_product_path};

use super::{bezier::{AnchorsAndHandles, CubicBezierTuple}, linear_algebra::TransformationMatrix};

/// A 2D point.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point2D {
    /// The x-coordinate of the point.
    pub x: f32,
    /// The y-coordinate of the point.
    pub y: f32,
}

impl Default for Point2D {
    fn default() -> Point2D {
        Point2D { x: 0.0, y: 0.0 }
    }
}

#[wasm_bindgen]
impl Point2D {
    /// Creates a new Point2D with the given coordinates.
    #[wasm_bindgen(constructor, return_description = "A 2D point.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The x-coordinate of the point.")]
        x: f32,
        #[wasm_bindgen(param_description = "The y-coordinate of the point.")]
        y: f32
    ) -> Point2D {
        Point2D { x, y }
    }

    /// Linearly interpolates between two Point2D objects.
    #[wasm_bindgen(return_description = "The interpolated point.")]
    pub fn lerp(
        #[wasm_bindgen(param_description = "The start point.")]
        point1: &Point2D,
        #[wasm_bindgen(param_description = "The end point.")]
        point2: &Point2D,
        #[wasm_bindgen(param_description = "The progress value.")]
        t: f32
    ) -> Point2D {
        Point2D {
            x: lerp(point1.x, point2.x, t),
            y: lerp(point1.y, point2.y, t),
        }
    }
    /// Gets a cartesian point from polar coordinates.
    #[wasm_bindgen(return_description = "A 2D point.")]
    pub fn from_polar(
        #[wasm_bindgen(param_description = "The radius of the point.")]
        radius: f32,
        #[wasm_bindgen(param_description = "The angle of the point.")]
        angle: f32
    ) -> Point2D {
        Point2D {
            x: radius * angle.cos(),
            y: radius * angle.sin(),
        }
    }
    /// Returns the distance between two Point2Ds.
    #[wasm_bindgen(return_description = "The distance between the two points.")]
    pub fn distance_squared(
        &self,
        #[wasm_bindgen(param_description = "The other point.")]
        other: &Point2D
    ) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
    /// Checks if two Point2D objects are equal within a given tolerance.
    #[wasm_bindgen(return_description = "A boolean indicating whether the two points are equal.")]
    pub fn equals(
        &self,
        #[wasm_bindgen(param_description = "The other point.")]
        other: &Point2D,
        #[wasm_bindgen(param_description = "The tolerance.")]
        tolerance: Option<f32>
    ) -> bool {
        let tolerance = tolerance.unwrap_or(0.01);
        self.distance_squared(other) < tolerance * tolerance
    }
    /// Returns the Point2D rotated around a given center point by a given angle.
    #[wasm_bindgen(return_description = "The rotated point.")]
    pub fn rotate_around(
        &self,
        #[wasm_bindgen(param_description = "The center point to rotate around.")]
        center: Point2D,
        #[wasm_bindgen(param_description = "The angle to rotate by.")]
        angle: f32
    ) -> Point2D {
        let (sin, cos) = angle.sin_cos();
        let x = cos * (self.x - center.x) - sin * (self.y - center.y) + center.x;
        let y = sin * (self.x - center.x) + cos * (self.y - center.y) + center.y;
        Point2D { x, y }
    }
    /// Returns the distance between two Point2Ds.
    #[wasm_bindgen(return_description = "The distance between the two points.")]
    pub fn distance(
        &self,
        #[wasm_bindgen(param_description = "The other point.")]
        other: &Point2D
    ) -> f32 {
        self.distance_squared(other).sqrt()
    }
    /// Clones the Point2D.
    #[wasm_bindgen(js_name = clone)]
    pub fn copy(&self) -> Point2D {
        *self
    }
    /// Returns the squared magnitude of the Point2D.
    #[wasm_bindgen(getter, return_description = "The squared magnitude of the point.")]
    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    /// Returns the magnitude of the Point2D.
    #[wasm_bindgen(getter, return_description = "The magnitude of the point.")]
    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }
    /// Returns the normalized Point2D.
    #[wasm_bindgen(getter, return_description = "The normalized point.")]
    pub fn normalized(&self) -> Point2D {
        let magnitude = self.magnitude();
        Point2D {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }
    /// Returns the dot product of Point2D objects.
    #[wasm_bindgen(return_description = "The dot product of the two points.")]
    pub fn dot(
        &self,
        #[wasm_bindgen(param_description = "The other point.")]
        other: &Point2D
    ) -> f32 {
        self.x * other.x + self.y * other.y
    }
    /// Returns the angle between two Point2Ds.
    #[wasm_bindgen(return_description = "The angle between the two points.")]
    pub fn angle(
        &self,
        #[wasm_bindgen(param_description = "The other point.")]
        other: &Point2D
    ) -> f32 {
        let dot = self.dot(other);
        let magnitude_product = self.magnitude() * other.magnitude();
        (dot / magnitude_product).acos()
    }
    /// Gets the direction angle with respect to (0, 0) of the Point2D.
    #[wasm_bindgen(getter, return_description = "The direction angle of the point.")]
    pub fn direction(&self) -> f32 {
        self.y.atan2(self.x)
    }
    /// Projects a point onto a line given the start and end points of the line.
    #[wasm_bindgen(return_description = "The proportion of the line.")]
    pub fn project_onto_line(
        &self,
        #[wasm_bindgen(param_description = "The start point of the line.")]
        start: &Point2D,
        #[wasm_bindgen(param_description = "The end point of the line.")]
        end: &Point2D
    ) -> f32 {
        let line = *end - *start;
        let line_squared = line.magnitude_squared();
        if line_squared == 0.0 {
            return 0.0;
        }
        let projection = *self - *start;
        (line.dot(&projection) / line_squared).max(0.0).min(1.0)
    }
    /// Checks if a point is finite
    #[wasm_bindgen(return_description = "A boolean indicating whether the point is finite.")]
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl Add<Point2D> for Point2D {
    type Output = Point2D;

    fn add(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Point2D> for f32 {
    type Output = Point2D;

    fn add(self, point: Point2D) -> Point2D {
        Point2D {
            x: self + point.x,
            y: self + point.y,
        }
    }
}

impl Add<f32> for Point2D {
    type Output = Point2D;

    fn add(self, scalar: f32) -> Point2D {
        Point2D {
            x: self.x + scalar,
            y: self.y + scalar,
        }
    }
}

impl AddAssign<Point2D> for Point2D {
    fn add_assign(&mut self, other: Point2D) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl AddAssign<f32> for Point2D {
    fn add_assign(&mut self, scalar: f32) {
        self.x += scalar;
        self.y += scalar;
    }
}

impl Sub<Point2D> for Point2D {
    type Output = Point2D;

    fn sub(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<Point2D> for f32 {
    type Output = Point2D;

    fn sub(self, point: Point2D) -> Point2D {
        Point2D {
            x: self - point.x,
            y: self - point.y,
        }
    }
}

impl Sub<f32> for Point2D {
    type Output = Point2D;

    fn sub(self, scalar: f32) -> Point2D {
        Point2D {
            x: self.x - scalar,
            y: self.y - scalar,
        }
    }
}

impl SubAssign<Point2D> for Point2D {
    fn sub_assign(&mut self, other: Point2D) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl SubAssign<f32> for Point2D {
    fn sub_assign(&mut self, scalar: f32) {
        self.x -= scalar;
        self.y -= scalar;
    }
}

impl Mul<f32> for Point2D {
    type Output = Point2D;

    fn mul(self, scalar: f32) -> Point2D {
        Point2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<Point2D> for f32 {
    type Output = Point2D;

    fn mul(self, point: Point2D) -> Point2D {
        Point2D {
            x: self * point.x,
            y: self * point.y,
        }
    }
}

impl Mul<Point2D> for Point2D {
    type Output = Point2D;

    fn mul(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl MulAssign<Point2D> for Point2D {
    fn mul_assign(&mut self, other: Point2D) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl MulAssign<f32> for Point2D {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

/// A 2D path.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Path2D {
    /// The points of the path.
    points: Rc<Vec<Point2D>>
}

#[wasm_bindgen]
impl Path2D {
    /// Creates a new Path2D with the given points.
    #[wasm_bindgen(constructor, return_description = "A 2D path.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The points of the path.")]
        points: Vec<Point2D>
    ) -> Path2D {
        Path2D { points: Rc::new(points) }
    }

    /// Creates a new Path2D given a AnchorsAndHandles object.
    #[wasm_bindgen(return_description = "A 2D path.")]
    pub fn from_anchors_and_handles(
        #[wasm_bindgen(param_description = "The AnchorsAndHandles object.")]
        anchors_and_handles: &AnchorsAndHandles
    ) -> Path2D {
        let mut points = Vec::new();
        for i in 0..anchors_and_handles.len() {
            points.push(anchors_and_handles.start_anchors()[i]);
            points.push(anchors_and_handles.first_controls()[i]);
            points.push(anchors_and_handles.second_controls()[i]);
            points.push(anchors_and_handles.end_anchors()[i]);
        }
        Path2D { points: Rc::new(points) }
    }

    /// Clones the Path2D.
    #[wasm_bindgen(js_name = clone)]
    pub fn copy(&self) -> Path2D {
        self.clone()
    }

    /// Repeats the Path2D a given number of times.
    #[wasm_bindgen(return_description = "The repeated path.")]
    pub fn repeat(
        &self,
        #[wasm_bindgen(param_description = "The number of times to repeat the path.")]
        count: usize
    ) -> Path2D {
        let mut points = Vec::with_capacity(self.points.len() * count);
        for _ in 0..count {
            points.extend(Rc::clone(&self.points).iter());
        }
        Path2D { points: Rc::new(points) }
    }

    /// Returns the Point2Ds of the Path2D.
    #[wasm_bindgen(getter, return_description = "The points of the path.")]
    pub fn points(&self) -> Vec<Point2D> {
        Rc::clone(&self.points).to_vec()
    }

    /// Sets the Point2Ds of the Path2D.
    #[wasm_bindgen(setter)]
    pub fn set_points(
        &mut self,
        #[wasm_bindgen(param_description = "The points of the path.")]
        points: Vec<Point2D>
    ) {
        self.points = Rc::new(points);
    }

    /// Returns whether the Path2D is empty.
    #[wasm_bindgen(getter, return_description = "A boolean indicating whether the path is empty.")]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Returns the closest Point2D in the Path2D to a given Point2D.
    #[wasm_bindgen(return_description = "The closest point in the path.")]
    pub fn closest_point(
        &self,
        #[wasm_bindgen(param_description = "The point to find the closest point to.")]
        point: &Point2D
    ) -> Point2D {
        let mut closest_point = self.points[0];
        let mut closest_distance = point.distance(&closest_point);
        for i in 1..self.points.len() {
            let distance = point.distance(&self.points[i]);
            if distance < closest_distance {
                closest_point = self.points[i];
                closest_distance = distance;
            }
        }
        closest_point
    }

    /// Returns the length of the Path2D.
    #[wasm_bindgen(getter, return_description = "The length of the path.")]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns the Point2D at a given index.
    #[wasm_bindgen(return_description = "The first point of the path.")]
    pub fn get(
        &self,
        #[wasm_bindgen(param_description = "The index of the point.")]
        index: usize
    ) -> Point2D {
        self.points[index]
    }

    /// Sets the Point2D at the given index.
    pub fn set(
        &mut self,
        #[wasm_bindgen(param_description = "The index of the point.")]
        index: usize,
        #[wasm_bindgen(param_description = "The new point.")]
        point: Point2D
    ) {
        Rc::make_mut(&mut self.points)[index] = point;
    }

    /// Appends a Point2D to the Path2D.
    pub fn push(
        &mut self,
        #[wasm_bindgen(param_description = "The point to append.")]
        point: Point2D
    ) {
        Rc::make_mut(&mut self.points).push(point);
    }

    /// Removes the last Point2D from the Path2D.
    #[wasm_bindgen(return_description = "The last point of the path.")]
    pub fn pop(&mut self) -> Option<Point2D> {
        Rc::make_mut(&mut self.points).pop()
    }

    /// Inserts a Point2D at a given index.
    pub fn insert(
        &mut self,
        #[wasm_bindgen(param_description = "The index to insert the point at.")]
        index: usize,
        #[wasm_bindgen(param_description = "The point to insert.")]
        point: Point2D
    ) {
        Rc::make_mut(&mut self.points).insert(index, point);
    }

    /// Removes a Point2D at a given index.
    #[wasm_bindgen(return_description = "The removed point.")]
    pub fn remove(
        &mut self,
        #[wasm_bindgen(param_description = "The index of the point to remove.")]
        index: usize
    ) -> Point2D {
        Rc::make_mut(&mut self.points).remove(index)
    }

    /// Removes all Point2Ds from the Path2D.
    pub fn clear(&mut self) {
        Rc::make_mut(&mut self.points).clear();
    }

    /// Returns a new Path2D representing a bezier curve portion of the Path2D.
    #[wasm_bindgen(return_description = "A Path2D object representing the portion of the input path.")]
    pub fn partial_bezier_path(
        &self,
        #[wasm_bindgen(param_description = "The start proportion of the input path. A number between 0 and 1.")]
        a: f32,
        #[wasm_bindgen(param_description = "The end proportion of the input path. A number between 0 and 1.")]
        b: f32,
    ) -> Path2D {
        if a == 1.0 {
            return Path2D::fill(self.get(self.len() - 1), self.len());
        }
        if b == 0.0 {
            return Path2D::fill(self.get(0), self.len());
        }
        let degree = self.len() - 1;
        if degree == 3 {
            let (ma, mb) = (
                1.0 - a,
                1.0 - b,
            );
            let (a2, b2, ma2, mb2) = (
                a * a,
                b * b,
                ma * ma,
                mb * mb,
            );
            let (a3, b3, ma3, mb3) = (
                a2 * a,
                b2 * b,
                ma2 * ma,
                mb2 * mb,
            );
            let portion_matrix = vec![
                ma3, 3.0 * ma2 * a, 3.0 * ma * a2, a3,
                ma2 * mb, 2.0 * ma * a * mb + ma2 * b, a2 * mb + 2.0 * ma * a * b, a2 * b,
                ma * mb2, a * mb2 + 2.0 * ma * mb * b, 2.0 * a * mb * b + ma * b2, a * b2,
                mb3, 3.0 * mb2 * b, 3.0 * mb * b2, b3,
            ];
            let a_rows = 4;
            let a_columns = 4;
            let b_columns = 2;
            let result = matrix_product_path(portion_matrix, &self, a_rows, a_columns, b_columns);
            return result;
        }
        if degree == 2 {
            let (ma, mb) = (
                1.0 - a,
                1.0 - b,
            );
            let (a2, b2, ma2, mb2) = (
                a * a,
                b * b,
                ma * ma,
                mb * mb,
            );
            let portion_matrix = vec![
                ma2, 2.0 * ma * a, a2,
                ma * mb, ma * b + a * mb, a * b,
                mb2, 2.0 * mb * b, b2,
            ];
            let a_rows = 3;
            let a_columns = 3;
            let b_columns = 2;
            let result = matrix_product_path(portion_matrix, &self, a_rows, a_columns, b_columns);
            return result;
        }
        if degree == 1 {
            let direction = self[1] - self[0];
            return Path2D::new(vec![
                self[0] + direction * a,
                self[0] + direction * b,
            ]);
        }
        if degree == 0 {
            return Path2D::fill(self[0], 1);
        }
        let n = self.len();
        let mut points = Rc::clone(&self.points);
        if a != 0.0 {
            for i in 1..n {
                let new_points = points[1..n - i + 1].iter().zip(points[0..n - i].iter()).map(|(a, b)| *a + *a * (*a - *b)).collect::<Vec<Point2D>>();
                Rc::make_mut(&mut points).splice(0..n - i, new_points);
            }
        }
        if b != 1.0 {
            let mu = (1.0 - b) / (1.0 - a);
            for i in 1..n {
                let new_points = points[i - 1..n - 1].iter().zip(points[i..n].iter()).map(|(a, b)| *a + mu * (*a - *b)).collect::<Vec<Point2D>>();
                Rc::make_mut(&mut points).splice(i..n, new_points);
            }
        }
        Path2D { points }
    }

    /// Creates a new Path2D by filling the Path2D with a given Point2D a given number of times.
    #[wasm_bindgen(return_description = "A path that is a portion of the input path.")]
    pub fn fill(
        #[wasm_bindgen(param_description = "The point to fill the path with.")]
        point: Point2D,
        #[wasm_bindgen(param_description = "The number of times to fill the path with the point.")]
        count: usize
    ) -> Path2D {
        Path2D {
            points: Rc::new(vec![point; count])
        }
    }

    /// Reverse the Path2D.
    pub fn reverse(&mut self) {
        Rc::make_mut(&mut self.points).reverse();
    }

    /// Sets a slice of the Path2D.
    pub fn set_slice(
        &mut self,
        #[wasm_bindgen(param_description = "The start index of the slice.")]
        start: usize,
        #[wasm_bindgen(param_description = "The end index of the slice.")]
        end: usize,
        #[wasm_bindgen(param_description = "The new path.")]
        path: Path2D
    ) {
        let points = Rc::clone(&path.points);
        Rc::make_mut(&mut self.points).splice(start..end, points.to_vec());
    }

    /// Returns a slice of the Path2D
    #[wasm_bindgen(return_description = "A slice of the path.")]
    pub fn slice(
        &self,
        #[wasm_bindgen(param_description = "The start index of the slice.")]
        start: usize,
        #[wasm_bindgen(param_description = "The end index of the slice.")]
        end: usize
    ) -> Path2D {
        Path2D {
            points: Rc::new(self.points[start..end].to_vec())
        }
    }

    /// Returns a CubicBezierTuple at a given index.
    #[wasm_bindgen(getter, return_description = "The cubic bezier tuples of the path.")]
    pub fn cubic_bezier_tuples(&self) -> Vec<CubicBezierTuple> {
        let remainder = self.points.len() % 4;
        let points = self.slice(0, self.points.len() - remainder);
        points.points.chunks(4).map(|chunk| {
            CubicBezierTuple::new(chunk[0], chunk[1], chunk[2], chunk[3])
        }).collect()
    }

    /// Appends a CubicBezierTuple to the Path2D.
    pub fn push_bezier(
        &mut self,
        #[wasm_bindgen(param_description = "The cubic bezier tuple to add.")]
        cubic_bezier: CubicBezierTuple
    ) {
        if self.points.len() % 4 != 0 && !cubic_bezier.start_anchor().equals(&self.points[self.points.len() - 1], None) {
            log("The path length must be a multiple of 4.");
            return;
        }
        if self.points.len() % 4 == 0 {
            Rc::make_mut(&mut self.points).push(cubic_bezier.start_anchor());
        }
        Rc::make_mut(&mut self.points).push(cubic_bezier.first_control());
        Rc::make_mut(&mut self.points).push(cubic_bezier.second_control());
        Rc::make_mut(&mut self.points).push(cubic_bezier.end_anchor());
    }

    /// Returns an approximation of the length of the path, based on sampling points along each cubic bezier curve in the path and an optional extra length to add to each approximation.
    #[wasm_bindgen(return_description = "An approximation of the length of the path.")]
    pub fn length(
        &self,
        #[wasm_bindgen(param_description = "The number of samples to take along each cubic bezier curve.")]
        samples_per_cubic: Option<usize>,
        #[wasm_bindgen(param_description = "An optional extra length to add to each cubic bezier length approximation.")]
        extra_length_per_cubic: Option<f32>
    ) -> f32 {
        self.cubic_bezier_tuples().iter().map(|tuple| tuple.length(samples_per_cubic, extra_length_per_cubic)).sum()
    }

    /// Gets the first point of the Path2D.
    #[wasm_bindgen(getter, return_description = "The first point of the path.")]
    pub fn first(&self) -> Point2D {
        self.points[0]
    }

    /// Gets the last point of the Path2D.
    #[wasm_bindgen(getter, return_description = "The last point of the path.")]
    pub fn last(&self) -> Option<Point2D> {
        self.points.last().copied()
    }
}

impl Path2D {
    /// Creates a Path2D from SVG path data.
    pub fn from_svg_path_data(
        data: &tiny_skia_path::Path,
    ) -> Path2D {
        let mut path = Path2D::default();
        let mut move_point = Point2D::default();
        let mut current_point = Point2D::default();
        for segment in data.segments() {
            match segment {
                tiny_skia_path::PathSegment::MoveTo(point) => {
                    current_point = Point2D::new(point.x, point.y);
                    move_point = current_point;
                }
                tiny_skia_path::PathSegment::LineTo(point) => {
                    path.push_bezier(CubicBezierTuple::from_line(current_point, Point2D::new(point.x, point.y)));
                    current_point = Point2D::new(point.x, point.y);
                }
                tiny_skia_path::PathSegment::QuadTo(control, point) => {
                    path.push_bezier(CubicBezierTuple::from_quadratic(current_point, Point2D::new(control.x, control.y), Point2D::new(point.x, point.y)));
                    current_point = Point2D::new(point.x, point.y);
                }
                tiny_skia_path::PathSegment::CubicTo(control1, control2, point) => {
                    path.push_bezier(CubicBezierTuple::new(current_point, Point2D::new(control1.x, control1.y), Point2D::new(control2.x, control2.y), Point2D::new(point.x, point.y)));
                    current_point = Point2D::new(point.x, point.y);
                }
                tiny_skia_path::PathSegment::Close => {
                    path.push_bezier(CubicBezierTuple::from_line(current_point, move_point));
                    current_point = move_point;
                }
            }
        }
        path
    }
    /// Transforms the path by a given CSS transformation matrix.
    pub fn transform(
        &self,
        matrix: &TransformationMatrix
    ) -> Path2D {
        Path2D {
            points: Rc::new(self.points.iter().map(|point| *matrix * *point).collect())
        }
    }
}

impl Add<Point2D> for Path2D {
    type Output = Path2D;

    fn add(self, point: Point2D) -> Path2D {
        Path2D {
            points: Rc::new(self.points.to_vec().into_iter().map(|p| p + point).collect())
        }
    }
}

impl Add<Path2D> for Point2D {
    type Output = Path2D;

    fn add(self, path: Path2D) -> Path2D {
        Path2D {
            points: Rc::new(path.points.to_vec().into_iter().map(|p| self + p).collect())
        }
    }
}

impl AddAssign<Point2D> for Path2D {
    fn add_assign(&mut self, point: Point2D) {
        Rc::make_mut(&mut self.points).iter_mut().for_each(|p| *p += point);
    }
}

impl Add<Path2D> for Path2D {
    type Output = Path2D;

    fn add(self, other: Path2D) -> Path2D {
        Path2D {
            points: Rc::new(self.points.iter().zip(other.points.iter()).map(|(a, b)| *a + *b).collect())
        }
    }
}

impl AddAssign<Path2D> for Path2D {
    fn add_assign(&mut self, other: Path2D) {
        Rc::make_mut(&mut self.points).iter_mut().zip(other.points.iter()).for_each(|(a, b)| *a += *b);
    }
}

impl Sub<Point2D> for Path2D {
    type Output = Path2D;

    fn sub(self, point: Point2D) -> Path2D {
        Path2D {
            points: Rc::new(self.points.to_vec().into_iter().map(|p| p - point).collect())
        }
    }
}

impl Sub<Path2D> for Point2D {
    type Output = Path2D;

    fn sub(self, path: Path2D) -> Path2D {
        Path2D {
            points: Rc::new(path.points.to_vec().into_iter().map(|p| self - p).collect())
        }
    }
}

impl SubAssign<Point2D> for Path2D {
    fn sub_assign(&mut self, point: Point2D) {
        Rc::make_mut(&mut self.points).iter_mut().for_each(|p| *p -= point);
    }
}

impl Sub<Path2D> for Path2D {
    type Output = Path2D;

    fn sub(self, other: Path2D) -> Path2D {
        Path2D {
            points: Rc::new(self.points.iter().zip(other.points.iter()).map(|(a, b)| *a - *b).collect())
        }
    }
}

impl SubAssign<Path2D> for Path2D {
    fn sub_assign(&mut self, other: Path2D) {
        Rc::make_mut(&mut self.points).iter_mut().zip(other.points.iter()).for_each(|(a, b)| *a -= *b);
    }
}

impl Mul<f32> for Path2D {
    type Output = Path2D;

    fn mul(self, scalar: f32) -> Path2D {
        Path2D {
            points: Rc::new(self.points.to_vec().into_iter().map(|p| p * scalar).collect())
        }
    }
}

impl Mul<Path2D> for f32 {
    type Output = Path2D;

    fn mul(self, path: Path2D) -> Path2D {
        Path2D {
            points: Rc::new(path.points.to_vec().into_iter().map(|p| self * p).collect())
        }
    }
}

impl Mul<Point2D> for Path2D {
    type Output = Path2D;

    fn mul(self, point: Point2D) -> Path2D {
        Path2D {
            points: Rc::new(self.points.to_vec().into_iter().map(|p| p * point).collect())
        }
    }
}

impl Mul<Path2D> for Point2D {
    type Output = Path2D;

    fn mul(self, path: Path2D) -> Path2D {
        Path2D {
            points: Rc::new(path.points.to_vec().into_iter().map(|p| self * p).collect())
        }
    }
}

impl MulAssign<f32> for Path2D {
    fn mul_assign(&mut self, scalar: f32) {
        Rc::make_mut(&mut self.points).iter_mut().for_each(|p| *p *= scalar);
    }
}

impl MulAssign<Point2D> for Path2D {
    fn mul_assign(&mut self, point: Point2D) {
        Rc::make_mut(&mut self.points).iter_mut().for_each(|p| *p *= point);
    }
}

impl Index<usize> for Path2D {
    type Output = Point2D;

    fn index(&self, index: usize) -> &Point2D {
        &self.points[index]
    }
}

impl Index<Range<usize>> for Path2D {
    type Output = [Point2D];

    fn index(&self, range: Range<usize>) -> &[Point2D] {
        &self.points[range]
    }
}

impl Index<RangeTo<usize>> for Path2D {
    type Output = [Point2D];

    fn index(&self, range: RangeTo<usize>) -> &[Point2D] {
        &self.points[range]
    }
}

impl Index<RangeFrom<usize>> for Path2D {
    type Output = [Point2D];

    fn index(&self, range: RangeFrom<usize>) -> &[Point2D] {
        &self.points[range]
    }
}

impl Index<RangeFull> for Path2D {
    type Output = [Point2D];

    fn index(&self, range: RangeFull) -> &[Point2D] {
        &self.points[range]
    }
}

impl IntoIterator for Path2D {
    type Item = Point2D;
    type IntoIter = std::vec::IntoIter<Point2D>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.to_vec().into_iter()
    }
}

impl Extend<Point2D> for Path2D {
    fn extend<T: IntoIterator<Item = Point2D>>(&mut self, iter: T) {
        Rc::make_mut(&mut self.points).extend(iter);
    }
}

impl Default for Path2D {
    fn default() -> Self {
        Path2D { points: Rc::new(Vec::new()) }
    }
}
