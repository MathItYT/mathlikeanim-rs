import numpy as np

from nicegui import ui
from nicegui.events import ValueChangeEventArguments

from manim import VMobject, CapStyleType, LineJointType, ThreeDCamera, ManimBanner, PI, Cylinder, config
from mathlikeanim_rs.scene import Scene
from mathlikeanim_rs.gradient_image_or_color import (
    GradientImageOrColor,
    Color,
    LinearGradient,
    GradientStop
)
from mathlikeanim_rs.vector_object import VectorObject
from mathlikeanim_rs.three_d_object import ThreeDObject, Camera, LightSource


@ui.page('/')
async def index():
    ui.markdown('''
# MathLikeAnim-rs and Manim
- This example shows how to integrate MathLikeAnim-rs with ManimCE.
''')
    scene = Scene()

    def manim_coords_to_mathlike_coords(x: float, y: float) -> tuple[float, float]:
        x = (x + config.frame_x_radius) / config.frame_width * 1920
        y = (config.frame_y_radius - y) / config.frame_height * 1080
        return x, y
    
    def manim_to_mathlike_coords_3d(x: float, y: float, z: float) -> tuple[float, float, float]:
        x = (x + config.frame_x_radius) / config.frame_width * 1920
        y = (config.frame_y_radius - y) / config.frame_height * 1080
        z = z / config.frame_height * 1080
        return x, y, z

    def rgbas_to_gradient_image_or_color(
        rgbas: np.ndarray,
        start_and_end_points: tuple[tuple[float, float], tuple[float, float]],
    ) -> GradientImageOrColor:
        if len(rgbas) == 1:
            fill_rgba = rgbas[0]
            return GradientImageOrColor(Color(fill_rgba[0], fill_rgba[1], fill_rgba[2], fill_rgba[3]))
        else:
            return GradientImageOrColor(
                LinearGradient(
                    start_and_end_points[0],
                    start_and_end_points[1],
                    [
                        GradientStop(i / (len(rgbas) - 1), Color(rgba[0], rgba[1], rgba[2], rgba[3]))
                        for i, rgba in enumerate(rgbas)
                    ],
                    1.0,
                )
            )
    
    async def get_start_and_end_points_for_gradient_fill(vmob: VMobject, camera: Camera | None = None) -> tuple[tuple[float, float], tuple[float, float]]:
        start, end = vmob.get_gradient_start_and_end_points()
        if vmob.shade_in_3d:
            start = manim_to_mathlike_coords_3d(*start)
            end = manim_to_mathlike_coords_3d(*end)
            start, end = await camera.project_points([start, end])
        else:
            start = manim_coords_to_mathlike_coords(*start[:2])
            end = manim_coords_to_mathlike_coords(*end[:2])
        return start, end
    
    def get_mathlike_line_cap(vmobject: VMobject) -> CapStyleType:
        cap_style = vmobject.cap_style
        if cap_style == CapStyleType.AUTO:
            return "butt"
        elif cap_style == CapStyleType.ROUND:
            return "round"
        elif cap_style == CapStyleType.BUTT:
            return "butt"
        elif cap_style == CapStyleType.SQUARE:
            return "square"
    
    def get_mathlike_line_join(vmobject: VMobject) -> LineJointType:
        joint_type = vmobject.joint_type
        if joint_type == LineJointType.AUTO:
            return "miter"
        elif joint_type == LineJointType.ROUND:
            return "round"
        elif joint_type == LineJointType.BEVEL:
            return "bevel"
        elif joint_type == LineJointType.MITER:
            return "miter"

    async def vmobject_to_mathlike_object(
        vmobject: VMobject,
        camera: Camera | None = None,
        index: int = 0
    ) -> VectorObject | ThreeDObject:
        if vmobject.shade_in_3d and not camera:
            raise ValueError("3D shading requires a camera")
        start_and_end_points = await get_start_and_end_points_for_gradient_fill(vmobject, camera)
        if vmobject.shade_in_3d:
            return ThreeDObject(
                scene,
                [tuple(manim_to_mathlike_coords_3d(*point)) for point in vmobject.points],
                [await vmobject_to_mathlike_object(submob, camera, index) for submob in vmobject.submobjects],
                rgbas_to_gradient_image_or_color(vmobject.fill_rgbas, start_and_end_points),
                rgbas_to_gradient_image_or_color(vmobject.stroke_rgbas, start_and_end_points),
                vmobject.stroke_width,
                index,
            )
        else:
            return VectorObject(
                scene,
                points=[manim_coords_to_mathlike_coords(*point[:2]) for point in vmobject.points],
                fill=rgbas_to_gradient_image_or_color(vmobject.fill_rgbas, start_and_end_points),
                stroke=rgbas_to_gradient_image_or_color(vmobject.stroke_rgbas, start_and_end_points),
                stroke_width=vmobject.stroke_width,
                line_cap=get_mathlike_line_cap(vmobject),
                line_join=get_mathlike_line_join(vmobject),
                subobjects=[await vmobject_to_mathlike_object(submob, camera, index) for submob in vmobject.submobjects],
                index=index,
            )
    
    async def on_click():
        manim_banner = ManimBanner(dark_theme=False)
        m_obj = await vmobject_to_mathlike_object(manim_banner)
        await scene.add(m_obj)
        await scene.render_frame()

    ui.button('Play', on_click=on_click)


ui.run(
    title="MathLikeAnim-rs and Manim",
)
