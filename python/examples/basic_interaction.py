from pathlib import Path

from nicegui import ui
from nicegui.events import ValueChangeEventArguments

from mathlikeanim_rs.scene import Scene
from mathlikeanim_rs.vector_object import VectorObject
from mathlikeanim_rs.gradient_image_or_color import GradientImageOrColor, Color
from mathlikeanim_rs.utils import hex_to_color


@ui.page('/')
async def index():
    ui.markdown('''
# MathLikeAnim-rs in Python
- This is a very basic example of MathLikeAnim-rs in Python.
- It's interactive, try to click the "Play" button and interact with the slider.
''')

    scene = Scene()

    async def run():
        button.disable()
        await scene.wait_until_ready()
        circle = await scene.new_circle(
            (960, 540),
            200
        )
        circle.fill = GradientImageOrColor(Color(1, 0, 0, 0.5))
        circle.stroke = GradientImageOrColor(Color(1, 0, 0, 1))
        circle.stroke_width = 8
        await scene.set_background(GradientImageOrColor(hex_to_color('#161616', 1)))
        await scene.add(circle)
        await scene.begin_recording(Path('media'))

        async def animation_func(objs: dict[int, VectorObject], t: float):
            objs[0] = await circle.draw_stroke_then_fill(t)
            return objs

        await scene.play(animation_func, [0], 60)
        await scene.render_frame()
    
    async def on_change(event: ValueChangeEventArguments):
        circle = await scene.new_circle(
            (960, 540),
            event.value
        )
        circle.fill = GradientImageOrColor(Color(1, 0, 0, 0.5))
        circle.stroke = GradientImageOrColor(Color(1, 0, 0, 1))
        circle.stroke_width = 8
        await scene.add(circle)
        await scene.render_frame()

    button = ui.button('Play', on_click=run)
    ui.button('Stop Recording', on_click=scene.stop_recording)
    ui.label('Radius')
    ui.slider(min=0, max=500, value=200, on_change=on_change)

ui.run(
    title="MathLikeAnim-rs in Python",
)