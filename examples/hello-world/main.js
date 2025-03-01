import init, { Color, Point2D, Style, VectorObjectBuilder, ParametricFunctionPlot, ClosedInterval } from '@mathlikeanim-rs/mathlikeanim-rs'
import { CanvasScene } from '@mathlikeanim-rs/renderer';

const scene = new CanvasScene(1920, 1080);
const renderButton = document.getElementById('render-button');
const stopRecordingButton = document.getElementById('stop-recording-button');
let firstTime = true;
let recorder;
let chunks = [];
let stream;
const stopRecording = () => {
    recorder.stop();
    stopRecordingButton.removeEventListener('click', stopRecording);
}

function text(text, x, y, font_size, font_family, fill_style, font_faces) {
    return VectorObjectBuilder.from_svg(`<svg xmlns="http://www.w3.org/2000/svg">
        <text x="${x}" y="${y}" font-size="${font_size}" font-family="${font_family}">${text}</text>
    </svg>`, font_faces)
        .set_fill(fill_style)
}

const run = async () => {
    if (firstTime) {
        firstTime = false;
        const canvasContainer = document.getElementById('canvas-container');
        const canvas = scene.canvas;
        const svg = scene.svg;
        stream = canvas.captureStream(60);
        recorder = new MediaRecorder(stream, { mimeType: 'video/mp4', videoBitsPerSecond: 1920*1080*60*8 });
        recorder.ondataavailable = e => {
            if (e.data.size && e.data.size > 0) {
                chunks.push(e.data);
            }
        };
        recorder.onstop = () => {
            const blob = new Blob(chunks, { type: 'video/mp4' });
            chunks = [];
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = 'video.mp4';
            a.click();
            URL.revokeObjectURL(url);
        }
        stopRecordingButton.addEventListener('click', stopRecording);
        recorder.start();
        canvas.style.width = '50vw';
        canvas.style.height = 'auto';
        svg.style.width = '50vw';
        svg.style.height = 'auto';
        canvasContainer.appendChild(canvas);
        canvasContainer.appendChild(svg);
    }
    const obj = new ParametricFunctionPlot(
        't',
        'sin(t)',
        new ClosedInterval(-10, 10),
        new ClosedInterval(-10, 10),
        new ClosedInterval(-1, 1),
    ).vector_object_builder
        .set_stroke(Style.from_color(new Color(255, 0, 0, 1)))
        .set_stroke_width(1)
        .set_name('ParametricFunctionPlot')
        .scale_to_width(1920)
        .move_to(new Point2D(960, 540))
        .build();
    scene.objects.push(obj);
    const animations = new Map();
    animations.set('ParametricFunctionPlot', (old, t) => {
        if (t == 1) {
            return old;
        }
        return new VectorObjectBuilder(old)
            .become_partial(0, t)
            .build();
    });
    await scene.play(animations, 2000, t => t);
    scene.objects.forEach(obj => obj.free());
    scene.objects = [];
};

const onClick = async () => {
    renderButton.removeEventListener('click', onClick);
    await run();
    renderButton.addEventListener('click', onClick);
};
init().then(() => renderButton.addEventListener('click', onClick));