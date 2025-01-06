from copy import deepcopy
import json
from typing import Callable

from .gradient_image_or_color import (
    GradientImageOrColor,
    Color,
    LinearGradient,
    RadialGradient,
    Image,
    GradientStop,
)
from .vector_object import VectorObject


class Camera:
    def __init__(
        self,
        scene,
        position: tuple[float, float, float],
        rotation: tuple[float, float, float],
        focal_distance: float,
        zoom: float,
    ) -> None:
        self.scene = scene
        self.position = position
        self.rotation = rotation
        self.focal_distance = focal_distance
        self.zoom = zoom
    
    async def project_points(self, points: list[tuple[float, float, float]]) -> list[tuple[float, float]]:
        data = json.dumps([[[point[0], point[1], point[2]] for point in points], self.to_dict()])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'projectPoints', {data})", timeout=10)
        return [(point[0], point[1]) for point in result]
    
    @staticmethod
    def from_dict(data) -> 'Camera':
        return Camera(
            (data['position'][0], data['position'][1], data['position'][2]),
            (data['rotation'][0], data['rotation'][1], data['rotation'][2]),
            data['focalDistance'],
            data['zoom'],
        )
    
    def to_dict(self):
        return {
            'type': 'camera',
            'position': [self.position[0], self.position[1], self.position[2]],
            'rotation': [self.rotation[0], self.rotation[1], self.rotation[2]],
            'focalDistance': self.focal_distance,
            'zoom': self.zoom,
        }


class LightSource:
    def __init__(self, scene, position: tuple[float, float, float]) -> None:
        self.scene = scene
        self.position = position

    @staticmethod
    def from_dict(data) -> 'LightSource':
        return LightSource((data['position'][0], data['position'][1], data['position'][2]))
    
    def to_dict(self):
        return {
            'type': 'lightSource',
            'position': [self.position[0], self.position[1], self.position[2]],
        }


class ThreeDObject:
    def __init__(
        self,
        scene,
        points: list[tuple[float, float, float]] = None,
        subobjects: list['ThreeDObject'] = None,
        fill: GradientImageOrColor | None = None,
        stroke: GradientImageOrColor | None = None,
        stroke_width: float = 0.0,
        index: int = 0,
    ) -> None:
        points = points or []
        subobjects = subobjects or []
        fill = fill or GradientImageOrColor.default()
        stroke = stroke or GradientImageOrColor.default()
        self.scene = scene
        self.points = points
        self.subobjects = subobjects
        self.fill = fill
        self.stroke = stroke
        self.stroke_width = stroke_width
        self.index = index

    async def create_axes_3d(self, t: float, default_stroke_width: float = 0.5) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), t, default_stroke_width])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'createAxes3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def create(self, t: float) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), t])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'createThreeDObject', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def draw_stroke_then_fill(self, t: float, default_stroke_width: float = 0.5) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), t, default_stroke_width])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'drawStrokeThenFill3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def fade_in(self, scale_factor: float, shift: tuple[float, float, float], t: float) -> "ThreeDObject":
        shift = [shift[0], shift[1], shift[2]]
        data = json.dumps([self.to_dict(), scale_factor, shift, t])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'fadeIn3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def fade_out(self, scale_factor: float, shift: tuple[float, float, float], t: float) -> "ThreeDObject":
        shift = [shift[0], shift[1], shift[2]]
        data = json.dumps([self.to_dict(), scale_factor, shift, t])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'fadeOut3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def grow_from_center(self, t: float) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), t])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'growFromCenter3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def morph_shape(self, target: "ThreeDObject", t: float) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), target.to_dict(), t])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'morphShape3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def rotate_x_animation(self, angle: float, t: float) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), angle, t])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'rotateXAnimation3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def rotate_y_animation(self, angle: float, t: float) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), angle, t])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'rotateYAnimation3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def rotate_z_animation(self, angle: float, t: float) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), angle, t])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'rotateZAnimation3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def scale_in_place(self, scale_factor: float, t: float) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), scale_factor, t])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'scaleInPlace3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def set_fill_animation(self, color: GradientImageOrColor, t: float) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), color.to_dict(), t])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'setFillAnimation3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def set_stroke_animation(self, color: GradientImageOrColor, t: float) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), color.to_dict(), t])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'setStrokeAnimation3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def shift_animation(self, shift: tuple[float, float, float], t: float) -> "ThreeDObject":
        shift = [shift[0], shift[1], shift[2]]
        data = json.dumps([self.to_dict(), shift, t])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'shiftAnimation3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    def set_points(self, points: list[tuple[float, float, float]]) -> "ThreeDObject":
        return ThreeDObject(
            self.scene,
            points=points,
            subobjects=self.subobjects,
            fill=self.fill,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            index=self.index,
        )
    
    def set_subobjects(self, subobjects: list['ThreeDObject']) -> "ThreeDObject":
        return ThreeDObject(
            self.scene,
            points=self.points,
            subobjects=subobjects,
            fill=self.fill,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            index=self.index,
        )
    
    def get_points(self) -> list[tuple[float, float, float]]:
        return self.points
    
    def get_subobjects(self) -> list['ThreeDObject']:
        return self.subobjects
    
    def get_fill(self) -> GradientImageOrColor:
        return self.fill
    
    def get_stroke(self) -> GradientImageOrColor:
        return self.stroke
    
    def get_stroke_width(self) -> float:
        return self.stroke_width
    
    def get_index(self) -> int:
        return self.index
    
    def set_fill(self, fill: GradientImageOrColor, recursive: bool) -> "ThreeDObject":
        subobjects = deepcopy(self.subobjects)
        if recursive:
            for i, subobject in enumerate(subobjects):
                subobjects[i] = subobject.set_fill(fill, recursive)
        return ThreeDObject(
            self.scene,
            points=self.points,
            subobjects=subobjects,
            fill=fill,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            index=self.index,
        )
    
    def set_stroke(self, stroke: GradientImageOrColor, recursive: bool) -> "ThreeDObject":
        subobjects = deepcopy(self.subobjects)
        if recursive:
            for i, subobject in enumerate(subobjects):
                subobjects[i] = subobject.set_stroke(stroke, recursive)
        return ThreeDObject(
            self.scene,
            points=self.points,
            subobjects=subobjects,
            fill=self.fill,
            stroke=stroke,
            stroke_width=self.stroke_width,
            index=self.index,
        )
    
    def set_stroke_width(self, stroke_width: float, recursive: bool) -> "ThreeDObject":
        subobjects = deepcopy(self.subobjects)
        if recursive:
            for i, subobject in enumerate(subobjects):
                subobjects[i] = subobject.set_stroke_width(stroke_width, recursive)
        return ThreeDObject(
            self.scene,
            points=self.points,
            subobjects=self.subobjects,
            fill=self.fill,
            stroke=self.stroke,
            stroke_width=stroke_width,
            index=self.index,
        )
    
    def set_index(self, index: int) -> "ThreeDObject":
        return ThreeDObject(
            self.scene,
            points=self.points,
            subobjects=self.subobjects,
            fill=self.fill,
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            index=index,
        )
    
    async def get_anchors_and_handles(self) -> tuple[list[tuple[float, float, float]], list[tuple[float, float, float]], list[tuple[float, float, float]], list[tuple[float, float, float]]]:
        data = json.dumps([self.to_dict()])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'getAnchorsAndHandles3D', {data})")
        return (
            [(point[0], point[1], point[2]) for point in result[0]],
            [(point[0], point[1], point[2]) for point in result[1]],
            [(point[0], point[1], point[2]) for point in result[2]],
            [(point[0], point[1], point[2]) for point in result[3]],
        )
    
    async def set_anchors_and_handles(
        self,
        anchors_and_handles: tuple[list[tuple[float, float, float]], list[tuple[float, float, float]], list[tuple[float, float, float]], list[tuple[float, float, float]]],
    ) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), anchors_and_handles])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'setAnchorsAndHandles3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def scale_handle_to_anchor_distances(self, scale_factor: float, recursive: bool) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), scale_factor, recursive])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'scaleHandleToAnchorDistances3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def get_cubic_bezier_tuples(self) -> list[tuple[tuple[float, float, float], tuple[float, float, float], tuple[float, float, float], tuple[float, float, float]]]:
        data = json.dumps([self.to_dict()])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'getCubicBezierTuples3D', {data})")
        return [
            (
                (point[0], point[1], point[2]),
                (point[3], point[4], point[5]),
                (point[6], point[7], point[8]),
                (point[9], point[10], point[11])
            )
            for point in result
        ]
    
    async def get_partial_copy(self, start: float, end: float, recursive: bool) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), start, end, recursive])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'getPartialCopy3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    def set_fill_opacity(self, opacity: float, recursive: bool) -> "ThreeDObject":
        old_fill = self.fill.gradient_image_or_color
        if isinstance(old_fill, Color):
            new_fill = Color(old_fill.r, old_fill.g, old_fill.b, opacity)
        elif isinstance(old_fill, LinearGradient):
            stops = [
                GradientStop(stop.offset, Color(stop.color.r, stop.color.g, stop.color.b, stop.color.a))
                for stop in old_fill.stops
            ]
            new_fill = LinearGradient(old_fill.start, old_fill.end, stops, old_fill.alpha)
        elif isinstance(old_fill, RadialGradient):
            stops = [
                GradientStop(stop.offset, Color(stop.color.r, stop.color.g, stop.color.b, stop.color.a))
                for stop in old_fill.stops
            ]
            new_fill = RadialGradient(old_fill.center, old_fill.radius, stops, old_fill.alpha)
        elif isinstance(old_fill, Image):
            new_fill = Image(
                old_fill.image_base64,
                old_fill.mime_type,
                old_fill.top_left,
                old_fill.bottom_right,
                opacity,
            )
        subobjects = deepcopy(self.subobjects)
        if recursive:
            for i, subobject in enumerate(subobjects):
                subobjects[i] = subobject.set_fill_opacity(opacity, recursive)
        return ThreeDObject(
            self.scene,
            points=self.points,
            subobjects=subobjects,
            fill=GradientImageOrColor(new_fill),
            stroke=self.stroke,
            stroke_width=self.stroke_width,
            index=self.index,
        )
    
    def set_stroke_opacity(self, opacity: float, recursive: bool) -> "ThreeDObject":
        old_stroke = self.stroke.gradient_image_or_color
        if isinstance(old_stroke, Color):
            new_stroke = Color(old_stroke.r, old_stroke.g, old_stroke.b, opacity)
        elif isinstance(old_stroke, LinearGradient):
            stops = [
                GradientStop(stop.offset, Color(stop.color.r, stop.color.g, stop.color.b, stop.color.a))
                for stop in old_stroke.stops
            ]
            new_stroke = LinearGradient(old_stroke.start, old_stroke.end, stops, old_stroke.alpha)
        elif isinstance(old_stroke, RadialGradient):
            stops = [
                GradientStop(stop.offset, Color(stop.color.r, stop.color.g, stop.color.b, stop.color.a))
                for stop in old_stroke.stops
            ]
            new_stroke = RadialGradient(old_stroke.center, old_stroke.radius, stops, old_stroke.alpha)
        elif isinstance(old_stroke, Image):
            new_stroke = Image(
                old_stroke.image_base64,
                old_stroke.mime_type,
                old_stroke.top_left,
                old_stroke.bottom_right,
                opacity,
            )
        subobjects = deepcopy(self.subobjects)
        if recursive:
            for i, subobject in enumerate(subobjects):
                subobjects[i] = subobject.set_stroke_opacity(opacity, recursive)
        return ThreeDObject(
            self.scene,
            points=self.points,
            subobjects=subobjects,
            fill=self.fill,
            stroke=GradientImageOrColor(new_stroke),
            stroke_width=self.stroke_width,
            index=self.index,
        )
    
    def get_fill_opacity(self) -> float:
        return self.fill.gradient_image_or_color.a if isinstance(self.fill.gradient_image_or_color, Color) else self.fill.gradient_image_or_color.alpha
    
    def get_stroke_opacity(self) -> float:
        return self.stroke.gradient_image_or_color.a if isinstance(self.stroke.gradient_image_or_color, Color) else self.stroke.gradient_image_or_color.alpha
    
    async def get_critical_point(self, key: tuple[float, float, float]) -> tuple[float, float, float]:
        data = json.dumps([self.to_dict(), [key[0], key[1], key[2]]])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'getCriticalPoint3D', {data})")
        return (result[0], result[1], result[2])
    
    async def next_to_other(
        self,
        other: "ThreeDObject",
        direction: tuple[float, float, float],
        buff: float,
        aligned_edge: tuple[float, float, float],
        recursive: bool,
    ) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), other.to_dict(), [direction[0], direction[1], direction[2]], buff, [aligned_edge[0], aligned_edge[1], aligned_edge[2]], recursive])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'nextToOther3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def next_to_point(
        self,
        point: tuple[float, float, float],
        direction: tuple[float, float, float],
        buff: float,
        aligned_edge: tuple[float, float, float],
        recursive: bool,
    ) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), [point[0], point[1], point[2]], [direction[0], direction[1], direction[2]], buff, [aligned_edge[0], aligned_edge[1], aligned_edge[2]], recursive])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'nextToPoint3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def arrange_subobjects(
        self,
        direction: tuple[float, float, float],
        buff: float,
        aligned_edge: tuple[float, float, float],
        recursive: bool,
    ) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), [direction[0], direction[1], direction[2]], buff, [aligned_edge[0], aligned_edge[1], aligned_edge[2]], recursive])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'arrangeSubobjects3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def scale(self, scale_factor: float, recursive: bool) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), scale_factor, recursive])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'scale3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def strecth(self, scale_factor: tuple[float, float, float], recursive: bool) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), [scale_factor[0], scale_factor[1], scale_factor[2]], recursive])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'stretch3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def shift(self, shift: tuple[float, float, float], recursive: bool) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), [shift[0], shift[1], shift[2]], recursive])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'shift3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def rotate_x(self, angle: float, recursive: bool) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), angle, recursive])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'rotateX3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def rotate_y(self, angle: float, recursive: bool) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), angle, recursive])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'rotateY3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def rotate_z(self, angle: float, recursive: bool) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), angle, recursive])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'rotateZ3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def project_and_shade(
        self,
        camera: Camera,
        light_source: LightSource,
    ) -> VectorObject:
        data = json.dumps([self.to_dict(), camera.to_dict(), light_source.to_dict()])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'projectAndShade3D', {data})", timeout=10)
        return VectorObject.from_dict(self.scene, result)
    
    async def apply_function(
        self,
        func: Callable[[tuple[float, float, float]], tuple[float, float, float]],
        recursive: bool,
    ) -> "ThreeDObject":
        id_ = self.scene.register_callback(func)
        data = json.dumps([self.to_dict(), id_, recursive])
        result = await self.scene.client.run_javascript(f"return await runMethod({self.scene.id}, 'applyFunction3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def get_bounding_box(self) -> tuple[tuple[float, float, float], tuple[float, float, float]]:
        data = json.dumps([self.to_dict()])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'getBoundingBox3D', {data})")
        return (
            (result[0][0], result[0][1], result[0][2]),
            (result[1][0], result[1][1], result[1][2]),
        )
    
    async def get_center(self) -> tuple[float, float, float]:
        data = json.dumps([self.to_dict()])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'getCenter3D', {data})")
        return (result[0], result[1], result[2])
    
    async def merged_points(self) -> list[tuple[float, float, float]]:
        data = json.dumps([self.to_dict()])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'mergedPoints3D', {data})")
        return [(point[0], point[1], point[2]) for point in result]
    
    async def move_to(self, point: tuple[float, float, float], recursive: bool) -> "ThreeDObject":
        data = json.dumps([self.to_dict(), [point[0], point[1], point[2]], recursive])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'moveTo3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def get_subobjects_recursively(self) -> list["ThreeDObject"]:
        data = json.dumps([self.to_dict()])
        result = await self.scene.client.run_javascript(f"return runMethod({self.scene.id}, 'getSubobjectsRecursively3D', {data})")
        return [ThreeDObject.from_dict(self.scene, subobject) for subobject in result]

    @staticmethod
    def from_dict(scene, data):
        return ThreeDObject(
            scene,
            points=[(point[0], point[1], point[2]) for point in data['points']],
            subobjects=[ThreeDObject.from_dict(scene, subobject) for subobject in data['subobjects']],
            fill=GradientImageOrColor.from_dict(data['fill']) if data['fill'] else None,
            stroke=GradientImageOrColor.from_dict(data['stroke']) if data['stroke'] else None,
            stroke_width=data['strokeWidth'],
            index=data['index'],
        )
    
    def to_dict(self):
        return {
            'type': 'threeDObject',
            'points': self.points,
            'subobjects': [subobject.to_dict() for subobject in self.subobjects],
            'fill': self.fill.to_dict(),
            'stroke': self.stroke.to_dict(),
            'strokeWidth': self.stroke_width,
            'index': self.index,
        }