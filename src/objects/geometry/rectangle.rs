use wasm_bindgen::prelude::*;

use crate::{objects::vector_object::VectorObjectBuilder, utils::{bounding_box::BoundingBox, point2d::Point2D}};

/// A Rectangle is a quadrilateral with four right angles.
#[wasm_bindgen]
pub struct Rectangle {
    /// The BoundingBox of the rectangle.
    bbox: BoundingBox,
    /// The rotation of the rectangle, if any.
    rotation: Option<f32>,
}

#[wasm_bindgen]
impl Rectangle {
    /// Creates a new Rectangle from a BoundingBox and an optional rotation.
    #[wasm_bindgen(constructor, return_description = "A new rectangle.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The BoundingBox of the rectangle.")]
        bbox: BoundingBox,
        #[wasm_bindgen(param_description = "The rotation of the rectangle, if any.")]
        rotation: Option<f32>
    ) -> Rectangle {
        Rectangle { bbox, rotation }
    }
    /// Gets a VectorObjectBuilder with the rectangle's points.
    #[wasm_bindgen(getter, return_description = "A VectorObjectBuilder representing the rectangle.")]
    pub fn vector_object_builder(&self) -> VectorObjectBuilder {
        let mut builder = VectorObjectBuilder::default()
            .move_point(self.top_left())
            .line_to(self.top_right())
            .line_to(self.bottom_right())
            .line_to(self.bottom_left())
            .close();
        if let Some(rotation) = self.rotation {
            builder = builder.rotate(rotation, Some(self.center()), None);
        }
        builder
    }
    /// Returns the position of the rectangle.
    #[wasm_bindgen(getter, return_description = "The position of the rectangle.")]
    pub fn position(&self) -> Point2D {
        Point2D::new(self.bbox.min_x(), self.bbox.min_y())
    }
    /// Returns the size of the rectangle.
    #[wasm_bindgen(getter, return_description = "The size of the rectangle.")]
    pub fn bbox(&self) -> BoundingBox {
        self.bbox.clone()
    }
    /// Returns the rotation of the rectangle.
    #[wasm_bindgen(getter, return_description = "The rotation of the rectangle.")]
    pub fn rotation(&self) -> Option<f32> {
        self.rotation
    }
    /// Returns the top left corner of the rectangle as a Point2D.
    #[wasm_bindgen(getter, return_description = "The top left corner of the rectangle.")]
    pub fn top_left(&self) -> Point2D {
        self.position()
    }
    /// Returns the top right corner of the rectangle as a Point2D.
    #[wasm_bindgen(getter, return_description = "The top right corner of the rectangle.")]
    pub fn top_right(&self) -> Point2D {
        self.position() + Point2D::new(self.bbox.width(), 0.0)
    }
    /// Returns the bottom left corner of the rectangle as a Point2D.
    #[wasm_bindgen(getter, return_description = "The bottom left corner of the rectangle.")]
    pub fn bottom_left(&self) -> Point2D {
        self.position() + Point2D::new(0.0, self.bbox.height())
    }
    /// Returns the bottom right corner of the rectangle as a Point2D.
    #[wasm_bindgen(getter, return_description = "The bottom right corner of the rectangle.")]
    pub fn bottom_right(&self) -> Point2D {
        self.position() + Point2D::new(self.bbox.width(), self.bbox.height())
    }
    /// Returns the top of the rectangle as a Point2D.
    #[wasm_bindgen(getter, return_description = "The top of the rectangle.")]
    pub fn top(&self) -> Point2D {
        self.position() + Point2D::new(self.bbox.width() / 2.0, 0.0)
    }
    /// Returns the bottom of the rectangle as a Point2D.
    #[wasm_bindgen(getter, return_description = "The bottom of the rectangle.")]
    pub fn bottom(&self) -> Point2D {
        self.position() + Point2D::new(self.bbox.width() / 2.0, self.bbox.height())
    }
    /// Returns the left of the rectangle as a Point2D.
    #[wasm_bindgen(getter, return_description = "The left of the rectangle.")]
    pub fn left(&self) -> Point2D {
        self.position() + Point2D::new(0.0, self.bbox.height() / 2.0)
    }
    /// Returns the right of the rectangle as a Point2D.
    #[wasm_bindgen(getter, return_description = "The right of the rectangle.")]
    pub fn right(&self) -> Point2D {
        self.position() + Point2D::new(self.bbox.width(), self.bbox.height() / 2.0)
    }
    /// Returns the area of the rectangle.
    #[wasm_bindgen(getter, return_description = "The area of the rectangle.")]
    pub fn area(&self) -> f32 {
        self.bbox.width() * self.bbox.height()
    }
    /// Returns the center of the rectangle as a Point2D.
    #[wasm_bindgen(getter, return_description = "The center of the rectangle.")]
    pub fn center(&self) -> Point2D {
        self.bbox.center()
    }
}

/// A square is a rectangle with equal width and height.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Square {
    /// The center point of the square.
    center: Point2D,
    /// The side length of the square.
    side_length: f32,
    /// The rotation of the square.
    rotation: Option<f32>,
}

#[wasm_bindgen]
impl Square {
    /// Creates a new Square from a center point, side length, and optional rotation.
    #[wasm_bindgen(constructor, return_description = "A square.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The center point of the square as a Point2D.")]
        center: Point2D,
        #[wasm_bindgen(param_description = "The side length of the square.")]
        side_length: f32,
        #[wasm_bindgen(param_description = "The rotation of the square, if any.")]
        rotation: Option<f32>
    ) -> Square {
        Square { center, side_length, rotation }
    }
    /// Creates a new Rectangle from the square.
    #[wasm_bindgen(getter, return_description = "A Rectangle representing the square.")]
    pub fn rectangle(&self) -> Result<Rectangle, JsError> {
        if self.side_length <= 0.0 {
            return Err(JsError::new("The side length must be positive."));
        }
        Ok(Rectangle::new(
            BoundingBox::new(
                self.center.x - self.side_length / 2.0,
                self.center.y - self.side_length / 2.0,
                self.side_length,
                self.side_length,
            ).unwrap(),
            self.rotation,
        ))
    }
    /// Creates a VectorObjectBuilder from the square.
    #[wasm_bindgen(getter, return_description = "A VectorObjectBuilder representing the square.")]
    pub fn vector_object_builder(&self) -> Result<VectorObjectBuilder, JsError> {
        let rectangle = self.rectangle()?;
        Ok(rectangle.vector_object_builder())
    }
    /// Returns the center point of the square.
    #[wasm_bindgen(getter, return_description = "The center point of the square as a Point2D.")]
    pub fn center(&self) -> Point2D {
        self.center
    }
    /// Returns the side length of the square.
    #[wasm_bindgen(getter, return_description = "The side length of the square as a Point2D.")]
    pub fn side_length(&self) -> f32 {
        self.side_length
    }
    /// Returns the rotation of the square.
    #[wasm_bindgen(getter, return_description = "The rotation of the square, if any.")]
    pub fn rotation(&self) -> Option<f32> {
        self.rotation
    }
    /// Returns the area of the square.
    #[wasm_bindgen(getter, return_description = "The area of the square.")]
    pub fn area(&self) -> f32 {
        self.side_length * self.side_length
    }
    /// Returns the perimeter of the square.
    #[wasm_bindgen(getter, return_description = "The perimeter of the square.")]
    pub fn perimeter(&self) -> f32 {
        4.0 * self.side_length
    }
}