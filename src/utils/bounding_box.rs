use wasm_bindgen::prelude::*;

use super::point2d::{Path2D, Point2D};

/// A bounding box is a rectangle that contains a set of points.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct BoundingBox {
    /// The minimum x-coordinate of the bounding box.
    min_x: f32,
    /// The minimum y-coordinate of the bounding box.
    min_y: f32,
    /// The width of the bounding box.
    width: f32,
    /// The height of the bounding box.
    height: f32,
}

#[wasm_bindgen]
impl BoundingBox {
    /// Creates a new bounding box from a minimum x-coordinate, minimum y-coordinate, width, and height.
    #[wasm_bindgen(constructor, return_description = "A bounding box with the given dimensions.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The minimum x-coordinate of the bounding box.")]
        min_x: f32,
        #[wasm_bindgen(param_description = "The minimum y-coordinate of the bounding box.")]
        min_y: f32,
        #[wasm_bindgen(param_description = "The width of the bounding box.")]
        width: f32,
        #[wasm_bindgen(param_description = "The height of the bounding box.")]
        height: f32,
    ) -> Result<BoundingBox, JsError> {
        if width < 0.0 || height < 0.0 {
            return Err(JsError::new("The width and height must be non-negative."));
        }
        Ok(BoundingBox {
            min_x,
            min_y,
            width,
            height,
        })
    }
    /// Creates the instance of a path's bounding box.
    #[wasm_bindgen(return_description = "The bounding box of the path.")]
    pub fn from_path(
        #[wasm_bindgen(param_description = "The path to calculate the bounding box of.")]
        path: &Path2D
    ) -> Option<BoundingBox> {
        if path.is_empty() {
            return None;
        }

        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for point in path.points().iter() {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        Some(BoundingBox {
            min_x,
            min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        })
    }

    /// Gets the minimum x-coordinate of the bounding box.
    #[wasm_bindgen(getter, return_description = "The minimum x-coordinate of the bounding box.")]
    pub fn min_x(&self) -> f32 {
        self.min_x
    }

    /// Gets the minimum y-coordinate of the bounding box.
    #[wasm_bindgen(getter, return_description = "The minimum y-coordinate of the bounding box.")]
    pub fn min_y(&self) -> f32 {
        self.min_y
    }

    /// Gets the width of the bounding box.
    #[wasm_bindgen(getter, return_description = "The width of the bounding box.")]
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Gets the height of the bounding box.
    #[wasm_bindgen(getter, return_description = "The height of the bounding box.")]
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Checks if a point is contained within the bounding box.
    #[wasm_bindgen(return_description = "A boolean indicating if the point is contained within the bounding box.")]
    pub fn contains(
        &self,
        #[wasm_bindgen(param_description = "The point to check if it is contained within the bounding box.")]
        point: Point2D,
    ) -> bool {
        let x = point.x;
        let y = point.y;
        x >= self.min_x && x <= self.min_x + self.width && y >= self.min_y && y <= self.min_y + self.height
    }

    /// Checks if the bounding box overlaps with another bounding box.
    #[wasm_bindgen(return_description = "A boolean indicating if the bounding box overlaps with the other bounding box.")]
    pub fn intersects(
        &self,
        #[wasm_bindgen(param_description = "The other bounding box to check for intersection.")]
        other: &BoundingBox
    ) -> bool {
        self.min_x < other.min_x + other.width
            && self.min_x + self.width > other.min_x
            && self.min_y < other.min_y + other.height
            && self.min_y + self.height > other.min_y
    }

    /// Returns the intersection of the bounding box with another bounding box, it means the area that is common to both bounding boxes.
    #[wasm_bindgen(return_description = "The intersection of the bounding box with the other bounding box.")]
    pub fn intersection(
        #[wasm_bindgen(param_description = "The bounding box to intersect with.")]
        this: Option<BoundingBox>,
        #[wasm_bindgen(param_description = "The other bounding box to intersect with.")]
        other: Option<BoundingBox>
    ) -> Option<BoundingBox> {
        if this.is_none() {
            return None;
        }
        let this = this.unwrap();
        if other.is_none() {
            return None;
        }
        let other = other.unwrap();

        if !this.intersects(&other) {
            return None;
        }

        let min_x = this.min_x.max(other.min_x);
        let min_y = this.min_y.max(other.min_y);
        let max_x = (this.min_x + this.width).min(other.min_x + other.width);
        let max_y = (this.min_y + this.height).min(other.min_y + other.height);

        Some(BoundingBox {
            min_x,
            min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        })
    }

    /// Returns the union of the bounding box with another bounding box, it means the smallest bounding box that contains both bounding boxes.
    #[wasm_bindgen(return_description = "The union of the bounding box with the other bounding box.")]
    pub fn union(
        #[wasm_bindgen(param_description = "The bounding box to union with.")]
        this: Option<BoundingBox>,
        #[wasm_bindgen(param_description = "The other bounding box to union with.")]
        other: Option<BoundingBox>
    ) -> Option<BoundingBox> {
        if this.is_none() {
            return other;
        }
        let this = this.unwrap();
        if other.is_none() {
            return Some(this);
        }
        let other = other.unwrap();

        let min_x = this.min_x.min(other.min_x);
        let min_y = this.min_y.min(other.min_y);
        let max_x = (this.min_x + this.width).max(other.min_x + other.width);
        let max_y = (this.min_y + this.height).max(other.min_y + other.height);

        Some(BoundingBox {
            min_x,
            min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        })
    }

    /// Returns the center point of the bounding box.
    #[wasm_bindgen(getter, return_description = "The center of the bounding box.")]
    pub fn center(&self) -> Point2D {
        Point2D {
            x: self.min_x + self.width / 2.0,
            y: self.min_y + self.height / 2.0,
        }
    }
}
