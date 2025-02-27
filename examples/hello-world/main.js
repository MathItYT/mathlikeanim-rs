import init, { Color, Point2D, Style, VectorObjectBuilder, ImageLibrary, IntegerLerp, FontFace, Typst, Rectangle, BoundingBox, Circle, LinearGradient, ColorStop, VectorObject, RadialGradient, ImageBitmap, ImageData } from '@mathlikeanim-rs/mathlikeanim-rs'
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
    scene.objects.forEach(obj => obj.free());
    scene.objects = [];
    // const obj = new Circle(
    //     new Point2D(960, 540),
    //     500,
    // )
    //     .vector_object_builder()
    //     .set_fill(Style.from_color(new Color(255, 0, 0, 0.5)))
    //     .set_stroke(Style.from_color(new Color(255, 0, 0, 1)))
    //     .set_stroke_width(0.05)
    //     .set_name('Circle')
    //     .build();
    // scene.objects.push(new Rectangle(new BoundingBox(0, 0, 1920, 1080)).vector_object_builder.set_fill(Style.from_color(new Color(0, 0, 0, 1))).build());
    // scene.objects.push(obj);
    // const animations = new Map();
    // animations.set('Circle', (old, t) => {
    //     if (t == 1) {
    //         return new VectorObjectBuilder(old)
    //             .set_fill(Style.from_color(new Color(255, 255, 0, 0.5)))
    //             .set_stroke(Style.from_color(new Color(255, 255, 0, 1)))
    //             .set_name('Circle')
    //             .build();
    //     }
    //     const drawPathT = new IntegerLerp(0, 2, t);
    //     return new VectorObjectBuilder(old)
    //         .become_partial(0, drawPathT.index === 0 ? drawPathT.remainder : 1)
    //         .fade_fill(drawPathT.index === 0 ? 1 : 1 - drawPathT.remainder)
    //         .lerp_fill(Style.from_color(new Color(255, 255, 0, 0.5)), drawPathT.index === 0 ? 0 : drawPathT.remainder)
    //         .lerp_stroke(Style.from_color(new Color(255, 255, 0, 1)), drawPathT.index === 0 ? 0 : drawPathT.remainder)
    //         .set_name('Circle')
    //         .build();
    // });
    // await scene.play(animations, 2000, t => t);
    // const animations2 = new Map();
    // animations2.set('Circle', (old, t) => {
    //     return new VectorObjectBuilder(old)
    //         .lerp_fill(Style.from_linear_gradient(new LinearGradient(
    //             new Point2D(-1, -1),
    //             new Point2D(1, 1),
    //             [
    //                 new ColorStop(new Color(255, 0, 0, 0.5), 0),
    //                 new ColorStop(new Color(255, 255, 0, 0.5), 0.5),
    //                 new ColorStop(new Color(255, 0, 255, 0.5), 1)
    //             ]
    //         )), t)
    //         .lerp_stroke(Style.from_linear_gradient(new LinearGradient(
    //             new Point2D(-1, -1),
    //             new Point2D(1, 1),
    //             [
    //                 new ColorStop(new Color(255, 0, 0, 1), 0),
    //                 new ColorStop(new Color(255, 255, 0, 1), 0.5),
    //                 new ColorStop(new Color(255, 0, 255, 1), 1)
    //             ]
    //         )), t)
    //         .set_name('Circle')
    //         .build();
    // });
    // await scene.play(animations2, 2000, t => t);
    // // const animations3 = new Map();
    // // animations3.set('Circle', (old, t) => {
    // //     console.log(t);
    // //     return new VectorObjectBuilder(old)
    // //         .lerp_fill(Style.from_radial_gradient(
    // //             new RadialGradient(
    // //                 new Point2D(0, 0),
    // //                 new Point2D(0, 0),
    // //                 1,
    // //                 [
    // //                     new ColorStop(new Color(255, 0, 0, 0.5), 0),
    // //                     new ColorStop(new Color(255, 255, 0, 0.5), 1)
    // //                 ]
    // //             )), t, 100, 100)
    // //         .lerp_stroke(
    // //             Style.from_radial_gradient(
    // //                 new RadialGradient(
    // //                     new Point2D(960, 540),
    // //                     new Point2D(960, 540),
    // //                     500,
    // //                     [
    // //                         new ColorStop(new Color(255, 0, 0, 1), 0),
    // //                         new ColorStop(new Color(255, 255, 0, 1), 1)
    // //                     ]
    // //                 )), t, 100, 100)
    // //         .build();
    // // });
    // // await scene.play(animations3, 2000, t => t);
    // let c = new VectorObjectBuilder(scene.objects[1])
    //     .set_fill(Style.from_image(ImageBitmap.fill_linear_gradient(
    //         -2,
    //         -2,
    //         4,
    //         4,
    //         300,
    //         300,
    //         new LinearGradient(
    //             new Point2D(-1, -1),
    //             new Point2D(1, 1),
    //             [
    //                 new ColorStop(new Color(255, 0, 0, 0.5), 0),
    //                 new ColorStop(new Color(255, 255, 0, 0.5), 0.5),
    //                 new ColorStop(new Color(255, 0, 255, 0.5), 1)
    //             ]
    //         )
    //     )))
    //     .set_stroke(Style.from_image(ImageBitmap.fill_linear_gradient(
    //         -2,
    //         -2,
    //         4,
    //         4,
    //         300,
    //         300,
    //         new LinearGradient(
    //             new Point2D(-1, -1),
    //             new Point2D(1, 1),
    //             [
    //                 new ColorStop(new Color(255, 0, 0, 1), 0),
    //                 new ColorStop(new Color(255, 255, 0, 1), 0.5),
    //                 new ColorStop(new Color(255, 0, 255, 1), 1)
    //             ]
    //         )
    //     )))
    //     .build();
    // scene.objects[1] = c;
    // scene.render();
    const imageLibrary = new ImageLibrary();
    const data = await fetch('mdn_logo_only_color.png').then(res => res.arrayBuffer()).then(buffer => new Uint8Array(buffer));
    imageLibrary.set('mdn_logo_only_color.png', new ImageData(data));
    const obj = text('Hello, World!', 960, 540, 100, 'Latin Modern Math', Style.from_color(new Color(0, 0, 0, 1)), [new FontFace(new Uint8Array(await fetch('latinmodern-math.otf').then(res => res.arrayBuffer())))]).set_name('Text').build();
    scene.objects.push(obj);
    const animations = new Map();
    animations.set('Text', (old, t) => {
        return new VectorObjectBuilder(old)
            .lerp_fill(Style.from_color(new Color(255, 0, 0, 1)), t)
            .set_name('Text')
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