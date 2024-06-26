/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_wasmgradientimageorcolor_free(a: number): void;
export function wasmgradientimageorcolor_fromColor(a: number): number;
export function wasmgradientimageorcolor_fromLinearGradient(a: number): number;
export function wasmgradientimageorcolor_fromRadialGradient(a: number): number;
export function wasmgradientimageorcolor_fromImage(a: number): number;
export function wasmgradientimageorcolor_isColor(a: number): number;
export function wasmgradientimageorcolor_isLinearGradient(a: number): number;
export function wasmgradientimageorcolor_isRadialGradient(a: number): number;
export function wasmgradientimageorcolor_isImage(a: number): number;
export function wasmgradientimageorcolor_getColor(a: number): number;
export function wasmgradientimageorcolor_getLinearGradient(a: number): number;
export function wasmgradientimageorcolor_getRadialGradient(a: number): number;
export function wasmgradientimageorcolor_getImage(a: number): number;
export function wasmgradientimageorcolor_clone(a: number): number;
export function __wbg_wasmcolor_free(a: number): void;
export function wasmcolor_new(a: number, b: number, c: number, d: number): number;
export function wasmcolor_getR(a: number): number;
export function wasmcolor_getG(a: number): number;
export function wasmcolor_getB(a: number): number;
export function wasmcolor_getA(a: number): number;
export function __wbg_wasmgradientstop_free(a: number): void;
export function wasmgradientstop_new(a: number, b: number): number;
export function wasmgradientstop_getOffset(a: number): number;
export function wasmgradientstop_getColor(a: number): number;
export function __wbg_wasmlineargradient_free(a: number): void;
export function wasmlineargradient_new(a: number, b: number, c: number, d: number, e: number, f: number, g: number): number;
export function wasmlineargradient_getStops(a: number, b: number): void;
export function __wbg_wasmradialgradient_free(a: number): void;
export function wasmradialgradient_new(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number): number;
export function wasmradialgradient_getStops(a: number, b: number): void;
export function wasmradialgradient_getAlpha(a: number): number;
export function __wbg_wasmimage_free(a: number): void;
export function wasmimage_new(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number): number;
export function wasmimage_getImageBase64(a: number, b: number): void;
export function wasmimage_getMimeType(a: number, b: number): void;
export function __wbg_wasmvectorobject_free(a: number): void;
export function wasmvectorobject_new(): number;
export function wasmvectorobject_getIndex(a: number): number;
export function wasmvectorobject_incrementIndex(a: number, b: number, c: number): number;
export function wasmvectorobject_getPoints(a: number): number;
export function wasmvectorobject_getFill(a: number): number;
export function wasmvectorobject_getStroke(a: number): number;
export function wasmvectorobject_getStrokeWidth(a: number): number;
export function wasmvectorobject_getLineCap(a: number, b: number): void;
export function wasmvectorobject_getLineJoin(a: number, b: number): void;
export function wasmvectorobject_getPartialCopy(a: number, b: number, c: number, d: number): number;
export function wasmvectorobject_getSubpaths(a: number): number;
export function wasmvectorobject_getCubicBezierTuples(a: number): number;
export function wasmvectorobject_getSubobjects(a: number, b: number): void;
export function wasmvectorobject_scale(a: number, b: number, c: number): number;
export function wasmvectorobject_stretch(a: number, b: number, c: number, d: number): number;
export function wasmvectorobject_shift(a: number, b: number, c: number, d: number): number;
export function wasmvectorobject_mergedPoints(a: number): number;
export function wasmvectorobject_getBoundingBox(a: number): number;
export function wasmvectorobject_getCenter(a: number): number;
export function wasmvectorobject_getCenterOfMass(a: number): number;
export function wasmvectorobject_getHeight(a: number): number;
export function wasmvectorobject_getWidth(a: number): number;
export function wasmvectorobject_setIndex(a: number, b: number): number;
export function wasmvectorobject_setFill(a: number, b: number, c: number): number;
export function wasmvectorobject_setFillOpacity(a: number, b: number, c: number): number;
export function wasmvectorobject_moveTo(a: number, b: number, c: number, d: number): number;
export function wasmvectorobject_setStroke(a: number, b: number, c: number): number;
export function wasmvectorobject_setStrokeOpacity(a: number, b: number, c: number): number;
export function wasmvectorobject_setStrokeWidth(a: number, b: number, c: number): number;
export function wasmvectorobject_setLineCap(a: number, b: number, c: number, d: number): number;
export function wasmvectorobject_setLineJoin(a: number, b: number, c: number, d: number): number;
export function wasmvectorobject_setPoints(a: number, b: number): number;
export function wasmvectorobject_setSubobjects(a: number, b: number, c: number): number;
export function wasmvectorobject_rotate(a: number, b: number, c: number): number;
export function wasmvectorobject_getCriticalPoint(a: number, b: number, c: number): number;
export function wasmvectorobject_getFillOpacity(a: number): number;
export function wasmvectorobject_getStrokeOpacity(a: number): number;
export function wasmvectorobject_nextToOther(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function wasmvectorobject_arrangeSubobjects(a: number, b: number, c: number, d: number, e: number): number;
export function wasmvectorobject_nextToPoint(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function wasmvectorobject_clone(a: number): number;
export function addFinalTip(a: number, b: number, c: number): number;
export function addInitialTip(a: number, b: number, c: number): number;
export function addBothSidesTips(a: number, b: number, c: number): number;
export function arc(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number): number;
export function circle(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number): number;
export function ellipticalArc(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number): number;
export function ellipse(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number): number;
export function annularSector(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number): number;
export function line(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number): number;
export function polygon(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number): number;
export function regularPolygon(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number): number;
export function square(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number): number;
export function rectangle(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number): number;
export function equilateralTriangle(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number): number;
export function triangle(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number): number;
export function rightTriangle(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number): number;
export function axes(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number, x: number, y: number, z: number, a1: number, b1: number): number;
export function coordsToPoint(a: number, b: number, c: number, d: number, e: number, f: number, g: number): number;
export function pointToCoords(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function parametricPlotInAxes(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number): number;
export function plotInAxes(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number): number;
export function contourPlotInAxes(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number): number;
export function areaUnderCurve(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number): number;
export function riemannRectanglesForPlot(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number): number;
export function secantLineForPlot(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number): number;
export function parametricFunction(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number): number;
export function contourPlot(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number): number;
export function realFunction(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number): number;
export function numberLine(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number): number;
export function numberToPoint(a: number, b: number, c: number, d: number): number;
export function pointToNumber(a: number, b: number, c: number, d: number): number;
export function getNumbersTex(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number): number;
export function svgToVector(a: number, b: number): number;
export function wasmlineargradient_getX1(a: number): number;
export function wasmlineargradient_getY1(a: number): number;
export function wasmlineargradient_getX2(a: number): number;
export function wasmlineargradient_getY2(a: number): number;
export function wasmlineargradient_getAlpha(a: number): number;
export function wasmradialgradient_getCx(a: number): number;
export function wasmradialgradient_getCy(a: number): number;
export function wasmradialgradient_getR(a: number): number;
export function wasmradialgradient_getFx(a: number): number;
export function wasmradialgradient_getFy(a: number): number;
export function wasmimage_getTop(a: number): number;
export function wasmimage_getLeft(a: number): number;
export function wasmimage_getBottom(a: number): number;
export function wasmimage_getRight(a: number): number;
export function wasmimage_getAlpha(a: number): number;
export function rotMatrix(a: number, b: number): number;
export function matrixProduct(a: number, b: number): number;
export function rotMatrixEuler(a: number, b: number, c: number): number;
export function transposeMatrix(a: number): number;
export function applyMatrix(a: number, b: number): number;
export function shiftPoints3D(a: number, b: number): number;
export function ensureValidThreeDColor(a: number): number;
export function __wbg_wasmlightsource_free(a: number): void;
export function wasmlightsource_new(a: number): number;
export function wasmlightsource_getPosition(a: number): number;
export function wasmlightsource_clone(a: number): number;
export function __wbg_wasmcamera_free(a: number): void;
export function wasmcamera_new(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function wasmcamera_getPosition(a: number): number;
export function wasmcamera_getRotation(a: number): number;
export function wasmcamera_getFocalDistance(a: number): number;
export function wasmcamera_getZoom(a: number): number;
export function wasmcamera_getWidth(a: number): number;
export function wasmcamera_getHeight(a: number): number;
export function wasmcamera_clone(a: number): number;
export function getShadedRgb(a: number, b: number, c: number, d: number): number;
export function getStartCorner(a: number): number;
export function getEndCorner(a: number): number;
export function crossProduct(a: number, b: number): number;
export function getUnitNormal(a: number, b: number): number;
export function getStartAnchors(a: number): number;
export function getEndAnchors(a: number): number;
export function getAnchors(a: number): number;
export function getCornerUnitNormal(a: number, b: number): number;
export function getStartCornerUnitNormal(a: number): number;
export function getEndCornerUnitNormal(a: number): number;
export function getShadedColor(a: number, b: number, c: number, d: number): number;
export function projectPoints(a: number, b: number): number;
export function lineAsCubicBezier3D(a: number, b: number): number;
export function __wbg_wasmthreedobject_free(a: number): void;
export function wasmthreedobject_new(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function wasmthreedobject_getPoints(a: number): number;
export function wasmthreedobject_getSubobjects(a: number, b: number): void;
export function wasmthreedobject_getFill(a: number): number;
export function wasmthreedobject_getStroke(a: number): number;
export function wasmthreedobject_getStrokeWidth(a: number): number;
export function wasmthreedobject_setPoints(a: number, b: number): number;
export function wasmthreedobject_setSubobjects(a: number, b: number, c: number): number;
export function wasmthreedobject_setFill(a: number, b: number): number;
export function wasmthreedobject_setStroke(a: number, b: number): number;
export function wasmthreedobject_setStrokeWidth(a: number, b: number): number;
export function wasmthreedobject_scale(a: number, b: number, c: number): number;
export function wasmthreedobject_stretch(a: number, b: number, c: number): number;
export function wasmthreedobject_shift(a: number, b: number, c: number): number;
export function wasmthreedobject_rotateX(a: number, b: number, c: number): number;
export function wasmthreedobject_rotateY(a: number, b: number, c: number): number;
export function wasmthreedobject_rotateZ(a: number, b: number, c: number): number;
export function wasmthreedobject_projectAndShade(a: number, b: number, c: number): number;
export function wasmthreedobject_fromUvFunction(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number): number;
export function wasmthreedobject_getBoundingBox(a: number): number;
export function wasmthreedobject_getCenter(a: number): number;
export function wasmthreedobject_moveTo(a: number, b: number, c: number): number;
export function wasmthreedobject_clone(a: number): number;
export function makeTimings(a: number, b: number, c: number): void;
export function animationGroup(a: number, b: number, c: number, d: number, e: number): number;
export function create(a: number, b: number): number;
export function drawStrokeThenFill(a: number, b: number): number;
export function write(a: number, b: number, c: number): number;
export function fadeIn(a: number, b: number, c: number, d: number): number;
export function fadeOut(a: number, b: number, c: number, d: number): number;
export function growArrowWithFinalTip(a: number, b: number): number;
export function growArrowWithInitialTip(a: number, b: number): number;
export function growArrowWithTipsAtBothEnds(a: number, b: number): number;
export function growFromCenter(a: number, b: number): number;
export function morphShape(a: number, b: number, c: number): number;
export function moveCameraSVG(a: number, b: number, c: number, d: number): void;
export function moveCamera(a: number, b: number, c: number, d: number): void;
export function rotateAnimation(a: number, b: number, c: number): number;
export function scaleInPlace(a: number, b: number, c: number): number;
export function setFillAnimation(a: number, b: number, c: number): number;
export function setStrokeAnimation(a: number, b: number, c: number): number;
export function shiftAnimation(a: number, b: number, c: number): number;
export function showTemporaily(a: number, b: number): number;
export function spinningGrow(a: number, b: number, c: number): number;
export function __wbg_scene_free(a: number): void;
export function scene_new_js(a: number, b: number, c: number): number;
export function scene_getFps(a: number): number;
export function scene_getHeight(a: number): number;
export function scene_getWidth(a: number): number;
export function scene_renderFrame(a: number): void;
export function scene_clear(a: number): void;
export function scene_restore(a: number, b: number): void;
export function scene_saveState(a: number, b: number): void;
export function scene_setTopLeftCorner(a: number, b: number, c: number): void;
export function scene_setBottomRightCorner(a: number, b: number, c: number): void;
export function scene_getTopLeftCorner(a: number): number;
export function scene_getBottomRightCorner(a: number): number;
export function scene_setBackground(a: number, b: number): void;
export function scene_add(a: number, b: number): void;
export function scene_insert(a: number, b: number, c: number): void;
export function scene_remove(a: number, b: number): void;
export function scene_getObjects(a: number): number;
export function scene_getObjectsFromIndices(a: number, b: number): number;
export function scene_setCanvasContext(a: number, b: number): void;
export function scene_sleep(a: number, b: number): number;
export function scene_setObjects(a: number, b: number): void;
export function scene_play(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function scene_makeFrame(a: number, b: number, c: number, d: number): number;
export function scene_wait(a: number, b: number): number;
export function scene_setCallback(a: number, b: number): void;
export function scene_callCallback(a: number): number;
export function __wbg_svgscene_free(a: number): void;
export function svgscene_new_js(a: number, b: number, c: number): number;
export function svgscene_getFps(a: number): number;
export function svgscene_getHeight(a: number): number;
export function svgscene_getWidth(a: number): number;
export function svgscene_renderFrame(a: number): void;
export function svgscene_clear(a: number): void;
export function svgscene_restore(a: number, b: number): void;
export function svgscene_saveState(a: number, b: number): void;
export function svgscene_setTopLeftCorner(a: number, b: number, c: number): void;
export function svgscene_setBottomRightCorner(a: number, b: number, c: number): void;
export function svgscene_getTopLeftCorner(a: number): number;
export function svgscene_getBottomRightCorner(a: number): number;
export function svgscene_setBackground(a: number, b: number): void;
export function svgscene_add(a: number, b: number): void;
export function svgscene_insert(a: number, b: number, c: number): void;
export function svgscene_remove(a: number, b: number): void;
export function svgscene_getObjectsFromIndices(a: number, b: number): number;
export function svgscene_setDivContainer(a: number, b: number): void;
export function svgscene_sleep(a: number, b: number): number;
export function svgscene_play(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function svgscene_makeFrame(a: number, b: number, c: number, d: number): number;
export function svgscene_setObjects(a: number, b: number): void;
export function svgscene_getObjects(a: number): number;
export function svgscene_wait(a: number, b: number): number;
export function svgscene_setCallback(a: number, b: number): void;
export function svgscene_callCallback(a: number): number;
export function svgscene_setClass(a: number, b: number, c: number, d: number): void;
export function svgscene_removeClass(a: number, b: number): void;
export function mathjax(a: number, b: number): number;
export function textToVector(a: number, b: number, c: number, d: number): number;
export function __wbg_genericscene_free(a: number): void;
export function genericscene_fromScene(a: number): number;
export function genericscene_fromSVGScene(a: number): number;
export function genericscene_isScene(a: number): number;
export function genericscene_isSVGScene(a: number): number;
export function genericscene_isVideoScene(a: number): number;
export function genericscene_getFps(a: number): number;
export function genericscene_getHeight(a: number): number;
export function genericscene_getWidth(a: number): number;
export function genericscene_renderFrame(a: number): void;
export function genericscene_clear(a: number): void;
export function genericscene_restore(a: number, b: number): void;
export function genericscene_saveState(a: number, b: number): void;
export function genericscene_setTopLeftCorner(a: number, b: number, c: number): void;
export function genericscene_setBottomRightCorner(a: number, b: number, c: number): void;
export function genericscene_getTopLeftCorner(a: number): number;
export function genericscene_getBottomRightCorner(a: number): number;
export function genericscene_setBackground(a: number, b: number): void;
export function genericscene_add(a: number, b: number): void;
export function genericscene_insert(a: number, b: number, c: number): void;
export function genericscene_remove(a: number, b: number): void;
export function genericscene_getObjects(a: number): number;
export function genericscene_getObjectsFromIndices(a: number, b: number): number;
export function genericscene_setCanvasContext(a: number, b: number): void;
export function genericscene_setDivContainer(a: number, b: number): void;
export function genericscene_sleep(a: number, b: number): number;
export function genericscene_setObjects(a: number, b: number): void;
export function genericscene_play(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function genericscene_makeFrame(a: number, b: number, c: number, d: number): number;
export function genericscene_wait(a: number, b: number): number;
export function genericscene_setCallback(a: number, b: number): void;
export function genericscene_callCallback(a: number): number;
export function genericscene_setClass(a: number, b: number, c: number, d: number): void;
export function genericscene_setStyle(a: number, b: number): void;
export function radian(a: number, b: number, c: number, d: number): number;
export function ellipticalArcPath(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number): number;
export function getBbox(a: number): number;
export function center(a: number, b: number): number;
export function factorial(a: number): number;
export function hexToColor(a: number, b: number, c: number): number;
export function bezier(a: number, b: number): number;
export function bezierNumber(a: number, b: number): number;
export function permutation(a: number, b: number): number;
export function choose(a: number, b: number): number;
export function distanceSquared(a: number, b: number, c: number, d: number): number;
export function interpolate(a: number, b: number, c: number): number;
export function interpolateTuple(a: number, b: number, c: number): number;
export function interpolateTuple3D(a: number, b: number, c: number): number;
export function interpolateColor(a: number, b: number, c: number): number;
export function pointsFromAnchorsAndHandles(a: number, b: number, c: number, d: number): number;
export function startNewPath(a: number, b: number): number;
export function hasNewPathBegun(a: number): number;
export function getNthSubpath(a: number, b: number): number;
export function insertNCurvesToPointList(a: number, b: number): number;
export function nullPointAlign(a: number, b: number): number;
export function alignPoints(a: number, b: number, c: number): number;
export function addNMoreSubobjects(a: number, b: number, c: number): number;
export function alignSubobjects(a: number, b: number, c: number, d: number): void;
export function alignData(a: number, b: number, c: number, d: number): number;
export function integerInterpolate(a: number, b: number, c: number): number;
export function lineAsCubicBezier(a: number, b: number, c: number, d: number): number;
export function quadraticBezierAsCubicBezier(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function considerPointsEquals(a: number, b: number, c: number, d: number): number;
export function sigmoid(a: number): number;
export function linear(a: number): number;
export function smooth(a: number, b: number): number;
export function smoothstep(a: number): number;
export function smootherstep(a: number): number;
export function smoothererstep(a: number): number;
export function rushInto(a: number, b: number): number;
export function rushFrom(a: number, b: number): number;
export function doubleSmooth(a: number): number;
export function thereAndBack(a: number, b: number): number;
export function thereAndBackWithPause(a: number, b: number): number;
export function runningStart(a: number, b: number): number;
export function notQuiteThere(a: number, b: number, c: number): number;
export function wiggle(a: number, b: number): number;
export function squishRateFunc(a: number, b: number, c: number, d: number): number;
export function lingering(a: number): number;
export function exponentialDecay(a: number, b: number): number;
export function easeInSine(a: number): number;
export function easeOutSine(a: number): number;
export function easeInOutSine(a: number): number;
export function easeInQuad(a: number): number;
export function easeOutQuad(a: number): number;
export function easeInOutQuad(a: number): number;
export function easeInCubic(a: number): number;
export function easeOutCubic(a: number): number;
export function easeInOutCubic(a: number): number;
export function easeInQuart(a: number): number;
export function easeOutQuart(a: number): number;
export function easeInOutQuart(a: number): number;
export function easeInQuint(a: number): number;
export function easeOutQuint(a: number): number;
export function easeInOutQuint(a: number): number;
export function easeInExpo(a: number): number;
export function easeOutExpo(a: number): number;
export function easeInOutExpo(a: number): number;
export function easeInCirc(a: number): number;
export function easeOutCirc(a: number): number;
export function easeInOutCirc(a: number): number;
export function easeInBack(a: number): number;
export function easeInOutBack(a: number): number;
export function easeInElastic(a: number): number;
export function easeOutElastic(a: number): number;
export function easeInOutElastic(a: number): number;
export function easeInBounce(a: number): number;
export function easeInOutBounce(a: number): number;
export function slowInto(a: number): number;
export function easeOutBounce(a: number): number;
export function easeOutBack(a: number): number;
export function __wbindgen_malloc(a: number, b: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number, d: number): number;
export const __wbindgen_export_2: WebAssembly.Table;
export function _dyn_core__ops__function__Fn__A_B___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf3018e53915a1917(a: number, b: number, c: number, d: number): number;
export function _dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h691c51e1fca6304a(a: number, b: number): number;
export function _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h36f54c9e7475dd01(a: number, b: number, c: number): void;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_free(a: number, b: number, c: number): void;
export function __wbindgen_exn_store(a: number): void;
export function wasm_bindgen__convert__closures__invoke2_mut__h271b87efd55193df(a: number, b: number, c: number, d: number): void;
