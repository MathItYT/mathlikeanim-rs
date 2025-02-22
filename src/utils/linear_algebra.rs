use std::ops::{Mul, MulAssign};

use wasm_bindgen::prelude::*;

use super::point2d::{Path2D, Point2D};

/// A @type {TransformationMatrix} is a 2D transformation matrix following the CSS matrix transform format.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransformationMatrix {
    /// The a component of the matrix.
    pub a: f32,
    /// The b component of the matrix.
    pub b: f32,
    /// The c component of the matrix.
    pub c: f32,
    /// The d component of the matrix.
    pub d: f32,
    /// The e component of the matrix.
    pub e: f32,
    /// The f component of the matrix.
    pub f: f32,
}

#[wasm_bindgen]
impl TransformationMatrix {
    /// Creates a new @type {TransformationMatrix} with the given components.
    #[wasm_bindgen(constructor, return_description = "A 2D transformation matrix following the CSS matrix transform format.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The a component of the matrix.")]
        a: f32,
        #[wasm_bindgen(param_description = "The b component of the matrix.")]
        b: f32,
        #[wasm_bindgen(param_description = "The c component of the matrix.")]
        c: f32,
        #[wasm_bindgen(param_description = "The d component of the matrix.")]
        d: f32,
        #[wasm_bindgen(param_description = "The e component of the matrix.")]
        e: f32,
        #[wasm_bindgen(param_description = "The f component of the matrix.")]
        f: f32
    ) -> TransformationMatrix {
        TransformationMatrix { a, b, c, d, e, f }
    }

    /// Returns the identity matrix.
    #[wasm_bindgen(return_description = "The identity matrix.")]
    pub fn identity() -> TransformationMatrix {
        TransformationMatrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Gets the matrix that translates by a given x and y value.
    #[wasm_bindgen(return_description = "The translated matrix.")]
    pub fn translate(
        #[wasm_bindgen(param_description = "The x value to translate by.")]
        x: f32,
        #[wasm_bindgen(param_description = "The y value to translate by.")]
        y: f32
    ) -> TransformationMatrix {
        TransformationMatrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: x,
            f: y,
        }
    }

    /// Gets the matrix that scales by a given x and y value.
    #[wasm_bindgen(return_description = "The scaled matrix.")]
    pub fn scale(
        #[wasm_bindgen(param_description = "The x value to scale by.")]
        x: f32,
        #[wasm_bindgen(param_description = "The y value to scale by.")]
        y: f32
    ) -> TransformationMatrix {
        TransformationMatrix {
            a: x,
            b: 0.0,
            c: 0.0,
            d: y,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Applies another @type {TransformationMatrix} to this matrix.
    #[wasm_bindgen]
    pub fn apply(&mut self, other: &TransformationMatrix) {
        *self = *other * *self;
    }

    /// Gets the matrix that rotates by a given angle in radians.
    #[wasm_bindgen(return_description = "The rotated matrix.")]
    pub fn rotate(
        #[wasm_bindgen(param_description = "The angle in radians to rotate by.")]
        angle: f32
    ) -> TransformationMatrix {
        let (sin, cos) = angle.sin_cos();
        TransformationMatrix {
            a: cos,
            b: sin,
            c: -sin,
            d: cos,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Gets the matrix that undoes this @type {TransformationMatrix}.
    #[wasm_bindgen(return_description = "The inverse matrix.")]
    pub fn inverse(self) -> TransformationMatrix {
        let det = self.a * self.d - self.b * self.c;
        TransformationMatrix {
            a: self.d / det,
            b: -self.b / det,
            c: -self.c / det,
            d: self.a / det,
            e: (self.c * self.f - self.d * self.e) / det,
            f: (self.b * self.e - self.a * self.f) / det,
        }
    }
}

impl TransformationMatrix {
    pub fn from_svg_transform(transform_matrix: usvg::Transform) -> TransformationMatrix {
        TransformationMatrix {
            a: transform_matrix.sx,
            b: transform_matrix.kx,
            c: transform_matrix.ky,
            d: transform_matrix.sy,
            e: transform_matrix.tx,
            f: transform_matrix.ty,
        }
    }
}

impl Mul<Vec<Point2D>> for TransformationMatrix {
    type Output = Vec<Point2D>;

    fn mul(self, points: Vec<Point2D>) -> Vec<Point2D> {
        points.into_iter().map(|point| self * point).collect()
    }
}

impl Mul<TransformationMatrix> for TransformationMatrix {
    type Output = TransformationMatrix;

    fn mul(self, other: TransformationMatrix) -> TransformationMatrix {
        TransformationMatrix {
            a: self.a * other.a + self.c * other.b,
            b: self.b * other.a + self.d * other.b,
            c: self.a * other.c + self.c * other.d,
            d: self.b * other.c + self.d * other.d,
            e: self.a * other.e + self.c * other.f + self.e,
            f: self.b * other.e + self.d * other.f + self.f,
        }
    }
}

impl MulAssign<TransformationMatrix> for TransformationMatrix {
    fn mul_assign(&mut self, other: TransformationMatrix) {
        *self = *self * other;
    }
}

impl Mul<Point2D> for TransformationMatrix {
    type Output = Point2D;

    fn mul(self, point: Point2D) -> Point2D {
        Point2D {
            x: self.a * point.x + self.c * point.y + self.e,
            y: self.b * point.x + self.d * point.y + self.f,
        }
    }
}

impl Mul<Path2D> for TransformationMatrix {
    type Output = Path2D;

    fn mul(self, path: Path2D) -> Path2D {
        Path2D::new(path.points().iter().map(|point| self * *point).collect())
    }
}

/// Multiplies two matrices together.
#[wasm_bindgen(return_description = "The product of the two matrices.", unchecked_return_type = "number[]")]
pub fn matrix_product(
    #[wasm_bindgen(param_description = "The first matrix to multiply as a flat array.", unchecked_param_type = "number[]")]
    a: Vec<f32>,
    #[wasm_bindgen(param_description = "The second matrix to multiply as a flat array.", unchecked_param_type = "number[]")]
    b: Vec<f32>,
    #[wasm_bindgen(param_description = "The number of rows in the first matrix.")]
    a_rows: usize,
    #[wasm_bindgen(param_description = "The number of columns in the first matrix.")]
    a_columns: usize,
    #[wasm_bindgen(param_description = "The number of columns in the second matrix.")]
    b_columns: usize,
) -> Vec<f32> {
    let mut result = vec![0.0; a_rows * b_columns];
    for i in 0..a_rows {
        for j in 0..b_columns {
            for k in 0..a_columns {
                result[i * b_columns + j] += a[i * a_columns + k] * b[k * b_columns + j];
            }
        }
    }
    result
}

/// Multiplies a matrix by a path of 2D points, returning a new path.
#[wasm_bindgen(return_description = "The product of the matrix and the path.")]
pub fn matrix_product_path(
    #[wasm_bindgen(param_description = "The matrix to multiply as a flat array.", unchecked_param_type = "number[]")]
    matrix: Vec<f32>,
    #[wasm_bindgen(param_description = "The path to multiply.")]
    path: &Path2D,
    #[wasm_bindgen(param_description = "The number of rows in the first matrix.")]
    a_rows: usize,
    #[wasm_bindgen(param_description = "The number of columns in the first matrix.")]
    a_columns: usize,
    #[wasm_bindgen(param_description = "The number of columns in the second matrix.")]
    b_columns: usize
) -> Path2D {
    let result = matrix_product(matrix, path.points().iter().map(|point| vec![point.x, point.y]).flatten().collect(), a_rows, a_columns, b_columns);
    Path2D::new(result.chunks(2).map(|chunk| Point2D { x: chunk[0], y: chunk[1] }).collect())
}