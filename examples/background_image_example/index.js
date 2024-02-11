import start from './pkg/background_image_example.js';


async function run() {
    const video = document.createElement('video');
    const canvas = document.getElementById('canvas');
    const videoStream = canvas.captureStream(60);
    const mediaRecorder = new MediaRecorder(videoStream, {
        videoBitsPerSecond: 8000000, // Best bitrate for 1080p
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
        document.body.appendChild(video);
    };
    mediaRecorder.start();
    setTimeout(() => {
        mediaRecorder.stop();
    }, 2000);
    await start();
}


run();
