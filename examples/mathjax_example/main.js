import  init, { Scene, WasmGradientImageOrColor, WasmVectorObject, circle, hexToColor, mathjax } from './js/mathlikeanim_rs.js';


const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");


async function run() {
    let scene = new Scene(BigInt(3840), BigInt(2160), BigInt(60));
    scene.setCanvasContext(ctx);
    scene.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
    let title = await mathjax("\\require{color} \\textbf{\\textcolor{#EBEBEB}{Now MathLikeAnim-rs supports MathJax natively!}}");
    title = title.scale(3000.0 / title.getWidth(), true);
    let subtitle = await mathjax("\\textcolor{#EBEBEB}{\\text{There's no need to use $\\rm\\LaTeX$ servers anymore!}}");
    subtitle = subtitle.scale(2000.0 / subtitle.getWidth(), true);
    let formula = await mathjax("\\textcolor{#FFFF00}{x}^{\\textcolor{#EBEBEB}{2}} \\textcolor{#EBEBEB}{+} \\textcolor{#FFFF00}{y}^{\\textcolor{#EBEBEB}{2}} \\textcolor{#EBEBEB}{=} \\textcolor{#58C4DD}{r}^{\\textcolor{#EBEBEB}{2}}");
    formula = formula.scale(750.0 / formula.getWidth(), true);
    let texts = new WasmVectorObject();
    texts = texts.setSubobjects([title, subtitle]);
    texts = texts.arrangeSubobjects([0.0, 1.0], 50.0, [0.0, 0.0], true);
    texts = texts.nextToPoint([1920.0, 0.0], [0.0, 1.0], 50.0, [0.0, 0.0], true);
    let circ = circle(
        [1920.0, 1080.0],
        500.0,
        undefined,
        hexToColor("#FC6255", 1.0),
        hexToColor("#EBEBEB", 0.0),
        8.0,
        undefined,
        undefined,
        1
    );
    let formulaAndCircle = new WasmVectorObject();
    formulaAndCircle = formulaAndCircle.setSubobjects([formula, circ]);
    formulaAndCircle = formulaAndCircle.arrangeSubobjects([0.0, 1.0], 50.0, [0.0, 0.0], true);
    formulaAndCircle = formulaAndCircle.nextToOther(texts.clone(), [0.0, 1.0], 200.0, [0.0, 0.0], true);
    formulaAndCircle = formulaAndCircle.setIndex(1);
    scene.add(texts);
    scene.add(formulaAndCircle);
    scene.renderFrame();
}


init().then(run);