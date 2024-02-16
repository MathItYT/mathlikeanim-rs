import start, { nextSlide, previousSlide } from './pkg/mathlike_functions_example.js'


async function run() {
    let stopped = false;
    let busy = false;
    let current_slide = 0;
    const leftButton = document.getElementById('left');
    const rightButton = document.getElementById('right');
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
        if (!busy && current_slide < 13) {
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
    const container = document.getElementById('container');
    const videoContainer = document.createElement('div');
    videoContainer.className = 'video-container';
    const video = document.createElement('video');
    const resultParagraph = document.createElement('p');
    resultParagraph.innerText = 'Resultado';
    resultParagraph.classList.add('roboto-bold');
    resultParagraph.classList.add('medium');
    videoContainer.appendChild(resultParagraph);
    videoContainer.appendChild(video);
    video.style.width = '640px';
    video.style.height = '360px';
    const canvas = document.getElementById('canvas');
    const stopRecordingButton = document.getElementById('stop-recording');
    const videoStream = canvas.captureStream(60);
    let audioStream = null;
    try {
        audioStream = await navigator.mediaDevices.getUserMedia({audio: true});
    } catch (e) {
        console.log('No audio');
    }
    if (audioStream) {
        const audioTrack = audioStream.getAudioTracks()[0];
        videoStream.addTrack(audioTrack);
    }
    const mediaRecorder = new MediaRecorder(videoStream, {
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
    };
    stopRecordingButton.addEventListener('click', () => {
        if (stopped) {
            return;
        }
        mediaRecorder.stop();
        stopped = true;
    });
    mediaRecorder.start();
    await start();
}


run();
