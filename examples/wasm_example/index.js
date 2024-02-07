import start, {randomDot} from './pkg/wasm_example.js';


async function run() {
    await start();
    const randomDotButton = document.getElementById('random-dot');
    randomDotButton.addEventListener('click', async () => {
        randomDot();
    });
}


run();
