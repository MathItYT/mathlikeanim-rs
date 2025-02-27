use std::{rc::Rc, sync::Arc};

use base64::{prelude::BASE64_STANDARD, Engine};
use usvg::{ImageHrefResolver, ImageKind};
use wasm_bindgen::{prelude::*, throw_str};
use crate::{objects::geometry::triangle::EquilateralTriangle, utils::{bezier::CubicBezierTuple, bounding_box::BoundingBox, console::log, font_face::FontFace, image_library::ImageLibrary, interpolation::IntegerLerp, linear_algebra::TransformationMatrix, point2d::{Path2D, Point2D}, style::{Color, ImageBitmap, Style}}};

use super::geometry::rectangle::Rectangle;

/// A VectorObject is a vector object that can be drawn on a vector graphics canvas.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct VectorObject {
    /// The path of the vector object.
    path: Path2D,
    /// The fill style of the vector object.
    fill: Style,
    /// The fill rule of the vector object.
    fill_rule: Rc<String>,
    /// The stroke style of the vector object.
    stroke: Style,
    /// The stroke width of the vector object.
    stroke_width: f32,
    /// The stroke line cap of the vector object.
    stroke_line_cap: Rc<String>,
    /// The stroke line join of the vector object.
    stroke_line_join: Rc<String>,
    /// The stroke miter limit of the vector object.
    stroke_miter_limit: f32,
    /// The stroke dash offset of the vector object.
    stroke_dash_offset: f32,
    /// The stroke dash array of the vector object.
    stroke_dash_array: Rc<Vec<f32>>,
    /// The children of the vector object.
    children: Vec<VectorObject>,
    /// Name of the vector object.
    name: Option<Rc<String>>,
    /// Transform matrix of the vector object.
    transform: TransformationMatrix,
}

#[derive(Clone)]
pub struct VectorOperationList {
    operations: Vec<&'static dyn VectorOperation>,
}

impl VectorOperationList {
    pub fn new() -> VectorOperationList {
        VectorOperationList {
            operations: Vec::new(),
        }
    }
    pub fn add_operation(&mut self, operation: &'static dyn VectorOperation) {
        self.operations.push(operation);
    }
    pub fn apply_and_return(&self, object: &mut VectorObject) -> VectorObject {
        self.apply(object);
        object.clone()
    }
}

impl VectorOperation for VectorOperationList {
    fn apply(&self, object: &mut VectorObject) {
        for operation in &self.operations {
            operation.apply(object);
        }
    }
}

pub trait VectorOperation {
    fn apply(&self, object: &mut VectorObject);
}

pub struct Shift {
    pub dx: f32,
    pub dy: f32,
    pub recursive: Option<bool>,
}

impl VectorOperation for Shift {
    fn apply(&self, object: &mut VectorObject) {
        let apply_transform = ApplyTransform {
            matrix: TransformationMatrix::translate(self.dx, self.dy),
            recursive: self.recursive,
        };
        apply_transform.apply(object);
    }
}

pub struct MoveTo {
    pub point: Point2D,
    pub recursive: Option<bool>,
}

impl VectorOperation for MoveTo {
    fn apply(&self, object: &mut VectorObject) {
        let center = object.center();
        if center.is_none() {
            return;
        }
        let center = center.unwrap();
        let shift = self.point - center;
        let shift = Shift {
            dx: shift.x,
            dy: shift.y,
            recursive: self.recursive,
        };
        shift.apply(object);
    }
}

pub struct Scale {
    pub factor_x: f32,
    pub factor_y: f32,
    pub about_point: Option<Point2D>,
    pub recursive: Option<bool>,
}

impl VectorOperation for Scale {
    fn apply(&self, object: &mut VectorObject) {
        let about_point = if let Some(point) = self.about_point {
            point
        } else {
            object.bounding_box(self.recursive).unwrap().center()
        };
        let shift = Shift {
            dx: -about_point.x,
            dy: -about_point.y,
            recursive: self.recursive,
        };
        shift.apply(object);
        let apply_transform = ApplyTransform {
            matrix: TransformationMatrix::scale(self.factor_x, self.factor_y),
            recursive: self.recursive,
        };
        apply_transform.apply(object);
        let shift = Shift {
            dx: about_point.x,
            dy: about_point.y,
            recursive: self.recursive,
        };
        shift.apply(object);
    }
}

pub struct ScaleToWidth {
    pub width: f32,
    pub stretch: Option<bool>,
    pub about_point: Option<Point2D>,
    pub recursive: Option<bool>,
}

impl VectorOperation for ScaleToWidth {
    fn apply(&self, object: &mut VectorObject) {
        let bounding_box = object.bounding_box(self.recursive);
        if bounding_box.is_none() {
            return;
        }
        let bounding_box = bounding_box.unwrap();
        let width = bounding_box.width();
        if width == 0.0 {
            return;
        }
        let factor_x = self.width / width;
        let factor_y = if self.stretch.unwrap_or(false) {
            1.0
        } else {
            factor_x
        };
        let scale = Scale {
            factor_x: factor_x,
            factor_y: factor_y,
            about_point: self.about_point,
            recursive: self.recursive,
        };
        scale.apply(object);
    }
}

pub struct ScaleToHeight {
    pub height: f32,
    pub stretch: Option<bool>,
    pub about_point: Option<Point2D>,
    pub recursive: Option<bool>,
}

impl VectorOperation for ScaleToHeight {
    fn apply(&self, object: &mut VectorObject) {
        let bounding_box = object.bounding_box(self.recursive);
        if bounding_box.is_none() {
            return;
        }
        let bounding_box = bounding_box.unwrap();
        let height = bounding_box.height();
        if height == 0.0 {
            return;
        }
        let factor_x = if self.stretch.unwrap_or(false) {
            1.0
        } else {
            self.height / height
        };
        let factor_y = self.height / height;
        let scale = Scale {
            factor_x: factor_x,
            factor_y: factor_y,
            about_point: self.about_point,
            recursive: self.recursive,
        };
        scale.apply(object);
    }
}

pub struct Rotate {
    pub angle: f32,
    pub from_point: Option<Point2D>,
    pub recursive: Option<bool>,
}

impl VectorOperation for Rotate {
    fn apply(&self, object: &mut VectorObject) {
        let bounding_box = object.bounding_box(self.recursive);
        if bounding_box.is_none() {
            return;
        }
        let bounding_box = bounding_box.unwrap();
        let center = bounding_box.center();
        let from_point = if let Some(point) = self.from_point {
            point
        } else {
            center
        };
        let shift = Shift {
            dx: -from_point.x,
            dy: -from_point.y,
            recursive: self.recursive,
        };
        shift.apply(object);
        let rotate = TransformationMatrix::rotate(self.angle);
        let apply_transform = ApplyTransform {
            matrix: rotate,
            recursive: self.recursive,
        };
        apply_transform.apply(object);
        let shift = Shift {
            dx: from_point.x,
            dy: from_point.y,
            recursive: self.recursive,
        };
        shift.apply(object);
    }
}

pub struct SetTransform {
    pub matrix: TransformationMatrix,
    pub recursive: Option<bool>,
}

impl VectorOperation for SetTransform {
    fn apply(&self, object: &mut VectorObject) {
        object.transform = self.matrix.clone();
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let transform = SetTransform {
                    matrix: self.matrix.clone(),
                    recursive: Some(true),
                };
                transform.apply(child);
            }
        }
    }
}

pub struct AddChildren {
    pub children: Vec<VectorObjectBuilder>,
}

impl VectorOperation for AddChildren {
    fn apply(&self, object: &mut VectorObject) {
        for child in &self.children {
            let child = child.clone().build();
            object.children.push(child);
        }
    }
}

pub struct RemoveChildByIndex {
    pub index: usize,
}

impl VectorOperation for RemoveChildByIndex {
    fn apply(&self, object: &mut VectorObject) {
        object.children.remove(self.index);
    }
}

pub struct RemoveChildByName {
    pub name: Rc<String>,
}

impl VectorOperation for RemoveChildByName {
    fn apply(&self, object: &mut VectorObject) {
        object.children.retain(|child| child.name != Some(Rc::clone(&self.name)));
    }
}

pub struct MatchStyleProperties {
    pub vector_object_builder: VectorObjectBuilder,
}

impl VectorOperation for MatchStyleProperties {
    fn apply(&self, object: &mut VectorObject) {
        let vector_object = self.vector_object_builder.clone().build();
        object.fill = vector_object.fill;
        object.fill_rule = vector_object.fill_rule;
        object.stroke = vector_object.stroke;
        object.stroke_width = vector_object.stroke_width;
        object.stroke_line_cap = vector_object.stroke_line_cap;
        object.stroke_line_join = vector_object.stroke_line_join;
        object.stroke_miter_limit = vector_object.stroke_miter_limit;
        object.stroke_dash_offset = vector_object.stroke_dash_offset;
        object.stroke_dash_array = vector_object.stroke_dash_array;
    }
}

pub struct BecomePartial {
    pub start: f32,
    pub end: f32,
    pub samples: Option<usize>,
    pub extra_length: Option<f32>,
    pub recursive: Option<bool>,
}

impl VectorOperation for BecomePartial {
    fn apply(&self, object: &mut VectorObject) {
        if self.start <= 0.0 && self.end >= 1.0 {
            return;
        }
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let become_partial = BecomePartial {
                    start: self.start,
                    end: self.end,
                    samples: self.samples,
                    extra_length: self.extra_length,
                    recursive: Some(true),
                };
                become_partial.apply(child);
            }
        }
        let length = object.path.length(self.samples, self.extra_length);
        object.stroke_dash_array = Rc::new(vec![length * (self.end - self.start), length]);
        object.stroke_dash_offset = -length * self.start;
    }
}

pub struct PointwiseBecomePartial {
    pub start: f32,
    pub end: f32,
    pub recursive: Option<bool>,
}

impl VectorOperation for PointwiseBecomePartial {
    fn apply(&self, object: &mut VectorObject) {
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let pointwise_become_partial = PointwiseBecomePartial {
                    start: self.start,
                    end: self.end,
                    recursive: Some(true),
                };
                pointwise_become_partial.apply(child);
            }
        }
        if self.start <= 0.0 && self.end >= 1.0 {
            return; 
        }
        if self.start == self.end {
            object.path = Path2D::default();
            return;
        }
        let num_curves = object.num_curves();
        if num_curves == 0 {
            return;
        }
        let lower = IntegerLerp::new(0.0, num_curves as f32, self.start);
        let upper = IntegerLerp::new(0.0, num_curves as f32, self.end);
        let lower_index = lower.index();
        let lower_remainder = lower.remainder();
        let upper_index = upper.index();
        let upper_remainder = upper.remainder();
        if lower_index == upper_index {
            let start = lower_remainder;
            let end = upper_remainder;
            let set_actual_path = SetActualPath {
                path: object.actual_path().slice(
                    4 * lower_index as usize,
                    (4 * lower_index + 4) as usize,
                ).partial_bezier_path(start, end),
            };
            set_actual_path.apply(object);
        } else {
            let mut actual_path = Path2D::fill(Point2D::default(), 4 * (upper_index - lower_index + 1) as usize);
            actual_path.set_slice(0, 4, object.actual_path().slice(
                4 * lower_index as usize,
                (4 * lower_index + 4) as usize,
            ).partial_bezier_path(lower_remainder, 1.0));
            actual_path.set_slice(4, actual_path.len() - 4, object.actual_path().slice(
                (4 * lower_index + 4) as usize,
                4 * upper_index as usize,
            ));
            actual_path.set_slice(actual_path.len() - 4, actual_path.len(), object.actual_path().slice(
                4 * upper_index as usize,
                (4 * upper_index + 4) as usize,
            ).partial_bezier_path(0.0, upper_remainder));
            let set_actual_path = SetActualPath {
                path: actual_path,
            };
            set_actual_path.apply(object);
        }
    }
}

pub struct MovePoint {
    pub point: Point2D,
}

impl VectorOperation for MovePoint {
    fn apply(&self, object: &mut VectorObject) {
        if object.num_points() % 4 == 0 {
            object.path.push(self.point);
        }
    }
}

pub struct LineTo {
    pub point: Point2D,
}

impl VectorOperation for LineTo {
    fn apply(&self, object: &mut VectorObject) {
        let last_point = object.path[object.path.len() - 1];
        object.path.push_bezier(CubicBezierTuple::new(last_point, last_point, self.point, self.point));
    }
}

pub struct QuadraticCurveTo {
    pub p1: Point2D,
    pub p2: Point2D,
}

impl VectorOperation for QuadraticCurveTo {
    fn apply(&self, object: &mut VectorObject) {
        let last_point = object.path[object.path.len() - 1];
        object.path.push_bezier(CubicBezierTuple::from_quadratic(last_point, self.p1, self.p2));
    }
}

pub struct BezierCurveTo {
    pub p1: Point2D,
    pub p2: Point2D,
    pub p3: Point2D,
}

impl VectorOperation for BezierCurveTo {
    fn apply(&self, object: &mut VectorObject) {
        let last_point = object.path[object.path.len() - 1];
        object.path.push_bezier(CubicBezierTuple::new(last_point, self.p1, self.p2, self.p3));
    }
}

pub struct Close {}

impl VectorOperation for Close {
    fn apply(&self, object: &mut VectorObject) {
        if object.is_closed() {
            return;
        }
        let line_to = LineTo {
            point: object.path[0],
        };
        line_to.apply(object);
    }
}

pub struct FadeFill {
    pub factor: f32,
    pub recursive: Option<bool>,
}

impl VectorOperation for FadeFill {
    fn apply(&self, object: &mut VectorObject) {
        object.fill = object.fill.fade(self.factor);
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let fade = FadeFill {
                    factor: self.factor,
                    recursive: Some(true),
                };
                fade.apply(child);
            }
        }
    }
}

pub struct FadeStroke {
    pub factor: f32,
    pub recursive: Option<bool>,
}

impl VectorOperation for FadeStroke {
    fn apply(&self, object: &mut VectorObject) {
        object.stroke = object.stroke.fade(self.factor);
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let fade = FadeStroke {
                    factor: self.factor,
                    recursive: Some(true),
                };
                fade.apply(child);
            }
        }
    }
}

pub struct SetPath {
    pub path: Path2D,
}

impl VectorOperation for SetPath {
    fn apply(&self, object: &mut VectorObject) {
        object.path = self.path.clone();
    }
}

pub struct SetActualPath {
    pub path: Path2D,
}

impl VectorOperation for SetActualPath {
    fn apply(&self, object: &mut VectorObject) {
        // To set the actual path, we must apply the inverse of the current transformation matrix to the path.
        // Because we want the actual path to be the path that would be drawn if the object were drawn with no transformation matrix.
        object.path = self.path.transform(&object.transform.inverse());
    }
}

pub struct SetFill {
    pub fill: Style,
    pub recursive: Option<bool>,
}

impl VectorOperation for SetFill {
    fn apply(&self, object: &mut VectorObject) {
        object.fill = self.fill.clone();
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let set_fill = SetFill {
                    fill: self.fill.clone(),
                    recursive: Some(true),
                };
                set_fill.apply(child);
            }
        }
    }
}

pub struct SetFillRule {
    pub fill_rule: Rc<String>,
    pub recursive: Option<bool>,
}

impl VectorOperation for SetFillRule {
    fn apply(&self, object: &mut VectorObject) {
        object.fill_rule = Rc::clone(&self.fill_rule);
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let set_fill_rule = SetFillRule {
                    fill_rule: Rc::clone(&self.fill_rule),
                    recursive: Some(true),
                };
                set_fill_rule.apply(child);
            }
        }
    }
}

pub struct SetStroke {
    pub stroke: Style,
    pub recursive: Option<bool>,
}

impl VectorOperation for SetStroke {
    fn apply(&self, object: &mut VectorObject) {
        object.stroke = self.stroke.clone();
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let set_stroke = SetStroke {
                    stroke: self.stroke.clone(),
                    recursive: Some(true),
                };
                set_stroke.apply(child);
            }
        }
    }
}

pub struct SetStrokeWidth {
    pub stroke_width: f32,
    pub recursive: Option<bool>,
}

impl VectorOperation for SetStrokeWidth {
    fn apply(&self, object: &mut VectorObject) {
        object.stroke_width = self.stroke_width;
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let set_stroke_width = SetStrokeWidth {
                    stroke_width: self.stroke_width,
                    recursive: Some(true),
                };
                set_stroke_width.apply(child);
            }
        }
    }
}

pub struct SetStrokeLineCap {
    pub stroke_line_cap: Rc<String>,
    pub recursive: Option<bool>,
}

impl VectorOperation for SetStrokeLineCap {
    fn apply(&self, object: &mut VectorObject) {
        object.stroke_line_cap = Rc::clone(&self.stroke_line_cap);
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let set_stroke_line_cap = SetStrokeLineCap {
                    stroke_line_cap: self.stroke_line_cap.clone(),
                    recursive: Some(true),
                };
                set_stroke_line_cap.apply(child);
            }
        }
    }
}

pub struct SetStrokeLineJoin {
    pub stroke_line_join: Rc<String>,
    pub recursive: Option<bool>,
}

impl VectorOperation for SetStrokeLineJoin {
    fn apply(&self, object: &mut VectorObject) {
        object.stroke_line_join = Rc::clone(&self.stroke_line_join);
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let set_stroke_line_join = SetStrokeLineJoin {
                    stroke_line_join: self.stroke_line_join.clone(),
                    recursive: Some(true),
                };
                set_stroke_line_join.apply(child);
            }
        }
    }
}

pub struct SetStrokeMiterLimit {
    pub stroke_miter_limit: f32,
    pub recursive: Option<bool>,
}

impl VectorOperation for SetStrokeMiterLimit {
    fn apply(&self, object: &mut VectorObject) {
        object.stroke_miter_limit = self.stroke_miter_limit;
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let set_stroke_miter_limit = SetStrokeMiterLimit {
                    stroke_miter_limit: self.stroke_miter_limit,
                    recursive: Some(true),
                };
                set_stroke_miter_limit.apply(child);
            }
        }
    }
}

pub struct SetStrokeDashOffset {
    pub stroke_dash_offset: f32,
    pub recursive: Option<bool>,
}

impl VectorOperation for SetStrokeDashOffset {
    fn apply(&self, object: &mut VectorObject) {
        object.stroke_dash_offset = self.stroke_dash_offset;
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let set_stroke_dash_offset = SetStrokeDashOffset {
                    stroke_dash_offset: self.stroke_dash_offset,
                    recursive: Some(true),
                };
                set_stroke_dash_offset.apply(child);
            }
        }
    }
}

pub struct SetStrokeDashArray {
    pub stroke_dash_array: Rc<Vec<f32>>,
    pub recursive: Option<bool>,
}

impl VectorOperation for SetStrokeDashArray {
    fn apply(&self, object: &mut VectorObject) {
        object.stroke_dash_array = Rc::clone(&self.stroke_dash_array);
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let set_stroke_dash_array = SetStrokeDashArray {
                    stroke_dash_array: self.stroke_dash_array.clone(),
                    recursive: Some(true),
                };
                set_stroke_dash_array.apply(child);
            }
        }
    }
}

pub struct SetChildren {
    pub children: Vec<VectorObjectBuilder>,
}

impl VectorOperation for SetChildren {
    fn apply(&self, object: &mut VectorObject) {
        object.children = self.children.iter().map(|child| child.clone().build()).collect();
    }
}

pub struct SetName {
    pub name: Option<Rc<String>>,
}

impl VectorOperation for SetName {
    fn apply(&self, object: &mut VectorObject) {
        if let Some(name) = &self.name {
            object.name = Some(Rc::clone(name));
        } else {
            object.name = None;
        }
    }
}

pub struct AddChild {
    pub child: VectorObjectBuilder,
}

impl VectorOperation for AddChild {
    fn apply(&self, object: &mut VectorObject) {
        let child = self.child.clone().build();
        object.children.push(child);
    }
}

pub struct InsertChild {
    pub index: usize,
    pub child: VectorObjectBuilder,
}

impl VectorOperation for InsertChild {
    fn apply(&self, object: &mut VectorObject) {
        object.children.insert(self.index, self.child.clone().build());
    }
}

pub struct InsertChildren {
    pub index: usize,
    pub children: Vec<VectorObjectBuilder>,
}

impl VectorOperation for InsertChildren {
    fn apply(&self, object: &mut VectorObject) {
        for (i, child) in self.children.iter().enumerate() {
            object.children.insert(self.index + i, child.clone().build());
        }
    }
}

pub struct RemoveChildrenAtIndices {
    pub indices: Vec<usize>,
}

impl VectorOperation for RemoveChildrenAtIndices {
    fn apply(&self, object: &mut VectorObject) {
        let mut new_children = Vec::new();
        for (i, child) in object.children.iter().enumerate() {
            if !self.indices.contains(&i) {
                new_children.push(child.clone());
            }
        }
        object.children = new_children;
    }
}

pub struct RemoveChildrenByNames {
    pub names: Vec<String>,
}

impl VectorOperation for RemoveChildrenByNames {
    fn apply(&self, object: &mut VectorObject) {
        object.children.retain(|child| {
            if let Some(name) = &child.name {
                !self.names.contains(&name)
            } else {
                true
            }
        });
    }
}

pub struct SetSliceChildren {
    pub start: usize,
    pub end: usize,
    pub children: Vec<VectorObjectBuilder>,
}

pub struct SetChildrenWithNames {
    pub names: Vec<String>,
    pub children: Vec<VectorObjectBuilder>,
}

impl VectorOperation for SetChildrenWithNames {
    fn apply(&self, object: &mut VectorObject) {
        let children = self.children.clone().iter().map(|child| child.clone().build()).collect::<Vec<VectorObject>>();
        for child in children.iter() {
            if let Some(name) = &child.name {
                if self.names.contains(&name) {
                    object.children.push(child.clone());
                }
            }
        }
    }
}

impl VectorOperation for SetSliceChildren {
    fn apply(&self, object: &mut VectorObject) {
        object.children.splice(self.start..self.end, self.children.iter().map(|child| child.clone().build()));
    }
}

pub struct NextToPoint {
    pub point: Point2D,
    pub direction: Option<Point2D>,
    pub buff: Option<f32>,
    pub aligned_edge: Option<Point2D>,
    pub recursive: Option<bool>,
}

impl VectorOperation for NextToPoint {
    fn apply(&self, object: &mut VectorObject) {
        let bounding_box = object.bounding_box(self.recursive);
        if bounding_box.is_none() {
            return;
        }
        let direction = if let Some(direction) = self.direction {
            direction
        } else {
            Point2D::new(1.0, 0.0)
        };
        let buff = self.buff.unwrap_or(0.0);
        let aligned_edge = if let Some(aligned_edge) = self.aligned_edge {
            aligned_edge
        } else {
            Point2D::new(0.0, 0.0)
        };
        let key2 = aligned_edge - direction;
        let key2_x = key2.x;
        let key2_y = key2.y;
        let point_to_align = object.get_critical_point(key2_x, key2_y, self.recursive).unwrap();
        let shift = self.point - point_to_align + buff * direction;
        let shift = Shift {
            dx: shift.x,
            dy: shift.y,
            recursive: self.recursive,
        };
        shift.apply(object);
    }
}

pub struct NextToOther {
    pub other: VectorObjectBuilder,
    pub direction: Option<Point2D>,
    pub buff: Option<f32>,
    pub aligned_edge: Option<Point2D>,
    pub recursive: Option<bool>,
}

impl VectorOperation for NextToOther {
    fn apply(&self, object: &mut VectorObject) {
        let bounding_box = object.bounding_box(self.recursive);
        if bounding_box.is_none() {
            return;
        }
        let other = self.other.clone().build();
        let other_bounding_box = other.bounding_box(self.recursive);
        if other_bounding_box.is_none() {
            return;
        }
        let direction = if let Some(direction) = self.direction {
            direction
        } else {
            Point2D::new(1.0, 0.0)
        };
        let buff = self.buff.unwrap_or(0.0);
        let aligned_edge = if let Some(aligned_edge) = self.aligned_edge {
            aligned_edge
        } else {
            Point2D::new(0.0, 0.0)
        };
        let key1 = aligned_edge + direction;
        let key1_x = key1.x;
        let key1_y = key1.y;
        let target_point = other.get_critical_point(key1_x, key1_y, self.recursive).unwrap();
        let next_to_point = NextToPoint {
            point: target_point,
            direction: Some(direction),
            buff: Some(buff),
            aligned_edge: Some(aligned_edge),
            recursive: self.recursive,
        };
        next_to_point.apply(object);
    }
}

pub struct ArrangeChildren {
    pub direction: Option<Point2D>,
    pub buff: Option<f32>,
    pub aligned_edge: Option<Point2D>,
    pub center: Option<Point2D>,
    pub recursive: Option<bool>,
}

impl VectorOperation for ArrangeChildren {
    fn apply(&self, object: &mut VectorObject) {
        let buff = self.buff.unwrap_or(0.0);
        let aligned_edge = if let Some(aligned_edge) = self.aligned_edge {
            aligned_edge
        } else {
            Point2D::new(0.0, 0.0)
        };
        let first_child = object.children[0].clone();
        for i in 1..object.children.len() {
            object.children[i].apply_operation(&NextToOther {
                other: VectorObjectBuilder::new(&first_child),
                direction: self.direction,
                buff: Some(buff),
                aligned_edge: Some(aligned_edge),
                recursive: self.recursive,
            });
        }
        if let Some(center) = self.center {
            let move_to = MoveTo {
                point: center,
                recursive: self.recursive,
            };
            move_to.apply(object);
        }
    }
}

pub struct ReversePath {}

impl VectorOperation for ReversePath {
    fn apply(&self, object: &mut VectorObject) {
        object.path.reverse();
    }
}

pub struct ApplyTransform {
    pub matrix: TransformationMatrix,
    pub recursive: Option<bool>,
}

impl VectorOperation for ApplyTransform {
    fn apply(&self, object: &mut VectorObject) {
        object.transform.apply(&self.matrix);
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let apply_transform = ApplyTransform {
                    matrix: self.matrix.clone(),
                    recursive: Some(true),
                };
                apply_transform.apply(child);
            }
        }
    }
}

pub struct LerpFill {
    pub fill: Style,
    pub t: f32,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub data_width: Option<usize>,
    pub data_height: Option<usize>,
    pub recursive: Option<bool>,
}

impl VectorOperation for LerpFill {
    fn apply(&self, object: &mut VectorObject) {
        let fill = Style::lerp(&object.fill, &self.fill, self.t, self.x, self.y, self.width, self.height, self.data_width, self.data_height);
        if fill.is_err() {
            throw_str(&fill.unwrap_err());
        }
        object.fill = fill.unwrap();
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let lerp_fill = LerpFill {
                    fill: self.fill.clone(),
                    t: self.t,
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height: self.height,
                    data_width: self.data_width,
                    data_height: self.data_height,
                    recursive: Some(true),
                };
                lerp_fill.apply(child);
            }
        }
    }
}

pub struct LerpStroke {
    pub stroke: Style,
    pub t: f32,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub data_width: Option<usize>,
    pub data_height: Option<usize>,
    pub recursive: Option<bool>,
}

impl VectorOperation for LerpStroke {
    fn apply(&self, object: &mut VectorObject) {
        let stroke = Style::lerp(&object.stroke, &self.stroke, self.t, self.x, self.y, self.width, self.height, self.data_width, self.data_height);
        if stroke.is_err() {
            throw_str(&stroke.unwrap_err());
        }
        object.stroke = stroke.unwrap();
        if self.recursive.unwrap_or(true) {
            for child in &mut object.children {
                let lerp_stroke = LerpStroke {
                    stroke: self.stroke.clone(),
                    t: self.t,
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height: self.height,
                    data_width: self.data_width,
                    data_height: self.data_height,
                    recursive: Some(true),
                };
                lerp_stroke.apply(child);
            }
        }
    }
}

impl Default for VectorObject {
    fn default() -> Self {
        VectorObject {
            path: Path2D::default(),
            fill: Style::default(),
            fill_rule: Rc::new("nonzero".to_string()),
            stroke: Style::default(),
            stroke_width: 1.0,
            stroke_line_cap: Rc::new("butt".to_string()),
            stroke_line_join: Rc::new("miter".to_string()),
            stroke_miter_limit: 4.0,
            stroke_dash_offset: 0.0,
            stroke_dash_array: Rc::new(Vec::new()),
            children: Vec::new(),
            name: None,
            transform: TransformationMatrix::identity(),
        }
    }
}

/// A VectorObjectBuilder can be used to build a VectorObject with operations.
#[wasm_bindgen]
#[derive(Clone)]
pub struct VectorObjectBuilder {
    /// The vector object being built.
    object: Rc<VectorObject>,
    /// The operations to apply to the vector object.
    ops: VectorOperationList,
}

impl Default for VectorObjectBuilder {
    fn default() -> Self {
        VectorObjectBuilder {
            object: Rc::new(VectorObject::default()),
            ops: VectorOperationList::new(),
        }
    }
}

#[wasm_bindgen]
impl VectorObjectBuilder {
    /// Creates a new VectorObjectBuilder from a VectorObject.
    #[wasm_bindgen(constructor, return_description = "A new vector object builder.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The vector object to start building on.")]
        vector_object: &VectorObject
    ) -> VectorObjectBuilder {
        VectorObjectBuilder {
            object: Rc::new(vector_object.clone()),
            ops: VectorOperationList::new(),
        }
    }

    /// Returns the default tip shape pointing to the right and centered at the origin as a VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The default tip shape pointing to the right and centered at the origin.")]
    pub fn default_tip_shape(
        #[wasm_bindgen(param_description = "The length of the tip shape.")]
        tip_length: f64,
    ) -> VectorObjectBuilder {
        EquilateralTriangle::new(
            Point2D::new(0.0, 0.0),
            tip_length as f32,
            Some(-std::f32::consts::FRAC_PI_2),
        ).vector_object_builder().set_fill(Style::from_color(Color::new(0, 0, 0, 1.0)), None)
    }

    /// Creates a new VectorObjectBuilder from an SVG string.
    #[wasm_bindgen(return_description = "A new vector object builder.")]
    pub fn from_svg(
        #[wasm_bindgen(param_description = "The SVG string to create the vector object builder from.")]
        svg: String,
        #[wasm_bindgen(param_description = "Data from font faces to use for text rendering.")]
        font_faces: Option<Vec<FontFace>>,
        #[wasm_bindgen(param_description = "Image library to use for image rendering.")]
        image_library: Option<ImageLibrary>,
    ) -> VectorObjectBuilder {
        let mut vector_object_builder = VectorObjectBuilder::default();
        let mut options = usvg::Options::default();
        let mut fontdatabase = options.fontdb.clone();
        for font_face in font_faces.unwrap_or_default() {
            Arc::make_mut(&mut fontdatabase).load_font_data(font_face.data());
        }
        let image_library = Box::new(image_library.unwrap_or(ImageLibrary::new()));
        let image_library = Box::leak(image_library);
        options.image_href_resolver = ImageHrefResolver {
            resolve_data: Box::new(|mime, data, _opts| {
                // Don't care about the mime type, just return the data.
                match mime {
                    "image/png" => {
                        Some(ImageKind::PNG(data))
                    }
                    "image/jpeg" | "image/jpg" => {
                        Some(ImageKind::JPEG(data))
                    }
                    "image/gif" => {
                        Some(ImageKind::GIF(data))
                    }
                    "image/webp" => {
                        Some(ImageKind::WEBP(data))
                    }
                    "image/svg+xml" => {
                        let data = image_library.get(&format!("data:image/png;base64,{}", BASE64_STANDARD.encode(data.to_vec())));
                        if data.is_none() {
                            return None;
                        }
                        Some(ImageKind::PNG(Arc::new(data.unwrap().data())))
                    }
                    _ => {
                        None
                    }
                }
            }),
            resolve_string: Box::new(|string, _opts| {
                let data = image_library.get(&string);
                if data.is_none() {
                    return None;
                }
                Some(ImageKind::PNG(Arc::new(data.unwrap().data())))
            }),
        };
        options.fontdb = fontdatabase;
        let tree = usvg::Tree::from_str(&svg, &options);
        if tree.is_err() {
            log(&format!("Error parsing SVG: {}", tree.err().unwrap()));
            return vector_object_builder;
        }
        let tree = tree.unwrap();
        if !tree.root().has_children() {
            log("The SVG path is empty. No operations were applied.");
            return vector_object_builder;
        }
        let node = tree.root().children();
        for child in node {
            vector_object_builder = vector_object_builder.add_child(VectorObjectBuilder::from_node(&child));
        }
        vector_object_builder
    }
    /// Clones the VectorObjectBuilder.
    #[wasm_bindgen(js_name = clone, return_description = "A clone of the vector object builder.")]
    pub fn clone_js(&self) -> VectorObjectBuilder {
        self.clone()
    }
    /// Creates a VectorObjectBuilder representing an empty VectorObject.
    #[wasm_bindgen(return_description = "A new vector object builder representing an empty vector object.")]
    pub fn default_vector_object_builder() -> VectorObjectBuilder {
        VectorObjectBuilder::default()
    }
    /// Shifts the VectorObjectBuilder by the given dx and dy.
    #[wasm_bindgen(return_description = "The vector object being built with the shift operation.")]
    pub fn shift(
        mut self,
        #[wasm_bindgen(param_description = "The x-coordinate to translate the vector object by.")]
        dx: f32,
        #[wasm_bindgen(param_description = "The y-coordinate to translate the vector object by.")]
        dy: f32,
        #[wasm_bindgen(param_description = "Whether to apply the shift operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let shift = Box::new(Shift { dx, dy, recursive });
        self.ops.add_operation(Box::leak(shift));
        self
    }
    /// Centers the VectorObjectBuilder at the given Point2D.
    #[wasm_bindgen(return_description = "The vector object being built with the move to operation.")]
    pub fn move_to(
        mut self,
        #[wasm_bindgen(param_description = "The point to center the vector object at.")]
        point: Point2D,
        #[wasm_bindgen(param_description = "Whether to apply the move to operation to the children of the vector object.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let move_to = Box::new(MoveTo { point, recursive });
        self.ops.add_operation(Box::leak(move_to));
        self
    }
    /// Scales the VectorObjectBuilder by the given factor.
    #[wasm_bindgen(return_description = "The vector object being built with the scale operation.")]
    pub fn scale(
        mut self,
        #[wasm_bindgen(param_description = "The factor to scale the vector object by.")]
        factor_x: f32,
        #[wasm_bindgen(param_description = "The factor to scale the vector object by.")]
        factor_y: f32,
        #[wasm_bindgen(param_description = "The point to scale the vector object about.")]
        about_point: Option<Point2D>,
        #[wasm_bindgen(param_description = "Whether to apply the scale operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let scale = Box::new(Scale { factor_x, factor_y, about_point, recursive });
        self.ops.add_operation(Box::leak(scale));
        self
    }
    /// Scales the VectorObjectBuilder to the given width.
    #[wasm_bindgen(return_description = "The vector object being built with the scale to width operation.")]
    pub fn scale_to_width(
        mut self,
        #[wasm_bindgen(param_description = "The width to scale the vector object to.")]
        width: f32,
        #[wasm_bindgen(param_description = "Whether to stretch the vector object to the given width, default is false.")]
        stretch: Option<bool>,
        #[wasm_bindgen(param_description = "The point to scale the vector object about, default is the center of the bounding box.")]
        about_point: Option<Point2D>,
        #[wasm_bindgen(param_description = "Whether to apply the scale to width operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let scale_to_width = Box::new(ScaleToWidth { width, stretch, about_point, recursive });
        self.ops.add_operation(Box::leak(scale_to_width));
        self
    }
    /// Scales the VectorObjectBuilder to the given height.
    #[wasm_bindgen(return_description = "The vector object being built with the scale to height operation.")]
    pub fn scale_to_height(
        mut self,
        #[wasm_bindgen(param_description = "The height to scale the vector object to.")]
        height: f32,
        #[wasm_bindgen(param_description = "Whether to stretch the vector object to the given height, default is false.")]
        stretch: Option<bool>,
        #[wasm_bindgen(param_description = "The point to scale the vector object about, default is the center of the bounding box.")]
        about_point: Option<Point2D>,
        #[wasm_bindgen(param_description = "Whether to apply the scale to height operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let scale_to_height = Box::new(ScaleToHeight { height, stretch, about_point, recursive });
        self.ops.add_operation(Box::leak(scale_to_height));
        self
    }
    /// Rotates the VectorObjectBuilder by the given angle in radians.
    #[wasm_bindgen(return_description = "The vector object being built with the rotate operation.")]
    pub fn rotate(
        mut self,
        #[wasm_bindgen(param_description = "The angle in radians to rotate the vector object by.")]
        angle: f32,
        #[wasm_bindgen(param_description = "The point to rotate the vector object about, default is the center of the bounding box.")]
        from_point: Option<Point2D>,
        #[wasm_bindgen(param_description = "Whether to apply the rotate operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let rotate = Box::new(Rotate { angle, from_point, recursive });
        self.ops.add_operation(Box::leak(rotate));
        self
    }
    /// Sets the TransformationMatrix of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the transform operation.")]
    pub fn set_transform(
        mut self,
        #[wasm_bindgen(param_description = "The matrix in CSS format to transform the vector object by.")]
        matrix: TransformationMatrix,
        #[wasm_bindgen(param_description = "Whether to apply the transform operation to the children of the vector object.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let transform = Box::new(SetTransform { matrix: matrix.clone(), recursive });
        self.ops.add_operation(Box::leak(transform));
        self
    }
    /// Applies the TransformationMatrix to the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the apply transform operation.")]
    pub fn apply_transform(
        mut self,
        #[wasm_bindgen(param_description = "The matrix in CSS format to apply to the vector object.")]
        matrix: TransformationMatrix,
        #[wasm_bindgen(param_description = "Whether to apply the apply transform operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let apply_transform = Box::new(ApplyTransform { matrix, recursive });
        self.ops.add_operation(Box::leak(apply_transform));
        self
    }
    /// Adds a child to the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the add children operation.")]
    pub fn add_children(
        mut self,
        #[wasm_bindgen(param_description = "The children to add at the end of the vector object.")]
        children: Vec<VectorObjectBuilder>
    ) -> VectorObjectBuilder {
        let add_children = Box::new(AddChildren { children });
        self.ops.add_operation(Box::leak(add_children));
        self
    }
    /// Inserts a child into the VectorObjectBuilder at the given index.
    #[wasm_bindgen(return_description = "The vector object being built with the insert child operation.")]
    pub fn insert_child(
        mut self,
        #[wasm_bindgen(param_description = "The index to insert the child at.")]
        index: usize,
        #[wasm_bindgen(param_description = "The child to insert into the vector object.")]
        child: VectorObjectBuilder
    ) -> VectorObjectBuilder {
        let insert_child = Box::new(InsertChild { index, child });
        self.ops.add_operation(Box::leak(insert_child));
        self
    }
    /// Inserts children into the VectorObjectBuilder at the given index.
    #[wasm_bindgen(return_description = "The vector object being built with the insert children operation.")]
    pub fn insert_children(
        mut self,
        #[wasm_bindgen(param_description = "The index to insert the children at.")]
        index: usize,
        #[wasm_bindgen(param_description = "The children to insert into the vector object.")]
        children: Vec<VectorObjectBuilder>
    ) -> VectorObjectBuilder {
        let insert_children = Box::new(InsertChildren { index, children });
        self.ops.add_operation(Box::leak(insert_children));
        self
    }
    /// Removes a child from the VectorObjectBuilder given its index.
    #[wasm_bindgen(return_description = "The vector object being built with the remove child by index operation.")]
    pub fn remove_child_by_index(
        mut self,
        #[wasm_bindgen(param_description = "The index of the child to remove from the vector object.")]
        index: usize
    ) -> VectorObjectBuilder {
        let remove_child_by_index = Box::new(RemoveChildByIndex { index });
        self.ops.add_operation(Box::leak(remove_child_by_index));
        self
    }
    /// Removes a child from the VectorObjectBuilder given its name.
    #[wasm_bindgen(return_description = "The vector object being built with the remove child by name operation.")]
    pub fn remove_child_by_name(
        mut self,
        #[wasm_bindgen(param_description = "The name of the child to remove from the vector object.")]
        name: String
    ) -> VectorObjectBuilder {
        let remove_child_by_name = Box::new(RemoveChildByName { name: Rc::new(name) });
        self.ops.add_operation(Box::leak(remove_child_by_name));
        self
    }
    /// Matches the style properties of the VectorObjectBuilder with another VectorObject.
    #[wasm_bindgen(return_description = "The vector object being built with the match style properties operation.")]
    pub fn match_style_properties(
        mut self,
        #[wasm_bindgen(param_description = "The vector object to match the style properties of.")]
        vector_object_builder: VectorObjectBuilder
    ) -> VectorObjectBuilder {
        let match_style_properties = Box::new(MatchStyleProperties { vector_object_builder });
        self.ops.add_operation(Box::leak(match_style_properties));
        self
    }
    /// Sets the actual path of the VectorObjectBuilder. Actual path is the path that is drawn with its transformation matrix applied.
    #[wasm_bindgen(return_description = "The vector object being built with the set actual path operation.")]
    pub fn set_actual_path(
        mut self,
        #[wasm_bindgen(param_description = "The path to set the vector object to.")]
        actual_path: Path2D
    ) -> VectorObjectBuilder {
        let set_actual_path = Box::new(SetActualPath { path: actual_path });
        self.ops.add_operation(Box::leak(set_actual_path));
        self
    }
    /// Trims the stroke of the VectorObjectBuilder to the given start and end proportions.
    #[wasm_bindgen(return_description = "The vector object being built with the become partial operation.")]
    pub fn become_partial(
        mut self,
        #[wasm_bindgen(param_description = "The proportion of the path to start at.")]
        start: f32,
        #[wasm_bindgen(param_description = "The proportion of the path to end at.")]
        end: f32,
        #[wasm_bindgen(param_description = "Number of samples to compute the length of each cubic bezier curve segment.")]
        samples: Option<usize>,
        #[wasm_bindgen(param_description = "Extra length to add to each length computation to ensure the path is not too short.")]
        extra_length: Option<f32>,
        #[wasm_bindgen(param_description = "Whether to apply the become partial operation to the children of the vector object, default is true")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let become_partial = Box::new(BecomePartial { start, end, samples, extra_length, recursive });
        self.ops.add_operation(Box::leak(become_partial));
        self
    }
    /// Trims the path of the VectorObjectBuilder to the given start and end proportions.
    pub fn pointwise_become_partial(
        mut self,
        #[wasm_bindgen(param_description = "The proportion of the path to start at.")]
        start: f32,
        #[wasm_bindgen(param_description = "The proportion of the path to end at.")]
        end: f32,
        #[wasm_bindgen(param_description = "Whether to apply the pointwise become partial operation to the children of the vector object, default is true")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let pointwise_become_partial = Box::new(PointwiseBecomePartial { start, end, recursive });
        self.ops.add_operation(Box::leak(pointwise_become_partial));
        self
    }
    /// Moves the current drawing point to the given Point2D.
    #[wasm_bindgen(return_description = "The vector object being built with the move point operation.")]
    pub fn move_point(
        mut self,
        #[wasm_bindgen(param_description = "The point to start a new bezier curve at.")]
        point: &Point2D
    ) -> VectorObjectBuilder {
        let move_point = Box::new(MovePoint { point: *point });
        self.ops.add_operation(Box::leak(move_point));
        self
    }
    /// Draws a line from the current drawing point to the given Point2D.
    #[wasm_bindgen(return_description = "The vector object being built with the line to operation.")]
    pub fn line_to(
        mut self,
        #[wasm_bindgen(param_description = "The point to draw a line to.")]
        p: &Point2D
    ) -> VectorObjectBuilder {
        let line_to = Box::new(LineTo { point: *p });
        self.ops.add_operation(Box::leak(line_to));
        self
    }
    /// Draws a quadratic bezier curve from the current drawing point with the given control Point2D and end Point2D.
    #[wasm_bindgen(return_description = "The vector object being built with the quadratic curve to operation.")]
    pub fn quadratic_curve_to(
        mut self,
        #[wasm_bindgen(param_description = "The control point of the quadratic curve.")]
        p1: &Point2D,
        #[wasm_bindgen(param_description = "The end point of the quadratic curve.")]
        p2: &Point2D
    ) -> VectorObjectBuilder {
        let quadratic_curve_to = Box::new(QuadraticCurveTo { p1: *p1, p2: *p2 });
        self.ops.add_operation(Box::leak(quadratic_curve_to));
        self
    }
    /// Draws a cubic bezier curve from the current drawing point with the given control Point2Ds and end Point2D.
    #[wasm_bindgen(return_description = "The vector object being built with the bezier curve to operation.")]
    pub fn bezier_curve_to(
        mut self,
        #[wasm_bindgen(param_description = "The first control point of the bezier curve.")]
        p1: &Point2D,
        #[wasm_bindgen(param_description = "The second control point of the bezier curve.")]
        p2: &Point2D,
        #[wasm_bindgen(param_description = "The end point of the bezier curve.")]
        p3: &Point2D
    ) -> VectorObjectBuilder {
        let bezier_curve_to = Box::new(BezierCurveTo { p1: *p1, p2: *p2, p3: *p3 });
        self.ops.add_operation(Box::leak(bezier_curve_to));
        self
    }
    /// Closes the current subpath of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the close operation.")]
    pub fn close(mut self) -> VectorObjectBuilder {
        let close = Box::new(Close {});
        self.ops.add_operation(Box::leak(close));
        self
    }
    /// Fades fill's opacity of the VectorObjectBuilder by the given factor.
    #[wasm_bindgen(return_description = "The vector object being built with the fade fill operation.")]
    pub fn fade_fill(
        mut self,
        #[wasm_bindgen(param_description = "The factor to fade the fill style by.")]
        factor: f32,
        #[wasm_bindgen(param_description = "Whether to apply the fade fill operation to the children of the vector object.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let fade_fill = Box::new(FadeFill { factor, recursive });
        self.ops.add_operation(Box::leak(fade_fill));
        self
    }
    /// Fades stroke's opacity of the VectorObjectBuilder by the given factor.
    #[wasm_bindgen(return_description = "The vector object being built with the fade stroke operation.")]
    pub fn fade_stroke(
        mut self,
        #[wasm_bindgen(param_description = "The factor to fade the stroke style by.")]
        factor: f32,
        #[wasm_bindgen(param_description = "Whether to apply the fade stroke operation to the children of the vector object.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let fade_stroke = Box::new(FadeStroke { factor, recursive });
        self.ops.add_operation(Box::leak(fade_stroke));
        self
    }
    /// Sets the Path2D of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the set path operation.")]
    pub fn set_path(
        mut self,
        #[wasm_bindgen(param_description = "The path to set the vector object to.")]
        path: Path2D
    ) -> VectorObjectBuilder {
        let set_path = Box::new(SetPath { path });
        self.ops.add_operation(Box::leak(set_path));
        self
    }
    /// Sets the fill Style of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the set fill operation.")]
    pub fn set_fill(
        mut self,
        #[wasm_bindgen(param_description = "The fill style to set the vector object to.")]
        fill: Style,
        #[wasm_bindgen(param_description = "Whether to apply the set fill operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let set_fill = Box::new(SetFill { fill, recursive });
        self.ops.add_operation(Box::leak(set_fill));
        self
    }
    /// Sets the fill rule of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the set fill rule operation.")]
    pub fn set_fill_rule(
        mut self,
        #[wasm_bindgen(param_description = "The fill rule to set the vector object to.")]
        fill_rule: String,
        #[wasm_bindgen(param_description = "Whether to apply the set fill rule operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let set_fill_rule = Box::new(SetFillRule { fill_rule: Rc::new(fill_rule), recursive });
        self.ops.add_operation(Box::leak(set_fill_rule));
        self
    }
    /// Sets the stroke Style of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the set stroke operation.")]
    pub fn set_stroke(
        mut self,
        #[wasm_bindgen(param_description = "The stroke style to set the vector object to.")]
        stroke: Style,
        #[wasm_bindgen(param_description = "Whether to apply the set stroke operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let set_stroke = Box::new(SetStroke { stroke, recursive });
        self.ops.add_operation(Box::leak(set_stroke));
        self
    }
    /// Sets the stroke width of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the set stroke width operation.")]
    pub fn set_stroke_width(
        mut self,
        #[wasm_bindgen(param_description = "The stroke width to set the vector object to.")]
        stroke_width: f32,
        #[wasm_bindgen(param_description = "Whether to apply the set stroke width operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let set_stroke_width = Box::new(SetStrokeWidth { stroke_width, recursive });
        self.ops.add_operation(Box::leak(set_stroke_width));
        self
    }
    /// Sets the stroke line cap of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the set stroke line cap operation.")]
    pub fn set_stroke_line_cap(
        mut self,
        #[wasm_bindgen(param_description = "The stroke line cap to set the vector object to.")]
        stroke_line_cap: String,
        #[wasm_bindgen(param_description = "Whether to apply the set stroke line cap operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let set_stroke_line_cap = Box::new(SetStrokeLineCap { stroke_line_cap: Rc::new(stroke_line_cap), recursive });
        self.ops.add_operation(Box::leak(set_stroke_line_cap));
        self
    }
    /// Sets the stroke line join of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the set stroke line join operation.")]
    pub fn set_stroke_line_join(
        mut self,
        #[wasm_bindgen(param_description = "The stroke line join to set the vector object to.")]
        stroke_line_join: String,
        #[wasm_bindgen(param_description = "Whether to apply the set stroke line join operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let set_stroke_line_join = Box::new(SetStrokeLineJoin { stroke_line_join: Rc::new(stroke_line_join), recursive });
        self.ops.add_operation(Box::leak(set_stroke_line_join));
        self
    }
    /// Sets the stroke miter limit of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the set stroke miter limit operation.")]
    pub fn set_stroke_miter_limit(
        mut self,
        #[wasm_bindgen(param_description = "The stroke miter limit to set the vector object to.")]
        stroke_miter_limit: f32,
        #[wasm_bindgen(param_description = "Whether to apply the set stroke miter limit operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let set_stroke_miter_limit = Box::new(SetStrokeMiterLimit { stroke_miter_limit, recursive });
        self.ops.add_operation(Box::leak(set_stroke_miter_limit));
        self
    }
    /// Sets the stroke dash offset of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the set stroke dash offset operation.")]
    pub fn set_stroke_dash_offset(
        mut self,
        #[wasm_bindgen(param_description = "The stroke dash offset to set the vector object to.")]
        stroke_dash_offset: f32,
        #[wasm_bindgen(param_description = "Whether to apply the set stroke dash offset operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let set_stroke_dash_offset = Box::new(SetStrokeDashOffset { stroke_dash_offset, recursive });
        self.ops.add_operation(Box::leak(set_stroke_dash_offset));
        self
    }
    /// Sets the stroke dash array of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the set stroke dash array operation.")]
    pub fn set_stroke_dash_array(
        mut self,
        #[wasm_bindgen(param_description = "The stroke dash array to set the vector object to.", unchecked_param_type = "number[]")]
        stroke_dash_array: Vec<f32>,
        #[wasm_bindgen(param_description = "Whether to apply the set stroke dash array operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let set_stroke_dash_array = Box::new(SetStrokeDashArray { stroke_dash_array: Rc::new(stroke_dash_array), recursive });
        self.ops.add_operation(Box::leak(set_stroke_dash_array));
        self
    }
    /// Sets the children of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the set children operation.")]
    pub fn set_children(
        mut self,
        #[wasm_bindgen(param_description = "The children to set the vector object to.")]
        children: Vec<VectorObjectBuilder>
    ) -> VectorObjectBuilder {
        let set_children = Box::new(SetChildren { children });
        self.ops.add_operation(Box::leak(set_children));
        self
    }
    /// Sets the name of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the set name operation.")]
    pub fn set_name(
        mut self,
        #[wasm_bindgen(param_description = "The name to set the vector object to.")]
        name: Option<String>
    ) -> VectorObjectBuilder {
        let set_name = Box::new(SetName { name: name.map(Rc::new) });
        self.ops.add_operation(Box::leak(set_name));
        self
    }
    /// Adds a child to the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the add child operation.")]
    pub fn add_child(
        mut self,
        #[wasm_bindgen(param_description = "The child to add to the vector object.")]
        child: VectorObjectBuilder
    ) -> VectorObjectBuilder {
        let add_child = Box::new(AddChild { child });
        self.ops.add_operation(Box::leak(add_child));
        self
    }
    /// Removes all children with the given indices from the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the remove children at indices operation.")]
    pub fn remove_children_at_indices(
        mut self,
        #[wasm_bindgen(param_description = "The indices of the children to remove from the vector object.", unchecked_param_type = "number[]")]
        indices: Vec<usize>
    ) -> VectorObjectBuilder {
        let remove_children_at_indices = Box::new(RemoveChildrenAtIndices { indices });
        self.ops.add_operation(Box::leak(remove_children_at_indices));
        self
    }
    /// Removes all children with the given names from the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the remove children by names operation.")]
    pub fn remove_children_by_names(
        mut self,
        #[wasm_bindgen(param_description = "The names of the children to remove from the vector object.", unchecked_param_type = "string[]")]
        names: Vec<String>
    ) -> VectorObjectBuilder {
        let remove_children_by_names = Box::new(RemoveChildrenByNames { names });
        self.ops.add_operation(Box::leak(remove_children_by_names));
        self
    }
    /// Sets the children from the start index to the end index of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the set slice children operation.")]
    pub fn set_slice_children(
        mut self,
        #[wasm_bindgen(param_description = "The start index of the children to replace.")]
        start: usize,
        #[wasm_bindgen(param_description = "The end index of the children to replace.")]
        end: usize,
        #[wasm_bindgen(param_description = "The children to replace the children from the start index to the end index with.")]
        children: Vec<VectorObjectBuilder>
    ) -> VectorObjectBuilder {
        let set_slice_children = Box::new(SetSliceChildren { start, end, children });
        self.ops.add_operation(Box::leak(set_slice_children));
        self
    }
    /// Puts the VectorObjectBuilder next to the given Point2D at the given direction, with a buff distance between them and aligning at the given edge.
    #[wasm_bindgen(return_description = "The vector object being built with the next to point operation.")]
    pub fn next_to_point(
        mut self,
        #[wasm_bindgen(param_description = "The point to translate the vector object to.")]
        point: Point2D,
        #[wasm_bindgen(param_description = "The direction to translate the vector object in. If not given, the vector object will be at the right of the point, default is right.")]
        direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The buffer to leave between the vector object and the point. If not given, the vector object will be touching the point, default is 0.")]
        buff: Option<f32>,
        #[wasm_bindgen(param_description = "The edge of the vector object to align with the point. If not given, the vector object will be aligned at the middle, default is the middle.")]
        aligned_edge: Option<Point2D>,
        #[wasm_bindgen(param_description = "Whether to apply the next to point operation to the children of the vector object. If not given, the operation will be applied to the children, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let next_to_point = Box::new(NextToPoint { point, direction, buff, aligned_edge, recursive });
        self.ops.add_operation(Box::leak(next_to_point));
        self
    }
    /// Puts the VectorObjectBuilder next to the given VectorObjectBuilder at the given direction, with a buff distance between them and aligning at the given edge.
    #[wasm_bindgen(return_description = "The vector object being built with the next to other operation.")]
    pub fn next_to_other(
        mut self,
        #[wasm_bindgen(param_description = "The other vector object to translate the vector object to.")]
        other: VectorObjectBuilder,
        #[wasm_bindgen(param_description = "The direction to translate the vector object in. If not given, the vector object will be at the right of the other vector object, default is right.")]
        direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The buffer to leave between the vector object and the other vector object. If not given, the vector object will be touching the other vector object, default is 0.")]
        buff: Option<f32>,
        #[wasm_bindgen(param_description = "The edge of the vector object to align with the other vector object. If not given, the vector object will be aligned at the middle, default is the middle.")]
        aligned_edge: Option<Point2D>,
        #[wasm_bindgen(param_description = "Whether to apply the next to other operation to the children of the vector object. If not given, the operation will be applied to the children, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let next_to_other = Box::new(NextToOther { other, direction, buff, aligned_edge, recursive });
        self.ops.add_operation(Box::leak(next_to_other));
        self
    }
    /// Arranges the children of the VectorObjectBuilder in the given direction, with a buff distance between them and aligning at the given edge.
    #[wasm_bindgen(return_description = "The vector object being built with the arrange subobjects operation.")]
    pub fn arrange_children(
        mut self,
        #[wasm_bindgen(param_description = "The direction to arrange the children. If not given, the children will be arranged horizontally in the positive x direction.")]
        direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The buffer to leave between the children. If not given, the children will be touching, default is 0.")]
        buff: Option<f32>,
        #[wasm_bindgen(param_description = "The edge of the children to align with the point. If not given, the children will be aligned at the middle.")]
        aligned_edge: Option<Point2D>,
        #[wasm_bindgen(param_description = "The center of the children. If not given, the children won't be centered at any point.")]
        center: Option<Point2D>,
        #[wasm_bindgen(param_description = "Whether to apply the arrange subobjects operation to the children of the vector object, default is true.")]
        recursive: Option<bool>
    ) -> VectorObjectBuilder {
        let arrange_subobjects = Box::new(ArrangeChildren { direction, buff, aligned_edge, center, recursive });
        self.ops.add_operation(Box::leak(arrange_subobjects));
        self
    }
    /// Reverses the path of the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the reverse path operation.")]
    pub fn reverse_path(mut self) -> VectorObjectBuilder {
        let reverse_path = Box::new(ReversePath {});
        self.ops.add_operation(Box::leak(reverse_path));
        self
    }
    /// Linearly interpolates the fill Style of the VectorObjectBuilder with another VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the interpolate fill operation.")]
    pub fn lerp_fill(
        mut self,
        #[wasm_bindgen(param_description = "The vector object to interpolate the fill style with.")]
        fill: Style,
        #[wasm_bindgen(param_description = "The factor to interpolate the fill style by.")]
        t: f32,
        #[wasm_bindgen(param_description = "The image's top left corner x-coordinate. It must be provided if the fill style contains an image or different kinds of gradients.")]
        x: Option<f32>,
        #[wasm_bindgen(param_description = "The image's top left corner y-coordinate. It must be provided if the fill style contains an image or different kinds of gradients.")]
        y: Option<f32>,
        #[wasm_bindgen(param_description = "The image's rendering width. It must be provided if the fill style contains an image or different kinds of gradients.")]
        width: Option<f32>,
        #[wasm_bindgen(param_description = "The image's rendering height. It must be provided if the fill style contains an image or different kinds of gradients.")]
        height: Option<f32>,
        #[wasm_bindgen(param_description = "The image's number of pixels in a row. It must be provided if the fill style contains an image or different kinds of gradients.")]
        data_width: Option<usize>,
        #[wasm_bindgen(param_description = "The image's number of pixels in a column. It must be provided if the fill style contains an image or different kinds of gradients.")]
        data_height: Option<usize>,
        #[wasm_bindgen(param_description = "Whether to apply the interpolate fill operation to the children of the vector object, default is true.")]
        recursive: Option<bool>,
    ) -> VectorObjectBuilder {
        let interpolate_fill = Box::new(LerpFill { fill, t, x, y, width, height, data_width, data_height, recursive });
        self.ops.add_operation(Box::leak(interpolate_fill));
        self
    }
    /// Linearly interpolates the stroke Style of the VectorObjectBuilder with another VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object being built with the interpolate stroke operation.")]
    pub fn lerp_stroke(
        mut self,
        #[wasm_bindgen(param_description = "The vector object to interpolate the stroke style with.")]
        stroke: Style,
        #[wasm_bindgen(param_description = "The factor to interpolate the stroke style by.")]
        t: f32,
        #[wasm_bindgen(param_description = "The image's top left corner x-coordinate. It must be provided if the stroke style contains an image or different kinds of gradients.")]
        x: Option<f32>,
        #[wasm_bindgen(param_description = "The image's top left corner y-coordinate. It must be provided if the stroke style contains an image or different kinds of gradients.")]
        y: Option<f32>,
        #[wasm_bindgen(param_description = "The image's rendering width. It must be provided if the stroke style contains an image or different kinds of gradients.")]
        width: Option<f32>,
        #[wasm_bindgen(param_description = "The image's rendering height. It must be provided if the stroke style contains an image or different kinds of gradients.")]
        height: Option<f32>,
        #[wasm_bindgen(param_description = "The image's number of pixels in a row. It must be provided if the stroke style contains an image or different kinds of gradients.")]
        data_width: Option<usize>,
        #[wasm_bindgen(param_description = "The image's number of pixels in a column. It must be provided if the stroke style contains an image or different kinds of gradients.")]
        data_height: Option<usize>,
        #[wasm_bindgen(param_description = "Whether to apply the interpolate stroke operation to the children of the vector object, default is true.")]
        recursive: Option<bool>,
    ) -> VectorObjectBuilder {
        let interpolate_stroke = Box::new(LerpStroke { stroke, t, x, y, width, height, data_width, data_height, recursive });
        self.ops.add_operation(Box::leak(interpolate_stroke));
        self
    }
    /// Builds the VectorObject by applying sequentially all the operations to the VectorObjectBuilder.
    #[wasm_bindgen(return_description = "The vector object built by applying the operations to it.")]
    pub fn build(self) -> VectorObject {
        let mut vector_object = Rc::clone(&self.object);
        let vector_object_as_mut = Rc::make_mut(&mut vector_object);
        self.ops.apply_and_return(vector_object_as_mut)
    }
}

impl VectorObjectBuilder {
    pub fn from_node(node: &usvg::Node) -> VectorObjectBuilder {
        let mut builder = VectorObjectBuilder::default();
        match node {
            usvg::Node::Path(ref path) => {
                builder = builder.add_child(VectorObjectBuilder::from_path(path));
            }
            usvg::Node::Group(ref group) => {
                builder = builder.add_child(VectorObjectBuilder::from_group(group));
            }
            usvg::Node::Text(ref text) => {
                builder = builder.add_child(VectorObjectBuilder::from_text(text));
            }
            usvg::Node::Image(ref image) => {
                builder = builder.add_child(VectorObjectBuilder::from_image(image));
            }
        }
        builder
    }
    pub fn from_path(internal_path: &usvg::Path) -> VectorObjectBuilder {
        let mut vector_object_builder = VectorObjectBuilder::default();
        let path = Path2D::from_svg_path_data(internal_path.data());
        vector_object_builder = vector_object_builder.set_path(path);
        let internal_fill_option = internal_path.fill();
        let fill;
        let fill_rule;
        if let Some(internal_fill) = internal_fill_option {
            fill = Style::from_paint_and_opacity(internal_fill.paint(), &internal_fill.opacity());
            fill_rule = match internal_fill.rule() {
                usvg::FillRule::NonZero => "nonzero",
                usvg::FillRule::EvenOdd => "evenodd",
            };
        } else {
            fill = Style::from_color(Color::new(0, 0, 0, 0.0));
            fill_rule = "nonzero";
        }
        vector_object_builder = vector_object_builder.set_fill(fill, Some(false)).set_fill_rule(fill_rule.to_string(), Some(false));
        let internal_stroke_option = internal_path.stroke();
        let stroke;
        let stroke_width;
        let stroke_line_cap;
        let stroke_line_join;
        let stroke_miter_limit;
        let stroke_dash_offset;
        let stroke_dash_array;
        if let Some(internal_stroke) = internal_stroke_option {
            stroke = Style::from_paint_and_opacity(internal_stroke.paint(), &internal_stroke.opacity());
            stroke_width = internal_stroke.width().get();
            stroke_line_cap = match internal_stroke.linecap() {
                usvg::LineCap::Butt => "butt",
                usvg::LineCap::Round => "round",
                usvg::LineCap::Square => "square",
            };
            stroke_line_join = match internal_stroke.linejoin() {
                usvg::LineJoin::Miter => "miter",
                usvg::LineJoin::Round => "round",
                usvg::LineJoin::Bevel => "bevel",
                usvg::LineJoin::MiterClip => "miter-clip",
            };
            stroke_miter_limit = internal_stroke.miterlimit().get();
            stroke_dash_offset = internal_stroke.dashoffset();
            stroke_dash_array = internal_stroke.dasharray().unwrap_or(&[]).to_vec();
        } else {
            stroke = Style::from_color(Color::new(0, 0, 0, 0.0));
            stroke_width = 1.0;
            stroke_line_cap = "butt";
            stroke_line_join = "miter";
            stroke_miter_limit = 4.0;
            stroke_dash_offset = 0.0;
            stroke_dash_array = Vec::new();
        }
        vector_object_builder = vector_object_builder.set_stroke(stroke, Some(false)).set_stroke_width(stroke_width, Some(false)).set_stroke_line_cap(stroke_line_cap.to_string(), Some(false)).set_stroke_line_join(stroke_line_join.to_string(), Some(false)).set_stroke_miter_limit(stroke_miter_limit, Some(false)).set_stroke_dash_offset(stroke_dash_offset, Some(false)).set_stroke_dash_array(stroke_dash_array, Some(false));
        let transform = TransformationMatrix::from_svg_transform(internal_path.abs_transform());
        vector_object_builder = vector_object_builder.set_transform(transform, Some(false));
        vector_object_builder
    }
    pub fn from_group(internal_group: &usvg::Group) -> VectorObjectBuilder {
        let mut vector_object_builder = VectorObjectBuilder::default();
        for node in internal_group.children() {
            vector_object_builder = vector_object_builder.add_child(VectorObjectBuilder::from_node(node));
        }
        let transform = TransformationMatrix::from_svg_transform(internal_group.abs_transform());
        vector_object_builder = vector_object_builder.set_transform(transform, Some(false));
        vector_object_builder
    }
    pub fn from_text(text: &usvg::Text) -> VectorObjectBuilder {
        VectorObjectBuilder::from_group(text.flattened())
            .apply_transform(TransformationMatrix::from_svg_transform(text.abs_transform()), None)
    }
    pub fn from_image(image: &usvg::Image) -> VectorObjectBuilder {
        let kind = image.kind();
        let dimensions = image.size();
        let data_width = dimensions.width().round() as usize;
        let data_height = dimensions.height().round() as usize;
        let data = match &kind {
            usvg::ImageKind::PNG(data) => {
                let image_data = data.to_vec();
                Some(image_data)
            }
            usvg::ImageKind::JPEG(data) => {
                let image_data = data.to_vec();
                Some(image_data)
            }
            usvg::ImageKind::WEBP(data) => {
                let image_data = data.to_vec();
                Some(image_data)
            }
            _ => None
        };
        if data.is_none() {
            log("Unsupported image format.");
            return VectorObjectBuilder::default();
        }
        let x = image.bounding_box().x();
        let y = image.bounding_box().y();
        let width = image.bounding_box().width();
        let height = image.bounding_box().height();
        let mut vector_object_builder = Rectangle::new(
            BoundingBox::new(x, y, width, height).unwrap(),
            None
        ).vector_object_builder().set_stroke_width(0.0, Some(false));
        let img = ImageBitmap::new(x, y, width, height, data_width, data_height, data.unwrap());
        if img.is_err() {
            log("Failed to create image bitmap.");
            return VectorObjectBuilder::default();
        }
        vector_object_builder = vector_object_builder.set_fill(
            Style::from_image(img.unwrap()),
            Some(false)
        );
        vector_object_builder = vector_object_builder.set_transform(
            TransformationMatrix::from_svg_transform(image.abs_transform()),
            Some(false)
        );
        vector_object_builder
    }
}

#[wasm_bindgen]
impl VectorObject {
    /// Creates a new VectorObject.
    #[wasm_bindgen(constructor, return_description = "A new vector object.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The Path2D of the vector object.")]
        path: Path2D,
        #[wasm_bindgen(param_description = "The fill Style of the vector object.")]
        fill: Style,
        #[wasm_bindgen(param_description = "The fill rule of the vector object.")]
        fill_rule: String,
        #[wasm_bindgen(param_description = "The stroke Style of the vector object.")]
        stroke: Style,
        #[wasm_bindgen(param_description = "The stroke width of the vector object.")]
        stroke_width: f32,
        #[wasm_bindgen(param_description = "The stroke line cap of the vector object.")]
        stroke_line_cap: String,
        #[wasm_bindgen(param_description = "The stroke line join of the vector object.")]
        stroke_line_join: String,
        #[wasm_bindgen(param_description = "The stroke miter limit of the vector object.")]
        stroke_miter_limit: f32,
        #[wasm_bindgen(param_description = "The stroke dash offset of the vector object.")]
        stroke_dash_offset: f32,
        #[wasm_bindgen(param_description = "The stroke dash array of the vector object.", unchecked_param_type = "number[]")]
        stroke_dash_array: Vec<f32>,
        #[wasm_bindgen(param_description = "The children of the vector object.")]
        children: Vec<VectorObject>,
        #[wasm_bindgen(param_description = "The name of the vector object.")]
        name: Option<String>,
        #[wasm_bindgen(param_description = "The TransformationMatrix of the vector object.")]
        transform: TransformationMatrix
    ) -> VectorObject {
        VectorObject {
            path,
            fill,
            fill_rule: Rc::new(fill_rule),
            stroke,
            stroke_width,
            stroke_line_cap: Rc::new(stroke_line_cap),
            stroke_line_join: Rc::new(stroke_line_join),
            stroke_miter_limit,
            stroke_dash_offset,
            stroke_dash_array: Rc::new(stroke_dash_array),
            children,
            name: name.map(Rc::new),
            transform,
        }
    }
    /// Clones the vector object.
    #[wasm_bindgen(js_name = clone, return_description = "The cloned vector object.")]
    pub fn clone_js(&self) -> VectorObject {
        self.clone()
    }
    /// Creates a new empty VectorObject.
    #[wasm_bindgen(return_description = "A new empty vector object.")]
    pub fn default_vector_object() -> VectorObject {
        VectorObject::default()
    }
    /// Gets the Path2D of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The path of the vector object.")]
    pub fn path(&self) -> Path2D {
        self.path.clone()
    }
    /// Gets the fill Style of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The fill style of the vector object.")]
    pub fn fill(&self) -> Style {
        self.fill.clone()
    }
    /// Gets the fill rule of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The fill rule of the vector object.")]
    pub fn fill_rule(&self) -> String {
        self.fill_rule.to_string()
    }
    /// Gets the stroke Style of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The stroke style of the vector object.")]
    pub fn stroke(&self) -> Style {
        self.stroke.clone()
    }
    /// Gets the stroke width of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The stroke width of the vector object.")]
    pub fn stroke_width(&self) -> f32 {
        self.stroke_width
    }
    /// Gets the stroke line cap of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The stroke line cap of the vector object.")]
    pub fn stroke_line_cap(&self) -> String {
        self.stroke_line_cap.to_string()
    }
    /// Gets the stroke line join of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The stroke line join of the vector object.")]
    pub fn stroke_line_join(&self) -> String {
        self.stroke_line_join.to_string()
    }
    /// Gets the stroke miter limit of VectorObject.
    #[wasm_bindgen(getter, return_description = "The stroke miter limit of the vector object.")]
    pub fn stroke_miter_limit(&self) -> f32 {
        self.stroke_miter_limit
    }
    /// Gets the stroke dash offset of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The stroke dash offset of the vector object.")]
    pub fn stroke_dash_offset(&self) -> f32 {
        self.stroke_dash_offset
    }
    /// Gets the stroke dash array of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The stroke dash array of the vector object.", unchecked_return_type = "number[]")]
    pub fn stroke_dash_array(&self) -> Vec<f32> {
        self.stroke_dash_array.to_vec()
    }
    /// Gets the children of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The children of the vector object.")]
    pub fn children(&self) -> Vec<VectorObject> {
        self.children.clone()
    }
    /// Gets the name of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The name of the vector object.")]
    pub fn name(&self) -> Option<String> {
        self.name.as_ref().map(|name| name.to_string())
    }
    /// Gets the transformation matrix of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The transformation matrix of the vector object.")]
    pub fn transform(&self) -> TransformationMatrix {
        self.transform.clone()
    }
    /// Gets the Path2D with the applied TransformationMatrix.
    #[wasm_bindgen(getter, return_description = "The path of the vector object with the applied transform.")]
    pub fn actual_path(&self) -> Path2D {
        self.path.transform(&self.transform)
    }
    /// Gets the BoundingBox of the VectorObject.
    #[wasm_bindgen(return_description = "The bounding box of the vector object.")]
    pub fn bounding_box(
        &self,
        #[wasm_bindgen(param_description = "Whether to include the children of the vector object.")]
        recursive: Option<bool>
    ) -> Option<BoundingBox> {
        let path = self.actual_path();
        let mut bbox = BoundingBox::from_path(&path);
        if recursive.unwrap_or(true) {
            for child in self.children.iter() {
                let child_bbox = child.bounding_box(Some(true));
                bbox = BoundingBox::union(bbox, child_bbox);
            }
        }
        bbox
    }
    /// Gets the center Point2D of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The center of the vector object.")]
    pub fn center(
        &self,
    ) -> Option<Point2D> {
        self.bounding_box(None).map(|bbox| bbox.center())
    }
    /// Gets the critical Point2D of the VectorObject.
    #[wasm_bindgen(return_description = "The critical point of the vector object.")]
    pub fn get_critical_point(
        &self,
        #[wasm_bindgen(param_description = "The x key of the critical point. If negative, the minimum x is returned. If positive, the maximum x is returned. If zero, the center x is returned.")]
        key_x: f32,
        #[wasm_bindgen(param_description = "The y key of the critical point. If negative, the minimum y is returned. If positive, the maximum y is returned. If zero, the center y is returned.")]
        key_y: f32,
        #[wasm_bindgen(param_description = "Whether to include the children of the vector object.")]
        recursive: Option<bool>
    ) -> Option<Point2D> {
        let bounding_box = self.bounding_box(recursive);
        if bounding_box.is_none() {
            return None;
        }
        let bounding_box = bounding_box.unwrap();
        let center = bounding_box.center();
        let min_x = bounding_box.min_x();
        let min_y = bounding_box.min_y();
        let width = bounding_box.width();
        let height = bounding_box.height();
        let x = if key_x < 0.0 {
            min_x
        } else if key_x > 0.0 {
            min_x + width
        } else {
            center.x
        };
        let y = if key_y < 0.0 {
            min_y
        } else if key_y > 0.0 {
            min_y + height
        } else {
            center.y
        };
        Some(Point2D { x, y })
    }
    /// Gets the children of the VectorObject recursively.
    #[wasm_bindgen(return_description = "The children of the vector object.")]
    pub fn get_children_recursive(
        &self,
        #[wasm_bindgen(param_description = "Whether to include the children of the children of the vector object, default is false.")]
        with_points: Option<bool>
    ) -> Vec<VectorObject> {
        let mut children = Vec::new();
        for child in self.children.iter() {
            if with_points.unwrap_or(false) {
                children.push(child.clone());
            }
            children.extend(child.get_children_recursive(with_points));
        }
        children
    }
    /// Gets the number of cubic bezier curves in the vector object.
    #[wasm_bindgen(getter, return_description = "The number of curves in the vector object.")]
    pub fn num_curves(&self) -> usize {
        self.path.len() / 4
    }
    /// Gets the number of points in the VectorObject's path.
    #[wasm_bindgen(getter, return_description = "The number of points in the vector object.")]
    pub fn num_points(&self) -> usize {
        self.path.len()
    }
    /// Gets the number of children in the VectorObject.
    #[wasm_bindgen(getter, return_description = "The number of children in the vector object.")]
    pub fn num_children(&self) -> usize {
        self.children.len()
    }
    /// Gets whether the VectorObject's path is closed.
    #[wasm_bindgen(getter, return_description = "Whether the vector object's path is closed.")]
    pub fn is_closed(&self) -> bool {
        self.path[0].equals(&self.path[self.path.len() - 1], None)
    }
    /// Gets the subpaths of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The subpaths of the vector object.")]
    pub fn subpaths(&self) -> Vec<Path2D> {
        let rng = (4..self.path.len()).step_by(4);
        let filtered = rng.filter(|&i| !self.path[i - 1].equals(&self.path[i], None)).collect::<Vec<usize>>();
        let split_indices = vec![0].into_iter().chain(filtered).chain(vec![self.path.len()]).collect::<Vec<usize>>();
        let subpaths = split_indices.iter().zip(split_indices[1..].iter()).filter(|(start, end)| *end - *start >= 4).map(|(start, end)| self.path.slice(*start, *end)).collect();
        subpaths
    }
    /// Gets the width of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The width of the vector object.")]
    pub fn width(&self) -> Option<f32> {
        self.bounding_box(None).map(|bbox| bbox.width())
    }
    /// Gets the height of the VectorObject.
    #[wasm_bindgen(getter, return_description = "The height of the vector object.")]
    pub fn height(&self) -> Option<f32> {
        self.bounding_box(None).map(|bbox| bbox.height())
    }
    /// Slices the VectorObject's children.
    #[wasm_bindgen(return_description = "The sliced children of the vector object.")]
    pub fn slice_children(
        &self,
        #[wasm_bindgen(param_description = "The start index of the children to slice.")]
        start: usize,
        #[wasm_bindgen(param_description = "The end index of the children to slice.")]
        end: usize
    ) -> Vec<VectorObject> {
        self.children[start..end].to_vec()
    }
    /// Gets the children of the VectorObject with the given names.
    #[wasm_bindgen(return_description = "The children with the given names.")]
    pub fn get_children_by_names(
        &self,
        #[wasm_bindgen(param_description = "The names of the children to get.", unchecked_param_type = "string[]")]
        names: Vec<String>
    ) -> Vec<VectorObject> {
        let mut children = Vec::new();
        for child in self.children.iter() {
            if let Some(name) = child.name() {
                if names.contains(&name) {
                    children.push(child.clone());
                }
            }
        }
        children
    }
}

impl VectorObject {
    pub fn apply_operation(&mut self, operation: &dyn VectorOperation) {
        operation.apply(self);
    }
}