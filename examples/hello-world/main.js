import init, { Color, Point2D, Style, VectorObjectBuilder, IntegerLerp, FontFace, Typst, Rectangle, BoundingBox, Circle } from '@mathlikeanim-rs/mathlikeanim-rs'
import { CanvasScene, SVGScene } from '@mathlikeanim-rs/renderer';

const scene = new CanvasScene(1920, 1080, 60);
const renderButton = document.getElementById('render-button');
const stopRecordingButton = document.getElementById('stop-recording-button');
let firstTime = true;
let fontFace;
let recorder;
let chunks = [];
let stream;
const stopRecording = () => {
    recorder.stop();
    stopRecordingButton.removeEventListener('click', stopRecording);
}

const run = async () => {
    if (firstTime) {
        firstTime = false;
        const canvas = scene.canvas;
        stream = canvas.captureStream(60);
        recorder = new MediaRecorder(stream, { bitsPerSecond: 5100000, mimeType: 'video/mp4' });
        recorder.ondataavailable = e => {
            if (e.data.size) {
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
        document.body.appendChild(canvas);
        fontFace = new Uint8Array(await fetch('latinmodern-math.otf').then(response => response.blob()).then(blob => blob.arrayBuffer()));
    }
    scene.objects.forEach(obj => obj.free());
    scene.objects = [];
    const obj = new Circle(
        new Point2D(960, 540),
        500,
    )
        .vector_object_builder()
        .set_fill(Style.from_color(new Color(255, 0, 0, 0.5)))
        .set_stroke(Style.from_color(new Color(255, 0, 0, 1)))
        .set_stroke_width(0.05)
        .set_name('Circle')
        .build();
    scene.objects.push(new Rectangle(new BoundingBox(0, 0, 1920, 1080)).vector_object_builder.set_fill(Style.from_color(new Color(0, 0, 0, 1))).build());
    scene.objects.push(obj);
    const animations = new Map();
    animations.set('Circle', (old, t) => {
        if (t == 1) {
            return old;
        }
        const drawPathT = new IntegerLerp(0, 2, t);
        return new VectorObjectBuilder(old)
            .become_partial(0, drawPathT.index === 0 ? drawPathT.remainder : 1)
            .fade_fill(drawPathT.index === 0 ? 1 : 1 - drawPathT.remainder)
            .build();
    });
    await scene.play(animations, 2000, t => t);
};

const onClick = async () => {
    renderButton.removeEventListener('click', onClick);
    await run();
    renderButton.addEventListener('click', onClick);
};
init().then(() => renderButton.addEventListener('click', onClick));