import start, { draw } from './pkg/html_canvas_example.js';


async function run() {
    await start();
    const canvas = document.getElementById('canvas');
    const stopButton = document.getElementById('stop');
    const stream = canvas.captureStream(144);
    const recorder = new MediaRecorder(stream, { bitsPerSecond: 25000000000});
    const chunks = [];
    recorder.ondataavailable = (event) => {
        chunks.push(event.data);
    };
    recorder.onstop = () => {
        const blob = new Blob(chunks, { type: 'video/mp4' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'canvas.mp4';
        a.click();
    };
    stopButton.onclick = () => {
        recorder.stop();
    };
    recorder.start();
    await draw();
}

run();