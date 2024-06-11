import init, {Scene, WasmColor, WasmGradientImageOrColor, circle, drawStrokeThenFill, svgToVector, write} from './js/mathlikeanim_rs.js';


async function run() {
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    let scene = new Scene(BigInt(3840), BigInt(2160), BigInt(60));
    scene.setCanvasContext(ctx);
    let circ = circle(
        [960.0, 540.0],
        500.0,
        undefined,
        new WasmColor(1.0, 0.0, 0.0, 1.0),
        new WasmColor(1.0, 0.0, 0.0, 0.5),
        8.0,
        undefined,
        undefined,
        0
    );
    const latex = "$$x^2$$";
    const xSquaredSvg = await fetch(`/latex?input=${encodeURIComponent(latex)}`);
    const xSquaredSvgText = await xSquaredSvg.text();
    let xSquaredObject = svgToVector(xSquaredSvgText).setIndex(1);
    xSquaredObject = xSquaredObject.scale(500 / xSquaredObject.getHeight(), true);
    xSquaredObject = xSquaredObject.moveTo(960.0, 540.0, true);
    xSquaredObject = xSquaredObject.setFill(WasmGradientImageOrColor.fromColor(new WasmColor(0.0, 0.0, 0.0, 1.0)), true);
    scene.add(circ);
    scene.add(xSquaredObject);
    await scene.play(
        async (vecs, t) => {
            try {
                let newCirc = drawStrokeThenFill(vecs[0].clone(), t);
                let newXSquared = write(vecs[1].getSubobjects().length, 0.4)(vecs[1].clone(), t);
                return [newCirc, newXSquared];
            } catch (e) {
                console.error(e);
            }
        },
        [0, 1],
        BigInt(60),
        function (t) { return t;}
    );
    scene.renderFrame();
}


init().then(run);