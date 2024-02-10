import start, {changeNRects} from './pkg/wasm_example.js';


async function run() {
    await start();
    const nRectsInput = document.getElementById('n-rects');
    nRectsInput.addEventListener('input', () => {
        changeNRects(nRectsInput.value);
    });
}


run();
