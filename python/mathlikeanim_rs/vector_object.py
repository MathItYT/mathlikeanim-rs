from copy import deepcopy
import json
from typing import Callable

from .gradient_image_or_color import (
    GradientImageOrColor,
    Color,
    GradientStop,
    LinearGradient,
    RadialGradient,
    Image,
)
from .utils import consider_points_equals


class VectorObject:
    def __init__(
        self,
        scene,
        points: list[tuple[float, float]] = None,
        fill: GradientImageOrColor | None = None,
        fill_rule: str = 'nonzero',
        stroke: GradientImageOrColor | None = None,
        stroke_width: float = 0.0,
        line_cap: str = 'butt',
        line_join: str = 'miter',
        subobjects: list['VectorObject'] = None,
        index: int = 0,
    ) -> None:
        self.scene = scene
        self.points = points or []
        self.fill = fill or GradientImageOrColor.default()
        self.fill_rule = fill_rule
        self.stroke = stroke or GradientImageOrColor.default()
        self.stroke_width = stroke_width
        self.line_cap = line_cap
        self.line_join = line_join
        self.subobjects = subobjects or []
        self.index = index
    
    async def create(self, t: float) -> "VectorObject":
        data = json.dumps([self.to_dict(), t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'createVectorObject', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def draw_stroke_then_fill(self, t: float, default_stroke_width: float = 2.0) -> "VectorObject":
        data = json.dumps([self.to_dict(), t, default_stroke_width])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'drawStrokeThenFill', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def fade_in(self, scale_factor: float, shift: tuple[float, float], t: float) -> "VectorObject":
        shift = [shift[0], shift[1]]
        data = json.dumps([self.to_dict(), scale_factor, shift, t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'fadeIn', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def fade_out(self, scale_factor: float, shift: tuple[float, float], t: float) -> "VectorObject":
        shift = [shift[0], shift[1]]
        data = json.dumps([self.to_dict(), scale_factor, shift, t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'fadeOut', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def grow_arrow_with_final_tip(self, t: float) -> "VectorObject":
        data = json.dumps([self.to_dict(), t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'growArrowWithFinalTip', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def grow_arrow_with_initial_tip(self, t: float) -> "VectorObject":
        data = json.dumps([self.to_dict(), t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'growArrowWithInitialTip', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def grow_arrow_with_tips_at_both_ends(self, t: float) -> "VectorObject":
        data = json.dumps([self.to_dict(), t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'growArrowWithTipsAtBothEnds', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def grow_from_center(self, t: float) -> "VectorObject":
        data = json.dumps([self.to_dict(), t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'growFromCenter', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def morph_shape(self, target: "VectorObject", t: float) -> "VectorObject":
        data = json.dumps([self.to_dict(), target.to_dict(), t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'morphShape', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def rotate_animation(self, angle: float, t: float) -> "VectorObject":
        data = json.dumps([self.to_dict(), angle, t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'rotateAnimation', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def scale_in_place(self, scale_factor: float, t: float) -> "VectorObject":
        data = json.dumps([self.to_dict(), scale_factor, t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'scaleInPlace', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def set_fill_animation(self, color: Color, t: float) -> "VectorObject":
        data = json.dumps([self.to_dict(), color.to_dict(), t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'setFillAnimation', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def set_stroke_animation(self, color: Color, t: float) -> "VectorObject":
        data = json.dumps([self.to_dict(), color.to_dict(), t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'setStrokeAnimation', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def shift_animation(self, shift: tuple[float, float], t: float) -> "VectorObject":
        shift = [shift[0], shift[1]]
        data = json.dumps([self.to_dict(), shift, t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'shiftAnimation', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def show_temporarily(self, t: float) -> "VectorObject":
        data = json.dumps([self.to_dict(), t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'showTemporarily', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def spinning_grow(self, angle: float, t: float) -> "VectorObject":
        data = json.dumps([self.to_dict(), angle, t])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'spinningGrow', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    def get_index(self) -> int:
        return self.index
    
    def set_index(self, index: int) -> "VectorObject":
        return VectorObject(
            self.scene,
            points=self.points,
            fill=self.fill,
            fill_rule=self.fill_rule,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            line_cap=self.line_cap,
            line_join=self.line_join,
            subobjects=self.subobjects,
            index=index,
        )
    
    async def increment_index(self, increment: int, recursive: bool) -> "VectorObject":
        data = json.dumps([self.to_dict(), increment, recursive])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'incrementIndex', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    def get_points(self) -> list[tuple[float, float]]:
        return self.points
    
    def set_points(self, points: list[tuple[float, float]]) -> "VectorObject":
        return VectorObject(
            self.scene,
            points=points,
            fill=self.fill,
            fill_rule=self.fill_rule,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            line_cap=self.line_cap,
            line_join=self.line_join,
            subobjects=self.subobjects,
            index=self.index,
        )
    
    def set_subobjects(self, subobjects: list['VectorObject']) -> "VectorObject":
        return VectorObject(
            self.scene,
            points=self.points,
            fill=self.fill,
            fill_rule=self.fill_rule,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            line_cap=self.line_cap,
            line_join=self.line_join,
            subobjects=subobjects,
            index=self.index,
        )
    
    def get_subobjects(self) -> list['VectorObject']:
        return self.subobjects
    
    async def get_subobjects_recursively(self, with_points: bool) -> list['VectorObject']:
        data = json.dumps([self.to_dict(), with_points])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getSubobjectsRecursively', {data})")
        return [VectorObject.from_dict(self.scene, subobject) for subobject in result]
    
    def add(self, subobject: "VectorObject") -> "VectorObject":
        return VectorObject(
            self.scene,
            points=self.points,
            fill=self.fill,
            fill_rule=self.fill_rule,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            line_cap=self.line_cap,
            line_join=self.line_join,
            subobjects=self.subobjects + [subobject],
            index=self.index,
        )
    
    def remove(self, i: int) -> "VectorObject":
        subobjects = self.subobjects.copy()
        subobjects.pop(i)
        return VectorObject(
            self.scene,
            points=self.points,
            fill=self.fill,
            fill_rule=self.fill_rule,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            line_cap=self.line_cap,
            line_join=self.line_join,
            subobjects=subobjects,
            index=self.index,
        )
    
    def get_subobject(self, i: int) -> "VectorObject":
        return self.subobjects[i]
    
    def set_subobject(self, i: int, subobject: "VectorObject") -> "VectorObject":
        subobjects = self.subobjects.copy()
        subobjects[i] = subobject
        return VectorObject(
            self.scene,
            points=self.points,
            fill=self.fill,
            fill_rule=self.fill_rule,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            line_cap=self.line_cap,
            line_join=self.line_join,
            subobjects=subobjects,
            index=self.index,
        )
    
    def slice_subobjects(self, start: int, end: int) -> list["VectorObject"]:
        return self.subobjects[start:end]
    
    def set_slice_subobjects(self, start: int, end: int, subobjects: list["VectorObject"]) -> "VectorObject":
        subobjects = self.subobjects[:start] + subobjects + self.subobjects[end:]
        return VectorObject(
            self.scene,
            points=self.points,
            fill=self.fill,
            fill_rule=self.fill_rule,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            line_cap=self.line_cap,
            line_join=self.line_join,
            subobjects=subobjects,
            index=self.index,
        )
    
    async def get_subcurve(self, start: float, end: float) -> "VectorObject":
        data = json.dumps([self.to_dict(), start, end])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getSubcurve', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def get_partial_copy(self, start: float, end: float, recursive: bool) -> "VectorObject":
        data = json.dumps([self.to_dict(), start, end, recursive])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getPartialCopy', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def get_anchors_and_handles(self) -> tuple[list[tuple[float, float]], list[tuple[float, float]], list[tuple[float, float]], list[tuple[float, float]]]:
        data = json.dumps([self.to_dict()])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getAnchorsAndHandles', {data})")
        return (
            [(point[0], point[1]) for point in result[0]],
            [(point[0], point[1]) for point in result[1]],
            [(point[0], point[1]) for point in result[2]],
            [(point[0], point[1]) for point in result[3]],
        )
    
    async def scale_handle_to_anchor_distances(self, scale: float, recursive: bool) -> "VectorObject":
        data = json.dumps([self.to_dict(), scale, recursive])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'scaleHandleToAnchorDistances', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def set_anchors_and_handles(
        self,
        anchors_and_handles: tuple[list[tuple[float, float]], list[tuple[float, float]], list[tuple[float, float]], list[tuple[float, float]]]
    ) -> "VectorObject":
        anchors_and_handles = [
            [[point[0], point[1]] for point in anchors_and_handles[0]],
            [[point[0], point[1]] for point in anchors_and_handles[1]],
            [[point[0], point[1]] for point in anchors_and_handles[2]],
            [[point[0], point[1]] for point in anchors_and_handles[3]],
        ]
        data = json.dumps([self.to_dict(), anchors_and_handles])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'setAnchorsAndHandles', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def get_nth_curve_points(self, n: int) -> list[tuple[float, float]]:
        data = json.dumps([self.to_dict(), n])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getNthCurvePoints', {data})")
        return [(point[0], point[1]) for point in result]
    
    async def get_nth_curve_length_pieces(self, n: int, sample_points: int = 10) -> list[float]:
        data = json.dumps([self.to_dict(), n, sample_points])
        return await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getNthCurveLengthPieces', {data})")
    
    def get_num_curves(self) -> int:
        return len(self.subobjects) // 4
    
    def is_closed(self) -> bool:
        return consider_points_equals(self.points[0], self.points[-1])
    
    async def get_subpaths(self) -> list[list[tuple[float, float]]]:
        data = json.dumps([self.to_dict()])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getSubpaths', {data})")
        return [[(point[0], point[1]) for point in subpath] for subpath in result]
    
    async def apply_function(
        self,
        function: Callable[[float, float], tuple[float, float]],
        recursive: bool,
        about_point: tuple[float, float] | None = None,
        about_edge: tuple[float, float] | None = None,
    ) -> "VectorObject":
        id_ = self.scene.register_callback(function)
        data = json.dumps([self.to_dict(), id_, recursive, about_point, about_edge])
        result = await self.scene.exec_js(f"return await runMethod({self.scene.id}, 'applyFunction', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def get_pieces(self, n_pieces: int) -> "VectorObject":
        data = json.dumps([self.to_dict(), n_pieces])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getPieces', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def get_cubic_bezier_tuples(self) -> list[tuple[tuple[float, float], tuple[float, float], tuple[float, float], tuple[float, float]]]:
        data = json.dumps([self.to_dict()])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getCubicBezierTuples', {data})")
        return [
            (
                (point[0], point[1]),
                (point[2], point[3]),
                (point[4], point[5]),
                (point[6], point[7]),
            )
            for point in result
        ]
    
    async def scale(self, scale_factor: float, recursive: bool) -> "VectorObject":
        data = json.dumps([self.to_dict(), scale_factor, recursive])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'scale', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def stretch(self, stretch_factor: tuple[float, float], recursive: bool) -> "VectorObject":
        data = json.dumps([self.to_dict(), [stretch_factor[0], stretch_factor[1]], recursive])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'stretch', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def shift(self, shift: tuple[float, float], recursive: bool) -> "VectorObject":
        data = json.dumps([self.to_dict(), [shift[0], shift[1]], recursive])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'shift', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def merged_points(self) -> list[tuple[float, float]]:
        data = json.dumps([self.to_dict()])
        return await self.scene.exec_js(f"return runMethod({self.scene.id}, 'mergedPoints', {data})")
    
    async def get_bounding_box(self) -> tuple[tuple[float, float], tuple[float, float]]:
        data = json.dumps([self.to_dict()])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getBoundingBox', {data})")
        return ((result[0][0], result[0][1]), (result[1][0], result[1][1]))
    
    async def get_center(self) -> tuple[float, float]:
        data = json.dumps([self.to_dict()])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getCenter', {data})")
        return (result[0], result[1])
    
    async def get_center_of_mass(self) -> tuple[float, float]:
        data = json.dumps([self.to_dict()])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getCenterOfMass', {data})")
        return (result[0], result[1])
    
    async def get_height(self) -> float:
        data = json.dumps([self.to_dict()])
        return await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getHeight', {data})")
    
    async def get_width(self) -> float:
        data = json.dumps([self.to_dict()])
        return await self.scene.exec_js(f"return runMethod({self.scene.id}, 'getWidth', {data})")
    
    def get_fill_opacity(self) -> float:
        return (
            self.fill.gradient_image_or_color.a
            if isinstance(self.fill.gradient_image_or_color, Color)
            else self.fill.gradient_image_or_color.alpha
        )
    
    def set_fill_opacity(self, opacity: float, recursive: bool) -> "VectorObject":
        fill = self.fill.gradient_image_or_color
        if isinstance(fill, Color):
            new_fill = Color(fill.r, fill.g, fill.b, a=opacity)
        elif isinstance(fill, LinearGradient):
            new_stops = [
                GradientStop(stop.offset, Color(stop.color.r, stop.color.g, stop.color.b, stop.color.a))
                for stop in fill.stops
            ]
            new_fill = LinearGradient(fill.start, fill.end, new_stops, alpha=opacity)
        elif isinstance(fill, RadialGradient):
            new_stops = [
                GradientStop(stop.offset, Color(stop.color.r, stop.color.g, stop.color.b, stop.color.a))
                for stop in fill.stops
            ]
            new_fill = RadialGradient(fill.c, fill.r, fill.f, new_stops, alpha=opacity)
        else:
            new_fill = Image(fill.image_base64, fill.mime_type, fill.top_left, fill.bottom_right, alpha=opacity)
        subobjects = deepcopy(self.subobjects)
        if recursive:
            subobjects = [subobject.set_fill_opacity(opacity, recursive) for subobject in subobjects]
        return VectorObject(
            self.scene,
            points=self.points,
            fill=GradientImageOrColor(new_fill),
            fill_rule=self.fill_rule,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            line_cap=self.line_cap,
            line_join=self.line_join,
            subobjects=subobjects,
            index=self.index,
        )
    
    def set_stroke_opacity(self, opacity: float, recursive: bool) -> "VectorObject":
        stroke = self.stroke.gradient_image_or_color
        if isinstance(stroke, Color):
            new_stroke = Color(stroke.r, stroke.g, stroke.b, a=opacity)
        elif isinstance(stroke, LinearGradient):
            new_stops = [
                GradientStop(stop.offset, Color(stop.color.r, stop.color.g, stop.color.b, stop.color.a))
                for stop in stroke.stops
            ]
            new_stroke = LinearGradient(stroke.start, stroke.end, new_stops, alpha=opacity)
        elif isinstance(stroke, RadialGradient):
            new_stops = [
                GradientStop(stop.offset, Color(stop.color.r, stop.color.g, stop.color.b, stop.color.a))
                for stop in stroke.stops
            ]
            new_stroke = RadialGradient(stroke.c, stroke.r, stroke.f, new_stops, alpha=opacity)
        else:
            new_stroke = Image(stroke.image_base64, stroke.mime_type, stroke.top_left, stroke.bottom_right, alpha=opacity)
        subobjects = deepcopy(self.subobjects)
        if recursive:
            subobjects = [subobject.set_stroke_opacity(opacity, recursive) for subobject in subobjects]
        return VectorObject(
            self.scene,
            points=self.points,
            fill=self.fill,
            fill_rule=self.fill_rule,
            stroke=GradientImageOrColor(new_stroke),
            stroke_width=self.stroke_width,
            line_cap=self.line_cap,
            line_join=self.line_join,
            subobjects=subobjects,
            index=self.index,
        )

    async def move_to(self, point: tuple[float, float], recursive: bool) -> "VectorObject":
        data = json.dumps([self.to_dict(), [point[0], point[1]], recursive])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'moveTo', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def rotate(self, angle: float, recursive: bool) -> "VectorObject":
        data = json.dumps([self.to_dict(), angle, recursive])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'rotate', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def next_to_other(
        self,
        other: "VectorObject",
        direction: tuple[float, float],
        buff: float,
        aligned_edge: tuple[float, float],
        recursive: bool,
    ) -> "VectorObject":
        data = json.dumps([self.to_dict(), other.to_dict(), [direction[0], direction[1]], buff, [aligned_edge[0], aligned_edge[1]], recursive])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'nextToOther', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def next_to_point(
        self,
        point: tuple[float, float],
        direction: tuple[float, float],
        buff: float,
        aligned_edge: tuple[float, float],
        recursive: bool,
    ) -> "VectorObject":
        data = json.dumps([self.to_dict(), [point[0], point[1]], [direction[0], direction[1]], buff, [aligned_edge[0], aligned_edge[1]], recursive])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'nextToPoint', {data})")
        return VectorObject.from_dict(self.scene, result)
    
    async def arrange_subobjects(
        self,
        direction: tuple[float, float],
        buff: float,
        aligned_edge: tuple[float, float],
        recursive: bool
    ) -> "VectorObject":
        data = json.dumps([self.to_dict(), [direction[0], direction[1]], buff, [aligned_edge[0], aligned_edge[1]], recursive])
        result = await self.scene.exec_js(f"return runMethod({self.scene.id}, 'arrangeSubobjects', {data})")
        return VectorObject.from_dict(self.scene, result)

    def get_fill(self) -> GradientImageOrColor:
        return self.fill
    
    def set_fill(self, fill: GradientImageOrColor, recursive: bool) -> "VectorObject":
        subobjects = deepcopy(self.subobjects)
        if recursive:
            subobjects = [subobject.set_fill(fill, recursive) for subobject in subobjects]
        return VectorObject(
            self.scene,
            points=self.points,
            fill=fill,
            fill_rule=self.fill_rule,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            line_cap=self.line_cap,
            line_join=self.line_join,
            subobjects=subobjects,
            index=self.index,
        )
    
    def get_fill_rule(self) -> str:
        return self.fill_rule
    
    def set_fill_rule(self, fill_rule: str, recursive: bool) -> "VectorObject":
        subobjects = deepcopy(self.subobjects)
        if recursive:
            subobjects = [subobject.set_fill_rule(fill_rule, recursive) for subobject in subobjects]
        return VectorObject(
            self.scene,
            points=self.points,
            fill=self.fill,
            fill_rule=fill_rule,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            line_cap=self.line_cap,
            line_join=self.line_join,
            subobjects=subobjects,
            index=self.index,
        )
    
    def get_stroke(self) -> GradientImageOrColor:
        return self.stroke
    
    def set_stroke(self, stroke: GradientImageOrColor, recursive: bool) -> "VectorObject":
        subobjects = deepcopy(self.subobjects)
        if recursive:
            subobjects = [subobject.set_stroke(stroke, recursive) for subobject in subobjects]
        return VectorObject(
            self.scene,
            points=self.points,
            fill=self.fill,
            fill_rule=self.fill_rule,
            stroke=stroke,
            stroke_width=self.stroke_width,
            line_cap=self.line_cap,
            line_join=self.line_join,
            subobjects=subobjects,
            index=self.index,
        )
    
    def get_stroke_width(self) -> float:
        return self.stroke_width
    
    def set_stroke_width(self, stroke_width: float, recursive: bool) -> "VectorObject":
        subobjects = deepcopy(self.subobjects)
        if recursive:
            subobjects = [subobject.set_stroke_width(stroke_width, recursive) for subobject in subobjects]
        return VectorObject(
            self.scene,
            points=self.points,
            fill=self.fill,
            fill_rule=self.fill_rule,
            stroke=self.stroke,
            stroke_width=stroke_width,
            line_cap=self.line_cap,
            line_join=self.line_join,
            subobjects=subobjects,
            index=self.index,
        )
    
    def get_line_cap(self) -> str:
        return self.line_cap
    
    def set_line_cap(self, line_cap: str, recursive: bool) -> "VectorObject":
        subobjects = deepcopy(self.subobjects)
        if recursive:
            subobjects = [subobject.set_line_cap(line_cap, recursive) for subobject in subobjects]
        return VectorObject(
            self.scene,
            points=self.points,
            fill=self.fill,
            fill_rule=self.fill_rule,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            line_cap=line_cap,
            line_join=self.line_join,
            subobjects=subobjects,
            index=self.index,
        )
    
    def get_line_join(self) -> str:
        return self.line_join
    
    def set_line_join(self, line_join: str, recursive: bool) -> "VectorObject":
        subobjects = deepcopy(self.subobjects)
        if recursive:
            subobjects = [subobject.set_line_join(line_join, recursive) for subobject in subobjects]
        return VectorObject(
            self.scene,
            points=self.points,
            fill=self.fill,
            fill_rule=self.fill_rule,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            line_cap=self.line_cap,
            line_join=line_join,
            subobjects=subobjects,
            index=self.index,
        )

    @staticmethod
    def from_dict(scene, data):
        return VectorObject(
            scene,
            points=[(point[0], point[1]) for point in data['points']],
            fill=GradientImageOrColor.from_dict(data['fill']) if data['fill'] else None,
            fill_rule=data['fillRule'],
            stroke=GradientImageOrColor.from_dict(data['stroke']) if data['stroke'] else None,
            stroke_width=data['strokeWidth'],
            line_cap=data['lineCap'],
            line_join=data['lineJoin'],
            subobjects=[VectorObject.from_dict(scene, subobject) for subobject in data['subobjects']],
            index=data['index'],
        )
    
    def to_dict(self):
        return {
            'type': 'vectorObject',
            'points': self.points,
            'fill': self.fill.to_dict(),
            'fillRule': self.fill_rule,
            'stroke': self.stroke.to_dict(),
            'strokeWidth': self.stroke_width,
            'lineCap': self.line_cap,
            'lineJoin': self.line_join,
            'subobjects': [subobject.to_dict() for subobject in self.subobjects],
            'index': self.index,
        }