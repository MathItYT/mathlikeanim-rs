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
        await scene.exec_js('async function run() { alert("This was executed from Python!"); } return run();')

    ui.button('Run JavaScript', on_click=run)


ui.run(
    title='JavaScript from Python'
)