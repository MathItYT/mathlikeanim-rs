"use client";

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from "@/components/ui/carousel";
import init, { InitOutput, SVGScene, Scene, WasmCamera, WasmGradientImageOrColor, WasmLightSource, WasmThreeDObject, WasmVectorObject, animationGroup, circle, drawStrokeThenFill, fadeIn, fadeOut, hexToColor, mathjax, rectangle, smooth, textToVector, threeDAxes } from "mathlikeanim-rs/browser";
import React from "react";
import hljs from "highlight.js";
import { Check, Copy } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Bodies, Engine, World } from "matter-js";
import { Slider } from "@/components/ui/slider";

function getSubobjectsWithPointsRecursive(obj: WasmVectorObject): WasmVectorObject[] {
  let subobjects: WasmVectorObject[] = [];
  if (obj.getPoints().length > 0) {
    subobjects.push(obj);
  }
  for (let i = 0; i < obj.getSubobjects().length; i++) {
    subobjects.push(...getSubobjectsWithPointsRecursive(obj.getSubobjects()[i]));
  }
  return subobjects;
}

export default function ExamplesPage() {
  const body1 = React.useRef(Bodies.circle(500, 200, 50, { restitution: 0.8 }));
  const body2 = React.useRef(Bodies.circle(1200, 200, 50, { restitution: 0.8 }));
  const ground = React.useRef(Bodies.rectangle(960, 1070, 1920, 20, { isStatic: true }));
  const [slide, setSlide] = React.useState(0);
  const [gotRequired, setGotRequired] = React.useState(false);
  const [isCopied, setIsCopied] = React.useState(false);
  const codeText = `import init, {
  Scene,
  WasmGradientImageOrColor,
  hexToColor,
  mathjax
} from "mathlikeanim-rs/browser";

async function run() {
  let scene = new Scene(
    BigInt(1920),
    BigInt(1080),
    BigInt(60)
  );
  let canvas = document.getElementById("canvas");
  let ctx = canvas.getContext("2d");
  scene.setCanvasContext(ctx);
  scene.setBackground(WasmGradientImageOrColor.fromColor(
    hexToColor("#161616", 1.0)
  ));
  let hello = await textToVector(
    "Hello, MathLikeAnim-rs!",
    "/fonts/Inter-Bold.ttf"
  );
  hello = hello.setFill(
    WasmGradientImageOrColor.fromColor(
      hexToColor("#EBEBEB", 1.0)
    ),
    true
  );
  hello = hello.scale(1000.0 / hello.getWidth(), true);
  hello = hello.moveTo(960.0, 540.0, true);
  scene.add(hello.clone());
  scene.renderFrame();
}

init().then(run);`;
  const codeText2 = `import init, {
  SVGScene,
  WasmGradientImageOrColor,
  hexToColor,
  mathjax
} from "mathlikeanim-rs/browser";

async function run() {
  let scene = new SVGScene(
    BigInt(1920),
    BigInt(1080),
    BigInt(60)
  );
  let container = document.getElementById("container");
  scene.setDivContainer(container);
  scene.setBackground(WasmGradientImageOrColor.fromColor(
    hexToColor("#161616", 1.0)
  ));
  let hello = await textToVector(
    "Hello, MathLikeAnim-rs!",
    "/fonts/Inter-Bold.ttf"
  );
  hello = hello.setFill(
    WasmGradientImageOrColor.fromColor(
      hexToColor("#EBEBEB", 1.0)
    ),
    true
  );
  hello = hello.scale(1000.0 / hello.getWidth(), true);
  hello = hello.moveTo(960.0, 540.0, true);
  scene.add(hello.clone());
  scene.renderFrame();
}

init().then(run);`;
  const codeText3 = `import init, {
  Scene,
  WasmGradientImageOrColor,
  hexToColor,
  mathjax,
  drawStrokeThenFill,
  fadeOut
} from "mathlikeanim-rs/browser";

async function run() {
  let scene = new Scene(
    BigInt(1920),
    BigInt(1080),
    BigInt(60)
  );
  let canvas = document.getElementById("canvas");
  let ctx = canvas.getContext("2d");
  scene.setCanvasContext(ctx);
  scene.setBackground(WasmGradientImageOrColor.fromColor(
    hexToColor("#161616", 1.0)
  ));
  let text = "\\\\textcolor{#EBEBEB}";
  text += "{\\\\text{Let's prove that }";
  text += "\\\\sqrt{2}\\\\text{ is irrational!}}";
  let letsProve = await mathjax(text);
  letsProve = letsProve.scale(
    1000.0 / letsProve.getWidth(),
    true
  );
  letsProve = letsProve.moveTo(
    960.0,
    540.0,
    true
  );
  scene.add(letsProve.clone());
  await scene.play(
    async (objects, t) => {
      return [drawStrokeThenFill(
        objects[0].clone(), t
      )];
    },
    Uint32Array.from([0]),
    30,
    t => smooth(t, 10.0)
  );
  await scene.sleep(500);
  await scene.play(
    async (objects, t) => {
      return [fadeOut(
        objects[0].clone(),
        5.0,
        [0.0, 0.0],
        t
      )];
    },
    Uint32Array.from([0]),
    30,
    t => smooth(t, 10.0)
  );
  let txt = "\\\\textcolor{#EBEBEB}";
  txt += "{\\\\text{Assume that}}";
  let assumeThat = await mathjax(txt);
  txt = "\\\\textcolor{#EBEBEB}{\\\\sqrt{2}}";
  let sqrt2 = await mathjax(txt);
  txt = "\\\\textcolor{#EBEBEB}";
  txt += "{\\\\text{is rational.}}";
  let isRational = await mathjax(txt);
  let text2 = new WasmVectorObject().setSubobjects([
    assumeThat.clone(),
    sqrt2.clone(),
    isRational.clone()
  ]);
  text2 = text2.scale(
    1000 / text2.getWidth(),
    true
  );
  text2 = text2.arrangeSubobjects(
    [0.0, 1.0],
    20.0,
    [0.0, 0.0],
    true
  );
  text2 = text2.moveTo(
    960.0,
    540.0,
    true
  );
  text2 = new WasmVectorObject().setSubobjects(
    getSubobjectsWithPointsRecursive(
      text2.clone()
    )
  );
  scene.add(text2.clone());
  await scene.play(
    async (objects, t) => {
      return [animationGroup(
        objects[0].clone(),
        animations,
        0.4,
        t
      )];
    },
    Uint32Array.from([0]),
    30,
    t => smooth(t, 10.0)
  );
  await scene.sleep(500);
  await scene.play(
    async (objects, t) => {
      return [fadeOut(
        objects[0].clone(),
        1.0,
        [0.0, 0.0],
        t
      )];
    },
    Uint32Array.from([0]),
    30,
    t => smooth(t, 10.0)
  );
  let ifSoText = "\\\\textcolor{#EBEBEB}";
  ifSoText += "{\\\\text{If so, then there exist }}";
  ifSoText += "\\\\textcolor{#FFFF00}{a}";
  ifSoText += "\\\\textcolor{#EBEBEB}{, }";
  ifSoText += "\\\\textcolor{#FFFF00}{b}";
  ifSoText += "\\\\textcolor{#EBEBEB}{\\\\in ";
  ifSoText += "\\\\mathbb{Z}^+}";
  let ifSo = await mathjax(ifSoText);
  let suchThatTxt = "\\\\textcolor{#EBEBEB}";
  suchThatTxt += "{\\\\text{such that:}}";
  let suchThat = await mathjax(suchThatTxt);
  let sqrt2EqualsText = "\\\\textcolor{#EBEBEB}";
  sqrt2EqualsText += "{\\\\sqrt{2} =}";
  sqrt2EqualsText += "\\\\textcolor{#EBEBEB}";
  sqrt2EqualsText += "{\\\\textcolor{#FFFF00}{a}";
  sqrt2EqualsText += "\\\\over \\\\textcolor{#FFFF00}{b}}";
  let sqrt2Equals = await mathjax(sqrt2EqualsText);
  let text3 = new WasmVectorObject().setSubobjects([
    ifSo.clone(),
    suchThat.clone(),
    sqrt2Equals.clone()
  ]);
  text3 = text3.scale(
    1000 / text3.getWidth(),
    true
  );
  text3 = text3.arrangeSubobjects(
    [0.0, 1.0],
    20.0,
    [-1.0, 0.0],
    true
  );
  let subobjs = text3.getSubobjects();
  let lastSubobj = subobjs[subobjs.length - 1].clone();
  lastSubobj = lastSubobj.moveTo(
    960.0,
    lastSubobj.getCenter()[1],
    true
  );
  subobjs[subobjs.length - 1] = lastSubobj;
  text3 = text3.setSubobjects(subobjs);
  text3 = new WasmVectorObject().setSubobjects(
    getSubobjectsWithPointsRecursive(
      text3.clone()
    )
  );
  scene.add(text3.clone());
  await scene.play(
    async (objects, t) => {
      return [animationGroup(
        objects[0].clone(),
        animations2,
        0.4,
        t
      )];
    },
    Uint32Array.from([0]),
    30,
    t => smooth(t, 10.0)
  );
  scene.renderFrame();
}

init().then(run);`;
  const codeText4 = `import init, {
  Scene,
  WasmGradientImageOrColor,
  hexToColor,
  circle,
  rectangle
} from "mathlikeanim-rs/browser";
import { Bodies, World, Engine } from "matter-js";

async function run() {
  let engine = Engine.create();
  let body1 = Bodies.circle(500, 200, 50);
  let body2 = Bodies.circle(1200, 200, 50);
  let ground = Bodies.rectangle(
    960,
    1070,
    1920,
    20,
    { isStatic: true }
  );
  World.add(engine.world, [body1, body2, ground]);
  let scene = new Scene(
    BigInt(1920),
    BigInt(1080),
    BigInt(60)
  );
  let canvas = document.getElementById("canvas");
  let ctx = canvas.getContext("2d");
  scene.setCanvasContext(ctx);
  scene.setBackground(
    WasmGradientImageOrColor.fromColor(
      hexToColor("#161616", 1.0)
    )
  );
  while (true) {
    let circ1 = circle(
      [body1.position.x, body1.position.y],
      50,
      undefined,
      hexToColor("#EBEBEB", 1.0),
      hexToColor("#FC6255", 1.0),
      4.0,
      undefined,
      undefined,
      0
    );
    let circ2 = circle(
      [body2.position.x, body2.position.y],
      50,
      undefined,
      hexToColor("#EBEBEB", 1.0),
      hexToColor("#58C4DD", 1.0),
      4.0,
      undefined,
      undefined,
      1
    );
    let rect = rectangle(
      [960, 1070],
      1920,
      20,
      hexToColor("#EBEBEB", 0.0),
      hexToColor("#EBEBEB", 1.0),
      0.0,
      undefined,
      undefined,
      2
    );
    scene.add(circ1);
    scene.add(circ2);
    scene.add(rect);
    scene.renderFrame();
    Engine.update(engine, 1000 / 60);
    await scene.sleep(Math.floor(1000 / 60));
  }
}

init().then(run);`;
  const codeText5 = `import init, {
  Scene,
  WasmGradientImageOrColor,
  hexToColor,
  WasmThreeDObject,
  WasmCamera,
  WasmLightSource,
  threeDAxes
} from "mathlikeanim-rs/browser";

const sliderPhi = document.getElementById("slider-phi");
const sliderTheta = document.getElementById("slider-theta");
const sliderGamma = document.getElementById("slider-gamma");
const sliderLightX = document.getElementById("slider-light-x");
const sliderLightY = document.getElementById("slider-light-y");
const sliderLightZ = document.getElementById("slider-light-z");

let object3D;
let camera;
let lightSource;

async function run() {
  let scene = new Scene(
    BigInt(1920),
    BigInt(1080),
    BigInt(60)
  );
  let canvas = document.getElementById("canvas");
  let ctx = canvas.getContext("2d");
  scene.setCanvasContext(ctx);
  scene.setBackground(
    WasmGradientImageOrColor.fromColor(
      hexToColor("#161616", 1.0)
    )
  );
  let sphere = WasmThreeDObject.fromUvFunction(
    (u, v) => {
      return [
        250.0 * Math.cos(u) * Math.sin(v),
        250.0 * Math.sin(u) * Math.sin(v),
        -250.0 * Math.cos(v)
      ];
    },
    [0, 2 * Math.PI],
    [0, Math.PI],
    50,
    50,
    [hexToColor("#29ABCA", 1.0), hexToColor("#236B8E", 1.0)],
    [hexToColor("#EBEBEB", 0.0)],
    0.0
  );
  let axes3D = threeDAxes(
    -1,
    1,
    0.1,
    -1,
    1,
    0.1,
    -1,
    1,
    0.1,
    [0, 0, 0],
    750,
    750,
    750,
    hexToColor("#EBEBEB", 1.0),
    4,
    false,
    false,
    false,
    0,
    0,
    0,
    true,
    true,
    true
  );
  object3D = new WasmThreeDObject(
    [],
    [sphere.clone(), axes3D.clone()],
    WasmGradientImageOrColor.fromColor(
      hexToColor("#161616", 0.0)
    ),
    WasmGradientImageOrColor.fromColor(
      hexToColor("#161616", 0.0)
    ),
    0
  );
  camera = new WasmCamera(
    [0, 0, 0],
    [0, -Math.PI / 2, 0],
    1350,
    1,
    1920,
    1080
  );
  lightSource = new WasmLightSource([960, -540, 0]);
  scene.add(object3D.projectAndShade(camera, lightSource));
  scene.renderFrame();
};

const update = () => {
  if (!object3D || !camera || !lightSource) return;
  camera = new WasmCamera(
    [0, 0, 0],
    [
      sliderPhi.value,
      sliderTheta.value,
      sliderGamma.value
    ],
    1350,
    1,
    1920,
    1080
  );
  lightSource = new WasmLightSource([
    liderLightX.value,
    sliderLightY.value,
    sliderLightZ.value
  ]);
  scene.add(object3D.projectAndShade(camera, lightSource));
  scene.renderFrame();
};

sliderPhi.addEventListener("input", update);
sliderTheta.addEventListener("input", update);
sliderGamma.addEventListener("input", update);
sliderLightX.addEventListener("input", update);
sliderLightY.addEventListener("input", update);
sliderLightZ.addEventListener("input", update);

init().then(run);`;
  const [current, setCurrent] = React.useState<HTMLCanvasElement | null>(null);
  const [current2, setCurrent2] = React.useState<HTMLDivElement | null>(null);
  const [current3, setCurrent3] = React.useState<HTMLCanvasElement | null>(null);
  const [current4, setCurrent4] = React.useState<HTMLCanvasElement | null>(null);
  const [current5, setCurrent5] = React.useState<HTMLCanvasElement | null>(null);
  const [out, setOut] = React.useState<InitOutput | null>(null);
  const [scene, setScene] = React.useState<Scene | null>(null);
  const [scene2, setScene2] = React.useState<SVGScene | null>(null);
  const [scene3, setScene3] = React.useState<Scene | null>(null);
  const [scene4, setScene4] = React.useState<Scene | null>(null);
  const [scene5, setScene5] = React.useState<Scene | null>(null);
  const [ctx, setCtx] = React.useState<CanvasRenderingContext2D | null>(null);
  const [ctx3, setCtx3] = React.useState<CanvasRenderingContext2D | null>(null);
  const [ctx4, setCtx4] = React.useState<CanvasRenderingContext2D | null>(null);
  const [ctx5, setCtx5] = React.useState<CanvasRenderingContext2D | null>(null);
  const [ctxSet3, setCtxSet3] = React.useState(false);
  const [ctxSet4, setCtxSet4] = React.useState(false);
  const [ctxSet5, setCtxSet5] = React.useState(false);
  const [object3D, setObject3D] = React.useState<WasmThreeDObject | null>(null);
  const [camera, setCamera] = React.useState<WasmCamera | null>(null);
  const [lightSource, setLightSource] = React.useState<WasmLightSource | null>(null);
  const engine = React.useRef(Engine.create({gravity: {x: 0, y: 2}}));
  const prevSlideRef = React.useCallback((el: HTMLButtonElement | null) => {
    if (!el) return;
    el.addEventListener("click", () => {
      setSlide((slide) => slide - 1);
    });
  }, []);
  const nextSlideRef = React.useCallback((el: HTMLButtonElement | null) => {
    if (!el) return;
    el.addEventListener("click", () => {
      setSlide((slide) => slide + 1);
    });
  }, []);
  const ref = React.useCallback((canvas: HTMLCanvasElement | null) => {
    if (!canvas) return;
    setCurrent(canvas);
  }, []);
  const ref2 = React.useCallback((container: HTMLDivElement | null) => {
    if (!container) return;
    setCurrent2(container);
  }, []);
  const ref3 = React.useCallback((canvas: HTMLCanvasElement | null) => {
    if (!canvas) return;
    setCurrent3(canvas);
  }, []);
  const ref4 = React.useCallback((canvas: HTMLCanvasElement | null) => {
    if (!canvas) return;
    setCurrent4(canvas);
  }, []);
  const ref5 = React.useCallback((canvas: HTMLCanvasElement | null) => {
    if (!canvas) return;
    setCurrent5(canvas);
  }, []);
  React.useEffect(() => {
    if (!current || !current2 || !current3 || !current4) return;
    init().then(setOut)
  }, [current, current2, current3, current4]);
  React.useEffect(() => {
    if (!current) return;
    setCtx(current.getContext("2d"));
  }, [current]);
  React.useEffect(() => {
    if (!current3) return;
    setCtx3(current3.getContext("2d"));
  }, [current3]);
  React.useEffect(() => {
    if (!current4) return;
    setCtx4(current4.getContext("2d"));
  }, [current4]);
  React.useEffect(() => {
    if (!current5) return;
    setCtx5(current5.getContext("2d"));
  }, [current5]);
  React.useEffect(() => {
    if (!out) return;
    setScene(new Scene(1920, 1080, 60));
  }, [out]);
  React.useEffect(() => {
    if (!out) return;
    setScene2(new SVGScene(1920, 1080, 60));
  }, [out]);
  React.useEffect(() => {
    if (!out) return;
    setScene3(new Scene(1920, 1080, 60));
  }, [out]);
  React.useEffect(() => {
    if (!out) return;
    setScene4(new Scene(1920, 1080, 60));
  }, [out]);
  React.useEffect(() => {
    if (!out) return;
    setScene5(new Scene(1920, 1080, 60));
  }, [out]);
  React.useEffect(() => {
    if (!scene3 || !ctx3) return;
    scene3.setCanvasContext(ctx3);
    scene3.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
    scene3.renderFrame();
    setCtxSet3(true);
  }, [scene3, ctx3]);
  React.useEffect(() => {
    if (!scene4 || !ctx4) return;
    World.add(engine.current.world, [body1.current, body2.current, ground.current]);
    scene4.setCanvasContext(ctx4);
    scene4.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
    scene4.renderFrame();
    setCtxSet4(true);
  }, [scene4, ctx4]);
  React.useEffect(() => {
    if (!scene5 || !ctx5) return;
    scene5.setCanvasContext(ctx5);
    scene5.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
    scene5.renderFrame();
    const sphere = WasmThreeDObject.fromUvFunction(
      (u: number, v: number) => {
        return [
          250.0 * Math.cos(u) * Math.sin(v),
          250.0 * Math.sin(u) * Math.sin(v),
          -250.0 * Math.cos(v)
        ];
      },
      [0, 2 * Math.PI],
      [0, Math.PI],
      50,
      50,
      [hexToColor('#29ABCA', 1.0), hexToColor('#236B8E', 1.0)],
      [hexToColor('#EBEBEB', 0.0)],
      0.0
    );
    const axes3D = threeDAxes(
      -1,
      1,
      0.1,
      -1,
      1,
      0.1,
      -1,
      1,
      0.1,
      [0, 0, 0],
      750,
      750,
      750,
      hexToColor('#EBEBEB', 1.0),
      4,
      false,
      false,
      false,
      0,
      0,
      0,
      true,
      true,
      true
    );
    setObject3D(new WasmThreeDObject(
      [],
      [sphere.clone(), axes3D.clone()],
      WasmGradientImageOrColor.fromColor(hexToColor('#161616', 0.0)),
      WasmGradientImageOrColor.fromColor(hexToColor('#161616', 0.0)),
      0
    ));
    setCamera(new WasmCamera([0, 0, 0], [0, -Math.PI / 2, 0], 1350, 1, 1920, 1080));
    setLightSource(new WasmLightSource([960, -540, 0]));
    setCtxSet5(true);
  }, [scene5, ctx5]);
  React.useEffect(() => {
    if (!scene || !ctx) return;
    const run = async () => {
      scene.setCanvasContext(ctx);
      scene.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
      let hello = await textToVector("Hello, MathLikeAnim-rs!", "/fonts/Inter-Bold.ttf", 0, 0, 144);
      hello = hello.setFill(WasmGradientImageOrColor.fromColor(hexToColor("#EBEBEB", 1.0)), true);
      await mathjax("\\require{color}");
      setGotRequired(true);
      hello = hello.scale(1000.0 / hello.getWidth(), true);
      hello = hello.moveTo(960.0, 540.0, true);
      scene.add(hello.clone());
      scene.renderFrame();
    }
    run();
  }, [scene, ctx]);
  React.useEffect(() => {
    if (!scene2 || !current2 || !out || !gotRequired) return;
    const run = async () => {
      scene2.setDivContainer(current2);
      scene2.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
      let hello = await textToVector("Hello, MathLikeAnim-rs!", "/fonts/Inter-Bold.ttf", 0, 0, 144);
      hello = hello.setFill(WasmGradientImageOrColor.fromColor(hexToColor("#EBEBEB", 1.0)), true);
      hello = hello.scale(1000.0 / hello.getWidth(), true);
      hello = hello.moveTo(960.0, 540.0, true);
      scene2.add(hello.clone());
      scene2.renderFrame();
    }
    run();
  }, [scene2, current2, out, gotRequired]);
  React.useEffect(() => {
    if (!scene3 || !ctxSet3 || !gotRequired || slide !== 2) return;
    const run = async () => {
      scene3.clear();
      scene3.renderFrame();
      await scene3.sleep(500);
      let letsProve = await mathjax("\\textcolor{#EBEBEB}{\\text{Let's prove that }\\sqrt{2}\\text{ is irrational!}}");
      letsProve = letsProve.scale(1000.0 / letsProve.getWidth(), true);
      letsProve = letsProve.moveTo(960.0, 540.0, true);
      scene3.add(letsProve.clone());
      if (slide !== 2) return;
      await scene3.play(
        async (objects: WasmVectorObject[], t: number) => {
          return [drawStrokeThenFill(objects[0].clone(), t)];
        },
        Uint32Array.from([0]),
        30,
        (t: number) => smooth(t, 10.0)
      );
      if (slide !== 2) return;
      await scene3.sleep(500);
      if (slide !== 2) return;
      await scene3.play(
        async (objects: WasmVectorObject[], t: number) => {
          return [fadeOut(objects[0].clone(), 5.0, [0.0, 0.0], t)];
        },
        Uint32Array.from([0]),
        30,
        (t: number) => smooth(t, 10.0)
      );
      let assumeThat = await mathjax("\\textcolor{#EBEBEB}{\\text{Assume that}}");
      let sqrt2 = await mathjax("\\textcolor{#EBEBEB}{\\sqrt{2}}");
      let isRational = await mathjax("\\textcolor{#EBEBEB}{\\text{is rational.}}");
      let text = new WasmVectorObject().setSubobjects([assumeThat.clone(), sqrt2.clone(), isRational.clone()]);
      text = text.scale(1000 / text.getWidth(), true);
      text = text.arrangeSubobjects([0.0, 1.0], 20.0, [0.0, 0.0], true);
      text = text.moveTo(960.0, 540.0, true);
      text = new WasmVectorObject().setSubobjects(getSubobjectsWithPointsRecursive(text.clone()));
      scene3.add(text.clone());
      const animations: Function[] = [];
      for (let i = 0; i < text.getSubobjects().length; i++) {
        animations.push((obj: WasmVectorObject, t: number) => {
          return fadeIn(obj.clone(), 1.5, [0.0, 0.0], t);
        });
      }
      if (slide !== 2) return;
      await scene3.play(
        async (objects: WasmVectorObject[], t: number) => {
          return [animationGroup(objects[0].clone(), animations, 0.4, t)];
        },
        Uint32Array.from([0]),
        30,
        (t: number) => smooth(t, 10.0)
      );
      if (slide !== 2) return;
      await scene3.sleep(500);
      if (slide !== 2) return;
      await scene3.play(
        async (objects: WasmVectorObject[], t: number) => {
          return [fadeOut(objects[0].clone(), 1.0, [0.0, 0.0], t)];
        },
        Uint32Array.from([0]),
        30,
        (t: number) => smooth(t, 10.0)
      );
      let ifSo = await mathjax("\\textcolor{#EBEBEB}{\\text{If so, then there exist }}\\textcolor{#FFFF00}{a}\\textcolor{#EBEBEB}{, }\\textcolor{#FFFF00}{b}\\textcolor{#EBEBEB}{\\in \\mathbb{Z}^+}");
      let suchThat = await mathjax("\\textcolor{#EBEBEB}{\\text{such that:}}");
      let sqrt2Equals = await mathjax("\\textcolor{#EBEBEB}{\\sqrt{2} =}\\textcolor{#EBEBEB}{\\textcolor{#FFFF00}{a}\\over\\textcolor{#FFFF00}{b}}");
      let text2 = new WasmVectorObject().setSubobjects([ifSo.clone(), suchThat.clone(), sqrt2Equals.clone()]);
      text2 = text2.scale(1000 / text2.getWidth(), true);
      text2 = text2.arrangeSubobjects([0.0, 1.0], 20.0, [-1.0, 0.0], true);
      text2 = text2.moveTo(960.0, 540.0, true);
      let subobjs = text2.getSubobjects();
      let lastSubobj = subobjs[subobjs.length - 1].clone();
      lastSubobj = lastSubobj.moveTo(960.0, lastSubobj.getCenter()[1], true);
      subobjs[subobjs.length - 1] = lastSubobj;
      text2 = text2.setSubobjects(subobjs);
      text2 = new WasmVectorObject().setSubobjects(getSubobjectsWithPointsRecursive(text2.clone()));
      scene3.add(text2.clone());
      const animations2: Function[] = [];
      for (let i = 0; i < text2.getSubobjects().length; i++) {
        animations2.push((obj: WasmVectorObject, t: number) => {
          return fadeIn(obj.clone(), 1.5, [0.0, 0.0], t);
        });
      }
      if (slide !== 2) return;
      await scene3.play(
        async (objects: WasmVectorObject[], t: number) => {
          return [animationGroup(objects[0].clone(), animations2, 0.4, t)];
        },
        Uint32Array.from([0]),
        30,
        (t: number) => smooth(t, 10.0)
      );
      scene3.renderFrame();
    }
    run();
  }, [scene3, gotRequired, slide, ctxSet3]);
  React.useEffect(() => {
    if (!scene4 || slide !== 3 || !ctxSet4) return;
    const run = async () => {
      while (true) {
        let circ1 = circle(
          [body1.current.position.x, body1.current.position.y],
          50,
          undefined,
          hexToColor("#EBEBEB", 1.0),
          hexToColor("#FC6255", 1.0),
          4.0,
          undefined,
          undefined,
          0
        );
        let circ2 = circle(
          [body2.current.position.x, body2.current.position.y],
          50,
          undefined,
          hexToColor("#EBEBEB", 1.0),
          hexToColor("#58C4DD", 1.0),
          4.0,
          undefined,
          undefined,
          1
        );
        let rect = rectangle(
          [960, 1070],
          1920,
          20,
          hexToColor("#EBEBEB", 0.0),
          hexToColor("#EBEBEB", 1.0),
          0.0,
          undefined,
          undefined,
          2
        );
        scene4.add(circ1.clone());
        scene4.add(circ2.clone());
        scene4.add(rect.clone());
        scene4.renderFrame();
        Engine.update(engine.current, 1000 / 60);
        await scene4.sleep(Math.floor(1000 / 60));
      }
    }
    run();
  }, [scene4, slide, ctxSet4]);
  React.useEffect(() => {
    if (!scene5 || !object3D || !camera || !lightSource || !ctxSet5 ) return;
    const run = async () => {
      scene5.add(object3D.projectAndShade(camera, lightSource));
      scene5.renderFrame();
    }
    run();
  }, [scene5, ctxSet5, object3D, camera, lightSource]);
  const codeRef = React.useCallback((code: HTMLDivElement | null) => {
    if (!code) return;
    code.innerHTML = hljs.highlight(
      codeText,
      { language: "javascript" }
    ).value;
  }, []);
  const codeRef2 = React.useCallback((code: HTMLDivElement | null) => {
    if (!code) return;
    code.innerHTML = hljs.highlight(
      codeText2,
      { language: "javascript" }
    ).value;
  }, []);
  const codeRef3 = React.useCallback((code: HTMLDivElement | null) => {
    if (!code) return;
    code.innerHTML = hljs.highlight(
      codeText3,
      { language: "javascript" }
    ).value;
  }, []);
  const codeRef4 = React.useCallback((code: HTMLDivElement | null) => {
    if (!code) return;
    code.innerHTML = hljs.highlight(
      codeText4,
      { language: "javascript" }
    ).value;
  }, []);
  const codeRef5 = React.useCallback((code: HTMLDivElement | null) => {
    if (!code) return;
    code.innerHTML = hljs.highlight(
      codeText5,
      { language: "javascript" }
    ).value;
  }, []);
  const changeCameraAndLightSource = (
    phi: number,
    theta: number,
    gamma: number,
    lightX: number,
    lightY: number,
    lightZ: number
  ) => {
    setCamera(new WasmCamera([0, 0, 0], [phi, theta, gamma], 1350, 1, 1920, 1080));
    setLightSource(new WasmLightSource([lightX, lightY, lightZ]));
  };
  return (
    <div className="flex flex-col items-center justify-center pb-[5vh]">
      <h1 className="text-4xl font-bold">Examples</h1>
      <div className="flex flex-col items-center justify-center pt-4">
        <Carousel className="w-[50vw] h-[80vh]">
          <CarouselContent>
            <CarouselItem>
              <Card>
                <CardContent>
                  <CardHeader>
                    <CardTitle>HTML Canvas Image</CardTitle>
                    <CardDescription>
                      A simple example of rendering an image on an HTML Canvas using MathLikeAnim-rs.
                    </CardDescription>
                  </CardHeader>
                  <div className="flex justify-center">
                    <canvas ref={ref} className="w-[35vw] h-auto rounded-[2vh]" width="1920" height="1080"></canvas>
                  </div>
                  <div className="flex justify-center relative mt-[1vh] w-[45vw]">
                    <Button variant="ghost" size="icon" className="w-[36px] h-[36px] p-[9px]" asChild onClick={async () => {
                      await navigator.clipboard.writeText(codeText);
                      setIsCopied(true);
                      setTimeout(() => {
                        setIsCopied(false);
                      }, 1500);
                    }}>
                      {isCopied ? <Check className="absolute top-[8px] right-[8px]"/> : <Copy className="absolute top-[8px] right-[8px]"/>}
                    </Button>
                    <div ref={codeRef} className="code w-[45vw]"/>
                  </div>
                </CardContent>
              </Card>
            </CarouselItem>
            <CarouselItem>
              <Card>
                <CardContent>
                  <CardHeader>
                    <CardTitle>SVG Image</CardTitle>
                    <CardDescription>
                      A simple example of rendering an SVG image using MathLikeAnim-rs.
                    </CardDescription>
                  </CardHeader>
                  <div ref={ref2} className="container-half container-rounded flex justify-center"/>
                  <div className="flex justify-center relative mt-[1vh] w-[45vw]">
                    <Button variant="ghost" size="icon" className="w-[36px] h-[36px] p-[9px]" asChild onClick={async () => {
                      await navigator.clipboard.writeText(codeText2);
                      setIsCopied(true);
                      setTimeout(() => {
                        setIsCopied(false);
                      }, 1500);
                    }}>
                      {isCopied ? <Check className="absolute top-[8px] right-[8px]"/> : <Copy className="absolute top-[8px] right-[8px]"/>}
                    </Button>
                  </div>
                  <div ref={codeRef2} className="code w-[45vw]"/>
                </CardContent>
              </Card>
            </CarouselItem>
            <CarouselItem>
              <Card>
                <CardContent>
                  <CardHeader>
                    <CardTitle>Mathematical Proof (Part 1)</CardTitle>
                    <CardDescription>
                      An animated example of a mathematical proof using MathLikeAnim-rs.
                    </CardDescription>
                  </CardHeader>
                  <div className="flex flex-col items-center justify-center">
                    <canvas ref={ref3} className="w-[35vw] h-auto rounded-[2vh]" width="1920" height="1080"></canvas>
                  </div>
                  <div className="flex justify-center relative mt-[1vh] w-[45vw]">
                    <Button variant="ghost" size="icon" className="w-[36px] h-[36px] p-[9px]" asChild onClick={async () => {
                      await navigator.clipboard.writeText(codeText3);
                      setIsCopied(true);
                      setTimeout(() => {
                        setIsCopied(false);
                      }, 1500);
                    }}>
                      {isCopied ? <Check className="absolute top-[8px] right-[8px]"/> : <Copy className="absolute top-[8px] right-[8px]"/>}
                    </Button>
                  </div>
                  <div ref={codeRef3} className="code w-[45vw]"/>
                </CardContent>
              </Card>
            </CarouselItem>
            <CarouselItem>
              <Card>
                <CardContent>
                  <CardHeader>
                    <CardTitle>Physics Example</CardTitle>
                    <CardDescription>
                      An animated example of a physics simulation using MathLikeAnim-rs and Matter.js together.
                    </CardDescription>
                  </CardHeader>
                  <div className="flex justify-center">
                    <canvas ref={ref4} className="w-[35vw] h-auto rounded-[2vh]" width="1920" height="1080"></canvas>
                  </div>
                  <div className="flex justify-center relative mt-[1vh] w-[45vw]">
                    <Button variant="ghost" size="icon" className="w-[36px] h-[36px] p-[9px]" asChild onClick={async () => {
                      await navigator.clipboard.writeText(codeText4);
                      setIsCopied(true);
                      setTimeout(() => {
                        setIsCopied(false);
                      }, 1500);
                    }}>
                      {isCopied ? <Check className="absolute top-[8px] right-[8px]"/> : <Copy className="absolute top-[8px] right-[8px]"/>}
                    </Button>
                  </div>
                  <div ref={codeRef4} className="code w-[45vw]"/>
                </CardContent>
              </Card>
            </CarouselItem>
            <CarouselItem>
              <Card>
                <CardContent>
                  <CardHeader>
                    <CardTitle>3D Example</CardTitle>
                    <CardDescription>
                      An interactive example of a 3D sphere with axes using MathLikeAnim-rs.
                    </CardDescription>
                  </CardHeader>
                  <div className="flex justify-center">
                    <canvas ref={ref5} className="w-[35vw] h-auto rounded-[2vh]" width="1920" height="1080"></canvas>
                  </div>
                  <div className="flex flex-col justify-center items-center">
                    <Slider defaultValue={[0]} min={0} max={360} step={1} onValueChange={(value) => {
                      if (!camera || !lightSource) return;
                      changeCameraAndLightSource(
                        value[0] * Math.PI / 180.0,
                        camera.getRotation()[1],
                        camera.getRotation()[2],
                        lightSource.getPosition()[0],
                        lightSource.getPosition()[1],
                        lightSource.getPosition()[2]
                      );
                    }} className="pt-[2vh] w-[30vw]"></Slider>
                    <Slider defaultValue={[0]} min={0} max={360} step={1} onValueChange={(value) => {
                      if (!camera || !lightSource) return;
                      changeCameraAndLightSource(
                        camera.getRotation()[0],
                        value[0] * Math.PI / 180.0 - Math.PI / 2,
                        camera.getRotation()[2],
                        lightSource.getPosition()[0],
                        lightSource.getPosition()[1],
                        lightSource.getPosition()[2]
                      );
                    }} className="pt-[2vh] w-[30vw]"></Slider>
                    <Slider defaultValue={[0]} min={0} max={360} step={1} onValueChange={(value) => {
                      if (!camera || !lightSource) return;
                      changeCameraAndLightSource(
                        camera.getRotation()[0],
                        camera.getRotation()[1],
                        value[0] * Math.PI / 180.0,
                        lightSource.getPosition()[0],
                        lightSource.getPosition()[1],
                        lightSource.getPosition()[2]
                      );
                    }} className="pt-[2vh] w-[30vw]"></Slider>
                    <Slider defaultValue={[960]} min={0} max={1920} step={1} onValueChange={(value) => {
                      if (!camera || !lightSource) return;
                      changeCameraAndLightSource(
                        camera.getRotation()[0],
                        camera.getRotation()[1],
                        camera.getRotation()[2],
                        value[0],
                        lightSource.getPosition()[1],
                        lightSource.getPosition()[2]
                      );
                    }} className="pt-[2vh] w-[30vw]"></Slider>
                    <Slider defaultValue={[-540]} min={-1080} max={0} step={1} onValueChange={(value) => {
                      if (!camera || !lightSource) return;
                      changeCameraAndLightSource(
                        camera.getRotation()[0],
                        camera.getRotation()[1],
                        camera.getRotation()[2],
                        lightSource.getPosition()[0],
                        value[0],
                        lightSource.getPosition()[2]
                      );
                    }} className="pt-[2vh] w-[30vw]"></Slider>
                    <Slider defaultValue={[0]} min={-1080} max={1080} step={1} onValueChange={(value) => {
                      if (!camera || !lightSource) return;
                      changeCameraAndLightSource(
                        camera.getRotation()[0],
                        camera.getRotation()[1],
                        camera.getRotation()[2],
                        lightSource.getPosition()[0],
                        lightSource.getPosition()[1],
                        value[0]
                      );
                    }} className="py-[2vh] w-[30vw]"></Slider>
                  </div>
                  <div className="flex justify-center relative mt-[1vh] w-[45vw]">
                    <Button variant="ghost" size="icon" className="w-[36px] h-[36px] p-[9px]" asChild onClick={async () => {
                      await navigator.clipboard.writeText(codeText5);
                      setIsCopied(true);
                      setTimeout(() => {
                        setIsCopied(false);
                      }, 1500);
                    }}>
                      {isCopied ? <Check className="absolute top-[8px] right-[8px]"/> : <Copy className="absolute top-[8px] right-[8px]"/>}
                    </Button>
                    <div ref={codeRef5} className="code w-[45vw]"/>
                  </div>
                </CardContent>
              </Card>
            </CarouselItem>
          </CarouselContent>
          <CarouselPrevious ref={prevSlideRef}/>
          <CarouselNext ref={nextSlideRef}/>
        </Carousel>
      </div>
    </div>
  )
}