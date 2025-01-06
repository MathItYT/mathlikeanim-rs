from nicegui import ui

from mathlikeanim_rs.scene import Scene


@ui.page('/')
async def index():
    ui.markdown('''
# Executing JavaScript from Python with MathLikeAnim-rs
- This example executes JavaScript code from Python using MathLikeAnim-rs.
''')

    scene = Scene()

    async def run():
        await scene.wait_until_ready()
        await scene.log('Hello from Python!')
        await scene.exec_js('''
alert('Hello from Python!');
const scene = scenes[0];
const circ = circle([960, 540], 400)
    .setStroke(WasmGradientImageOrColor.fromColor(hexToColor('#FF0000', 1)), false)
    .setStrokeWidth(8, false);
scene.add(circ.clone());
await scene.renderFrame();
''')

    ui.button('Run JavaScript', on_click=run)


ui.run(
    title='JavaScript from Python'
)