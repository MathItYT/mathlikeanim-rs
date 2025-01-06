from asyncio import Event, iscoroutine
import json
from pathlib import Path
from typing import Callable, Coroutine, Any

from nicegui.element import Element
from nicegui.dependencies import libraries, Library
from nicegui.events import GenericEventArguments

import mathlikeanim_rs
from .gradient_image_or_color import GradientImageOrColor, Color
from .three_d_object import ThreeDObject, Camera, LightSource
from .vector_object import VectorObject


libraries['mathlikeanim-rs'] = Library(
    key='mathlikeanim-rs',
    name='mathlikeanim-rs',
    path=Path(mathlikeanim_rs.__file__).parent / 'node_modules' / 'mathlikeanim-rs' / 'index.js',
    expose=True,
)
libraries['mathlikeanim-rs/index_bg.wasm'] = Library(
    key='mathlikeanim-rs/index_bg.wasm',
    name='mathlikeanim-rs/index_bg.wasm',
    path=Path(mathlikeanim_rs.__file__).parent / 'node_modules' / 'mathlikeanim-rs' / 'index_bg.wasm',
    expose=True,
)


class Scene(
    Element,
    component='scene.js',
):
    def __init__(
        self,
        width: int = 1920,
        height: int = 1080,
        css_width: str = '50vw',
        css_height: str = 'auto',
        fps: int = 60,
        svg: bool = False,
    ) -> None:
        super().__init__()
        self._props['width'] = width
        self._props['height'] = height
        self._props['cssWidth'] = css_width
        self._props['cssHeight'] = css_height
        self._props['fps'] = fps
        self._props['svg'] = svg
        self._ready = Event()
        self._funcs = {}
        self.on('ready', self._on_ready)
        self.on('python-request', self._on_python_request, ['pythonFuncId', 'args'])
    
    async def _on_python_request(self, data: GenericEventArguments):
        data = data.args
        id_ = data['pythonFuncId']
        func = self._funcs[id_]
        result = func(*data['args'])
        if iscoroutine(result):
            result = await result
        data = json.dumps([result])
        self.client.run_javascript(f'return runMethod({self.id}, "emitPythonResponse", {data})')

    async def wait_until_ready(self):
        await self._ready.wait()
    
    def _on_ready(self):
        self._ready.set()

    def new_empty_object(self) -> VectorObject:
        return VectorObject(self)
    
    def new_empty_three_d_object(self) -> ThreeDObject:
        return ThreeDObject(self)
    
    async def add_final_tip_to_object(
        self,
        shape: VectorObject,
        tip_side_length: float = 20,
        tip_color: Color | None = None,
    ) -> VectorObject:
        tip_color = tip_color or Color(1, 1, 1, a=1)
        data = json.dumps([shape.to_dict(), tip_side_length, tip_color.to_dict()])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "addFinalTipToObject", {data})')
        return VectorObject.from_dict(self, result)

    async def add_initial_tip_to_object(
        self,
        shape: VectorObject,
        tip_side_length: float = 20,
        tip_color: Color | None = None,
    ) -> VectorObject:
        tip_color = tip_color or Color(1, 1, 1, a=1)
        data = json.dumps([shape.to_dict(), tip_side_length, tip_color.to_dict()])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "addInitialTipToObject", {data})')
        return VectorObject.from_dict(self, result)

    async def add_both_sides_tips_to_object(
        self,
        shape: VectorObject,
        tip_side_length: float = 20,
        tip_color: Color | None = None,
    ) -> VectorObject:
        tip_color = tip_color or Color(1, 1, 1, a=1)
        data = json.dumps([shape.to_dict(), tip_side_length, tip_color.to_dict()])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "addBothSidesTipsToObject", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_arc(
        self,
        center: tuple[float, float],
        radius: float,
        start_angle: float,
        end_angle: float,
        n_samples: int = 10,
    ) -> VectorObject:
        data = json.dumps([[center[0], center[1]], radius, start_angle, end_angle, n_samples])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newArc", {data})')
        return VectorObject.from_dict(self, result)

    async def new_circle(
        self,
        center: tuple[float, float],
        radius: float,
        num_points: int = 10,
    ) -> VectorObject:
        data = json.dumps([[center[0], center[1]], radius, num_points])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newCircle", {data})')
        return VectorObject.from_dict(self, result)

    async def new_elliptical_arc(
        self,
        center: tuple[float, float],
        radius_x: float,
        radius_y: float,
        start_angle: float,
        end_angle: float,
        n_samples: int = 10,
    ) -> VectorObject:
        data = json.dumps([[center[0], center[1]], radius_x, radius_y, start_angle, end_angle, n_samples])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newEllipticalArc", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_ellipse(
        self,
        center: tuple[float, float],
        radius_x: float,
        radius_y: float,
        num_points: int = 10,
    ) -> VectorObject:
        data = json.dumps([[center[0], center[1]], radius_x, radius_y, num_points])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newEllipse", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_annular_sector(
        self,
        center: tuple[float, float],
        inner_radius: float,
        outer_radius: float,
        start_angle: float,
        end_angle: float,
        n_samples: int = 10,
    ) -> VectorObject:
        data = json.dumps([[center[0], center[1]], inner_radius, outer_radius, start_angle, end_angle, n_samples])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newAnnularSector", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_dashed_object(
        self,
        vector_object: VectorObject,
        num_dashes: int = 15,
        dashed_ratio: float = 0.5,
        dash_offset: float = 0.0,
        equal_length: bool = True,
    ) -> VectorObject:
        data = json.dumps([vector_object.to_dict(), num_dashes, dashed_ratio, dash_offset, equal_length])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newDashedObject", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_line(
        self,
        start: tuple[float, float],
        end: tuple[float, float],
        color: Color | None = None,
        stroke_width: float = 4.0,
    ) -> VectorObject:
        color = color or Color(1, 1, 1, a=1)
        data = json.dumps([[start[0], start[1]], [end[0], end[1]], color.to_dict(), stroke_width])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newLine", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_polygon(
        self,
        points: list[tuple[float, float]],
    ) -> VectorObject:
        data = json.dumps([[[point[0], point[1]] for point in points]])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newPolygon", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_regular_polygon(
        self,
        center: tuple[float, float],
        side_length: float,
        num_sides: int,
    ) -> VectorObject:
        data = json.dumps([[center[0], center[1]], side_length, num_sides])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newRegularPolygon", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_square(
        self,
        center: tuple[float, float],
        side_length: float,
    ) -> VectorObject:
        data = json.dumps([[center[0], center[1]], side_length])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newSquare", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_rectangle(
        self,
        center: tuple[float, float],
        width: float,
        height: float,
    ) -> VectorObject:
        data = json.dumps([[center[0], center[1]], width, height])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newRectangle", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_equilateral_triangle(
        self,
        center: tuple[float, float],
        side_length: float,
    ) -> VectorObject:
        data = json.dumps([[center[0], center[1]], side_length])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newEquilateralTriangle", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_triangle(
        self,
        point1: tuple[float, float],
        point2: tuple[float, float],
        point3: tuple[float, float],
    ) -> VectorObject:
        data = json.dumps([[point1[0], point1[1]], [point2[0], point2[1]], [point3[0], point3[1]]])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newTriangle", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_right_triangle(
        self,
        point1: tuple[float, float],
        point2: tuple[float, float],
    ) -> VectorObject:
        data = json.dumps([[point1[0], point1[1]], [point2[0], point2[1]]])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newRightTriangle", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_axes(
        self,
        x_range: tuple[float, float, float],
        y_range: tuple[float, float, float],
        center: tuple[float, float],
        x_length: float = 1000.0,
        y_length: float = 1000.0,
        color: Color | None = None,
        stroke_width: float = 4.0,
        line_cap: str = 'butt',
        line_join: str = 'miter',
        index: int = 0,
        add_x_ticks: bool = True,
        add_y_ticks: bool = True,
        x_tick_size: float = 20.0,
        y_tick_size: float = 20.0,
        add_x_tip: bool = True,
        add_y_tip: bool = True,
    ) -> VectorObject:
        color = color or Color(1, 1, 1, a=1)
        x_min, x_max, x_step = x_range
        y_min, y_max, y_step = y_range
        data = json.dumps([
            x_min, x_max, x_step,
            y_min, y_max, y_step,
            [center[0], center[1]],
            x_length, y_length,
            color.to_dict(),
            stroke_width,
            line_cap, line_join,
            index,
            add_x_ticks, add_y_ticks,
            x_tick_size, y_tick_size,
            add_x_tip, add_y_tip,
        ])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newAxes", {data})')
        return VectorObject.from_dict(self, result)

    async def axes_coords_to_point(
        self,
        axes: VectorObject,
        x: float,
        y: float,
        x_min: float,
        x_max: float,
        y_min: float,
        y_max: float,
    ) -> tuple[float, float]:
        data = json.dumps([axes.to_dict(), x, y, x_min, x_max, y_min, y_max])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "axesCoordsToPoint", {data})')
        return tuple(result)
    
    async def point_to_axes_coords(
        self,
        axes: VectorObject,
        point: tuple[float, float],
        x_min: float,
        x_max: float,
        y_min: float,
        y_max: float,
    ) -> tuple[float, float]:
        data = json.dumps([axes.to_dict(), [point[0], point[1]], x_min, x_max, y_min, y_max])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "pointToAxesCoords", {data})')
        return tuple(result)
    
    async def new_parametric_plot_in_axes(
        self,
        f: Callable[[float], tuple[float, float]],
        t_range: tuple[float, float, float],
        axes: VectorObject,
        xMin: float,
        xMax: float,
        yMin: float,
        yMax: float,
    ) -> VectorObject:
        t_min, t_max, t_step = t_range
        id_ = self.register_callback(f)
        data = json.dumps([id_, t_min, t_max, t_step, axes.to_dict(), xMin, xMax, yMin, yMax])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "newParametricPlotInAxes", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_plot_in_axes(
        self,
        f: Callable[[float], float],
        x_min: float,
        x_max: float,
        y_min: float,
        y_max: float,
        x_range: tuple[float, float, float],
        axes: VectorObject,
    ) -> VectorObject:
        x_1, x_2, x_step = x_range
        id_ = self.register_callback(f)
        data = json.dumps([id_, x_min, x_max, y_min, y_max, x_1, x_2, x_step, axes.to_dict()])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "newPlotInAxes", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_contour_plot_in_axes(
        self,
        f: Callable[[float, float], float],
        x_min: float,
        x_max: float,
        y_min: float,
        y_max: float,
        x_range: tuple[float, float, float],
        y_range: tuple[float, float, float],
        axes: VectorObject,
        intervals: list[float],
    ) -> VectorObject:
        x_1, x_2, x_step = x_range
        y_1, y_2, y_step = y_range
        id_ = self.register_callback(f)
        data = json.dumps([id_, x_min, x_max, y_min, y_max, x_1, x_2, x_step, y_1, y_2, y_step, axes.to_dict(), intervals])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "newContourPlotInAxes", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_area_under_curve(
        self,
        axes: VectorObject,
        plot: VectorObject,
        x_min: float,
        x_max: float,
        y_min: float,
        y_max: float,
        x_range: tuple[float, float],
    ) -> VectorObject: 
        x_1, x_2 = x_range
        data = json.dumps([axes.to_dict(), plot.to_dict(), x_min, x_max, y_min, y_max, x_1, x_2])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newAreaUnderCurve", {data})')
        return VectorObject.from_dict(self, result)

    async def riemann_rectangles_for_plot(
        self,
        f: Callable[[float], float],
        x_min: float,
        x_max: float,
        y_min: float,
        y_max: float,
        direction: float,
        x_range: tuple[float, float],
        n_rects: int,
        axes: VectorObject,
    ) -> VectorObject:
        x_1, x_2 = x_range
        id_ = self.register_callback(f)
        data = json.dumps([id_, x_min, x_max, y_min, y_max, direction, x_1, x_2, n_rects, axes.to_dict()])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "riemannRectanglesForPlot", {data})')
        return VectorObject.from_dict(self, result)

    async def secant_line_for_plot(
        self,
        f: Callable[[float], float],
        x_range: tuple[float, float],
        length: float,
        axes: VectorObject,
        x_min: float,
        x_max: float,
        y_min: float,
        y_max: float,
    ) -> VectorObject:
        x_1, x_2 = x_range
        id_ = self.register_callback(f)
        data = json.dumps([id_, x_1, x_2, length, axes.to_dict(), x_min, x_max, y_min, y_max])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "secantLineForPlot", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_parametric_plot(
        self,
        f: Callable[[float], tuple[float, float]],
        t_range: tuple[float, float, float],
    ) -> VectorObject:
        t_min, t_max, t_step = t_range
        id_ = self.register_callback(f)
        data = json.dumps([id_, t_min, t_max, t_step])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "newParametricPlot", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_real_function(
        self,
        f: Callable[[float], float],
        x_range: tuple[float, float, float],
    ) -> VectorObject:
        x_min, x_max, x_step = x_range
        id_ = self.register_callback(f)
        data = json.dumps([id_, x_min, x_max, x_step])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "newRealFunction", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_contour_plot(
        self,
        f: Callable[[float, float], float],
        x_range: tuple[float, float, float],
        y_range: tuple[float, float, float],
        intervals: list[float],
    ) -> VectorObject:
        x_min, x_max, x_step = x_range
        y_min, y_max, y_step = y_range
        id_ = self.register_callback(f)
        data = json.dumps([id_, x_min, x_max, x_step, y_min, y_max, y_step, intervals])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "newContourPlot", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_number_line(
        self,
        x_range: tuple[float, float, float],
        center: tuple[float, float],
        color: Color | None = None,
        stroke_width: float = 4.0,
        line_cap: str = 'butt',
        line_join: str = 'miter',
        index: int = 0,
        length: float = 1000.0,
        add_tip: bool = True,
        add_ticks: bool = True,
        tick_size: float = 20.0,
        angle: float = 0.0,
    ) -> VectorObject:
        color = color or Color(1, 1, 1, a=1)
        x_min, x_max, x_step = x_range
        data = json.dumps([
            x_min, x_max, x_step,
            [center[0], center[1]],
            color.to_dict(),
            stroke_width,
            line_cap, line_join,
            index,
            length,
            add_tip,
            add_ticks,
            tick_size,
            angle,
        ])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newNumberLine", {data})')
        return VectorObject.from_dict(self, result)
    
    async def number_to_point_on_number_line(
        self,
        number_line: VectorObject,
        number: float,
        x_min: float,
        x_max: float,
    ) -> tuple[float, float]:
        data = json.dumps([number_line.to_dict(), number, x_min, x_max])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "numberToPointOnNumberLine", {data})')
        return tuple(result)
    
    async def point_on_number_line_to_number(
        self,
        number_line: VectorObject,
        point: tuple[float, float],
        x_min: float,
        x_max: float,
    ) -> float:
        data = json.dumps([number_line.to_dict(), [point[0], point[1]], x_min, x_max])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "pointOnNumberLineToNumber", {data})')
        return result
    
    async def get_numbers_tex(
        self,
        number_line: VectorObject,
        numbers: list[float],
        number_to_vector_object: Callable[[float], Coroutine[None, None, VectorObject]],
        x_min: float,
        x_max: float,
        height: float,
        direction: tuple[float, float] | None = None,
        buff: float = 20.0,
        index: int = 0,
    ) -> VectorObject:
        direction = direction or (0, 1)
        direction = [direction[0], direction[1]]
        async def number_to_vector_object_async(number):
            result = await number_to_vector_object(number)
            return result.to_dict()
        id_ = self.register_callback(number_to_vector_object_async)
        data = json.dumps([number_line.to_dict(), numbers, id_, x_min, x_max, height, direction, buff, index])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "getNumbersTex", {data})')
        return VectorObject.from_dict(self, result)

    async def new_mathjax(
        self,
        expression: str,
        font_base64: dict[str, str] | None = None,
    ) -> VectorObject:
        font_base64 = font_base64 or {}
        data = json.dumps([expression, font_base64])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "newMathjax", {data})')
        return VectorObject.from_dict(self, result)
    
    async def svg_to_vector_object(
        self,
        svg: str,
        font_base64: dict[str, str] | None = None,
    ) -> VectorObject:
        font_base64 = font_base64 or {}
        data = json.dumps([svg, font_base64])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "svgToVectorObject", {data})')
        return VectorObject.from_dict(self, result)
    
    async def text_to_vector_object(
        self,
        text: str,
        font_base64: dict[str, str] | None = None,
        font_weight: str = 'normal',
        font_style: str = 'normal',
        x: float = 0.0,
        y: float = 0.0,
        font_size: float = 20.0,
    ) -> VectorObject:
        font_base64 = font_base64 or {}
        data = json.dumps([text, font_base64, font_weight, font_style, x, y, font_size])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "textToVectorObject", {data})')
        return VectorObject.from_dict(self, result)
    
    async def new_sphere(
        self,
        center: tuple[float, float, float],
        radius: float,
        u_segments: int,
        v_segments: int,
        fill_colors: list[Color],
        stroke_colors: list[Color],
        stroke_width: float,
        index: int,
    ) -> ThreeDObject:
        data = json.dumps([
            [center[0], center[1], center[2]],
            radius,
            u_segments,
            v_segments,
            [color.to_dict() for color in fill_colors],
            [color.to_dict() for color in stroke_colors],
            stroke_width,
            index,
        ])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "newSphere", {data})', timeout=60)
        return ThreeDObject.from_dict(self, result)
    
    async def new_three_d_axes(
        self,
        x_range: tuple[float, float, float],
        y_range: tuple[float, float, float],
        z_range: tuple[float, float, float],
        center: tuple[float, float, float],
        x_length: float = 1000.0,
        y_length: float = 1000.0,
        z_length: float = 1000.0,
        color: Color | None = None,
        stroke_width: float = 4.0,
        add_x_ticks: bool = True,
        add_y_ticks: bool = True,
        add_z_ticks: bool = True,
        x_tick_size: float = 20.0,
        y_tick_size: float = 20.0,
        z_tick_size: float = 20.0,
        add_x_tip: bool = True,
        add_y_tip: bool = True,
        add_z_tip: bool = True,
        n_pieces: int = 20,
        index: int = 0,
    ) -> ThreeDObject:
        color = color or Color(1, 1, 1, a=1)
        x_min, x_max, x_step = x_range
        y_min, y_max, y_step = y_range
        z_min, z_max, z_step = z_range
        data = json.dumps([
            x_min, x_max, x_step,
            y_min, y_max, y_step,
            z_min, z_max, z_step,
            [center[0], center[1], center[2]],
            x_length, y_length, z_length,
            color.to_dict(),
            stroke_width,
            add_x_ticks, add_y_ticks, add_z_ticks,
            x_tick_size, y_tick_size, z_tick_size,
            add_x_tip, add_y_tip, add_z_tip,
            n_pieces,
            index,
        ])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "newThreeDAxes", {data})')
        return ThreeDObject.from_dict(self, result)
    
    async def coords_to_point_on_three_d_axes(
        self,
        axes: ThreeDObject,
        coords: tuple[float, float, float],
        x_min: float,
        x_max: float,
        y_min: float,
        y_max: float,
        z_min: float,
        z_max: float,
    ) -> tuple[float, float, float]:
        coords = [coords[0], coords[1], coords[2]]
        data = json.dumps([axes.to_dict(), coords, x_min, x_max, y_min, y_max, z_min, z_max])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "coordsToPointOnThreeDAxes", {data})')
        return tuple(result)
    
    async def point_on_three_d_axes_to_coords(
        self,
        axes: ThreeDObject,
        point: tuple[float, float, float],
        x_min: float,
        x_max: float,
        y_min: float,
        y_max: float,
        z_min: float,
        z_max: float,
    ) -> tuple[float, float, float]:
        point = [point[0], point[1], point[2]]
        data = json.dumps([axes.to_dict(), point, x_min, x_max, y_min, y_max, z_min, z_max])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "pointOnThreeDAxesToCoords", {data})')
        return tuple(result)
    
    async def new_parametric_plot_in_three_d_axes(
        self,
        axes: ThreeDObject,
        f: Callable[[float, float], tuple[float, float, float]],
        u_range: tuple[float, float, int],
        v_range: tuple[float, float, int],
        x_min: float,
        x_max: float,
        y_min: float,
        y_max: float,
        z_min: float,
        z_max: float,
        fills: list[Color],
        strokes: list[Color],
        stroke_width: float,
        index: int = 0,
    ) -> ThreeDObject:
        u_min, u_max, u_steps = u_range
        v_min, v_max, v_steps = v_range
        id_ = self.register_callback(f)
        data = json.dumps([
            axes.to_dict(),
            id_,
            u_min, u_max,
            v_min, v_max,
            u_steps, v_steps,
            x_min, x_max,
            y_min, y_max,
            z_min, z_max,
            [color.to_dict() for color in fills],
            [color.to_dict() for color in strokes],
            stroke_width,
            index])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "newParametricPlotInThreeDAxes", {data})')
        return ThreeDObject.from_dict(self, result)
    
    async def new_plot_in_three_d_axes(
        self,
        axes: ThreeDObject,
        f: Callable[[float, float], float],
        u_range: tuple[float, float, int],
        v_range: tuple[float, float, int],
        x_min: float,
        x_max: float,
        y_min: float,
        y_max: float,
        z_min: float,
        z_max: float,
        fills: list[Color],
        strokes: list[Color],
        stroke_width: float,
        index: int = 0,
    ) -> ThreeDObject:
        u_min, u_max, u_steps = u_range
        v_min, v_max, v_steps = v_range
        id_ = self.register_callback(f)
        data = json.dumps([
            axes.to_dict(),
            id_,
            u_min, u_max,
            v_min, v_max,
            u_steps, v_steps,
            x_min, x_max,
            y_min, y_max,
            z_min, z_max,
            [color.to_dict() for color in fills],
            [color.to_dict() for color in strokes],
            stroke_width,
            index,
        ])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "newPlotInThreeDAxes", {data})')
        return ThreeDObject.from_dict(self, result)
    
    async def new_parametric_line_plot_in_three_d_axes(
        self,
        axes: ThreeDObject,
        f: Callable[[float], tuple[float, float, float]],
        t_range: tuple[float, float, int],     
        x_min: float,
        x_max: float,
        y_min: float,
        y_max: float,
        z_min: float,
        z_max: float,   
        color: Color,
        stroke_width: float,
        index: int = 0,
    ) -> ThreeDObject:
        t_min, t_max, t_steps = t_range
        id_ = self.register_callback(f)
        data = json.dumps([
            axes.to_dict(),
            id_,
            t_min, t_max, t_steps,
            x_min, x_max,
            y_min, y_max,
            z_min, z_max,
            color.to_dict(),
            stroke_width,
            index,
        ])
        result = await self.client.run_javascript(f'return await runMethod({self.id}, "newParametricLinePlotInThreeDAxes", {data})')
        return ThreeDObject.from_dict(self, result)

    async def from_uv_function(
        self,
        func: Callable[[tuple[float, float]], tuple[float, float, float]],
        u_range: tuple[float, float],
        v_range: tuple[float, float],
        u_num_steps: int,
        v_num_steps: int,
        fills: list[Color],
        strokes: list[Color],
        stroke_width: float,
        index: int = 0,
    ) -> "ThreeDObject":
        id_ = self.register_callback(func)
        data = json.dumps([id_, u_range, v_range, u_num_steps, v_num_steps, [fill.to_dict() for fill in fills], [stroke.to_dict() for stroke in strokes], stroke_width, index])
        result = await self.client.run_javascript(f"return await runMethod({self.scene.id}, 'fromUVFunction3D', {data})")
        return ThreeDObject.from_dict(self.scene, result)
    
    async def three_d_object_from_vector_object(
        self,
        vector_object: VectorObject,
    ) -> ThreeDObject:
        data = json.dumps([vector_object.to_dict()])
        result = await self.client.run_javascript(f"return await runMethod({self.id}, 'threeDObjectFromVectorObject', {data})")
        return ThreeDObject.from_dict(self, result)
    
    async def add(self, vector_object: VectorObject) -> None:
        data = json.dumps([vector_object.to_dict()])
        await self.client.run_javascript(f'return runMethod({self.id}, "addToScene", {data})')

    async def remove(self, index: int) -> None:
        await self.client.run_javascript(f'return runMethod({self.id}, "removeFromScene", {index})')

    async def clear(self) -> None:
        data = json.dumps([])
        await self.client.run_javascript(f'return runMethod({self.id}, "clearScene", {data})')
    
    async def restore(self, index: int) -> None:
        data = json.dumps([index])
        await self.client.run_javascript(f'return runMethod({self.id}, "restoreState", {data})')
    
    async def save_state(self, index: int) -> None:
        data = json.dumps([index])
        await self.client.run_javascript(f'return runMethod({self.id}, "saveState", {data})')

    async def set_top_left_corner(self, x: float, y: float) -> None:
        data = json.dumps([x, y])
        await self.client.run_javascript(f'return runMethod({self.id}, "setTopLeftCorner", {data})')

    async def set_bottom_right_corner(self, x: float, y: float) -> None:
        data = json.dumps([x, y])
        await self.client.run_javascript(f'return runMethod({self.id}, "setBottomRightCorner", {data})')
    
    async def set_background(self, background: GradientImageOrColor) -> None:
        data = json.dumps([background.to_dict()])
        await self.client.run_javascript(f'return runMethod({self.id}, "setBackground", {data})')

    async def play(
        self,
        animation_func: Callable[[dict[int, VectorObject], float], Coroutine[None, None, dict[int, VectorObject]]],
        object_indices: list[int],
        duration_in_frames: int,
    ) -> None:
        async def animation_func_async(objects, time):
            result = await animation_func({int(index): VectorObject.from_dict(self, obj) for index, obj in objects.items()}, time)
            return {str(index): obj.to_dict() for index, obj in result.items()}
        id_ = self.register_callback(animation_func_async)
        data = json.dumps([id_, object_indices, duration_in_frames])
        await self.client.run_javascript(f'return await runMethod({self.id}, "play", {data})', timeout=2.5*duration_in_frames / self._props['fps'])
    
    async def set_updater(self, index: int, updater: Callable[[VectorObject], VectorObject]) -> None:
        id_ = self.register_callback(lambda obj: updater(VectorObject.from_dict(self, obj)).to_dict())
        data = json.dumps([index, id_])
        await self.client.run_javascript(f'return runMethod({self.id}, "setUpdater", {data})')
    
    async def remove_updater(self, index: int) -> None:
        data = json.dumps([index])
        await self.client.run_javascript(f'return runMethod({self.id}, "removeUpdater", {data})')
    
    async def wait(self, duration_in_frames: int, object_indices: list[int]) -> None:
        data = json.dumps([duration_in_frames, object_indices])
        await self.client.run_javascript(f'return await runMethod({self.id}, "wait", {data})', timeout=2.5*duration_in_frames / self._props['fps'])
    
    async def wait_until(
        self,
        condition: Callable[[], Coroutine[None, None, bool]],
        object_indices: list[int],
    ) -> None:
        id_ = self.register_callback(condition)
        data = json.dumps([id_, object_indices])
        await self.client.run_javascript(f'return await runMethod({self.id}, "waitUntil", {data})', timeout=10e9)
    
    async def set_on_rendered(self, func: Callable[[], Coroutine[None, None, None]]) -> None:
        id_ = self.register_callback(func)
        data = json.dumps([id_])
        await self.client.run_javascript(f'return runMethod({self.id}, "setOnRendered", {data})')
    
    async def render_frame(self) -> None:
        data = json.dumps([])
        await self.client.run_javascript(f'return await runMethod({self.id}, "renderFrame", {data})')
    
    async def get_objects(self) -> list[VectorObject]:
        data = json.dumps([])
        result = await self.client.run_javascript(f'return runMethod({self.id}, "getObjects", {data})')
        return [VectorObject.from_dict(self, obj) for obj in result]
    
    def new_camera(
        self,
        position: tuple[float, float, float],
        rotation: tuple[float, float, float],
        focal_distance: float,
        zoom: float,
    ) -> Camera:
        return Camera(self, position, rotation, focal_distance, zoom)
    
    def new_light_source(
        self,
        position: tuple[float, float, float],
    ) -> LightSource:
        return LightSource(self, position)

    async def exec_js(self, js: str) -> Any:
        data = json.dumps([js])
        return await self.client.run_javascript(f"return await runMethod({self.id}, 'execJS', {data})")
    
    async def log(self, message: str) -> None:
        data = json.dumps([message])
        await self.client.run_javascript(f"return runMethod({self.id}, 'log', {data})")

    def register_callback(self, func: Callable):
        id_ = id(func)
        self._funcs[id_] = func
        return id_