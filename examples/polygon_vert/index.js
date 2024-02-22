import start, { nextSlide, previousSlide } from './pkg/discrete_math_1.js'


function svgToImage(outerHTML) {
    const blob = new Blob([outerHTML], {type: 'image/svg+xml'});
    const url = URL.createObjectURL(blob);
    const image = new Image();
    image.src = url;
    image.width = 1920;
    image.height = 1080;
    return image;
}


function drawFrame(ctx) {
    const svg = document.querySelector('svg');
    const image = svgToImage(svg.outerHTML);
    image.onload = () => {
        ctx.clearRect(0, 0, 1920, 1080);
        ctx.drawImage(image, 0, 0);
    }
}


async function run() {
    let stopped = false;
    let busy = false;
    let current_slide = 0;
    const canvas = document.createElement('canvas');
    canvas.width = 1920;
    canvas.height = 1080;
    const ctx = canvas.getContext('2d');
    const stream = canvas.captureStream(60);
    const container = document.getElementById('container');
    const video = document.createElement('video');
    const videoContainer = document.createElement('div');
    videoContainer.className = 'video-container';
    const mediaRecorder = new MediaRecorder(stream, {
        videoBitsPerSecond: 80000000
    });
    let chunks = [];
    mediaRecorder.ondataavailable = (event) => {
        chunks.push(event.data);
    };
    mediaRecorder.onstop = (event) => {
        const blob = new Blob(chunks, {type: 'video/mp4'});
        const url = URL.createObjectURL(blob);
        video.src = url;
        video.controls = true;
        container.appendChild(videoContainer);
        videoContainer.appendChild(video);
    }
    const leftButton = document.getElementById('left');
    const rightButton = document.getElementById('right');
    const stopRecordingButton = document.getElementById('stop-recording');
    leftButton.addEventListener('click', async () => {
        if (!busy && current_slide > 0) {
            busy = true;
            await previousSlide(current_slide);
            current_slide -= 1;
            busy = false;
        } else if (busy) {
            console.log('busy');
        } else {
            console.log('start');
        }
    });
    rightButton.addEventListener('click', async () => {
        if (!busy && current_slide < 2) {
            busy = true;
            await nextSlide(current_slide);
            current_slide += 1;
            busy = false;
        } else if (busy) {
            console.log('busy');
        } else {
            console.log('end');
        }
    });
    stopRecordingButton.addEventListener('click', () => {
        if (stopped) {
            return;
        }
        mediaRecorder.stop();
        clearInterval(id);
        stopped = true;
    });
    const id = setInterval(drawFrame, 1000 / 60, ctx);
    mediaRecorder.start();
    await start();
}


run();
