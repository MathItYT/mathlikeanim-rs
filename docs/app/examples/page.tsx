"use client";

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from "@/components/ui/carousel";
import init, { InitOutput, SVGScene, Scene, WasmGradientImageOrColor, WasmVectorObject, drawStrokeThenFill, fadeOut, hexToColor, mathjax, smooth } from "mathlikeanim-rs";
import React from "react";
import hljs from "highlight.js";
import { Check, Copy } from "lucide-react";
import Link from "next/link";
import { Button } from "@/components/ui/button";

export default function ExamplesPage() {
  const [slide, setSlide] = React.useState(0);
  const [gotRequired, setGotRequired] = React.useState(false);
  const [isCopied, setIsCopied] = React.useState(false);
  const codeText = `import init, { Scene, WasmGradientImageOrColor, hexToColor, mathjax } from "mathlikeanim-rs";

async function run() {
  let scene = new Scene(BigInt(1920), BigInt(1080), BigInt(60));
  let canvas = document.getElementById("canvas");
  let ctx = canvas.getContext("2d");
  scene.setCanvasContext(ctx);
  scene.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
  let hello = await mathjax("\\\\require{color}\\\\textcolor{#EBEBEB}{\\\\text{Hello, MathLikeAnim-rs!}}");
  hello = hello.scale(1000.0 / hello.getWidth(), true);
  hello = hello.moveTo(960.0, 540.0, true);
  scene.add(hello.clone());
  scene.renderFrame();
}

init().then(run);`;
  const codeText2 = `import init, { SVGScene, WasmGradientImageOrColor, hexToColor, mathjax } from "mathlikeanim-rs";

async function run() {
  let scene = new SVGScene(BigInt(1920), BigInt(1080), BigInt(60));
  let container = document.getElementById("container");
  scene.setDivContainer(container);
  scene.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
  let hello = await mathjax("\\\\textcolor{#EBEBEB}{\\\\text{Hello, MathLikeAnim-rs!}}");
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
} from "mathlikeanim-rs";

// Unfinished example
async function run() {
  let scene = new Scene(BigInt(1920), BigInt(1080), BigInt(60));
  let canvas = document.getElementById("canvas");
  let ctx = canvas.getContext("2d");
  scene.setCanvasContext(ctx);
  scene.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
  const text = "\\\\textcolor{#EBEBEB}{\\\\text{Let's prove that }\\\\sqrt{2}\\\\text{ is irrational!}}";
  let letsProve = await mathjax(text);
  letsProve = letsProve.scale(1000.0 / letsProve.getWidth(), true);
  letsProve = letsProve.moveTo(960.0, 540.0, true);
  scene.add(letsProve.clone());
  await scene.play(
    async (objects, t) => {
      return [drawStrokeThenFill(objects[0].clone(), t)];
    },
    Uint32Array.from([0]),
    BigInt(30),
    t => smooth(t, 10.0)
  );
  await scene.sleep(500);
  await scene.play(
    async (objects, t) => {
      return [fadeOut(objects[0].clone(), 5.0, [0.0, 0.0], t)];
    },
    Uint32Array.from([0]),
    BigInt(30),
    t => smooth(t, 10.0)
  );
  scene.renderFrame();
}

init().then(run);`;
  const [current, setCurrent] = React.useState<HTMLCanvasElement | null>(null);
  const [current2, setCurrent2] = React.useState<HTMLDivElement | null>(null);
  const [current3, setCurrent3] = React.useState<HTMLCanvasElement | null>(null);
  const [out, setOut] = React.useState<InitOutput | null>(null);
  const [scene, setScene] = React.useState<Scene | null>(null);
  const [scene2, setScene2] = React.useState<SVGScene | null>(null);
  const [scene3, setScene3] = React.useState<Scene | null>(null);
  const [ctx, setCtx] = React.useState<CanvasRenderingContext2D | null>(null);
  const [ctx3, setCtx3] = React.useState<CanvasRenderingContext2D | null>(null);
  const [ctxSet3, setCtxSet3] = React.useState(false);
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
  React.useEffect(() => {
    if (!current || !current2 || !current3) return;
    init().then(setOut)
  }, [current, current2, current3]);
  React.useEffect(() => {
    if (!current) return;
    setCtx(current.getContext("2d"));
  }, [current]);
  React.useEffect(() => {
    if (!current3) return;
    setCtx3(current3.getContext("2d"));
  }, [current3]);
  React.useEffect(() => {
    if (!out) return;
    setScene(new Scene(BigInt(1920), BigInt(1080), BigInt(60)));
  }, [out]);
  React.useEffect(() => {
    if (!out) return;
    setScene2(new SVGScene(BigInt(1920), BigInt(1080), BigInt(60)));
  }, [out]);
  React.useEffect(() => {
    if (!out) return;
    setScene3(new Scene(BigInt(1920), BigInt(1080), BigInt(60)));
  }, [out]);
  React.useEffect(() => {
    if (!scene3 || !ctx3) return;
    scene3.setCanvasContext(ctx3);
    scene3.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
    scene3.renderFrame();
    setCtxSet3(true);
  }, [scene3, ctx3]);
  React.useEffect(() => {
    if (!scene || !ctx) return;
    const run = async () => {
      scene.setCanvasContext(ctx);
      scene.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
      let hello = await mathjax("\\require{color}\\textcolor{#EBEBEB}{\\text{Hello, MathLikeAnim-rs!}}");
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
      let hello = await mathjax("\\textcolor{#EBEBEB}{\\text{Hello, MathLikeAnim-rs!}}");
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
      await scene3.play(
        async (objects: WasmVectorObject[], t: number) => {
          return [drawStrokeThenFill(objects[0].clone(), t)];
        },
        Uint32Array.from([0]),
        BigInt(30),
        (t: number) => smooth(t, 10.0)
      );
      await scene3.sleep(500);
      await scene3.play(
        async (objects: WasmVectorObject[], t: number) => {
          return [fadeOut(objects[0].clone(), 5.0, [0.0, 0.0], t)];
        },
        Uint32Array.from([0]),
        BigInt(30),
        (t: number) => smooth(t, 10.0)
      );
      scene3.renderFrame();
    }
    run();
  }, [scene3, gotRequired, slide, ctxSet3]);
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
                    <CardTitle>Mathematical Proof</CardTitle>
                    <CardDescription>
                      An animated example of a mathematical proof using MathLikeAnim-rs.
                    </CardDescription>
                  </CardHeader>
                  <div className="flex justify-center">
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
          </CarouselContent>
          <CarouselPrevious ref={prevSlideRef}/>
          <CarouselNext ref={nextSlideRef}/>
        </Carousel>
      </div>
    </div>
  )
}