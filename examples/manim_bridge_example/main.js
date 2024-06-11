// Manim Bridge Example - MathLikeAnim-rs

import init, {Scene, WasmGradientImageOrColor, WasmVectorObject, hexToColor, linear, smooth, spinningGrow, svgToVector} from './js/mathlikeanim_rs.js';


let scene;
let passedFrames = 0;
let urls = [];
const canvas = document.getElementById('canvas');
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
        a.download = `manim_bridge_example_${i.toString().padStart(4, '0')}.png`;
        a.click();
        await new Promise((resolve) => setTimeout(resolve, 100));
    }
    message.innerHTML = 'Done!';
}


function vectorObjectFromJson(json, index) {
    let result = new WasmVectorObject();
    result = result.setIndex(index);
    result = result.setSubobjects(json.submobjects.map((subobj, i) => vectorObjectFromJson(subobj, i)));
    result = result.setPoints(json.points.map((point) => [point[0], point[1]]));
    result = result.setFill(WasmGradientImageOrColor.fromColor(hexToColor(json["fill-color"], json["fill-opacity"])), false);
    result = result.setStroke(WasmGradientImageOrColor.fromColor(hexToColor(json["stroke-color"], json["stroke-opacity"])), false);
    result = result.setStrokeWidth(json["stroke-width"], false);
    return result;
}


async function getManimBannerCreation(t) {
    const response = await fetch(`/banner-creation?t=${t}`);
    const json = await response.json();
    return vectorObjectFromJson(json, 0);
}


async function getManimBannerExpand(t) {
    const response = await fetch(`/banner-expand?t=${t}`);
    const json = await response.json();
    return vectorObjectFromJson(json, 0);
}


async function run() {
    scene = new Scene(BigInt(3840), BigInt(2160), BigInt(60));
    scene.setCanvasContext(ctx);
    scene.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
    const latexString1 = "\\textbf{MathLikeAnim-rs}";
    const latexString2 = "\\textbf{Interactive math and animations}";
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
    title = title.setIndex(1);
    let subtitle = svgToVector(subtitleContent);
    subtitle = subtitle.setFill(textColor.clone(), true);
    subtitle = subtitle.setStroke(textColor.clone(), true);
    subtitle = subtitle.scale(2500 / subtitle.getWidth(), true);
    subtitle = subtitle.nextToOther(title.clone(), [0.0, 1.0], 50.0, [0.0, 0.0], true);
    subtitle = subtitle.setIndex(2);
    let svg = await fetch("/mathlike.svg");
    let svgText = await svg.text();
    let mathLikeLogo = svgToVector(svgText);
    mathLikeLogo = mathLikeLogo.scale(500 / mathLikeLogo.getWidth(), true);
    mathLikeLogo = mathLikeLogo.nextToPoint([3840.0, 2160.0], [-1.0, -1.0], 100.0, [0.0, 0.0], true);
    mathLikeLogo = mathLikeLogo.setIndex(3);
    let subobjects = mathLikeLogo.getSubobjects();
    let firstSubobject = subobjects[0].setStrokeWidth(8.0, true);
    subobjects[0] = firstSubobject;
    mathLikeLogo = mathLikeLogo.setSubobjects(subobjects);
    scene.add(new WasmVectorObject());
    scene.add(title);
    scene.add(subtitle);
    scene.setCallback(callback);
    await scene.play(
        async (vecs, t) => {
            let newObject = await getManimBannerCreation(t);
            return [newObject];
        },
        [],
        BigInt(60),
        linear
    );
    await scene.play(
        async (vecs, t) => {
            let newObject = await getManimBannerExpand(t);
            return [newObject];
        },
        [],
        BigInt(60),
        linear
    );
    scene.add(mathLikeLogo);
    await scene.play(
        async (vecs, t) => {
            let newLogo = spinningGrow(Math.PI / 2)(vecs[0].clone(), t);
            return [newLogo];
        },
        [3],
        BigInt(60),
        (t) => smooth(t, 10.0)
    );
    scene.renderFrame();
    await finish();
}


init().then(run);
