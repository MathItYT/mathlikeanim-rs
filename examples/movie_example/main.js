import init, {Scene, WasmGradientImageOrColor, WasmGradientStop, WasmLinearGradient, animationGroup, axes, create, drawStrokeThenFill, fadeIn, hexToColor, plotInAxes, smooth, spinningGrow, svgToVector, write} from './js/mathlikeanim_rs.js';


let scene;
let passedFrames = 0;
let urls = [];
const canvas = document.getElementById('canvas');
const message = document.getElementById('message');
const ctx = canvas.getContext('2d');


async function callback() {
    urls.push(canvas.toDataURL('image/png', 1.0));
    passedFrames++;
}


async function finish() {
    while (urls.length < passedFrames) {
        await new Promise((resolve) => setTimeout(resolve, 100));
    }
    for (let i = 0; i < urls.length; i++) {
        const a = document.createElement('a');
        a.href = urls[i];
        a.download = `movie_example_${i.toString().padStart(4, '0')}.png`;
        a.click();
        await new Promise((resolve) => setTimeout(resolve, 100));
    }
    message.innerHTML = 'Done!';
}


async function run() {
    scene = new Scene(BigInt(1920), BigInt(1080), BigInt(60));
    scene.setCallback(callback);
    scene.setCanvasContext(ctx);
    scene.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
    const latexString1 = "\\textbf{MathLikeAnim-rs}";
    const latexString2 = "\\textbf{Interactive math and animations}"
    const titleSvg = await fetch(`/latex?input=${encodeURIComponent(latexString1)}`);
    const titleContent = await titleSvg.text();
    const subtitleSvg = await fetch(`/latex?input=${encodeURIComponent(latexString2)}`);
    const subtitleContent = await subtitleSvg.text();
    const textColor = WasmGradientImageOrColor.fromColor(hexToColor("#efefef", 1.0));
    let title = svgToVector(titleContent);
    title = title.scale(1500 / title.getWidth(), true);
    title = title.setFill(textColor.clone(), true);
    title = title.setStroke(textColor.clone(), true);
    title = title.nextToPoint([960.0, 0.0], [0.0, 1.0], 50.0, [0.0, 0.0], true);
    let subtitle = svgToVector(subtitleContent);
    subtitle = subtitle.setFill(textColor.clone(), true);
    subtitle = subtitle.setStroke(textColor.clone(), true);
    subtitle = subtitle.scale(1250 / subtitle.getWidth(), true);
    subtitle = subtitle.nextToOther(title.clone(), [0.0, 1.0], 25.0, [0.0, 0.0], true);
    subtitle = subtitle.setIndex(1);
    let mathLikeLogo = svgToVector(await fetch("/mathlike.svg").then((response) => response.text()));
    mathLikeLogo = mathLikeLogo.scale(250 / mathLikeLogo.getWidth(), true);
    mathLikeLogo = mathLikeLogo.nextToPoint([1920.0, 1080.0], [-1.0, -1.0], 100.0, [0.0, 0.0], true);
    mathLikeLogo = mathLikeLogo.setIndex(2);
    let subobjects = mathLikeLogo.getSubobjects();
    let firstSubobject = subobjects[0].setStrokeWidth(4.0, true);
    subobjects[0] = firstSubobject;
    mathLikeLogo = mathLikeLogo.setSubobjects(subobjects);
    scene.add(title);
    await scene.play(
        async (vecs, t) => {
            let newTitle = write(vecs[0].getSubobjects().length, 0.4)(vecs[0].clone(), t);
            return [newTitle];
        },
        [0],
        BigInt(60),
        (t) => smooth(t, 10.0)
    );
    let animations = [];
    for (let i = 0; i < subtitle.getSubobjects().length; i++) {
        animations.push(fadeIn(2.0, [0.0, 0.0]));
    }
    scene.add(subtitle);
    await scene.play(
        async (vecs, t) => {
            let newSubtitle = animationGroup(animations, 0.4)(vecs[0].clone(), t);
            return [newSubtitle];
        },
        [1],
        BigInt(60),
        (t) => smooth(t, 10.0)
    );
    scene.add(mathLikeLogo);
    await scene.play(
        async (vecs, t) => {
            let newLogo = spinningGrow(Math.PI / 2)(vecs[0].clone(), t);
            return [newLogo];
        },
        [2],
        BigInt(60),
        (t) => smooth(t, 10.0)
    );
    let ax = axes(
        0,
        10,
        1,
        0,
        10,
        1,
        [960.0, 540.0],
        375.0,
        375.0,
        undefined,
        4.0,
        undefined,
        undefined,
        3,
        false,
        false,
        0,
        0,
        true,
        true
    );
    scene.add(ax.clone());
    await scene.play(
        async (vecs, t) => {
            let newAx = drawStrokeThenFill(vecs[0].clone(), t);
            return [newAx];
        },
        [3],
        BigInt(60),
        (t) => smooth(t, 10.0)
    );
    let plot = plotInAxes(
        (x) => x * x / 10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        0.1,
        ax,
        undefined,
        4.0,
        undefined,
        undefined,
        4
    );
    let p1 = plot.getCriticalPoint(-1.0, 1.0);
    let p2 = plot.getCriticalPoint(1.0, -1.0);
    plot = plot.setStroke(WasmGradientImageOrColor.fromLinearGradient(
        new WasmLinearGradient(
            p1[0],
            p1[1],
            p2[0],
            p2[1],
            [
                new WasmGradientStop(0.0, hexToColor("##fc6255", 1.0)),
                new WasmGradientStop(1.0, hexToColor("#ffff00", 1.0))
            ],
            1.0
        )
    ), true);
    plot = plot.setStrokeWidth(4.0, true);
    plot = plot.setIndex(4);
    scene.add(plot);
    await scene.play(
        async (vecs, t) => {
            let newPlot = create(vecs[0].clone(), t);
            return [newPlot];
        },
        [4],
        BigInt(60),
        (t) => smooth(t, 10.0)
    );
    scene.renderFrame();
    await finish();
}


init().then(run);