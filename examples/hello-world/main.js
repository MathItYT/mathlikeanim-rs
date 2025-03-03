import init, { Color, Point2D, Style, VectorObjectBuilder, Tick, Typst, FontFace, ClosedInterval, CartesianAxes } from '@mathlikeanim-rs/mathlikeanim-rs'
import { CanvasScene } from '@mathlikeanim-rs/renderer';

const scene = new CanvasScene(1920, 1080);
const renderButton = document.getElementById('render-button');
const stopRecordingButton = document.getElementById('stop-recording-button');
let firstTime = true;
let recorder;
let chunks = [];
let stream;
let data;
const stopRecording = () => {
    recorder.stop();
    stopRecordingButton.removeEventListener('click', stopRecording);
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
        data = new Uint8Array(await fetch('/latinmodern-math.otf').then(response => response.arrayBuffer()));
    }
    const axes = new CartesianAxes(
        new ClosedInterval(0, 10),
        new ClosedInterval(0, 10),
        new Point2D(960 - 400, 540 + 400),
        new Point2D(960 + 400, 540 - 400),
        Style.from_color(new Color(0, 0, 0, 1)),
        5,
        new Typst(`#show math.equation: set text(font: "Latin Modern Math")
  #set page(fill: none)
  $ x $`).vector_object_builder([new FontFace(data)]).scale(5, 5),
        new Typst(`#show math.equation: set text(font: "Latin Modern Math")
    #set page(fill: none)
    $ y $`).vector_object_builder([new FontFace(data)]).scale(5, 5),
        [new Tick(5, new Typst(`#show math.equation: set text(font: "Latin Modern Math")
    #set page(fill: none)
    $ 5 $`).vector_object_builder([new FontFace(data)]).scale(2.5, 2.5))]
    );
    const obj = axes.with_tips_at_ends(VectorObjectBuilder.default_tip_shape(50)).set_name('Axes').build();
    scene.objects.push(obj);
    const animations = new Map();
    const plot = axes.plot_function(
        't',
        't^2 / 10',
        new ClosedInterval(0, 10),
        new ClosedInterval(0, 10),
        new ClosedInterval(0, 10),
    );
    const obj2 = plot.vector_object_builder
        .set_stroke(Style.from_color(new Color(255, 0, 0, 1)))
        .set_stroke_width(5)
        .set_name('Plot')
        .build();
    console.log(obj2.path);
    scene.objects.push(obj2);
    animations.set('Axes', (old, t) => {
        if (t == 1) {
            return old;
        }
        return new VectorObjectBuilder(old)
            .become_partial(0, t)
            .build();
    });
    animations.set('Plot', (old, t) => {
        if (t == 1) {
            return old;
        }
        return new VectorObjectBuilder(old)
            .become_partial(0, t)
            .build();
    });
    await scene.play(animations, 2000, t => t);
    console.log('done');
    scene.objects.forEach(obj => obj.free());
    scene.objects = [];
};

const onClick = async () => {
    renderButton.removeEventListener('click', onClick);
    await run();
    renderButton.addEventListener('click', onClick);
};
init().then(() => renderButton.addEventListener('click', onClick));