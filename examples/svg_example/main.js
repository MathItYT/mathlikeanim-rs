import start, { draw, changeNRects } from './pkg/svg_example.js';


async function run() {
    await start();
    await draw();
    const slider = document.getElementById('slider');
    slider.oninput = () => {
        changeNRects(slider.value);
    }
}

run();
