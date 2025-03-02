use std::rc::Rc;

use wasm_bindgen::prelude::*;

use crate::{objects::{geometry::tipable::Tipable, plotting::function_plotter::ParametricFunctionPlot, vector_object::VectorObjectBuilder}, utils::{interpolation::{inverse_lerp, lerp}, interval::ClosedInterval, point2d::Point2D, style::{Color, Style}}};

/// A Tick is a mark on an axis at a specific value.
#[wasm_bindgen]
#[derive(Clone)]
pub struct Tick {
    value: f32,
    label: Option<VectorObjectBuilder>,
    size: f32,
    style: Style,
    stroke_width: f32,
}

#[wasm_bindgen]
impl Tick {
    /// Creates a new Tick with the given value.
    #[wasm_bindgen(constructor, return_description = "A new tick.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The value of the tick.")]
        value: f32,
        #[wasm_bindgen(param_description = "The label of the tick.")]
        label: Option<VectorObjectBuilder>,
        #[wasm_bindgen(param_description = "The size of the tick.")]
        size: Option<f32>,
        #[wasm_bindgen(param_description = "The style of the tick.")]
        style: Option<Style>,
        #[wasm_bindgen(param_description = "The stroke width of the tick.")]
        stroke_width: Option<f32>,
    ) -> Tick {
        Tick { value, label, size: size.unwrap_or(10.0), stroke_width: stroke_width.unwrap_or(4.0), style: style.unwrap_or(Style::from_color(Color::new(0, 0, 0, 1.0))) }
    }

    /// Returns the value of the tick.
    #[wasm_bindgen(getter, return_description = "The value of the tick.")]
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Returns the label of the tick, if any.
    #[wasm_bindgen(getter, return_description = "The label of the tick.")]
    pub fn label(&self) -> Option<VectorObjectBuilder> {
        match &self.label {
            Some(label) => Some(label.clone()),
            None => None,
        }
    }

    /// Returns the size of the tick.
    #[wasm_bindgen(getter, return_description = "The size of the tick.")]
    pub fn size(&self) -> f32 {
        self.size
    }

    /// Returns the style of the tick.
    #[wasm_bindgen(getter, return_description = "The style of the tick.")]
    pub fn style(&self) -> Style {
        self.style.clone()
    }

    /// Returns the stroke width of the tick.
    #[wasm_bindgen(getter, return_description = "The stroke width of the tick.")]
    pub fn stroke_width(&self) -> f32 {
        self.stroke_width
    }
}

/// An Axis for plotting data.
#[wasm_bindgen]
#[derive(Clone)]
pub struct Axis {
    range: ClosedInterval,
    render_start: Point2D,
    render_end: Point2D,
    label: Option<VectorObjectBuilder>,
    ticks: Rc<Vec<Tick>>,
    style: Style,
    stroke_width: f32,
}

#[wasm_bindgen]
impl Axis {
    /// Creates a new Axis with the given range and label.
    #[wasm_bindgen(constructor, return_description = "A new axis.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The range of the axis.")]
        range: ClosedInterval,
        #[wasm_bindgen(param_description = "The minimum coordinates of the axis when rendered.")]
        render_start: Point2D,
        #[wasm_bindgen(param_description = "The maximum coordinates of the axis when rendered.")]
        render_end: Point2D,
        #[wasm_bindgen(param_description = "The label of the axis.")]
        label: Option<VectorObjectBuilder>,
        #[wasm_bindgen(param_description = "The ticks of the axis.")]
        ticks: Option<Vec<Tick>>,
        #[wasm_bindgen(param_description = "The style of the axis.")]
        style: Option<Style>,
        #[wasm_bindgen(param_description = "The stroke width of the axis.")]
        stroke_width: Option<f32>,
    ) -> Axis {
        let style = style.unwrap_or(Style::from_color(Color::new(0, 0, 0, 1.0)));
        let stroke_width = stroke_width.unwrap_or(5.0);
        Axis { range, label, ticks: Rc::new(ticks.unwrap_or_default()), render_start, render_end, style, stroke_width }
    }

    /// Returns the range of the axis.
    #[wasm_bindgen(getter, return_description = "The range of the axis.")]
    pub fn range(&self) -> ClosedInterval {
        self.range.clone()
    }

    /// Returns the minimum x-coordinate of the axis when rendered.
    #[wasm_bindgen(getter, return_description = "The minimum x-coordinate of the axis when rendered.")]
    pub fn render_start(&self) -> Point2D {
        self.render_start
    }

    /// Returns the maximum x-coordinate of the axis when rendered.
    #[wasm_bindgen(getter, return_description = "The maximum x-coordinate of the axis when rendered.")]
    pub fn render_end(&self) -> Point2D {
        self.render_end
    }

    /// Returns the label of the axis.
    #[wasm_bindgen(getter, return_description = "The label of the axis.")]
    pub fn label(&self) -> Option<VectorObjectBuilder> {
        self.label.clone()
    }

    /// Returns the ticks of the axis.
    #[wasm_bindgen(getter, return_description = "The ticks of the axis.")]
    pub fn ticks(&self) -> Vec<Tick> {
        self.ticks.to_vec()
    }

    /// Returns the style of the axis.
    #[wasm_bindgen(getter, return_description = "The style of the axis.")]
    pub fn style(&self) -> Style {
        self.style.clone()
    }

    /// Returns the number of ticks on the axis.
    #[wasm_bindgen(getter, return_description = "The number of ticks on the axis.")]
    pub fn num_ticks(&self) -> usize {
        self.ticks.len()
    }

    /// Returns the tick at the given number.
    #[wasm_bindgen(return_description = "The tick at the given number.")]
    pub fn tick(
        &self,
        #[wasm_bindgen(param_description = "The number coo")]
        num: f32
    ) -> Option<Tick> {
        self.ticks.iter().find(|tick| (tick.value - num).abs() < 0.0001).cloned()
    }

    /// Gets the coordinates of a point in the axis.
    #[wasm_bindgen(return_description = "The coordinates of the point in the axis.")]
    pub fn coord_to_point(&self, num: f32) -> Point2D {
        let t = inverse_lerp(self.range.start(), self.range.end(), num);
        Point2D::lerp(&self.render_start, &self.render_end, t)
    }

    /// Gets the value of a point in the axis.
    #[wasm_bindgen(return_description = "The value of the point in the axis.")]
    pub fn point_to_coord(&self, point: Point2D) -> f32 {
        let t = point.project_onto_line(&self.render_start, &self.render_end);
        lerp(self.range.start(), self.range.end(), t)
    }

    /// Gets the VectorObjectBuilder such that when built and rendered, the axis is drawn.
    #[wasm_bindgen(return_description = "A vector object builder with the axis.")]
    pub fn vector_object_builder(
        &self,
        #[wasm_bindgen(param_description = "The direction of the label. The default is (1, 0).")]
        label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the label. The default is 20.0.")]
        label_offset: Option<f32>,
        #[wasm_bindgen(param_description = "The direction of the tick label. The default is (0, 1).")]
        tick_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the tick label. The default is 10.0.")]
        tick_label_offset: Option<f32>,
        #[wasm_bindgen(param_description = "Whether to add the axis label. The default is true.")]
        add_label: Option<bool>,
    ) -> VectorObjectBuilder {
        let mut builder = VectorObjectBuilder::default()
            .move_point(&self.render_start)
            .line_to(&self.render_end)
            .set_stroke(self.style.clone(), None)
            .set_stroke_width(self.stroke_width, None);
        for tick in self.ticks.iter() {
            let point = self.coord_to_point(tick.value());
            let rotation = (self.render_end - self.render_start).direction();
            let tick_start = (point + Point2D::new(0.0, -tick.size() / 2.0)).rotate_around(point, rotation);
            let tick_end = (point + Point2D::new(0.0, tick.size() / 2.0)).rotate_around(point, rotation);
            let mut child_builder = VectorObjectBuilder::default()
                .move_point(&tick_start)
                .line_to(&tick_end)
                .set_stroke(tick.style.clone(), None)
                .set_stroke_width(tick.stroke_width, None);
            if let Some(label) = &tick.label {
                let the_child_builder = child_builder.clone();
                let label_direction = tick_label_direction.unwrap_or(Point2D::new(0.0, 1.0));
                let label_offset = tick_label_offset.unwrap_or(10.0);
                child_builder = child_builder.add_child(label.clone().next_to_other(the_child_builder, Some(label_direction), Some(label_offset), None, None));
            }
            builder = builder.add_child(child_builder.clone());
        }
        if add_label.unwrap_or(true) {
            if let Some(label) = &self.label {
                let the_builder = builder.clone();
                let label_direction = label_direction.unwrap_or(Point2D::new(1.0, 0.0));
                let label_offset = label_offset.unwrap_or(20.0);
                builder = builder.add_child(label.clone().next_to_other(the_builder, Some(label_direction), Some(label_offset), None, None));
            }
        }
        builder
    }

    /// Returns the VectorObjectBuilder with the axis and tip at the end.
    #[wasm_bindgen(return_description = "A VectorObjectBuilder with the axis and tip at the end.")]
    pub fn with_tip_at_the_end(
        &self,
        #[wasm_bindgen(param_description = "The shape of the tip. The shape must be pointing to the right and centered to (0, 0), this function will rotate and move it to the correct angle.")]
        tip_shape: VectorObjectBuilder,
        #[wasm_bindgen(param_description = "The direction of the label. The default is (1, 0).")]
        label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the label. The default is 20.0.")]
        label_offset: Option<f32>,
        #[wasm_bindgen(param_description = "The direction of the tick label. The default is (0, 1).")]
        tick_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the tick label. The default is 10.0.")]
        tick_label_offset: Option<f32>,
    ) -> VectorObjectBuilder {
        let mut builder = self.vector_object_builder(label_direction, label_offset, tick_label_direction, tick_label_offset, Some(false));
        builder = builder.add_child(self.tip_at_end(tip_shape));
        if let Some(label) = &self.label {
            let the_builder = builder.clone();
            let label_direction = label_direction.unwrap_or(Point2D::new(1.0, 0.0));
            let label_offset = label_offset.unwrap_or(20.0);
            builder = builder.add_child(label.clone().next_to_other(the_builder, Some(label_direction), Some(label_offset), None, None));
        }
        builder
    }

    /// Returns the VectorObjectBuilder with the axis and tips at both ends.
    #[wasm_bindgen(return_description = "A VectorObjectBuilder with the axis and tips at both ends.")]
    pub fn with_tips_at_both_ends(
        &self,
        #[wasm_bindgen(param_description = "The shape of the tip. The shape must be pointing to the right, this function will rotate and move it to the correct angle.")]
        tip_shape: VectorObjectBuilder,
        #[wasm_bindgen(param_description = "The direction of the label. The default is (1, 0).")]
        label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the label. The default is 20.0.")]
        label_offset: Option<f32>,
        #[wasm_bindgen(param_description = "The direction of the tick label. The default is (0, 1).")]
        tick_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the tick label. The default is 10.0.")]
        tick_label_offset: Option<f32>,
    ) -> VectorObjectBuilder {
        let mut builder = self.vector_object_builder(label_direction, label_offset, tick_label_direction, tick_label_offset, Some(false));
        builder = builder.add_child(self.tip_at_start(tip_shape.clone()));
        builder = builder.add_child(self.tip_at_end(tip_shape));
        if let Some(label) = &self.label {
            let the_builder = builder.clone();
            let label_direction = label_direction.unwrap_or(Point2D::new(1.0, 0.0));
            let label_offset = label_offset.unwrap_or(20.0);
            builder = builder.add_child(label.clone().next_to_other(the_builder, Some(label_direction), Some(label_offset), None, None));
        }
        builder
    }

    /// Clones the axis.
    #[wasm_bindgen(js_name = clone)]
    pub fn copy(&self) -> Axis {
        self.clone()
    }
}

impl Tipable for Axis {
    fn angle_at_end(&self) -> f32 {
        (self.render_end - self.render_start).direction()
    }

    fn angle_at_start(&self) -> f32 {
        (self.render_start - self.render_end).direction()
    }

    fn end(&self) -> Point2D {
        self.render_end
    }

    fn start(&self) -> Point2D {
        self.render_start
    }
}

/// A CartesianAxes represents the axes of a Cartesian plot.
#[wasm_bindgen]
#[derive(Clone)]
pub struct CartesianAxes {
    x_axis: Axis,
    y_axis: Axis,
    coord_to_point: &'static dyn Fn(Point2D) -> Point2D
}

#[wasm_bindgen]
impl CartesianAxes {
    /// Creates a new CartesianAxes with the given x and y axes.
    #[wasm_bindgen(constructor, return_description = "A new Cartesian axes.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The x-range of the Cartesian axes.")]
        x_range: ClosedInterval,
        #[wasm_bindgen(param_description = "The y-range of the Cartesian axes.")]
        y_range: ClosedInterval,
        #[wasm_bindgen(param_description = "The minimum coordinates of the Cartesian axes when rendered.")]
        render_start: Point2D,
        #[wasm_bindgen(param_description = "The maximum coordinates of the Cartesian axes when rendered.")]
        render_end: Point2D,
        #[wasm_bindgen(param_description = "The style of each axis.")]
        style: Option<Style>,
        #[wasm_bindgen(param_description = "The stroke width of each axis.")]
        stroke_width: Option<f32>,
        #[wasm_bindgen(param_description = "The x-axis label.")]
        x_label: Option<VectorObjectBuilder>,
        #[wasm_bindgen(param_description = "The y-axis label.")]
        y_label: Option<VectorObjectBuilder>,
        #[wasm_bindgen(param_description = "The x-axis ticks.")]
        x_ticks: Option<Vec<Tick>>,
        #[wasm_bindgen(param_description = "The y-axis ticks.")]
        y_ticks: Option<Vec<Tick>>,
    ) -> CartesianAxes {
        let x_ticks = match x_ticks {
            Some(ticks) => Rc::new(ticks),
            None => Rc::new(vec![]),
        };
        let y_ticks = match y_ticks {
            Some(ticks) => Rc::new(ticks),
            None => Rc::new(vec![]),
        };
        let mut x_axis = Box::new(Axis::new(x_range.clone(), render_start, render_end, x_label.clone(), Some(x_ticks.to_vec()), style.clone(), stroke_width));
        let mut y_axis = Box::new(Axis::new(y_range.clone(), render_start, render_end, y_label.clone(), Some(y_ticks.to_vec()), style.clone(), stroke_width));
        if y_range.start() > 0.0 {
            x_axis = Box::new(Axis::new(
                x_range.clone(),
                Point2D::new(render_start.x, x_axis.coord_to_point(y_range.start()).y),
                Point2D::new(render_end.x, x_axis.coord_to_point(y_range.start()).y),
                x_label.clone(),
                Some(x_ticks.to_vec()),
                style.clone(),
                stroke_width
            ));
        } else if y_range.end() < 0.0 {
            x_axis = Box::new(Axis::new(
                x_range.clone(),
                Point2D::new(render_start.x, x_axis.coord_to_point(y_range.end()).y),
                Point2D::new(render_end.x, x_axis.coord_to_point(y_range.end()).y),
                x_label.clone(),
                Some(x_ticks.to_vec()),
                style.clone(),
                stroke_width
            ));
        } else {
            x_axis = Box::new(Axis::new(
                x_range.clone(),
                Point2D::new(render_start.x, x_axis.coord_to_point(0.0).y),
                Point2D::new(render_end.x, x_axis.coord_to_point(0.0).y),
                x_label.clone(),
                Some(x_ticks.to_vec()),
                style.clone(),
                stroke_width
            ));
        }
        if x_range.start() > 0.0 {
            y_axis = Box::new(Axis::new(
                y_range.clone(),
                Point2D::new(y_axis.coord_to_point(x_range.start()).x, render_start.y),
                Point2D::new(y_axis.coord_to_point(x_range.start()).x, render_end.y),
                y_label.clone(),
                Some(y_ticks.to_vec()),
                style.clone(),
                stroke_width
            ));
        } else if x_range.end() < 0.0 {
            y_axis = Box::new(Axis::new(
                y_range.clone(),
                Point2D::new(y_axis.coord_to_point(x_range.end()).x, render_start.y),
                Point2D::new(y_axis.coord_to_point(x_range.end()).x, render_end.y),
                y_label.clone(),
                Some(y_ticks.to_vec()),
                style.clone(),
                stroke_width
            ));
        } else {
            y_axis = Box::new(Axis::new(
                y_range.clone(),
                Point2D::new(y_axis.coord_to_point(0.0).x, render_start.y),
                Point2D::new(y_axis.coord_to_point(0.0).x, render_end.y),
                y_label.clone(),
                Some(y_ticks.to_vec()),
                style.clone(),
                stroke_width
            ));
        }
        let the_x_axis = Box::leak(x_axis.clone());
        let the_y_axis = Box::leak(y_axis.clone());
        CartesianAxes {
            x_axis: the_x_axis.clone(),
            y_axis: the_y_axis.clone(),
            coord_to_point: Box::leak(Box::new(|point: Point2D| -> Point2D {
                Point2D::new(the_x_axis.coord_to_point(point.x).x, the_y_axis.coord_to_point(point.y).y)
            }))
        }
    }

    /// Returns the x-axis of the Cartesian axes.
    #[wasm_bindgen(getter, return_description = "The x-axis of the Cartesian axes.")]
    pub fn x_axis(&self) -> Axis {
        self.x_axis.clone()
    }

    /// Returns the y-axis of the Cartesian axes.
    #[wasm_bindgen(getter, return_description = "The y-axis of the Cartesian axes.")]
    pub fn y_axis(&self) -> Axis {
        self.y_axis.clone()
    }

    /// Gets the VectorObjectBuilder such that when built and rendered, the Cartesian axes are drawn.
    #[wasm_bindgen(return_description = "A vector object builder with the Cartesian axes.")]
    pub fn vector_object_builder(
        &self,
        #[wasm_bindgen(param_description = "The direction of the x-label. The default is (1, 0).")]
        x_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the x-label. The default is 20.0.")]
        x_label_offset: Option<f32>,
        #[wasm_bindgen(param_description = "The direction of the x-tick label. The default is (0, 1).")]
        x_tick_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the x-tick label. The default is 10.0.")]
        x_tick_label_offset: Option<f32>,
        #[wasm_bindgen(param_description = "The direction of the y-label. The default is (1, 0).")]
        y_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the y-label. The default is 20.0.")]
        y_label_offset: Option<f32>,
        #[wasm_bindgen(param_description = "The direction of the y-tick label. The default is (0, 1).")]
        y_tick_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the y-tick label. The default is 10.0.")]
        y_tick_label_offset: Option<f32>,
    ) -> VectorObjectBuilder {
        let mut builder = VectorObjectBuilder::default();
        builder = builder.add_child(self.x_axis.vector_object_builder(x_label_direction, x_label_offset, x_tick_label_direction, x_tick_label_offset, None));
        builder = builder.add_child(self.y_axis.vector_object_builder(y_label_direction, y_label_offset, y_tick_label_direction, y_tick_label_offset, None));
        builder
    }


    /// Returns the VectorObjectBuilder with the Cartesian axes and tip at the end of both axes.
    #[wasm_bindgen(return_description = "A VectorObjectBuilder with the Cartesian axes and tip at the end of both axes.")]
    pub fn with_tips_at_ends(
        &self,
        #[wasm_bindgen(param_description = "The shape of the tip. The shape must be pointing to the right, this function will rotate and move it to the correct angle.")]
        tip_shape: VectorObjectBuilder,
        #[wasm_bindgen(param_description = "The direction of the x-label. The default is (1, 0).")]
        x_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the x-label. The default is 20.0.")]
        x_label_offset: Option<f32>,
        #[wasm_bindgen(param_description = "The direction of the x-tick label. The default is (0, 1).")]
        x_tick_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the x-tick label. The default is 10.0.")]
        x_tick_label_offset: Option<f32>,
        #[wasm_bindgen(param_description = "The direction of the y-label. The default is (1, 0).")]
        y_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the y-label. The default is 20.0.")]
        y_label_offset: Option<f32>,
        #[wasm_bindgen(param_description = "The direction of the y-tick label. The default is (0, 1).")]
        y_tick_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the y-tick label. The default is 10.0.")]
        y_tick_label_offset: Option<f32>,
    ) -> VectorObjectBuilder {
        let mut builder = VectorObjectBuilder::default();
        let y_label_direction = y_label_direction.unwrap_or(Point2D::new(0.0, -1.0));
        let y_tick_label_direction = y_tick_label_direction.unwrap_or(Point2D::new(-1.0, 0.0));
        builder = builder.add_child(self.x_axis.with_tip_at_the_end(tip_shape.clone(), x_label_direction, x_label_offset, x_tick_label_direction, x_tick_label_offset));
        builder = builder.add_child(self.y_axis.with_tip_at_the_end(tip_shape, Some(y_label_direction), y_label_offset, Some(y_tick_label_direction), y_tick_label_offset));
        builder
    }

    /// Returns the VectorObjectBuilder with the Cartesian axes and tips at both ends of both axes.
    #[wasm_bindgen(return_description = "A VectorObjectBuilder with the Cartesian axes and tips at both ends of both axes.")]
    pub fn with_tips_at_both_ends(
        &self,
        #[wasm_bindgen(param_description = "The shape of the tip. The shape must be pointing to the right, this function will rotate and move it to the correct angle.")]
        tip_shape: VectorObjectBuilder,
        #[wasm_bindgen(param_description = "The direction of the x-label. The default is (1, 0).")]
        x_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the x-label. The default is 20.0.")]
        x_label_offset: Option<f32>,
        #[wasm_bindgen(param_description = "The direction of the x-tick label. The default is (0, 1).")]
        x_tick_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the x-tick label. The default is 10.0.")]
        x_tick_label_offset: Option<f32>,
        #[wasm_bindgen(param_description = "The direction of the y-label. The default is (1, 0).")]
        y_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the y-label. The default is 20.0.")]
        y_label_offset: Option<f32>,
        #[wasm_bindgen(param_description = "The direction of the y-tick label. The default is (0, 1).")]
        y_tick_label_direction: Option<Point2D>,
        #[wasm_bindgen(param_description = "The offset of the y-tick label. The default is 10.0.")]
        y_tick_label_offset: Option<f32>,
    ) -> VectorObjectBuilder {
        let mut builder = VectorObjectBuilder::default();
        builder = builder.add_child(self.x_axis.with_tips_at_both_ends(tip_shape.clone(), x_label_direction, x_label_offset, x_tick_label_direction, x_tick_label_offset));
        builder = builder.add_child(self.y_axis.with_tips_at_both_ends(tip_shape, y_label_direction, y_label_offset, y_tick_label_direction, y_tick_label_offset));
        builder
    }

    /// Returns the associated point of the given coordinates in the Cartesian axes.
    #[wasm_bindgen(return_description = "The associated point of the given coordinates in the Cartesian axes.")]
    pub fn coord_to_point(&self, p: Point2D) -> Point2D {
        Point2D::new(self.x_axis.coord_to_point(p.x).x, self.y_axis.coord_to_point(p.y).y)
    }

    /// Returns the associated coordinates of the given point in the Cartesian axes.
    #[wasm_bindgen(return_description = "The associated coordinates of the given point in the Cartesian axes.")]
    pub fn point_to_coord(&self, point: Point2D) -> Point2D {
        Point2D::new(self.x_axis.point_to_coord(Point2D::new(point.x, 0.0)), self.y_axis.point_to_coord(Point2D::new(0.0, point.y)))
    }

    /// Returns the ParametricFunctionPlot object with the given function, relative to the Cartesian axes' coordinates.
    #[wasm_bindgen(return_description = "The ParametricFunctionPlot object with the given function.")]
    pub fn plot_function(
        &self,
        #[wasm_bindgen(param_description = "The x function to plot.")]
        expression_x: String,
        #[wasm_bindgen(param_description = "The y function to plot.")]
        expression_y: String,
        #[wasm_bindgen(param_description = "The domain of the parametric function.")]
        domain: ClosedInterval,
        #[wasm_bindgen(param_description = "The x-range of the plot.")]
        x_range: ClosedInterval,
        #[wasm_bindgen(param_description = "The y-range of the plot.")]
        y_range: ClosedInterval,
        #[wasm_bindgen(param_description = "The discontinuities of the plot.", unchecked_param_type = "number[]")]
        discontinuities: Option<Vec<f32>>,
        #[wasm_bindgen(param_description = "The minimum depth of the plot.")]
        min_depth: Option<u32>,
        #[wasm_bindgen(param_description = "The maximum depth of the plot.")]
        max_depth: Option<u32>,
        #[wasm_bindgen(param_description = "The threshold of the plot.")]
        threshold: Option<f32>,
    ) -> Result<ParametricFunctionPlot, JsError> {
        let mut plot = ParametricFunctionPlot::new(
            expression_x,
            expression_y,
            domain,
            x_range,
            y_range,
            discontinuities,
            min_depth,
            max_depth,
            threshold,
        )?;
        plot.compose(Box::new(self.coord_to_point));
        Ok(plot)
    }

    /// Clones the Cartesian axes.
    #[wasm_bindgen(js_name = clone)]
    pub fn copy(&self) -> CartesianAxes {
        self.clone()
    }
}
