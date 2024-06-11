import mathlikeanimRs, { Scene, hexToColor, svgToVector, WasmGradientImageOrColor, coordsToPoint, plotInAxes, WasmGradientStop, mathjax, WasmVectorObject, WasmLinearGradient, axes, circle } from 'https://cdn.jsdelivr.net/npm/mathlikeanim-rs@0.5.8/+esm';


const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");


async function run() {
    let scene = new Scene(BigInt(1280), BigInt(640), BigInt(60));
    scene.setCanvasContext(ctx);
    scene.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
    let title = await mathjax("\\require{color}\\textcolor{#EBEBEB}{\\text{MathLikeAnim-rs}}");
    title = title.scale(1050.0 / title.getWidth(), true);
    let subtitle = await mathjax("\\textcolor{#EBEBEB}{\\text{Interactive math and animations}}");
    subtitle = subtitle.scale(900.0 / subtitle.getWidth(), true);
    const axColor = "#636363"
    let ax = axes(
        0,
        10,
        1,
        0,
        10,
        1,
        [640.0, 320.0],
        400.0,
        400.0,
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
    ax = ax.setStroke(WasmGradientImageOrColor.fromColor(hexToColor(axColor, 1.0)), true);
    ax = ax.setFill(WasmGradientImageOrColor.fromColor(hexToColor(axColor, 1.0)), true);
    let plot = plotInAxes(
        (x) => x * x / 10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        0.1,
        ax.clone(),
        undefined,
        undefined,
        "round",
        "round",
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
                new WasmGradientStop(0.0, hexToColor("#fc6255", 0.45)),
                new WasmGradientStop(1.0, hexToColor("#ffff00", 0.45))
            ],
            1.0
        )
    ), true);
    const dotPoint = coordsToPoint(
        ax.clone(),
        0.0,
        0.0,
        0.0,
        10.0,
        0.0,
        10.0
    );
    let dot = circle(
        dotPoint,
        10.0,
        undefined,
        hexToColor(axColor, 1.0),
        hexToColor(axColor, 1.0),
        0.0,
        undefined,
        undefined,
        1
    );
    let titles = new WasmVectorObject();
    titles = titles.setSubobjects([title, subtitle]);
    titles = titles.arrangeSubobjects([0.0, 1.0], 10.0, [0.0, 0.0], true);
    titles = titles.moveTo(640.0, 320.0, true);
    let backgroundObjects = new WasmVectorObject();
    backgroundObjects = backgroundObjects.setSubobjects([plot, ax, dot]);
    backgroundObjects = backgroundObjects.setIndex(1);
    let response = await fetch("/mathlike.svg");
    let mathLikeLogo = svgToVector(await response.text());
    mathLikeLogo = mathLikeLogo.scale(150.0 / mathLikeLogo.getHeight(), true);
    mathLikeLogo = mathLikeLogo.nextToPoint([1280.0, 640.0], [-1.0, -1.0], 35.0, [0.0, 0.0], true);
    let subobjects = mathLikeLogo.getSubobjects();
    let firstSubobject = subobjects[0].setStrokeWidth(3.0, true);
    subobjects[0] = firstSubobject;
    mathLikeLogo = mathLikeLogo.setSubobjects(subobjects);
    mathLikeLogo = mathLikeLogo.setIndex(2);
    scene.add(backgroundObjects);
    scene.add(titles);
    scene.add(mathLikeLogo);
    scene.renderFrame();
}


mathlikeanimRs().then(run);
