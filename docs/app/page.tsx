"use client";

import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { InputOTP, InputOTPGroup, InputOTPSlot } from "@/components/ui/input-otp";
import { Separator } from "@/components/ui/separator";
import { useToast } from "@/components/ui/use-toast";
import useLandscape from "@/lib/use-landscape";
import { MathJax, MathJaxContext } from "better-react-mathjax";
import { REGEXP_ONLY_DIGITS } from "input-otp";
import { RotateCcw } from "lucide-react";
import init, { InitOutput, SVGScene, WasmGradientImageOrColor, WasmGradientStop, WasmLinearGradient, WasmVectorObject, animationGroup, areaUnderCurve, axes, coordsToPoint, create, drawStrokeThenFill, equilateralTriangle, fadeIn, fadeOut, getNumbersTex, hexToColor, interpolate, interpolateTuple, mathjax, morphShape, plotInAxes, rectangle, scaleInPlace, shiftAnimation, smooth, write, addFinalTip, line, showTemporaily, alignData } from "mathlikeanim-rs";
import React, { use } from "react";


function getSubobjectsRecursiveWithPoints(object: WasmVectorObject): WasmVectorObject[] {
  let result = [object];
  for (let subobject of object.getSubobjects()) {
    result = result.concat(getSubobjectsRecursiveWithPoints(subobject));
  }
  result = result.filter((object) => object.getPoints().length > 0);
  return result;
}


export default function Home() {
  const isLandscape = useLandscape();
  const [disabledAnswer, setDisabledAnswer] = React.useState<boolean>(true);
  const [answer, setAnswer] = React.useState<string>("$$\\text{Your answer:}$$");
  const [disabledSubmit, setDisabledSubmit] = React.useState<boolean>(true);
  const [numAnswer, setNumAnswer] = React.useState<number | null>(null);
  const { toast } = useToast();
  const notAnsweredToast = () => {
    toast({
      title: "Please answer the question first!",
      description: "You need to answer the question before continuing.",
      variant: "destructive"
    });
  }
  const handleChange = (value: string) => {
    if (value.length == 0) {
      setAnswer("$$\\text{Your answer:}$$");
      setNumAnswer(null);
      setDisabledSubmit(true);
    } else if (value.length === 1) {
      setAnswer(`$$\\text{Your answer: }${value}$$`);
      setNumAnswer(parseInt(value));
      setDisabledSubmit(false);
    } else if (value[0] === "0") {
      setAnswer(`$$\\text{Your answer: }${value.slice(1)}$$`);
      setNumAnswer(parseInt(value));
      setDisabledSubmit(false);
    } else {
      setAnswer(`$$\\text{Your answer: }${value}$$`);
      setNumAnswer(parseInt(value));
      setDisabledSubmit(false);
    }
  };
  const [current, setCurrent] = React.useState<HTMLDivElement | null>(null);
  const handleAnswered = React.useCallback(() => {
    if (!numAnswer || !current) {
      return;
    }
    if (numAnswer === 32) {
      toast({
        title: 'Correct answer! 😎',
        description: 'Great job! Tap to continue to the next slide',
        duration: 5000
      });
      setDisabledAnswer(true);
      setDisabledSubmit(true);
      current.onmousedown = () => continue4(true);
    } else {
      toast({
        title: 'Incorrect answer! 😞',
        description: 'Please try again or tap to see the explanation.',
        duration: 5000
      });
      current.onmousedown = () => {
        setDisabledAnswer(true);
        setDisabledSubmit(true);
        continue4(false)
      };
    }
  }, [numAnswer, current]);
  const ref = React.useCallback((node: HTMLDivElement | null) => {
    if (node) {
      setCurrent(node);
    }
  }, []);
  const [output, setOutput] = React.useState<InitOutput | null>(null);
  React.useEffect(() => {
    if (current) {
      init().then(setOutput);
    }
  }, [current]);
  const [scene, setScene] = React.useState<SVGScene | null>(null);
  React.useEffect(() => {
    if (output) {
      setScene(new SVGScene(BigInt(1920), BigInt(1080), BigInt(60)));
    }
  }, [output]);
  const continue4 = React.useCallback(async (right: boolean) => {
    if (!scene || !current) {
      return;
    }
    current.onmousedown = null;
    let text;
    if (right) {
      text = await mathjax("\\textcolor{#EBEBEB}{\\text{You got it right! Here's an explanation}}");
    } else {
      text = await mathjax("\\textcolor{#EBEBEB}{\\text{Here's an explanation to help you}}");
    }
    const scaleFactor = 1500.0 / text.getWidth();
    text = text.scale(scaleFactor, true);
    text = text.moveTo(960.0, 540.0, true);
    text = text.setIndex(18);
    let fadeRect = rectangle(
      [960.0, 540.0],
      1920.0,
      1080.0,
      hexToColor("#000000", 0.0),
      hexToColor("#000000", 0.7),
      undefined,
      undefined,
      undefined,
      19
    );
    scene.add(fadeRect.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [
          fadeIn(objects[0].clone(), 1.0, [0.0, 0.0], t)
        ];
      },
      Uint32Array.from([19]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.add(text.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [
          write(objects[0].clone(), 0.4, t)
        ];
      },
      Uint32Array.from([18]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.renderFrame();
    await scene.sleep(500);
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        if (right) {
          return [
            fadeOut(objects[0].clone(), 1.0, [0.0, 0.0], t),
            fadeOut(objects[1].clone(), 1.0, [0.0, 0.0], t)
          ];
        }
        return [
          fadeOut(objects[1].clone(), 1.0, [0.0, 0.0], t),
          fadeOut(objects[0].clone(), 1.0, [0.0, 0.0], t)
        ];
      },
      Uint32Array.from([18, 19]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.remove(18);
    scene.remove(19);
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [
          objects[0].setFillOpacity(1.0 - t, true).setStrokeOpacity(1.0 - t, true),
          objects[1].setFillOpacity(1.0 - t, true).setStrokeOpacity(1.0 - t, true)
        ];
      },
      Uint32Array.from([15, 17]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.remove(15);
    scene.remove(17);
    let rectGroupIndices = [13, 2, 3, 4, 5, 6, 7, 8, 9, 11];
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return objects.map((object) => {
          let result = shiftAnimation(object.clone(), [500.0, 0.0], t);
          if (result.getIndex() === 3 || result.getIndex() === 13) {
            let bottomLeft = result.getCriticalPoint(-1.0, 1.0);
            let topRight = result.getCriticalPoint(1.0, -1.0);
            let lg = WasmGradientImageOrColor.fromLinearGradient(
              new WasmLinearGradient(
                bottomLeft[0],
                bottomLeft[1],
                topRight[0],
                topRight[1],
                [
                  new WasmGradientStop(0.0, hexToColor("#FC6255", 1.0)),
                  new WasmGradientStop(1.0, hexToColor("#FFFF00", 1.0))
                ],
                result.getIndex() === 13 ? 0.7 : 1.0
              )
            );
            if (result.getIndex() === 13) {
              return result.setFill(lg, true);
            } else {
              return result.setStroke(lg, true);
            }
          }
          return result;
        });
      },
      Uint32Array.from(rectGroupIndices),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let ax = scene.getObjectsFromIndices([2]).get(2) as WasmVectorObject;
    let l = line(
      coordsToPoint(ax.clone(), 0.0, 8.0, 0.0, 10.0, 0.0, 10.0),
      coordsToPoint(ax.clone(), 8.0, 8.0, 0.0, 10.0, 0.0, 10.0),
      hexToColor("#EBEBEB", 1.0),
      4.0,
      undefined,
      undefined,
      20
    );
    scene.insert(0, l.clone());
    for (let i = 0; i < 30; i++) {
      const t = smooth(i / 30, 10.0);
      scene.insert(0, create(l.clone(), t));
      scene.renderFrame();
      await scene.sleep(Math.floor(1000 / 60));
    }
    scene.insert(0, l.clone());
    let braceX = await mathjax("\\textcolor{#EBEBEB}{\\Huge\\{}");
    braceX = braceX.rotate(Math.PI / 2, true);
    let diffX = coordsToPoint(ax.clone(), 8.0, 8.0, 0.0, 10.0, 0.0, 10.0)[0] - coordsToPoint(ax.clone(), 0.0, 8.0, 0.0, 10.0, 0.0, 10.0)[0];
    braceX = braceX.stretch(diffX / braceX.getWidth(), 0.5 * diffX / braceX.getWidth(), true);
    braceX = braceX.nextToOther(l.clone(), [0.0, -1.0], 25.0, [0.0, 0.0], true);
    braceX = braceX.setIndex(21);
    scene.add(braceX.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [drawStrokeThenFill(objects[0].clone(), t)];
      },
      Uint32Array.from([21]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let textXLength = await mathjax("\\textcolor{#EBEBEB}{8}");
    textXLength = textXLength.scale(30.0 / textXLength.getHeight(), true);
    textXLength = textXLength.nextToOther(braceX.clone(), [0.0, -1.0], 25.0, [0.0, 0.0], true);
    textXLength = textXLength.setIndex(22);
    scene.add(textXLength.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [fadeIn(objects[0].clone(), 1.5, [0.0, 0.0], t)];
      },
      Uint32Array.from([22]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let area = scene.getObjectsFromIndices([13]).get(13) as WasmVectorObject;
    let braceY = await mathjax("\\textcolor{#EBEBEB}{\\Huge\\}}");
    let diffY = coordsToPoint(ax.clone(), 0.0, 0.0, 0.0, 10.0, 0.0, 10.0)[1] - coordsToPoint(ax.clone(), 0.0, 8.0, 0.0, 10.0, 0.0, 10.0)[1];
    braceY = braceY.stretch(0.5 * diffY / braceY.getHeight(), diffY / braceY.getHeight(), true);
    braceY = braceY.nextToOther(area.clone(), [1.0, 0.0], 25.0, [0.0, 0.0], true);
    braceY = braceY.setIndex(23);
    scene.add(braceY.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [drawStrokeThenFill(objects[0].clone(), t)];
      },
      Uint32Array.from([23]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let textYLength = await mathjax("\\textcolor{#EBEBEB}{8}");
    textYLength = textYLength.scale(30.0 / textYLength.getHeight(), true);
    textYLength = textYLength.nextToOther(braceY.clone(), [1.0, 0.0], 25.0, [0.0, 0.0], true);
    textYLength = textYLength.setIndex(24);
    scene.add(textYLength.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [fadeIn(objects[0].clone(), 1.5, [0.0, 0.0], t)];
      },
      Uint32Array.from([24]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    const areaGroupIndices = [13, 21, 22, 23, 24];
    let areaGroupFromScene = scene.getObjectsFromIndices(areaGroupIndices);
    let areaGroupObjects = [];
    for (let i = 0; i < areaGroupIndices.length; i++) {
      let obj = areaGroupFromScene.get(areaGroupIndices[i]) as WasmVectorObject;
      areaGroupObjects.push(obj);
      scene.remove(areaGroupIndices[i]);
    }
    let areaGroup = new WasmVectorObject().setSubobjects(areaGroupObjects).setIndex(25);
    let fadeRect2 = rectangle(
      [960.0, 540.0],
      1920.0,
      1080.0,
      hexToColor("#000000", 0.0),
      hexToColor("#000000", 0.7),
      undefined,
      undefined,
      undefined,
      26
    );
    scene.add(fadeRect2.clone());
    scene.add(areaGroup.clone());
    let target = areaGroup.scale(1.5, true);
    target = target.moveTo(960.0, 540.0, true);
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        let result = objects.map((object) => {
          if (object.getIndex() === 26) {
            return fadeIn(object.clone(), 1.0, [0.0, 0.0], t);
          }
          let result = morphShape(object.clone(), target.clone(), t);
          let subobjs = result.getSubobjects();
          let areaIndex = subobjs.findIndex((object) => object.getIndex() === 13);
          let area = subobjs[areaIndex];
          let bottomLeft = area.getCriticalPoint(-1.0, 1.0);
          let topRight = area.getCriticalPoint(1.0, -1.0);
          let lg = WasmGradientImageOrColor.fromLinearGradient(
            new WasmLinearGradient(
              bottomLeft[0],
              bottomLeft[1],
              topRight[0],
              topRight[1],
              [
                new WasmGradientStop(0.0, hexToColor("#FC6255", 1.0)),
                new WasmGradientStop(1.0, hexToColor("#FFFF00", 1.0))
              ],
              interpolate(0.7, 1.0, t)
            )
          );
          subobjs[areaIndex] = area.setFill(lg, true);
          return result.setSubobjects(subobjs);
        });
        result = [result.find((object) => object.getIndex() === 26) as WasmVectorObject, result.find((object) => object.getIndex() === 25) as WasmVectorObject];
        return result;
      },
      Uint32Array.from([25, 26]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.renderFrame();
    await scene.sleep(500);
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        let result = shiftAnimation(objects[0].clone(), [-objects[0].getWidth() / 2, 0.0], t)
        let subobjs = result.getSubobjects();
        let areaIndex = subobjs.findIndex((object) => object.getIndex() === 13);
        let area = subobjs[areaIndex];
        let bottomLeft = area.getCriticalPoint(-1.0, 1.0);
        let topRight = area.getCriticalPoint(1.0, -1.0);
        let lg = WasmGradientImageOrColor.fromLinearGradient(
          new WasmLinearGradient(
            bottomLeft[0],
            bottomLeft[1],
            topRight[0],
            topRight[1],
            [
              new WasmGradientStop(0.0, hexToColor("#FC6255", 1.0)),
              new WasmGradientStop(1.0, hexToColor("#FFFF00", 1.0))
            ],
            1.0
          )
        );
        subobjs[areaIndex] = area.setFill(lg, true);
        return [result.setSubobjects(subobjs)];
      },
      Uint32Array.from([25]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let text1 = await mathjax("\\textcolor{#EBEBEB}{\\frac{1}{2}(8)(8)}");
    const factor = 350.0 / text1.getHeight();
    text1 = text1.scale(factor, true);
    let currentAreaGroup = scene.getObjectsFromIndices([25]).get(25) as WasmVectorObject;
    text1 = text1.nextToOther(currentAreaGroup.clone(), [1.0, 0.0], 100.0, [0.0, 0.0], true);
    let subobjects = text1.getSubobjects();
    let firstEight = subobjects.splice(3, 1)[0];
    let secondEight = subobjects.splice(5, 1)[0];
    text1 = new WasmVectorObject().setSubobjects(subobjects).setIndex(27);
    let eightX = (currentAreaGroup.getSubobjects().find((object) => object.getIndex() === 22) as WasmVectorObject).clone();
    let eightY = (currentAreaGroup.getSubobjects().find((object) => object.getIndex() === 24) as WasmVectorObject).clone();
    eightX = eightX.setIndex(28);
    eightY = eightY.setIndex(29);
    scene.add(text1.clone());
    let aligned3 = alignData(eightX.clone(), firstEight.clone(), false, [960.0, 540.0]);
    let aligned4 = alignData(eightY.clone(), secondEight.clone(), false, [960.0, 540.0]);
    scene.add((aligned3[0] as WasmVectorObject).clone());
    scene.add((aligned4[0] as WasmVectorObject).clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return objects.map((object) => {
          if (object.getIndex() === 27) {
            return write(object.clone(), 0.4, t);
          }
          if (object.getIndex() === 28) {
            return morphShape(object.clone(), (aligned3[1] as WasmVectorObject).clone(), t);
          }
          return morphShape(object.clone(), (aligned4[1] as WasmVectorObject).clone(), t);
        });
      },
      Uint32Array.from([27, 28, 29]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.remove(28);
    scene.remove(29);
    text1 = new WasmVectorObject().setSubobjects([text1.clone(), firstEight.clone(), secondEight.clone()]);
    text1 = new WasmVectorObject().setSubobjects(getSubobjectsRecursiveWithPoints(text1.clone())).setIndex(27);
    let text2 = await mathjax("\\textcolor{#EBEBEB}{\\frac{1}{2}(64)}");
    text2 = text2.scale(factor, true);
    const text1Center = text1.getCenter();
    text2 = text2.moveTo(text1Center[0], text1Center[1], true);
    text2 = text2.setIndex(27);
    let aligned = alignData(text1.clone(), text2.clone(), false, [960.0, 540.0]);
    scene.add((aligned[0] as WasmVectorObject).clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [morphShape(objects[0].clone(), (aligned[1] as WasmVectorObject).clone(), t)];
      },
      Uint32Array.from([27]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.add(text2.clone());
    let text3 = await mathjax("\\textcolor{#EBEBEB}{32}");
    text3 = text3.scale(factor, true);
    text3 = text3.moveTo(text1Center[0], text1Center[1], true);
    text3 = text3.setIndex(27);
    let aligned2 = alignData(text2.clone(), text3.clone(), false, [960.0, 540.0]);
    scene.add((aligned2[0] as WasmVectorObject).clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [morphShape(objects[0].clone(), (aligned2[1] as WasmVectorObject).clone(), t)];
      },
      Uint32Array.from([27]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.add(text3.clone());
    let surroundingRectangle = rectangle(
      text3.getCenter(),
      text3.getWidth() + 20.0,
      text3.getHeight() + 20.0,
      hexToColor("#FFFF00", 1.0),
      hexToColor("#FFFF00", 0.0),
      8.0,
      undefined,
      undefined,
      30
    );
    scene.add(surroundingRectangle.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [showTemporaily(objects[0].clone(), t)];
      },
      Uint32Array.from([30]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.remove(30);
    toast({
      title: "Lesson completed! 🎉",
      description: "This is how MathLikeAnim-rs looks like.",
      variant: "success"
    });
    scene.renderFrame();
  }, [scene, current]);
  const continue3 = React.useCallback(async () => {
    if (!scene || !current) {
      return;
    }
    current.onmousedown = null;
    for (let i = 0; i < 30; i++) {
      const t = smooth(i / 30, 10.0);
      const newTop = interpolate(1080.0, 0.0, t);
      const newBottom = interpolate(2160.0, 1080.0, t);
      scene.setTopLeftCorner(0.0, newTop);
      scene.setBottomRightCorner(1920.0, newBottom);
      scene.renderFrame();
      await scene.sleep(Math.floor(1000 / 60));
    }
    scene.setTopLeftCorner(0.0, 0.0);
    scene.setBottomRightCorner(1920.0, 1080.0);
    scene.renderFrame();
    scene.remove(12);
    let axes = scene.getObjectsFromIndices([2]).get(2) as WasmVectorObject;
    let plot = scene.getObjectsFromIndices([3]).get(3) as WasmVectorObject;
    let area = areaUnderCurve(
      axes.clone(),
      plot.clone(),
      0.0,
      10.0,
      0.0,
      10.0,
      0.01,
      8.0,
      undefined,
      13
    );
    let areaBottomLeft = area.getCriticalPoint(-1.0, 1.0);
    let areaTopRight = area.getCriticalPoint(1.0, -1.0);
    area = area.setFill(WasmGradientImageOrColor.fromLinearGradient(
      new WasmLinearGradient(
        areaBottomLeft[0],
        areaBottomLeft[1],
        areaTopRight[0],
        areaTopRight[1],
        [
          new WasmGradientStop(0.0, hexToColor("#FC6255", 1.0)),
          new WasmGradientStop(1.0, hexToColor("#FFFF00", 1.0))
        ],
        0.7
      )
    ), true);
    for (let i = 0; i < 30; i++) {
      const t = smooth(i / 30, 10.0);
      scene.insert(0, fadeIn(area.clone(), 1.5, [0.0, 0.0], t));
      scene.renderFrame();
      await scene.sleep(Math.floor(1000 / 60));
    }
    scene.insert(0, area.clone());
    let tri = equilateralTriangle(
      [0.0, 0.0],
      40.0,
      hexToColor("#FFFF00", 0.0),
      hexToColor("#FFFF00", 1.0),
      undefined,
      undefined,
      undefined,
      14
    ).rotate(-Math.PI / 2, true);
    let ax = scene.getObjectsFromIndices([2]).get(2) as WasmVectorObject;
    tri = tri.nextToPoint(coordsToPoint(axes.clone(), 0.0, 0.0, 0.0, 10.0, 0.0, 10.0), [0.0, 1.0], 0.0, [0.0, 0.0], true);
    scene.add(tri.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [fadeIn(objects[0].clone(), 1.5, [0.0, 0.0], t)];
      },
      Uint32Array.from([14]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [objects[0].nextToPoint(coordsToPoint(ax.clone(), interpolate(0.0, 8.0, t), 0.0, 0.0, 10.0, 0.0, 10.0), [0.0, 1.0], 0.0, [0.0, 0.0], true)];
      },
      Uint32Array.from([14]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [fadeOut(objects[0].clone(), 1.0, [0.0, 0.0], t)];
      },
      Uint32Array.from([14]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let fadeRect = rectangle(
      [960.0, 540.0],
      1920.0,
      1080.0,
      hexToColor("#000000", 0.0),
      hexToColor("#000000", 0.7),
      undefined,
      undefined,
      undefined,
      15
    );
    scene.add(fadeRect.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [
          fadeIn(objects[0].clone(), 1.0, [0.0, 0.0], t),
          objects[1].clone()
        ];
      },
      Uint32Array.from([1, 15]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.renderFrame();
    await scene.sleep(500);
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        let oldCenter = objects[0].getCenter();
        let result = objects[0].scale(interpolate(1.0, 1000.0 / objects[0].getWidth(), t), true);
        let newCenter = interpolateTuple(oldCenter, [960.0, 540.0], t);
        return [result.moveTo(newCenter[0], newCenter[1], true)];
      },
      Uint32Array.from([1]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let oldIntegral = scene.getObjectsFromIndices([1]).get(1) as WasmVectorObject;
    let newIntegral = await mathjax("\\textcolor{#EBEBEB}{\\int_{0}^{8}}\\textcolor{#FFFF00}{x}\\textcolor{#EBEBEB}{\\,d}\\textcolor{#FFFF00}{x}");
    newIntegral = newIntegral.scale(oldIntegral.getHeight() / newIntegral.getHeight(), true);
    newIntegral = newIntegral.moveTo(960.0, 540.0, true);
    newIntegral = newIntegral.setIndex(1);
    let aligned = alignData(oldIntegral.clone(), newIntegral.clone(), false, [960.0, 540.0]);
    scene.add((aligned[0] as WasmVectorObject).clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [morphShape(objects[0].clone(), (aligned[1] as WasmVectorObject).clone(), t)];
      },
      Uint32Array.from([1]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.add(newIntegral.clone());
    let target = newIntegral.scale(1 / 5, true);
    let title = scene.getObjectsFromIndices([0]).get(0) as WasmVectorObject;
    target = target.nextToOther(title.clone(), [0.0, 1.0], 25.0, [0.0, 0.0], true);
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [
          fadeOut(objects[0].clone(), 1.0, [0.0, 0.0], t),
          morphShape(objects[1].clone(), target.clone(), t),
        ];
      },
      Uint32Array.from([1, 15]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let rectGroupIndices = [13, 2, 3, 4, 5, 6, 7, 8, 9, 11];
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return objects.map((object) => {
          let result = shiftAnimation(object.clone(), [-500.0, 0.0], t);
          if (result.getIndex() === 3 || result.getIndex() === 13) {
            let bottomLeft = result.getCriticalPoint(-1.0, 1.0);
            let topRight = result.getCriticalPoint(1.0, -1.0);
            let lg = WasmGradientImageOrColor.fromLinearGradient(
              new WasmLinearGradient(
                bottomLeft[0],
                bottomLeft[1],
                topRight[0],
                topRight[1],
                [
                  new WasmGradientStop(0.0, hexToColor("#FC6255", 1.0)),
                  new WasmGradientStop(1.0, hexToColor("#FFFF00", 1.0))
                ],
                result.getIndex() === 13 ? 0.7 : 1.0
              )
            );
            if (result.getIndex() === 13) {
              return result.setFill(lg, true);
            } else {
              return result.setStroke(lg, true);
            }
          }
          return result;
        });
      },
      Uint32Array.from(rectGroupIndices),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let rect = scene.getObjectsFromIndices([8]).get(8) as WasmVectorObject;
    area = scene.getObjectsFromIndices([13]).get(13) as WasmVectorObject;
    const rectRight = rect.getCriticalPoint(1.0, 0.0);
    const areaRight = area.getCriticalPoint(1.0, 0.0);
    let a = line(
      [rectRight[0] + 20.0, areaRight[1]],
      [areaRight[0] + 20.0, areaRight[1]],
      hexToColor("#EBEBEB", 1.0),
      4.0,
      undefined,
      undefined,
      16
    );
    a = addFinalTip(a.clone(), 20.0, hexToColor("#EBEBEB", 1.0));
    let subobjects = getSubobjectsRecursiveWithPoints(a.clone());
    a = new WasmVectorObject().setSubobjects(subobjects).setIndex(15);
    scene.add(a.clone());
    let animations = [];
    for (let i = 0; i < a.getSubobjects().length; i++) {
      animations.push(create);
    }
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [animationGroup(objects[0].clone(), animations, 1.0, t)];
      },
      Uint32Array.from([15]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let text1 = await mathjax("\\textcolor{#EBEBEB}{\\text{This area is a triangle!}}");
    let text2 = await mathjax("\\textcolor{#EBEBEB}{\\text{Tap the ``Answer Question'' button below.}}");
    let text3 = await mathjax("\\textcolor{#EBEBEB}{\\text{When you're ready, tap here to continue.}}");
    let text = new WasmVectorObject().setSubobjects([text1, text2, text3]).setIndex(17);
    text = text.scale(1000.0 / text.getWidth(), true);
    text = text.arrangeSubobjects([0.0, 1.0], 15.0, [0.0, 0.0], true);
    text = text.nextToOther(a.clone(), [1.0, 0.0], 25.0, [0.0, 0.0], true);
    scene.add(text.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        let center = objects[0].getCenter();
        let object = objects[0].setFillOpacity(t, true);
        object = object.scale(interpolate(1.5, 1.0, t), true);
        return [object.moveTo(center[0], center[1], true)];
      },
      Uint32Array.from([17]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.renderFrame();
    setDisabledAnswer(false);
    current.onmousedown = notAnsweredToast;
  }, [scene, current]);

  const continue2 = React.useCallback(async () => {
    if (!scene || !current) {
      return;
    }
    current.onmousedown = null;
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [
          fadeOut(objects[0].clone(), 1.0, [0.0, 0.0], t),
          fadeOut(objects[1].clone(), 1.5, [0.0, 0.0], t)
        ];
      },
      Uint32Array.from([9, 10]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.renderFrame();
    let target = scene.getObjectsFromIndices([1]).get(1) as WasmVectorObject;
    let title = scene.getObjectsFromIndices([0]).get(0) as WasmVectorObject;
    target = target.scale(0.5, true);
    target = target.nextToOther(title.clone(), [0.0, 1.0], 25.0, [0.0, 0.0], true);
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [morphShape(objects[0].clone(), target.clone(), t)];
      },
      Uint32Array.from([1]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let rect = scene.getObjectsFromIndices([8]).get(8) as WasmVectorObject;
    let f_x = await mathjax("\\textcolor{#EBEBEB}{f(}\\textcolor{#FFFF00}{x}\\textcolor{#EBEBEB}{)}\\textcolor{#EBEBEB}{=}\\textcolor{#FFFF00}{x}");
    f_x = f_x.scale(200.0 / f_x.getWidth(), true);
    f_x = f_x.nextToOther(rect.clone(), [0.0, -1.0], 25.0, [0.0, 0.0], true);
    f_x = f_x.setIndex(11);
    scene.add(f_x.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [write(objects[0].clone(), 0.4, t)];
      },
      Uint32Array.from([11]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    for (let i = 0; i < 30; i++) {
      const t = smooth(i / 30, 10.0);
      const newTop = interpolate(0.0, 1080.0, t);
      const newBottom = interpolate(1080.0, 2160.0, t);
      scene.setTopLeftCorner(0.0, newTop);
      scene.setBottomRightCorner(1920.0, newBottom);
      scene.renderFrame();
      await scene.sleep(Math.floor(1000 / 60));
    }
    scene.setTopLeftCorner(0.0, 1080.0);
    scene.setBottomRightCorner(1920.0, 2160.0);
    let intuitiveDefPart1 = await mathjax("\\textcolor{#EBEBEB}{\\text{The \\textbf{definite integral} of a function }f(}\\textcolor{#FFFF00}{x}\\textcolor{#EBEBEB}{)}");
    let intuitiveDefPart2 = await mathjax("\\textcolor{#EBEBEB}{\\text{from }a\\text{ to }b\\text{ is the area under the curve}}");
    let intuitiveDefPart = await mathjax("\\textcolor{#EBEBEB}{\\text{of }f(}\\textcolor{#FFFF00}{x}\\textcolor{#EBEBEB}{)\\text{ between }}\\textcolor{#FFFF00}{x}\\textcolor{#EBEBEB}{=a}\\text{ and }\\textcolor{#FFFF00}{x}\\textcolor{#EBEBEB}{=b}");
    let intuitiveDef = new WasmVectorObject();
    intuitiveDef = intuitiveDef.setSubobjects([
      intuitiveDefPart1,
      intuitiveDefPart2,
      intuitiveDefPart
    ]);
    intuitiveDef = intuitiveDef.arrangeSubobjects(
      [0.0, 1.0],
      25.0 * intuitiveDef.getWidth() / 1500.0,
      [0.0, 0.0],
      true
    );
    intuitiveDef = intuitiveDef.scale(1500.0 / intuitiveDef.getWidth(), true);
    intuitiveDef = intuitiveDef.moveTo(960.0, 1620.0, true);
    intuitiveDef = new WasmVectorObject().setSubobjects(getSubobjectsRecursiveWithPoints(intuitiveDef.clone()));
    let animations = [];
    for (let i = 0; i < intuitiveDef.getSubobjects().length; i++) {
      animations.push((object: WasmVectorObject, t: number) => fadeIn(object.clone(), 1.5, [0.0, 0.0], t));
    }
    intuitiveDef = intuitiveDef.setIndex(12);
    scene.add(intuitiveDef.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [animationGroup(objects[0].clone(), animations, 0.4, t)];
      },
      Uint32Array.from([12]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.renderFrame();
    await scene.sleep(500);
    let text1 = await mathjax("\\textcolor{#EBEBEB}{\\text{We'll explain better in the next slide}}");
    const scaleFactor = 1000.0 / text1.getWidth();
    text1 = text1.scale(scaleFactor, true);
    text1 = text1.nextToOther(intuitiveDef.clone(), [0.0, 1.0], 100.0, [0.0, 0.0], true);
    let text2 = await mathjax("\\textcolor{#EBEBEB}{\\text{(Tap here to continue)}}");
    text2 = text2.scale(scaleFactor, true);
    text2 = text2.nextToOther(text1.clone(), [0.0, 1.0], 25.0, [0.0, 0.0], true);
    let text = new WasmVectorObject().setSubobjects([text1, text2]).setIndex(13);
    text = scaleInPlace(text.clone(), 1.5, 1.0);
    scene.add(text.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [(scaleInPlace(objects[0].clone(), 1 / 1.5, t) as WasmVectorObject).setFillOpacity(t, true)];
      },
      Uint32Array.from([13]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.renderFrame();
    current.onmousedown = continue3;
  }, [scene, current]);
  const continue1 = React.useCallback(async () => {
    if (!scene || !current) {
      return;
    }
    current.onmousedown = null;
    let objects = scene.getObjectsFromIndices([0, 1]);
    let target = (objects.get(1) as WasmVectorObject).clone();
    target = target.scale(0.5, true);
    target = target.nextToOther(objects.get(0) as WasmVectorObject, [0.0, 1.0], 25.0, [0.0, 0.0], true);
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [
          fadeOut(objects[0].clone(), 1.0, [0.0, 0.0], t),
          morphShape(objects[1].clone(), target.clone(), t),
          fadeOut(objects[2].clone(), 1.0, [0.0, 0.0], t)
        ];
      },
      Uint32Array.from([1, 2, 3]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let ax = axes(
      0.0,
      10.0,
      1.0,
      0.0,
      10.0,
      1.0,
      [960.0, 740.0],
      500.0,
      500.0,
      hexToColor("#EBEBEB", 1.0),
      4.0,
      undefined,
      undefined,
      2,
      false,
      false,
      0.0,
      0.0,
      true,
      true
    );
    scene.add(ax.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [write(objects[0].clone(), 0.4, t)];
      },
      Uint32Array.from([2]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let plot = plotInAxes(
      (x: number) => x,
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
      undefined,
      undefined,
      3
    );
    const p1 = plot.getCriticalPoint(-1.0, 1.0);
    const p2 = plot.getCriticalPoint(1.0, -1.0);
    plot = plot.setStroke(WasmGradientImageOrColor.fromLinearGradient(
      new WasmLinearGradient(
        p1[0],
        p1[1],
        p2[0],
        p2[1],
        [
          new WasmGradientStop(0.0, hexToColor("#FC6255", 1.0)),
          new WasmGradientStop(1.0, hexToColor("#FFFF00", 1.0))
        ],
        1.0
      )
    ), true);
    scene.add(plot);
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [create(objects[0].clone(), t)];
      },
      Uint32Array.from([3]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let axSubobjects = ax.getSubobjects();
    let xAxis = axSubobjects[axSubobjects.length - 2];
    let yAxis = axSubobjects[axSubobjects.length - 1];
    let xNumbers = await getNumbersTex(
      xAxis.clone(),
      [2.0, 4.0, 6.0, 8.0],
      async (x: number) => await mathjax(`\\textcolor{#EBEBEB}{${x}}`),
      0.0,
      10.0,
      30.0,
      [0.0, 1.0],
      10.0,
      4
    );
    let yNumbers = await getNumbersTex(
      yAxis.clone(),
      [2.0, 4.0, 6.0, 8.0],
      async (y: number) => await mathjax(`\\textcolor{#EBEBEB}{${y}}`),
      0.0,
      10.0,
      30.0,
      [-1.0, 0.0],
      10.0,
      5
    );
    let xLabel = await mathjax("\\textcolor{#FFFF00}{x}");
    let yLabel = await mathjax("\\textcolor{#EBEBEB}{f(}\\textcolor{#FFFF00}{x}\\textcolor{#EBEBEB}{)}");
    const factor = 30.0 / xLabel.getHeight();
    xLabel = xLabel.scale(factor, true);
    xLabel = xLabel.nextToOther(xAxis.clone(), [1.0, 0.0], 25.0, [0.0, 0.0], true);
    xLabel = xLabel.setIndex(6);
    yLabel = yLabel.scale(factor, true);
    yLabel = yLabel.nextToOther(yAxis.clone(), [0.0, -1.0], 25.0, [0.0, 0.0], true);
    yLabel = yLabel.setIndex(7);
    let animations = [];
    for (let i = 0; i < xNumbers.getSubobjects().length; i++) {
      animations.push((object: WasmVectorObject, t: number) => fadeIn(object.clone(), 1.5, [0.0, 0.0], t));
    }
    scene.add(xLabel.clone());
    scene.add(yLabel.clone());
    scene.add(xNumbers.clone());
    scene.add(yNumbers.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [
          animationGroup(objects[0].clone(), animations, 0.4, t),
          fadeIn(objects[1].clone(), 1.5, [0.0, 0.0], t),
          animationGroup(objects[2].clone(), animations, 0.4, t),
          fadeIn(objects[3].clone(), 1.5, [0.0, 0.0], t)
        ];
      },
      Uint32Array.from([4, 5, 6, 7]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let figure = new WasmVectorObject();
    const indices = [2, 3, 4, 5, 6, 7];
    let figureSubobjectsUntyped = scene.getObjectsFromIndices(indices);
    let figureSubobjects = indices.map((index) => (figureSubobjectsUntyped.get(index) as WasmVectorObject).clone());
    figure = figure.setSubobjects(
      figureSubobjects
    );
    let rect = rectangle(
      figure.getCenter(),
      figure.getWidth() + 20.0,
      figure.getHeight() + 20.0,
      hexToColor("#EBEBEB", 1.0),
      hexToColor("#EBEBEB", 0.0),
      4.0,
      undefined,
      undefined,
      8
    );
    scene.add(rect.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [create(objects[0].clone(), t)];
      },
      Uint32Array.from([8]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let fadeRect = rectangle(
      [960.0, 540.0],
      1920.0,
      1080.0,
      hexToColor("#000000", 0.0),
      hexToColor("#000000", 0.7),
      undefined,
      undefined,
      undefined,
      9
    );
    scene.add(fadeRect.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [fadeIn(objects[0].clone(), 1.0, [0.0, 0.0], t)];
      },
      Uint32Array.from([9]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let tap = await mathjax("\\textcolor{#EBEBEB}{\\text{Tap here to continue}}");
    tap = tap.scale(800.0 / tap.getWidth(), true);
    tap = tap.moveTo(960.0, 540.0, true);
    tap = tap.setIndex(10);
    scene.add(tap.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [fadeIn(objects[0].clone(), 1.5, [0.0, 0.0], t)];
      },
      Uint32Array.from([10]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    current.onmousedown = continue2;
    scene.renderFrame();
  }, [scene, current]);
  const start = React.useCallback(async () => {
    if (!scene || !current) {
      return;
    }
    current.onmousedown = null;
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [fadeOut(objects[0].clone(), 1.0, [0.0, 0.0], t)];
      },
      Uint32Array.from([0]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let title = await mathjax("\\textcolor{#EBEBEB}{\\text{Understanding integrals}}");
    title = title.scale(1000.0 / title.getWidth(), true);
    title = title.moveTo(960.0, 540.0, true);
    const titleFirstCenter = title.getCenter();
    scene.add(title.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [drawStrokeThenFill(objects[0].clone(), t)];
      },
      Uint32Array.from([0]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.renderFrame();
    await scene.sleep(500);
    const titleTopCenter = title.nextToPoint([960.0, 0.0], [0.0, 1.0], 25.0, [0.0, 0.0], true).getCenter();
    const shift = [titleTopCenter[0] - titleFirstCenter[0], titleTopCenter[1] - titleFirstCenter[1]];
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [shiftAnimation(objects[0].clone(), shift, t)];
      },
      Uint32Array.from([0]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let integral = await mathjax("\\textcolor{#EBEBEB}{\\int_{a}^{b}f(}\\textcolor{#FFFF00}{x}\\textcolor{#EBEBEB}{)\\,d}\\textcolor{#FFFF00}{x}");
    integral = integral.scale(750.0 / integral.getWidth(), true);
    integral = integral.moveTo(960.0, 540.0, true);
    integral = integral.setIndex(1);
    let animations = [];
    for (let i = 0; i < integral.getSubobjects().length; i++) {
      animations.push((object: WasmVectorObject, t: number) => fadeIn(object.clone(), 1.5, [0.0, 0.0], t));
    }
    scene.add(integral.clone());
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [animationGroup(objects[0].clone(), animations, 0.4, t)];
      },
      Uint32Array.from([1]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    scene.renderFrame();
    await scene.sleep(500);
    let description = await mathjax("\\textcolor{#EBEBEB}{\\text{This is a \\textbf{definite} integral.}}");
    description = description.scale(625.0 / description.getWidth(), true);
    description = description.nextToPoint([960.0, 1080.0], [0.0, -1.0], 50.0, [0.0, 0.0], true);
    description = description.setIndex(2);
    scene.add(description);
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [write(objects[0].clone(), 0.4, t)];
      },
      Uint32Array.from([2]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    let tap = await mathjax("\\textcolor{#EBEBEB}{\\text{(Tap here to continue)}}");
    tap = tap.scale(800.0 / tap.getWidth(), true);
    tap = tap.nextToOther(integral, [0.0, 1.0], 75.0, [0.0, 0.0], true);
    tap = tap.setIndex(3);
    scene.add(tap);
    await scene.play(
      async (objects: WasmVectorObject[], t: number) => {
        return [fadeIn(objects[0].clone(), 1.5, [0.0, 0.0], t)];
      },
      Uint32Array.from([3]),
      BigInt(30),
      (t: number) => smooth(t, 10.0)
    );
    current.onmousedown = continue1;
    scene.renderFrame();
  }, [scene, current]);
  React.useEffect(() => {
    if (scene && current) {
      const run = async () => {
        scene.setDivContainer(current);
        scene.setBackground(WasmGradientImageOrColor.fromColor(hexToColor("#161616", 1.0)));
        let text = await mathjax("\\require{color}\\textcolor{#EBEBEB}{\\text{Tap here to start}}");
        text = text.scale(1000.0 / text.getWidth(), true);
        text = text.moveTo(960.0, 540.0, true);
        scene.add(text);
        scene.renderFrame();
        current.onmousedown = start;
      };
      run();
    }
  }, [scene, current]);
  return (
    <>
      { !isLandscape ?
      <div className="flex flex-col justify-center items-center h-[80vh]">
        <RotateCcw className="w-56 h-56"/>
        <p className="text-center font-bold text-[5vw]">
          Please rotate your device
          <br/>
          to landscape (horizontal) mode
        </p>
      </div> : null }
      { isLandscape ?
      <>
        <h1 className="text-[3vw] font-bold text-center">
          Imagine having an interactive math lesson like this
        </h1>
        <div className="flex flex-col justify-center items-center h-[50vw]">
          <div ref={ref} className="container flex justify-center pt-[3vh] pb-3"/>
          <div className="flex flex-col justify-center">
            <Dialog>
              <div className="flex justify-center">
                <Button variant="outline" disabled={disabledAnswer} className="w-40" asChild>
                  <DialogTrigger>Answer Question</DialogTrigger>
                </Button>
              </div>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Question</DialogTitle>
                  <DialogDescription>
                    Select the correct answer
                  </DialogDescription>
                </DialogHeader>
                <MathJaxContext>
                  <b>What&apos;s the result of the following expression?</b>
                  <div className="flex justify-center">
                    <MathJax>
                      {'$$\\int_{0}^{8}x\\,dx$$'}
                    </MathJax>
                  </div>
                  <p><b>Hint</b>: Consider the base length and height of the triangle</p>
                  <Separator />
                  <div className="flex justify-center">
                    <InputOTP
                      maxLength={2}
                      minLength={2}
                      onChange={handleChange}
                      pattern={REGEXP_ONLY_DIGITS}
                    >
                      <InputOTPGroup>
                        <InputOTPSlot index={0}/>
                        <InputOTPSlot index={1}/>
                      </InputOTPGroup>
                    </InputOTP>
                  </div>
                  <div></div>
                  <MathJax dynamic>{answer}</MathJax>
                  <DialogFooter>
                    <Button disabled={disabledSubmit} onClick={handleAnswered}>Submit</Button>
                  </DialogFooter>
                </MathJaxContext>
              </DialogContent>
            </Dialog>
          </div>
        </div>
      </> : null }
    </>
  );
}
