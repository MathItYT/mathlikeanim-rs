import init, {SVGScene, WasmGradientImageOrColor, WasmGradientStop, WasmLinearGradient, animationGroup, axes, create, drawStrokeThenFill, fadeIn, hexToColor, plotInAxes, smooth, spinningGrow, svgToVector, write} from './js/mathlikeanim_rs.js';


let scene;
const container = document.getElementById('container');


function moveMathLikeLogo(x, y) {
    if (scene.getObjectsFromIndices([1]).size === 0) {
        return;
    }
    let mathLikeLogo = scene.getObjectsFromIndices([2]).get(2);
    mathLikeLogo = mathLikeLogo.moveTo(x, y, true);
    scene.add(mathLikeLogo);
    scene.renderFrame();
}


async function run() {
    scene = new SVGScene(BigInt(3840), BigInt(2160), BigInt(60));
    scene.setDivContainer(container);
    scene.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
    const latexString1 = "\\textbf{MathLikeAnim-rs}";
    const latexString2 = "\\textbf{Interactive math and animations}"
    const titleSvg = await fetch(`/latex?input=${encodeURIComponent(latexString1)}`);
    const titleContent = await titleSvg.text();
    const subtitleSvg = await fetch(`/latex?input=${encodeURIComponent(latexString2)}`);
    const subtitleContent = await subtitleSvg.text();
    const textColor = WasmGradientImageOrColor.fromColor(hexToColor("#efefef", 1.0));
    let title = svgToVector(titleContent);
    title = title.scale(3000 / title.getWidth(), true);
    title = title.setFill(textColor.clone(), true);
    title = title.setStroke(textColor.clone(), true);
    title = title.nextToPoint([1920.0, 0.0], [0.0, 1.0], 100.0, [0.0, 0.0], true);
    let subtitle = svgToVector(subtitleContent);
    subtitle = subtitle.setFill(textColor.clone(), true);
    subtitle = subtitle.setStroke(textColor.clone(), true);
    subtitle = subtitle.scale(2500 / subtitle.getWidth(), true);
    subtitle = subtitle.nextToOther(title.clone(), [0.0, 1.0], 50.0, [0.0, 0.0], true);
    subtitle = subtitle.setIndex(1);
    let mathLikeLogo = svgToVector(await fetch("/mathlike.svg").then((response) => response.text()));
    mathLikeLogo = mathLikeLogo.scale(500 / mathLikeLogo.getWidth(), true);
    mathLikeLogo = mathLikeLogo.nextToPoint([3840.0, 2160.0], [-1.0, -1.0], 200.0, [0.0, 0.0], true);
    mathLikeLogo = mathLikeLogo.setIndex(2);
    let subobjects = mathLikeLogo.getSubobjects();
    let firstSubobject = subobjects[0].setStrokeWidth(8.0, true);
    subobjects[0] = firstSubobject;
    mathLikeLogo = mathLikeLogo.setSubobjects(subobjects);
    scene.add(title);
    await scene.play(
        (vecs, t) => {
            let newTitle = write(vecs[0].getSubobjects().length, 0.4)(vecs[0], t);
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
        (vecs, t) => {
            let newSubtitle = animationGroup(animations, 0.4)(vecs[0], t);
            return [newSubtitle];
        },
        [1],
        BigInt(60),
        (t) => smooth(t, 10.0)
    );
    scene.add(mathLikeLogo);
    await scene.play(
        (vecs, t) => {
            let newLogo = spinningGrow(Math.PI / 2)(vecs[0], t);
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
        [1920.0, 1080.0],
        750.0,
        750.0,
        undefined,
        8.0,
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
        (vecs, t) => {
            let newAx = drawStrokeThenFill(vecs[0], t);
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
        undefined,
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
    plot = plot.setStrokeWidth(8.0, true);
    plot = plot.setIndex(4);
    scene.add(plot);
    await scene.play(
        (vecs, t) => {
            let newPlot = create(vecs[0], t);
            return [newPlot];
        },
        [4],
        BigInt(60),
        (t) => smooth(t, 10.0)
    );
    scene.renderFrame();
    container.onmousedown = (e) => {
        moveMathLikeLogo(e.clientX * 3840 / 960, e.clientY * 2160 / 540);
    };
}


init().then(run);
