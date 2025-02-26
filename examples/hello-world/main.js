import init, { Color, Point2D, Style, VectorObjectBuilder, IntegerLerp, FontFace, Typst, Rectangle, BoundingBox, Circle, LinearGradient, ColorStop, VectorObject, RadialGradient, ImageBitmap } from '@mathlikeanim-rs/mathlikeanim-rs'
import { CanvasScene, SVGScene } from '@mathlikeanim-rs/renderer';

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

const run = async () => {
    if (firstTime) {
        firstTime = false;
        const canvas = scene.canvas;
        // stream = canvas.captureStream(60);
        // recorder = new MediaRecorder(stream, { bitsPerSecond: 5100000, mimeType: 'video/mp4' });
        // recorder.ondataavailable = e => {
        //     if (e.data.size) {
        //         chunks.push(e.data);
        //     }
        // };
        // recorder.onstop = () => {
        //     const blob = new Blob(chunks, { type: 'video/mp4' });
        //     chunks = [];
        //     const url = URL.createObjectURL(blob);
        //     const a = document.createElement('a');
        //     a.href = url;
        //     a.download = 'video.mp4';
        //     a.click();
        //     URL.revokeObjectURL(url);
        // }
        // stopRecordingButton.addEventListener('click', stopRecording);
        // recorder.start();
        canvas.style.width = '50vw';
        canvas.style.height = 'auto';
        document.body.appendChild(canvas);
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
            return new VectorObjectBuilder(old)
                .set_fill(Style.from_color(new Color(255, 255, 0, 0.5)))
                .set_stroke(Style.from_color(new Color(255, 255, 0, 1)))
                .set_name('Circle')
                .build();
        }
        const drawPathT = new IntegerLerp(0, 2, t);
        return new VectorObjectBuilder(old)
            .become_partial(0, drawPathT.index === 0 ? drawPathT.remainder : 1)
            .fade_fill(drawPathT.index === 0 ? 1 : 1 - drawPathT.remainder)
            .lerp_fill(Style.from_color(new Color(255, 255, 0, 0.5)), drawPathT.index === 0 ? 0 : drawPathT.remainder)
            .lerp_stroke(Style.from_color(new Color(255, 255, 0, 1)), drawPathT.index === 0 ? 0 : drawPathT.remainder)
            .set_name('Circle')
            .build();
    });
    await scene.play(animations, 2000, t => t);
    const animations2 = new Map();
    animations2.set('Circle', (old, t) => {
        return new VectorObjectBuilder(old)
            .lerp_fill(Style.from_linear_gradient(new LinearGradient(
                new Point2D(-1, -1),
                new Point2D(1, 1),
                [
                    new ColorStop(new Color(255, 0, 0, 0.5), 0),
                    new ColorStop(new Color(255, 255, 0, 0.5), 0.5),
                    new ColorStop(new Color(255, 0, 255, 0.5), 1)
                ]
            )), t)
            .lerp_stroke(Style.from_linear_gradient(new LinearGradient(
                new Point2D(-1, -1),
                new Point2D(1, 1),
                [
                    new ColorStop(new Color(255, 0, 0, 1), 0),
                    new ColorStop(new Color(255, 255, 0, 1), 0.5),
                    new ColorStop(new Color(255, 0, 255, 1), 1)
                ]
            )), t)
            .set_name('Circle')
            .build();
    });
    await scene.play(animations2, 2000, t => t);
    // const animations3 = new Map();
    // animations3.set('Circle', (old, t) => {
    //     console.log(t);
    //     return new VectorObjectBuilder(old)
    //         .lerp_fill(Style.from_radial_gradient(
    //             new RadialGradient(
    //                 new Point2D(0, 0),
    //                 new Point2D(0, 0),
    //                 1,
    //                 [
    //                     new ColorStop(new Color(255, 0, 0, 0.5), 0),
    //                     new ColorStop(new Color(255, 255, 0, 0.5), 1)
    //                 ]
    //             )), t, 100, 100)
    //         .lerp_stroke(
    //             Style.from_radial_gradient(
    //                 new RadialGradient(
    //                     new Point2D(960, 540),
    //                     new Point2D(960, 540),
    //                     500,
    //                     [
    //                         new ColorStop(new Color(255, 0, 0, 1), 0),
    //                         new ColorStop(new Color(255, 255, 0, 1), 1)
    //                     ]
    //                 )), t, 100, 100)
    //         .build();
    // });
    // await scene.play(animations3, 2000, t => t);
    let c = new VectorObjectBuilder(scene.objects[1])
        .set_fill(Style.from_image(ImageBitmap.fill_linear_gradient(
            -2,
            -2,
            4,
            4,
            300,
            300,
            new LinearGradient(
                new Point2D(-1, -1),
                new Point2D(1, 1),
                [
                    new ColorStop(new Color(255, 0, 0, 0.5), 0),
                    new ColorStop(new Color(255, 255, 0, 0.5), 0.5),
                    new ColorStop(new Color(255, 0, 255, 0.5), 1)
                ]
            )
        )))
        .set_stroke(Style.from_image(ImageBitmap.fill_linear_gradient(
            -2,
            -2,
            4,
            4,
            300,
            300,
            new LinearGradient(
                new Point2D(-1, -1),
                new Point2D(1, 1),
                [
                    new ColorStop(new Color(255, 0, 0, 1), 0),
                    new ColorStop(new Color(255, 255, 0, 1), 0.5),
                    new ColorStop(new Color(255, 0, 255, 1), 1)
                ]
            )
        )))
        .build();
    scene.objects[1] = c;
    scene.render();
};

const onClick = async () => {
    renderButton.removeEventListener('click', onClick);
    await run();
    renderButton.addEventListener('click', onClick);
};
init().then(() => renderButton.addEventListener('click', onClick));