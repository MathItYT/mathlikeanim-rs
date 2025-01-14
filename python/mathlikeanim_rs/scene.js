import init, {
    Scene,
    SVGScene,
    WasmVectorObject,
    WasmGradientImageOrColor,
    WasmColor,
    WasmGradientStop,
    WasmLinearGradient,
    WasmRadialGradient,
    WasmImage,
    circle,
    addFinalTip,
    addInitialTip,
    addBothSidesTips,
    arc,
    ellipticalArc,
    ellipse,
    annularSector,
    dashedObject,
    line,
    polygon,
    regularPolygon,
    square,
    rectangle,
    equilateralTriangle,
    triangle,
    rightTriangle,
    axes,
    coordsToPoint,
    parametricPlotInAxes,
    plotInAxes,
    contourPlotInAxes,
    areaUnderCurve,
    riemannRectanglesForPlot,
    secantLineForPlot,
    parametricFunction,
    realFunction,
    numberLine,
    numberToPoint,
    pointToNumber,
    getNumbersTex,
    WasmThreeDObject,
    mathjax,
    svgToVector,
    textToVector,
    sphere,
    threeDAxes,
    coordsToPoint3D,
    pointToCoords3D,
    parametricPlotInAxes3D,
    plotInAxes3D,
    parametricLinePlotInAxes3D,
    create,
    drawStrokeThenFill,
    fadeIn,
    fadeOut,
    growArrowWithFinalTip,
    growArrowWithInitialTip,
    growArrowWithTipsAtBothEnds,
    growFromCenter,
    morphShape,
    rotateAnimation,
    scaleInPlace,
    setFillAnimation,
    setStrokeAnimation,
    shiftAnimation,
    showTemporaily,
    spinningGrow,
    createAxes3D,
    create3D,
    drawStrokeThenFill3D,
    fadeIn3D,
    fadeOut3D,
    growFromCenter3D,
    morphShape3D,
    rotateXAnimation3D,
    rotateYAnimation3D,
    rotateZAnimation3D,
    scaleInPlace3D,
    setFillAnimation3D,
    setStrokeAnimation3D,
    shiftAnimation3D,
    WasmCamera,
    WasmLightSource,
    contourPlot,
    linear,
    projectPoints,
} from 'mathlikeanim-rs';

const colorToJson = (obj) => {
    return {
        type: 'color',
        r: obj.getR(),
        g: obj.getG(),
        b: obj.getB(),
        a: obj.getA(),
    };
};

const gradientStopToJson = (obj) => {
    return {
        type: 'gradientStop',
        offset: obj.getOffset(),
        color: colorToJson(obj.getColor()),
    };
};

const linearGradientToJson = (obj) => {
    return {
        type: 'linearGradient',
        x1: obj.getX1(),
        y1: obj.getY1(),
        x2: obj.getX2(),
        y2: obj.getY2(),
        stops: obj.getStops().map(gradientStopToJson),
        alpha: obj.getAlpha(),
    };
};

const radialGradientToJson = (obj) => {
    return {
        type: 'radialGradient',
        cx: obj.getCx(),
        cy: obj.getCy(),
        r: obj.getR(),
        fx: obj.getFx(),
        fy: obj.getFy(),
        stops: obj.getStops().map(gradientStopToJson),
        alpha: obj.getAlpha(),
    };
};

const imageToJson = (obj) => {
    return {
        type: 'image',
        imageBase64: obj.getImageBase64(),
        mimeType: obj.getMimeType(),
        top: obj.getTop(),
        left: obj.getLeft(),
        bottom: obj.getBottom(),
        right: obj.getRight(),
        alpha: obj.getAlpha(),
    };
};

const gradientImageOrColorToJson = (obj) => {
    if (obj.isLinearGradient()) {
        return {
            type: 'gradientImageOrColor',
            gradientImageOrColor: linearGradientToJson(obj.getLinearGradient()),
        };
    } else if (obj.isRadialGradient()) {
        return {
            type: 'gradientImageOrColor',
            gradientImageOrColor: radialGradientToJson(obj.getRadialGradient()),
        };
    } else if (obj.isImage()) {
        return {
            type: 'gradientImageOrColor',
            gradientImageOrColor: imageToJson(obj.getImage()),
        };
    } else {
        return {
            type: 'gradientImageOrColor',
            gradientImageOrColor: colorToJson(obj.getColor()),
        };
    }
};

const vectorObjectToJson = (obj) => {
    return {
        type: 'vectorObject',
        points: obj.getPoints(),
        fill: gradientImageOrColorToJson(obj.getFill()),
        fillRule: obj.getFillRule(),
        stroke: gradientImageOrColorToJson(obj.getStroke()),
        strokeWidth: obj.getStrokeWidth(),
        lineCap: obj.getLineCap(),
        lineJoin: obj.getLineJoin(),
        subobjects: obj.getSubobjects().map(vectorObjectToJson),
        index: obj.getIndex(),
    };
};

const cameraToJson = (obj) => {
    return {
        type: 'camera',
        position: obj.getPosition(),
        rotation: obj.getRotation(),
        focalDistance: obj.getFocalDistance(),
        zoom: obj.getZoom(),
    };
};

const jsonToCamera = (obj) => {
    return new WasmCamera(
        obj.position,
        obj.rotation,
        obj.focalDistance,
        obj.zoom,
    );
};

const lightSourceToJson = (obj) => {
    return {
        type: 'lightSource',
        position: obj.getPosition(),
    };
};

const jsonToLightSource = (obj) => {
    return new WasmLightSource(obj.position);
};

const jsonToColor = (obj) => {
    return new WasmColor(obj.r, obj.g, obj.b, obj.a);
}

const jsonToGradientStop = (obj) => {
    return new WasmGradientStop(obj.offset, jsonToColor(obj.color));
}

const jsonToLinearGradient = (obj) => {
    return new WasmLinearGradient(
        obj.x1, obj.y1, obj.x2, obj.y2,
        obj.stops.map(jsonToGradientStop),
        obj.alpha,
    );
}

const jsonToRadialGradient = (obj) => {
    return new WasmRadialGradient(
        obj.cx, obj.cy, obj.r, obj.fx, obj.fy,
        obj.stops.map(jsonToGradientStop),
        obj.alpha,
    );
}

const jsonToImage = (obj) => {
    return new WasmImage(
        obj.imageBase64, obj.mimeType,
        obj.top, obj.left, obj.bottom, obj.right,
        obj.alpha,
    );
};

const jsonToGradientImageOrColor = (obj) => {
    if (obj.gradientImageOrColor.type === 'linearGradient') {
        return WasmGradientImageOrColor.fromLinearGradient(jsonToLinearGradient(obj.gradientImageOrColor));
    } else if (obj.gradientImageOrColor.type === 'radialGradient') {
        return WasmGradientImageOrColor.fromRadialGradient(jsonToRadialGradient(obj.gradientImageOrColor));
    } else if (obj.gradientImageOrColor.type === 'image') {
        return WasmGradientImageOrColor.fromImage(jsonToImage(obj.gradientImageOrColor));
    } else {
        return WasmGradientImageOrColor.fromColor(jsonToColor(obj.gradientImageOrColor));
    }
};

const jsonToVectorObject = (obj) => {
    return new WasmVectorObject()
        .setPoints(obj.points)
        .setFill(jsonToGradientImageOrColor(obj.fill), false)
        .setFillRule(obj.fillRule, false)
        .setStroke(jsonToGradientImageOrColor(obj.stroke), false)
        .setStrokeWidth(obj.strokeWidth, false)
        .setLineCap(obj.lineCap, false)
        .setLineJoin(obj.lineJoin, false)
        .setSubobjects(obj.subobjects.map(jsonToVectorObject))
        .setIndex(obj.index);
};

const objectMapToJson = (obj) => {
    const map = {};
    obj.forEach((value, key) => {
        map[key] = vectorObjectToJson(value);
    });
    return map;
};

const jsonToObjectMap = (obj) => {
    const map = new Map();
    for (const key in obj) {
        map.set(parseInt(key), jsonToVectorObject(obj[key]));
    }
    return map;
};

const threeDObjectToJson = (obj) => {
    const result = {
        type: 'threeDObject',
        points: obj.getPoints(),
        subobjects: obj.getSubobjects().map(threeDObjectToJson),
        fill: gradientImageOrColorToJson(obj.getFill()),
        stroke: gradientImageOrColorToJson(obj.getStroke()),
        strokeWidth: obj.getStrokeWidth(),
        index: obj.getIndex(),
    };
    return result;
};

const jsonToThreeDObject = (obj) => {
    return new WasmThreeDObject(
        obj.points,
        obj.subobjects.map(jsonToThreeDObject),
        jsonToGradientImageOrColor(obj.fill),
        jsonToGradientImageOrColor(obj.stroke),
        obj.strokeWidth,
        obj.index,
    );
};

export default {
    template: `<div></div>`,
    data: () => ({
        scene: null,
        resolveArray: [],
    }),
    async mounted() {
        await init({
            module_or_path: import.meta.resolve('mathlikeanim-rs/index_bg.wasm'),
        });
        const mrs = await import('mathlikeanim-rs');
        if (!window.mrsInitialized) {
            for (const key in mrs) {
                window[key] = mrs[key];
            }
            window.mrsInitialized = true;
        }
        if (!window.scenes) {
            window.scenes = [];
        }
        this.scene = this.svg ? new SVGScene(
            this.width,
            this.height,
            this.fps,
        ) : new Scene(
            this.width,
            this.height,
            this.fps,
        );
        if (this.svg) {
            this.scene.setDivContainer(this.$el);
            await this.scene.renderFrame();
        } else {
            const canvas = document.createElement('canvas');
            canvas.width = this.width;
            canvas.height = this.height;
            const ctx = canvas.getContext('2d');
            this.$el.appendChild(canvas);
            this.scene.setCanvasContext(ctx);
        }
        const id = this.$el.id;
        const style = `
            #${id} svg {
                width: ${this.cssWidth};
                height: ${this.cssHeight};
                display: block;
            }
            #${id} canvas {
                width: ${this.cssWidth};
                height: ${this.cssHeight};
                display: block;
            }
        `;
        const styleElement = document.createElement('style');
        styleElement.innerText = style;
        document.head.appendChild(styleElement);
        this.$el.addEventListener('python-response', this.onPythonResponse);
        window.scenes.push(this.scene);
    },
    methods: {
        emitReady() {
            this.$emit('ready', {});
        },
        onPythonResponse(event) {
            const resolve = this.resolveArray.pop();
            resolve(event.detail);
        },
        addFinalTipToObject(shapeJson, tipSideLength, tipColor) {
            const shape = jsonToVectorObject(shapeJson);
            return vectorObjectToJson(addFinalTip(
                shape,
                tipSideLength,
                jsonToColor(tipColor),
            ));
        },
        addInitialTipToObject(shapeJson, tipSideLength, tipColor) {
            const shape = jsonToVectorObject(shapeJson);
            return vectorObjectToJson(addInitialTip(
                shape,
                tipSideLength,
                jsonToColor(tipColor),
            ));
        },
        addBothSidesTipsToObject(shapeJson, tipSideLength, tipColor) {
            const shape = jsonToVectorObject(shapeJson);
            return vectorObjectToJson(addBothSidesTips(
                shape,
                tipSideLength,
                jsonToColor(tipColor),
            ));
        },
        newArc(center, radius, startAngle, endAngle, numPoints) {
            return vectorObjectToJson(arc(center, radius, startAngle, endAngle, numPoints));
        },
        newCircle(center, radius, numPoints) {
            return vectorObjectToJson(circle(center, radius, numPoints));
        },
        newEllipticalArc(center, radiusX, radiusY, startAngle, endAngle, numPoints) {
            return vectorObjectToJson(ellipticalArc(center, radiusX, radiusY, startAngle, endAngle, numPoints));
        },
        newEllipse(center, radiusX, radiusY, numPoints) {
            return vectorObjectToJson(ellipse(center, radiusX, radiusY, numPoints));
        },
        newAnnularSector(center, innerRadius, outerRadius, startAngle, endAngle, numPoints) {
            return vectorObjectToJson(annularSector(center, innerRadius, outerRadius, startAngle, endAngle, numPoints));
        },
        newDashedObject(objectJson, numDashes, dashedRatio, dashOffset, equalLengths) {
            const object = jsonToVectorObject(objectJson);
            return vectorObjectToJson(dashedObject(object, numDashes, dashedRatio, dashOffset, equalLengths));
        },
        newLine(start, end, color, strokeWidth) {
            return vectorObjectToJson(line(start, end, jsonToColor(color), strokeWidth));
        },
        newPolygon(points) {
            return vectorObjectToJson(polygon(points));
        },
        newRegularPolygon(center, sideLength, numSides) {
            return vectorObjectToJson(regularPolygon(center, sideLength, numSides));
        },
        newSquare(center, sideLength) {
            return vectorObjectToJson(square(center, sideLength));
        },
        newRectangle(center, width, height) {
            return vectorObjectToJson(rectangle(center, width, height));
        },
        newEquilateralTriangle(center, sideLength) {
            return vectorObjectToJson(equilateralTriangle(center, sideLength));
        },
        newTriangle(point1, point2, point3) {
            return vectorObjectToJson(triangle(point1, point2, point3));
        },
        newRightTriangle(point1, point2) {
            return vectorObjectToJson(rightTriangle(point1, point2));
        },
        newAxes(
            xMin, xMax, xStep,
            yMin, yMax, yStep,
            center,
            xLength, yLength,
            color, strokeWidth,
            lineCap, lineJoin,
            index,
            addXTicks, addYTicks,
            xTickSize, yTickSize,
            addXTip, addYTip,
        ) {
            return vectorObjectToJson(axes(
                xMin, xMax, xStep,
                yMin, yMax, yStep,
                center,
                xLength, yLength,
                jsonToColor(color),
                strokeWidth,
                lineCap, lineJoin,
                index,
                addXTicks, addYTicks,
                xTickSize, yTickSize,
                addXTip, addYTip,
            ))
        },
        axesCoordsToPoint(axesJson, x, y, xMin, xMax, yMin, yMax) {
            const axes = jsonToVectorObject(axesJson);
            return coordsToPoint(axes, x, y, xMin, xMax, yMin, yMax);
        },
        pointToAXesCoords(axesJson, point, xMin, xMax, yMin, yMax) {
            const axes = jsonToVectorObject(axesJson);
            return pointToAxesCoords(axes, point, xMin, xMax, yMin, yMax);
        },
        async newParametricPlotInAxes(
            funcId,
            tMin, tMax, tStep,
            axesJson,
            xMin, xMax,
            yMin, yMax,
        ) {
            const func = (t) => {
                return this.getPythonOutput(funcId, [t]);
            };
            const axes = jsonToVectorObject(axesJson);
            return vectorObjectToJson(await parametricPlotInAxes(
                func,
                tMin, tMax, tStep,
                axes,
                xMin, xMax,
                yMin, yMax,
            ));
        },
        async newPlotInAxes(
            funcId,
            xMin, xMax,
            yMin, yMax,
            x1, x2, xStep,
            axesJson,
        ) {
            const func = (x) => {
                return this.getPythonOutput(funcId, [x]);
            };
            const axes = jsonToVectorObject(axesJson);
            return vectorObjectToJson(await plotInAxes(
                func,
                xMin, xMax,
                yMin, yMax,
                x1, x2, xStep,
                axes,
            ));
        },
        async newContourPlotInAxes(
            funcId,
            xMin, xMax,
            yMin, yMax,
            x1, x2, xStep,
            y1, y2, yStep,
            axesJson,
            intervals,
        ) {
            const func = (x, y) => {
                return this.getPythonOutput(funcId, [x, y]);
            };
            const axes = jsonToVectorObject(axesJson);
            return vectorObjectToJson(await contourPlotInAxes(
                func,
                xMin, xMax,
                yMin, yMax,
                x1, x2, xStep,
                y1, y2, yStep,
                axes,
                intervals,
            ));
        },
        async newParametricPlot(
            funcId,
            tMin, tMax, tStep,
        ) {
            const func = (t) => {
                return this.getPythonOutput(funcId, [t]);
            };
            return vectorObjectToJson(await parametricFunction(
                func,
                tMin, tMax, tStep,
            ));
        },
        async newRealFunction(
            funcId,
            xMin, xMax, xStep,
        ) {
            const func = (x) => {
                return this.getPythonOutput(funcId, [x]);
            };
            return vectorObjectToJson(await realFunction(
                func,
                xMin, xMax, xStep,
            ));
        },
        async newContourPlot(
            funcId,
            xMin, xMax, xStep,
            yMin, yMax, yStep,
            intervals,
        ) {
            const func = (x, y) => {
                return this.getPythonOutput(funcId, [x, y]);
            };
            return vectorObjectToJson(await contourPlot(
                func,
                xMin, xMax, xStep,
                yMin, yMax, yStep,
                intervals,
            ));
        },
        newAreaUnderCurve(
            axes,
            plot,
            xMin, xMax,
            yMin, yMax,
            x1, x2,
        ) {
            return vectorObjectToJson(areaUnderCurve(
                jsonToVectorObject(axes),
                jsonToVectorObject(plot),
                xMin, xMax,
                yMin, yMax,
                x1, x2,
            ));
        },
        async riemannRectanglesForPlot(
            funcId,
            xMin, xMax,
            yMin, yMax,
            direction,
            x1, x2,
            nRects,
            axesJson,
        ) {
            const func = (x) => {
                return this.getPythonOutput(funcId, [x]);
            };
            const axes = jsonToVectorObject(axesJson);
            return vectorObjectToJson(await riemannRectanglesForPlot(
                func,
                xMin, xMax,
                yMin, yMax,
                direction,
                x1, x2,
                nRects,
                axes,
            ));
        },
        async secantLineForPlot(
            funcId,
            x1, x2,
            length,
            axesJson,
            xMin, xMax,
            yMin, yMax,
        ) {
            const func = (x) => {
                return this.getPythonOutput(funcId, [x]);
            };
            const axes = jsonToVectorObject(axesJson);
            return vectorObjectToJson(await secantLineForPlot(
                func,
                x1, x2,
                length,
                axes,
                xMin, xMax,
                yMin, yMax,
            ));
        },
        newNumberLine(
            xMin, xMax, xStep,
            center,
            color,
            strokeWidth,
            lineCap,
            lineJoin,
            index,
            length,
            addTip,
            addTicks,
            tickSize,
            angle,
        ) {
            return vectorObjectToJson(numberLine(
                xMin, xMax, xStep,
                center,
                jsonToColor(color),
                strokeWidth,
                lineCap,
                lineJoin,
                index,
                length,
                addTip,
                addTicks,
                tickSize,
                angle,
            ));
        },
        numberToPointOnNumberLine(numberLineJson, number, xMin, xMax) {
            const numberLine = jsonToVectorObject(numberLineJson);
            return numberToPoint(numberLine, number, xMin, xMax);
        },
        pointOnNumberLineToNumber(numberLineJson, point, xMin, xMax) {
            const numberLine = jsonToVectorObject(numberLineJson);
            return pointToNumber(numberLine, point, xMin, xMax);
        },
        async getNumbersTex(
            numberLine,
            numbers,
            numberToVectorObjectFuncId,
            xMin, xMax,
            height, direction,
            buff,
            index,
        ) {
            const numberLineObj = jsonToVectorObject(numberLine);
            const numberToVectorObjectFunc = async (number) => {
                return jsonToVectorObject(await this.getPythonOutput(numberToVectorObjectFuncId, [number]));
            };
            return await getNumbersTex(
                numberLineObj,
                numbers,
                numberToVectorObjectFunc,
                xMin, xMax,
                height, direction,
                buff,
                index,
            );
        },
        async newMathjax(
            expression,
            fontBase64,
        ) {
            const fontBase64ArrayBuffer = {};
            for (const key in fontBase64) {
                const base64 = fontBase64[key];
                const binary = atob(base64);
                const len = binary.length;
                const bytes = new Uint8Array(len);
                for (let i = 0; i < len; i++) {
                    bytes[i] = binary.charCodeAt(i);
                }
                fontBase64ArrayBuffer[key] = bytes.buffer;
            }
            return await mathjax(expression, fontBase64ArrayBuffer);
        },
        async svgToVectorObject(
            svg,
            fontBase64,
        ) {
            const fontBase64ArrayBuffer = {};
            for (const key in fontBase64) {
                const base64 = fontBase64[key];
                const binary = atob(base64);
                const len = binary.length;
                const bytes = new Uint8Array(len);
                for (let i = 0; i < len; i++) {
                    bytes[i] = binary.charCodeAt(i);
                }
                fontBase64ArrayBuffer[key] = bytes.buffer;
            }
            return await svgToVector(svg, fontBase64ArrayBuffer);
        },
        async textToVectorObject(
            text,
            fontBase64,
            fontFamily,
            fontWeight,
            fontStyle,
            x, y,
            fontSize,
        ) {
            const fontBase64ArrayBuffer = {};
            for (const key in fontBase64) {
                const base64 = fontBase64[key];
                const binary = atob(base64);
                const len = binary.length;
                const bytes = new Uint8Array(len);
                for (let i = 0; i < len; i++) {
                    bytes[i] = binary.charCodeAt(i);
                }
                fontBase64ArrayBuffer[key] = bytes.buffer;
            }
            return await textToVector(
                text,
                fontBase64ArrayBuffer,
                fontFamily,
                fontWeight,
                fontStyle,
                x, y,
                fontSize,
            );
        },
        async newSphere(
            center,
            radius,
            uSegments,
            vSegments,
            fillColors,
            strokeColors,
            strokeWidth,
            index,
        ) {
            return threeDObjectToJson(await sphere(
                center,
                radius,
                uSegments,
                vSegments,
                fillColors.map(jsonToColor),
                strokeColors.map(jsonToColor),
                strokeWidth,
                index,
            ));
        },
        newThreeDAxes(
            xMin, xMax, xStep,
            yMin, yMax, yStep,
            zMin, zMax, zStep,
            center,
            xLength, yLength, zLength,
            color, strokeWidth,
            addXTicks, addYTicks, addZTicks,
            xTickSize, yTickSize, zTickSize,
            addXTip, addYTip, addZTip,
            nPieces, index,
        ) {
            return vectorObjectToJson(threeDAxes(
                xMin, xMax, xStep,
                yMin, yMax, yStep,
                zMin, zMax, zStep,
                center,
                xLength, yLength, zLength,
                jsonToColor(color),
                strokeWidth,
                addXTicks, addYTicks, addZTicks,
                xTickSize, yTickSize, zTickSize,
                addXTip, addYTip, addZTip,
                nPieces, index,
            ));
        },
        coordsToPointsOnThreeDAxes(
            axesJson,
            coords,
            xMin, xMax,
            yMin, yMax,
            zMin, zMax,
        ) {
            const axes = jsonToVectorObject(axesJson);
            return coordsToPoint3D(axes, coords, xMin, xMax, yMin, yMax, zMin, zMax);
        },
        pointOnThreeDAxesToCoords(
            axesJson,
            point,
            xMin, xMax,
            yMin, yMax,
            zMin, zMax,
        ) {
            const axes = jsonToVectorObject(axesJson);
            return pointToCoords3D(axes, point, xMin, xMax, yMin, yMax, zMin, zMax);
        },
        async newParametricPlotInThreeDAxes(
            axesJson, funcId,
            uMin, uMax,
            vMin, vMax,
            uSteps, vSteps,
            xMin, xMax,
            yMin, yMax,
            zMin, zMax,
            fills, strokes,
            strokeWidth,
            index,
        ) {
            const func = (u, v) => {
                return this.getPythonOutput(funcId, [u, v]);
            };
            const axes = jsonToVectorObject(axesJson);
            return vectorObjectToJson(await parametricPlotInAxes3D(
                axes,
                func,
                uMin, uMax,
                vMin, vMax,
                uSteps, vSteps,
                xMin, xMax,
                yMin, yMax,
                zMin, zMax,
                fills.map(jsonToColor),
                strokes.map(jsonToColor),
                strokeWidth,
                index,
            ));
        },
        async newPlotInThreeDAxes(
            axesJson, funcId,
            uMin, uMax,
            vMin, vMax,
            uSteps, vSteps,
            xMin, xMax,
            yMin, yMax,
            zMin, zMax,
            fills, strokes,
            strokeWidth,
            index,
        ) {
            const func = (x, y) => {
                return this.getPythonOutput(funcId, [x, y]);
            };
            const axes = jsonToVectorObject(axesJson);
            return vectorObjectToJson(await plotInAxes3D(
                axes,
                func,
                uMin, uMax,
                vMin, vMax,
                uSteps, vSteps,
                xMin, xMax,
                yMin, yMax,
                zMin, zMax,
                fills.map(jsonToColor),
                strokes.map(jsonToColor),
                strokeWidth,
                index,
            ));
        },
        async newParametricLinePlotInThreeDAxes(
            axesJson, funcId,
            tMin, tMax, tSteps,
            xMin, xMax,
            yMin, yMax,
            zMin, zMax,
            color,
            strokeWidth,
            index,
        ) {
            const func = (u) => {
                return this.getPythonOutput(funcId, [u]);
            };
            const axes = jsonToVectorObject(axesJson);
            return vectorObjectToJson(await parametricLinePlotInAxes3D(
                axes,
                func,
                tMin, tMax, tSteps,
                xMin, xMax,
                yMin, yMax,
                zMin, zMax,
                [jsonToColor(color)],
                strokeWidth,
                index,
            ));
        },
        createVectorObject(objJson, t) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(create(obj, t));
        },
        drawStrokeThenFill(objJson, t, defaultStrokeWidth) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(drawStrokeThenFill(obj, t, defaultStrokeWidth));
        },
        fadeIn(objJson, scaleFactor, shift, t) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(fadeIn(obj, scaleFactor, shift, t));
        },
        fadeOut(objJson, scaleFactor, shift, t) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(fadeOut(obj, scaleFactor, shift, t));
        },
        growArrowWithFinalTip(objJson, t) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(growArrowWithFinalTip(obj, t));
        },
        growArrowWithInitialTip(objJson, t) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(growArrowWithInitialTip(obj, t));
        },
        growArrowWithTipsAtBothEnds(objJson, t) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(growArrowWithTipsAtBothEnds(obj, t));
        },
        growFromCenter(objJson, t) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(growFromCenter(obj, t));
        },
        morphShape(objJson, targetObjJson, t) {
            const obj = jsonToVectorObject(objJson);
            const targetObj = jsonToVectorObject(targetObjJson);
            return vectorObjectToJson(morphShape(obj, targetObj, t));
        },
        rotateAnimation(objJson, angle, t) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(rotateAnimation(obj, angle, t));
        },
        scaleInPlace(objJson, scaleFactor, t) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(scaleInPlace(obj, scaleFactor, t));
        },
        setFillAnimation(objJson, fillJson, t) {
            const obj = jsonToVectorObject(objJson);
            const fill = jsonToColor(fillJson);
            return vectorObjectToJson(setFillAnimation(obj, fill, t));
        },
        setStrokeAnimation(objJson, strokeJson, t) {
            const obj = jsonToVectorObject(objJson);
            const stroke = jsonToColor(strokeJson);
            return vectorObjectToJson(setStrokeAnimation(obj, stroke, t));
        },
        shiftAnimation(objJson, shift, t) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(shiftAnimation(obj, shift, t));
        },
        showTemporaily(objJson, t) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(showTemporaily(obj, t));
        },
        spinningGrow(objJson, t) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(spinningGrow(obj, t));
        },
        createAxes3D(objJson, t, defaultStrokeWidth) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(createAxes3D(obj, t, defaultStrokeWidth));
        },
        createThreeDObject(objJson, t) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(create3D(obj, t));
        },
        drawStrokeThenFill3D(objJson, t, defaultStrokeWidth) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(drawStrokeThenFill3D(obj, t, defaultStrokeWidth));
        },
        fadeIn3D(objJson, scaleFactor, shift, t) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(fadeIn3D(obj, scaleFactor, shift, t));
        },
        fadeOut3D(objJson, scaleFactor, shift, t) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(fadeOut3D(obj, scaleFactor, shift, t));
        },
        growFromCenter3D(objJson, t) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(growFromCenter3D(obj, t));
        },
        morphShape3D(objJson, targetObjJson, t) {
            const obj = jsonToThreeDObject(objJson);
            const targetObj = jsonToThreeDObject(targetObjJson);
            return threeDObjectToJson(morphShape3D(obj, targetObj, t));
        },
        rotateXAnimation3D(objJson, angle, t) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(rotateXAnimation3D(obj, angle, t));
        },
        rotateYAnimation3D(objJson, angle, t) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(rotateYAnimation3D(obj, angle, t));
        },
        rotateZAnimation3D(objJson, angle, t) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(rotateZAnimation3D(obj, angle, t));
        },
        scaleInPlace3D(objJson, scaleFactor, t) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(scaleInPlace3D(obj, scaleFactor, t));
        },
        setFillAnimation3D(objJson, fillJson, t) {
            const obj = jsonToThreeDObject(objJson);
            const fill = jsonToColor(fillJson);
            return threeDObjectToJson(setFillAnimation3D(obj, fill, t));
        },
        setStrokeAnimation3D(objJson, strokeJson, t) {
            const obj = jsonToThreeDObject(objJson);
            const stroke = jsonToColor(strokeJson);
            return threeDObjectToJson(setStrokeAnimation3D(obj, stroke, t));
        },
        shiftAnimation3D(objJson, shift, t) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(shiftAnimation3D(obj, shift, t));
        },
        incrementIndex(objJson, increment, recursive) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(obj.incrementIndex(increment, recursive));
        },
        getSubobjectsRecursively(objJson, withPoints) {
            const obj = jsonToVectorObject(objJson);
            return obj.getSubobjectsRecursively(withPoints).map(vectorObjectToJson);
        },
        getSubcurve(objJson, t1, t2) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(obj.getSubcurve(t1, t2));
        },
        getPartialCopy(objJson, t1, t2, recursive) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(obj.getPartialCopy(t1, t2, recursive));
        },
        getAnchorsAndHandles(objJson) {
            const obj = jsonToVectorObject(objJson);
            return obj.getAnchorsAndHandles();
        },
        scaleHandleToAnchorDistances(objJson, scaleFactor, recursive) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(obj.scaleHandleToAnchorDistances(scaleFactor, recursive));
        },
        setAnchorsAndHandles(objJson, anchorsAndHandles) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(obj.setAnchorsAndHandles(anchorsAndHandles));
        },
        getNthCurvePoints(objJson, n) {
            const obj = jsonToVectorObject(objJson);
            return obj.getNthCurvePoints(n);
        },
        getNthCurveLengthPieces(objJson, n, nPieces) {
            const obj = jsonToVectorObject(objJson);
            return obj.getNthCurveLengthPieces(n, nPieces);
        },
        getSubpaths(objJson) {
            const obj = jsonToVectorObject(objJson);
            return obj.getSubpaths();
        },
        async applyFunction(objJson, funcId, recursive, aboutPoint, aboutEdge) {
            const obj = jsonToVectorObject(objJson);
            const func = (point) => {
                return this.getPythonOutput(funcId, [point]);
            };
            return vectorObjectToJson(await obj.applyFunction(func, recursive, aboutPoint, aboutEdge));
        },
        getPieces(objJson, nPieces) {
            const obj = jsonToVectorObject(objJson);
            return obj.getPieces(nPieces);
        },
        getCubicBezierTuples(objJson) {
            const obj = jsonToVectorObject(objJson);
            return obj.getCubicBezierTuples();
        },
        scale(objJson, scaleFactor, recursive) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(obj.scale(scaleFactor, recursive));
        },
        stretch(objJson, stretchFactor, recursive) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(obj.stretch(stretchFactor[0], stretchFactor[1], recursive));
        },
        shift(objJson, shift, recursive) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(obj.shift(shift[0], shift[1], recursive));
        },
        mergedPoints(objJson) {
            const obj = jsonToVectorObject(objJson);
            return obj.mergedPoints();
        },
        getBoundingBox(objJson) {
            const obj = jsonToVectorObject(objJson);
            return obj.getBoundingBox();
        },
        getCenter(objJson) {
            const obj = jsonToVectorObject(objJson);
            return obj.getCenter();
        },
        getCenterOfMass(objJson) {
            const obj = jsonToVectorObject(objJson);
            return obj.getCenterOfMass();
        },
        getWidth(objJson) {
            const obj = jsonToVectorObject(objJson);
            return obj.getWidth();
        },
        getHeight(objJson) {
            const obj = jsonToVectorObject(objJson);
            return obj.getHeight();
        },
        moveTo(objJson, point, recursive) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(obj.moveTo(point[0], point[1], recursive));
        },
        rotate(objJson, angle, recursive) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(obj.rotate(angle, recursive));
        },
        nextToOther(objJson, otherJson, direction, buff, alignedEdge, recursive) {
            const obj = jsonToVectorObject(objJson);
            const other = jsonToVectorObject(otherJson);
            return vectorObjectToJson(obj.nextToOther(other, direction, buff, alignedEdge, recursive));
        },
        nextToPoint(objJson, point, direction, buff, alignedEdge, recursive) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(obj.nextToPoint(point, direction, buff, alignedEdge, recursive));
        },
        arrangeSubobjects(objJson, direction, buff, alignedEdge, recursive) {
            const obj = jsonToVectorObject(objJson);
            return vectorObjectToJson(obj.arrangeSubobjects(direction, buff, alignedEdge, recursive));
        },
        getAnchorsAndHandles3D(objJson) {
            const obj = jsonToThreeDObject(objJson);
            return obj.getAnchorsAndHandles();
        },
        setAnchorsAndHandles3D(objJson, anchorsAndHandles) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(obj.setAnchorsAndHandles(anchorsAndHandles));
        },
        scaleHandleToAnchorDistances3D(objJson, scaleFactor, recursive) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(obj.scaleHandleToAnchorDistances(scaleFactor, recursive));
        },
        getCubicBezierTuples3D(objJson) {
            const obj = jsonToThreeDObject(objJson);
            return obj.getCubicBezierTuples();
        },
        getPartialCopy3D(objJson, t1, t2, recursive) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(obj.getPartialCopy(t1, t2, recursive));
        },
        getCriticalPoint3D(objJson, key) {
            const obj = jsonToThreeDObject(objJson);
            return obj.getCriticalPoint(key[0], key[1], key[2]);
        },
        nextToOther3D(objJson, otherJson, direction, buff, alignedEdge, recursive) {
            const obj = jsonToThreeDObject(objJson);
            const other = jsonToThreeDObject(otherJson);
            return threeDObjectToJson(obj.nextToOther(other, direction, buff, alignedEdge, recursive));
        },
        nextToPoint3D(objJson, point, direction, buff, alignedEdge, recursive) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(obj.nextToPoint(point, direction, buff, alignedEdge, recursive));
        },
        nextToPoint3D(objJson, point, direction, buff, alignedEdge, recursive) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(obj.nextToPoint(point, direction, buff, alignedEdge, recursive));
        },
        scale3D(objJson, scaleFactor, recursive) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(obj.scale(scaleFactor, recursive));
        },
        stretch3D(objJson, stretchFactor, recursive) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(obj.stretch(stretchFactor, recursive));
        },
        shift3D(objJson, shift, recursive) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(obj.shift(shift, recursive));
        },
        rotateX3D(objJson, angle, recursive) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(obj.rotateX(angle, recursive));
        },
        rotateY3D(objJson, angle, recursive) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(obj.rotateY(angle, recursive));
        },
        rotateZ3D(objJson, angle, recursive) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(obj.rotateZ(angle, recursive));
        },
        projectAndShade3D(objJson, cameraJson, lightJson) {
            const obj = jsonToThreeDObject(objJson);
            const camera = jsonToCamera(cameraJson);
            const light = jsonToLightSource(lightJson);
            return vectorObjectToJson(obj.projectAndShade(camera, light));
        },
        async applyFunction3D(objJson, funcId, recursive) {
            const obj = jsonToThreeDObject(objJson);
            const func = (point) => {
                return this.getPythonOutput(funcId, [point]);
            };
            return threeDObjectToJson(await obj.applyFunction(func, recursive));
        },
        async fromUVFunction3D(funcId, uRange, vRange, uSegments, vSegments, fills, strokes, strokeWidth, index) {
            const func = (u, v) => {
                return this.getPythonOutput(funcId, [u, v]);
            };
            return threeDObjectToJson(await WasmThreeDObject.fromUvFunction(
                func,
                uRange, vRange,
                uSegments, vSegments,
                fills.map(jsonToColor),
                strokes.map(jsonToColor),
                strokeWidth,
                index,
            ));
        },
        getBoundingBox3D(objJson) {
            const obj = jsonToThreeDObject(objJson);
            return obj.getBoundingBox();
        },
        getCenter3D(objJson) {
            const obj = jsonToThreeDObject(objJson);
            return obj.getCenter();
        },
        mergedPoints3D(objJson) {
            const obj = jsonToThreeDObject(objJson);
            return obj.mergedPoints();
        },
        moveTo3D(objJson, point, recursive) {
            const obj = jsonToThreeDObject(objJson);
            return threeDObjectToJson(obj.moveTo(point, recursive));
        },
        threeDObjectFromVectorObject(objJson) {
            const obj = jsonToVectorObject(objJson);
            return threeDObjectToJson(WasmThreeDObject.fromVectorObject(obj));
        },
        getSubobjectsRecursively3D(objJson, withPoints) {
            const obj = jsonToThreeDObject(objJson);
            return obj.getSubobjectsRecursively(withPoints).map(threeDObjectToJson);
        },
        addToScene(objJson) {
            const obj = jsonToVectorObject(objJson);
            this.scene.add(obj.clone());
        },
        removeFromScene(i) {
            this.scene.remove(i);
        },
        clearScene() {
            this.scene.clear();
        },
        restoreState(state) {
            this.scene.restore(state);
        },
        saveState(state) {
            return this.scene.saveState(state);
        },
        setTopLeftCorner(x, y) {
            this.scene.setTopLeftCorner(x, y);
        },
        setBottomRightCorner(x, y) {
            this.scene.setBottomRightCorner(x, y);
        },
        setBackground(gradientImageOrColor) {
            this.scene.setBackground(jsonToGradientImageOrColor(gradientImageOrColor));
        },
        async play(animationFuncId, objectIndices, durationInFrames) {
            const animationFunc = async (objsMap, t) => {
                const result = await this.getPythonOutput(animationFuncId, [objectMapToJson(objsMap), t]);
                return jsonToObjectMap(result);
            };
            await this.scene.play(animationFunc, objectIndices, durationInFrames, linear);
        },
        setUpdater(index, updaterFuncId) {
            const updaterFunc = async (vectorObject) => {
                const result = await this.getPythonOutput(updaterFuncId, [vectorObjectToJson(vectorObject)]);
                return jsonToVectorObject(result);
            };
            this.scene.setUpdater(index, updaterFunc);
        },
        removeUpdater(index) {
            this.scene.removeUpdater(index);
        },
        async wait(durationInFrames, objectIndices) {
            await this.scene.wait(durationInFrames, objectIndices);
        },
        async waitUntil(conditionId, objectIndices) {
            const conditionFunc = () => {
                return this.getPythonOutput(conditionId, []);
            };
            await this.scene.waitUntil(conditionFunc, objectIndices);
        },
        setOnRendered(onRenderedFuncId) {
            const onRenderedFunc = () => {
                return this.getPythonOutput(onRenderedFuncId, []);
            };
            this.scene.setOnRendered(onRenderedFunc);
        },
        async renderFrame() {
            await this.scene.renderFrame();
        },
        getObjects() {
            return this.scene.getObjects().map(vectorObjectToJson);
        },
        projectPoints(points, cameraJson) {
            const camera = jsonToCamera(cameraJson);
            return projectPoints(points, camera);
        },
        log(message) {
            console.log(message);
        },
        setScene3DIndex(index) {
            this.scene.set3DIndex(index);
        },
        projectAndShadeScene() {
            return vectorObjectToJson(this.scene.projectAndShade());
        },
        setSceneCameraPosition(position) {
            this.scene.setCameraPosition(...position);
        },
        setSceneCameraRotation(rotation) {
            this.scene.setCameraRotation(...rotation);
        },
        setSceneCameraFocalDistance(focalDistance) {
            this.scene.setCameraFocalDistance(focalDistance);
        },
        setSceneCameraZoom(zoom) {
            this.scene.setCameraZoom(zoom);
        },
        setSceneLightSourcePosition(position) {
            this.scene.setLightSourcePosition(...position);
        },
        getScene3DIndex() {
            return this.scene.get3DIndex();
        },
        getSceneCameraPosition() {
            return this.scene.getCameraPosition();
        },
        getSceneCameraRotation() {
            return this.scene.getCameraRotation();
        },
        getSceneCameraFocalDistance() {
            return this.scene.getCameraFocalDistance();
        },
        getSceneCameraZoom() {
            return this.scene.getCameraZoom();
        },
        getSceneLightSourcePosition() {
            return this.scene.getLightSourcePosition();
        },
        addScene3D(objJson) {
            const obj = jsonToThreeDObject(objJson);
            this.scene.add3D(obj.clone());
        },
        insertScene3D(index, objJson) {
            const obj = jsonToThreeDObject(objJson);
            this.scene.insert3D(index, obj.clone());
        },
        removeScene3D(i) {
            this.scene.remove3D(i);
        },
        getScene3DObjects() {
            return this.scene.get3DObjects().map(threeDObjectToJson);
        },
        setScene3DObjects(objsJson) {
            const objs = objsJson.map(jsonToThreeDObject);
            this.scene.set3DObjects(objs);
        },
        setScene3DObject(objJson) {
            const obj = jsonToThreeDObject(objJson);
            this.scene.set3DObject(obj.clone());
        },
        async getPythonOutput(pythonFuncId, args) {
            this.emitEvent('python-request', { pythonFuncId, args });
            return await new Promise((resolve) => {
                this.resolveArray.push(resolve);
            });
        },
        emitEvent(eventName, data) {
            this.$emit(eventName, data);
        },
        emitPythonResponse(data) {
            this.$el.dispatchEvent(new CustomEvent('python-response', { detail: data }));
        },
        beginRecording() {
            this.scene.setOnRendered(() => {
                this.emitFrame();
            });
        },
        stopRecording() {
            this.scene.setOnRendered(async () => {});
        },
        emitFrame() {
            const data = { svg: this.svg };
            if (this.svg) {
                data.frame = this.$el.children[0].outerHTML;
            } else {
                data.frame = this.$el.children[0].toDataURL();
            }
            this.emitEvent('frame', data);
        },
    },
    props: {
        width: Number,
        height: Number,
        cssWidth: Number,
        cssHeight: Number,
        fps: Number,
        svg: Boolean,
    },
}