import init, { Color, Point2D, Style, VectorObjectBuilder, IntegerLerp, FontFace, Typst } from '@mathlikeanim-rs/mathlikeanim-rs'
import { CanvasScene } from '@mathlikeanim-rs/renderer';

const scene = new CanvasScene(1920, 1080, 60);
let firstTime = true;
let fontFace;

const run = async () => {
    if (firstTime) {
        firstTime = false;
        const canvas = scene.canvas;
        canvas.style.width = '50vw';
        canvas.style.height = 'auto';
        document.body.appendChild(canvas);
        fontFace = new Uint8Array(await fetch('latinmodern-math.otf').then(response => response.blob()).then(blob => blob.arrayBuffer()));
    }
    scene.objects.forEach(obj => obj.free());
    scene.objects = [];
    const obj = new Typst(`#show math.equation: set text(font: "Latin Modern Math")
        #set page(fill: none)
        #let name = "Typst"
        Hola`).render_to_vector_object_builder([new FontFace(fontFace)])
                .scale_to_width(1000)
                .move_to(new Point2D(960, 540))
                .set_name('Typst')
                .build()
    scene.objects.push(obj);
    const animations = new Map();
    animations.set('Typst', (old, t) => {
        if (t == 1) {
            return old;
        }
        const drawPathT = new IntegerLerp(0, 2, t);
        const strokeWidth = 0.25;
        return new VectorObjectBuilder(old)
            .pointwise_become_partial(0, drawPathT.index === 0 ? drawPathT.remainder : 1)
            .set_stroke(Style.from_color(new Color(0, 0, 0, 1)))
            .set_stroke_width(drawPathT.index === 0 ? strokeWidth : strokeWidth * (1 - drawPathT.remainder))
            .fade_fill(drawPathT.index === 0 ? 1 : 1 - drawPathT.remainder)
            .build();
    });
    await scene.play(animations, 2000, t => t);
};

const renderButton = document.getElementById('render-button');
const onClick = async () => {
    renderButton.removeEventListener('click', onClick);
    await run();
    renderButton.addEventListener('click', onClick);
};
init().then(() => renderButton.addEventListener('click', onClick));