use crate::{objects::vector_object::VectorObjectBuilder, utils::point2d::Point2D};

pub trait Tipable {
    fn angle_at_end(&self) -> f32;
    fn angle_at_start(&self) -> f32;
    fn end(&self) -> Point2D;
    fn start(&self) -> Point2D;
    fn tip_at_end(&self, shape: VectorObjectBuilder) -> VectorObjectBuilder {
        let angle = self.angle_at_end();
        let end = self.end();
        let builder = shape
            .rotate(angle, Some(Point2D::new(0.0, 0.0)), None)
            .shift(end.x, end.y, None);
        builder
    }
    fn tip_at_start(&self, shape: VectorObjectBuilder) -> VectorObjectBuilder {
        let angle = self.angle_at_start();
        let start = self.start();
        let builder = shape
            .rotate(angle, Some(Point2D::new(0.0, 0.0)), None)
            .shift(start.x, start.y, None);
        builder
    }
}