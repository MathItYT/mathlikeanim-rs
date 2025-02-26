use wasm_bindgen::prelude::*;

use crate::{objects::vector_object::VectorObjectBuilder, utils::point2d::Point2D};

/// A Triangle is a polygon with three edges and three vertices.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    /// The first vertex of the triangle as Point2D.
    a: Point2D,
    /// The second vertex of the triangle as Point2D.
    b: Point2D,
    /// The third vertex of the triangle as Point2D.
    c: Point2D,
}

#[wasm_bindgen]
impl Triangle {
    /// Creates a new Triangle object from three points.
    #[wasm_bindgen(constructor, return_description = "A triangle with the given vertices.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The first vertex of the triangle as Point2D.")]
        a: Point2D,
        #[wasm_bindgen(param_description = "The second vertex of the triangle as Point2D.")]
        b: Point2D,
        #[wasm_bindgen(param_description = "The third vertex of the triangle as Point2D.")]
        c: Point2D,
    ) -> Triangle {
        Triangle { a, b, c }
    }

    /// Returns the area of the triangle.
    #[wasm_bindgen(getter, return_description = "The area of the triangle.")]
    pub fn area(&self) -> f32 {
        0.5 * ((self.b.x - self.a.x) * (self.c.y - self.a.y) - (self.c.x - self.a.x) * (self.b.y - self.a.y)).abs()
    }

    /// Returns the perimeter of the triangle.
    #[wasm_bindgen(getter, return_description = "The perimeter of the triangle.")]
    pub fn perimeter(&self) -> f32 {
        self.a.distance(&self.b) + self.b.distance(&self.c) + self.c.distance(&self.a)
    }

    /// Creates a VectorObjectBuilder with the triangle's points.
    #[wasm_bindgen(return_description = "A VectorObjectBuilder with the triangle's points.")]
    pub fn vector_object_builder(&self) -> VectorObjectBuilder {
        VectorObjectBuilder::default().move_point(&self.a).line_to(&self.b).line_to(&self.c).close()
    }

    /// Returns the first vertex of the triangle as Point2D.
    #[wasm_bindgen(getter, return_description = "The first vertex of the triangle.")]
    pub fn a(&self) -> Point2D {
        self.a
    }

    /// Returns the second vertex of the triangle as Point2D.
    #[wasm_bindgen(getter, return_description = "The second vertex of the triangle.")]
    pub fn b(&self) -> Point2D {
        self.b
    }

    /// Returns the third vertex of the triangle as Point2D.
    #[wasm_bindgen(getter, return_description = "The third vertex of the triangle.")]
    pub fn c(&self) -> Point2D {
        self.c
    }
}

/// An EquilateralTriangle is a triangle in which all three sides are equal in length.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct EquilateralTriangle {
    /// The center Point2D of the equilateral triangle.
    center: Point2D,
    /// The side length of the equilateral triangle.
    side_length: f32,
    /// The rotation of the equilateral triangle, if any.
    rotation: Option<f32>,
}

#[wasm_bindgen]
impl EquilateralTriangle {
    /// Creates a new EquilateralTriangle from a center point and side length.
    #[wasm_bindgen(constructor, return_description = "An equilateral triangle.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The center Point2D of the equilateral triangle.")]
        center: Point2D,
        #[wasm_bindgen(param_description = "The side length of the equilateral triangle.")]
        side_length: f32,
        #[wasm_bindgen(param_description = "The rotation of the equilateral triangle, if any.")]
        rotation: Option<f32>,
    ) -> EquilateralTriangle {
        EquilateralTriangle { center, side_length, rotation }
    }

    /// Creates a Triangle from the equilateral triangle.
    #[wasm_bindgen(getter, return_description = "A triangle with the equilateral triangle's vertices.")]
    pub fn triangle(&self) -> Triangle {
        let mut a = Point2D::new(self.center.x - self.side_length / 2.0, self.center.y - self.side_length / 2.0 / 3.0f32.sqrt());
        let mut b = Point2D::new(self.center.x, self.center.y + 2.0 / 3.0f32.sqrt() * self.side_length / 2.0);
        let mut c = Point2D::new(self.center.x + self.side_length / 2.0, self.center.y - self.side_length / 2.0 / 3.0f32.sqrt());
        if let Some(rotation) = self.rotation {
            a = a.rotate_around(self.center, rotation);
            b = b.rotate_around(self.center, rotation);
            c = c.rotate_around(self.center, rotation);
        }
        Triangle::new(a, b, c)
    }

    /// Creates a VectorObjectBuilder from the equilateral triangle.
    #[wasm_bindgen(getter, return_description = "A vector object builder with the equilateral triangle's points.")]
    pub fn vector_object_builder(&self) -> VectorObjectBuilder {
        self.triangle().vector_object_builder()
    }

    /// Returns the center Point2D of the equilateral triangle.
    #[wasm_bindgen(getter, return_description = "The center point of the equilateral triangle.")]
    pub fn center(&self) -> Point2D {
        self.center
    }

    /// Returns the side length of the equilateral triangle.
    #[wasm_bindgen(getter, return_description = "The side length of the equilateral triangle.")]
    pub fn side_length(&self) -> f32 {
        self.side_length
    }

    /// Returns the area of the equilateral triangle.
    #[wasm_bindgen(getter, return_description = "The area of the equilateral triangle.")]
    pub fn area(&self) -> f32 {
        0.25 * 3.0f32.sqrt() * self.side_length * self.side_length
    }

    /// Returns the perimeter of the equilateral triangle.
    #[wasm_bindgen(getter, return_description = "The perimeter of the equilateral triangle.")]
    pub fn perimeter(&self) -> f32 {
        3.0 * self.side_length
    }

    /// Returns the height of the equilateral triangle.
    #[wasm_bindgen(getter, return_description = "The height of the equilateral triangle.")]
    pub fn height(&self) -> f32 {
        3.0f32.sqrt() / 2.0 * self.side_length
    }
}

/// A RightTriangle is a triangle in which one angle is a right angle.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct RightTriangle {
    /// The center Point2D of the right triangle.
    center: Point2D,
    /// The base length of the right triangle.
    base_length: f32,
    /// The height of the right triangle.
    height: f32,
    /// The rotation of the right triangle, if any.
    rotation: Option<f32>,
    /// Whether the right triangle must be flipped, by default false.
    flip: Option<bool>,
}

#[wasm_bindgen]
impl RightTriangle {
    /// Creates a new RightTriangle from a center point, base length, height, rotation, and flip.
    #[wasm_bindgen(constructor, return_description = "A right triangle.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The center Point2D of the right triangle.")]
        center: Point2D,
        #[wasm_bindgen(param_description = "The base length of the right triangle.")]
        base_length: f32,
        #[wasm_bindgen(param_description = "The height of the right triangle.")]
        height: f32,
        #[wasm_bindgen(param_description = "The rotation of the right triangle, if any.")]
        rotation: Option<f32>,
        #[wasm_bindgen(param_description = "The flip of the right triangle, by default false.")]
        flip: Option<bool>,
    ) -> RightTriangle {
        RightTriangle { center, base_length, height, rotation, flip }
    }

    /// Creates a Triangle from the right triangle.
    #[wasm_bindgen(getter, return_description = "A triangle.")]
    pub fn triangle(&self) -> Triangle {
        let mut a = Point2D::new(self.center.x - self.base_length / 2.0, self.center.y + self.height / 2.0);
        let mut b = Point2D::new(self.center.x + self.base_length / 2.0, self.center.y + self.height / 2.0);
        let mut c = if self.flip.unwrap_or(false) {
            Point2D::new(self.center.x - self.base_length / 2.0, self.center.y - self.height / 2.0)
        } else {
            Point2D::new(self.center.x + self.base_length / 2.0, self.center.y - self.height / 2.0)
        };
        if let Some(rotation) = self.rotation {
            a = a.rotate_around(self.center, rotation);
            b = b.rotate_around(self.center, rotation);
            c = c.rotate_around(self.center, rotation);
        }
        Triangle::new(a, b, c)
    }

    /// Creates a VectorObjectBuilder from the right triangle.
    #[wasm_bindgen(getter, return_description = "A vector object builder with the right triangle's points.")]
    pub fn vector_object_builder(&self) -> VectorObjectBuilder {
        self.triangle().vector_object_builder()
    }

    /// Returns the center Point2D of the right triangle.
    #[wasm_bindgen(getter, return_description = "The center point of the right triangle.")]
    pub fn center(&self) -> Point2D {
        self.center
    }

    /// Returns the base length of the right triangle.
    #[wasm_bindgen(getter, return_description = "The base length of the right triangle.")]
    pub fn base_length(&self) -> f32 {
        self.base_length
    }

    /// Returns the height of the right triangle.
    #[wasm_bindgen(getter, return_description = "The height of the right triangle.")]
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Returns the area of the right triangle.
    #[wasm_bindgen(getter, return_description = "The area of the right triangle.")]
    pub fn area(&self) -> f32 {
        0.5 * self.base_length * self.height
    }

    /// Returns the perimeter of the RightTriangle.
    #[wasm_bindgen(getter, return_description = "The perimeter of the right triangle.")]
    pub fn perimeter(&self) -> f32 {
        self.base_length + self.height + (self.base_length.powi(2) + self.height.powi(2)).sqrt()
    }

    /// Returns the hypotenuse of the RightTriangle.
    #[wasm_bindgen(getter, return_description = "The hypotenuse of the right triangle.")]
    pub fn hypotenuse(&self) -> f32 {
        (self.base_length.powi(2) + self.height.powi(2)).sqrt()
    }
}