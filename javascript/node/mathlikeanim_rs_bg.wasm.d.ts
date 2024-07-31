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
export function wasmvectorobject_add(a: number, b: number): number;
export function wasmvectorobject_remove(a: number, b: number): number;
export function wasmvectorobject_getSubobject(a: number, b: number): number;
export function wasmvectorobject_sliceSubobjects(a: number, b: number, c: number, d: number): void;
export function wasmvectorobject_setSubobject(a: number, b: number, c: number): number;
export function wasmvectorobject_setSliceSubobjects(a: number, b: number, c: number, d: number, e: number): number;
export function wasmvectorobject_getFill(a: number): number;
export function wasmvectorobject_getStroke(a: number): number;
export function wasmvectorobject_getStrokeWidth(a: number): number;
export function wasmvectorobject_getLineCap(a: number, b: number): void;
export function wasmvectorobject_getLineJoin(a: number, b: number): void;
export function wasmvectorobject_getPartialCopy(a: number, b: number, c: number, d: number): number;
export function wasmvectorobject_getSubpaths(a: number): number;
export function wasmvectorobject_getPieces(a: number, b: number): number;
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
export function svgToVector(a: number, b: number, c: number, d: number, e: number, f: number): number;
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
export function __wbg_lexer_free(a: number): void;
export function lexer_new(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number, x: number, y: number, z: number, a1: number, b1: number, c1: number, d1: number, e1: number, f1: number, g1: number, h1: number, i1: number, j1: number, k1: number, l1: number): number;
export function lexer_getKeywords(a: number, b: number): void;
export function lexer_getSpecials(a: number, b: number): void;
export function lexer_getMethodDeclarations(a: number, b: number): void;
export function lexer_getIllegals(a: number, b: number): void;
export function lexer_getDeclarations(a: number, b: number): void;
export function lexer_getOperators(a: number, b: number): void;
export function lexer_getSeparators(a: number, b: number): void;
export function lexer_getLParens(a: number, b: number): void;
export function lexer_getRParens(a: number, b: number): void;
export function lexer_getFormatOpens(a: number, b: number): void;
export function lexer_getFormatCloses(a: number, b: number): void;
export function lexer_getStringOpenDelimiters(a: number, b: number): void;
export function lexer_getFormatedStringOpenDelimiters(a: number, b: number): void;
export function lexer_getFormatedStringCloseDelimiters(a: number, b: number): void;
export function lexer_getStringCloseDelimiters(a: number, b: number): void;
export function lexer_getCommentCharacters(a: number, b: number): void;
export function lexer_getAssignments(a: number, b: number): void;
export function lexer_getConstants(a: number, b: number): void;
export function lexer_isKeyword(a: number, b: number, c: number): number;
export function lexer_isSpecial(a: number, b: number, c: number): number;
export function lexer_isIllegal(a: number, b: number, c: number): number;
export function lexer_isFormatedStringOpenDelimiter(a: number, b: number, c: number): number;
export function lexer_isFormatedStringCloseDelimiter(a: number, b: number, c: number): number;
export function lexer_isMethodDeclaration(a: number, b: number, c: number): number;
export function lexer_isStringCloseDelimiter(a: number, b: number, c: number): number;
export function lexer_isCommentCharacter(a: number, b: number, c: number): number;
export function lexer_isWhitespace(a: number, b: number, c: number): number;
export function lexer_isDigit(a: number, b: number, c: number): number;
export function lexer_isQuote(a: number, b: number, c: number): number;
export function lexer_containsQuoteInitial(a: number, b: number, c: number): number;
export function lexer_isDeclaration(a: number, b: number, c: number): number;
export function lexer_isSeparator(a: number, b: number, c: number): number;
export function lexer_isOperator(a: number, b: number, c: number): number;
export function lexer_isLParen(a: number, b: number, c: number): number;
export function lexer_isRParen(a: number, b: number, c: number): number;
export function lexer_isNewline(a: number, b: number, c: number): number;
export function lexer_isConstant(a: number, b: number, c: number): number;
export function lexer_hasFormatedStringOpenInitial(a: number, b: number, c: number): number;
export function lexer_hasFormatedStringCloseInitial(a: number, b: number, c: number): number;
export function lexer_getClassIdentifierPattern(a: number, b: number): void;
export function lexer_containsOperator(a: number, b: number, c: number): number;
export function lexer_containsAssignment(a: number, b: number, c: number): number;
export function lexer_hasFormatOpen(a: number, b: number, c: number): number;
export function lexer_hasFormatClose(a: number, b: number, c: number): number;
export function lexer_removeLastOperator(a: number, b: number, c: number, d: number): void;
export function lexer_removeLastAssignment(a: number, b: number, c: number, d: number): void;
export function lexer_removeLastSeparator(a: number, b: number, c: number, d: number): void;
export function lexer_removeLastQuote(a: number, b: number, c: number, d: number): void;
export function lexer_hasOperatorInitial(a: number, b: number, c: number): number;
export function lexer_hasAssignmentInitial(a: number, b: number, c: number): number;
export function lexer_isAssignment(a: number, b: number, c: number): number;
export function lexer_hasSeparatorInitial(a: number, b: number, c: number): number;
export function lexer_containsQuote(a: number, b: number, c: number): number;
export function lexer_containsNonAlphabeticalOperator(a: number, b: number, c: number): number;
export function lexer_endsWithStringCloseDelimiter(a: number, b: number, c: number): number;
export function lexer_startsWithStringOpenDelimiter(a: number, b: number, c: number): number;
export function lexer_hasFormatStringClose(a: number, b: number, c: number): number;
export function lexer_clone(a: number): number;
export function lexer_removeFormatOpen(a: number, b: number, c: number, d: number): void;
export function lexer_startsWithCommentCharacter(a: number, b: number, c: number): number;
export function lexer_hasCommentInitial(a: number, b: number, c: number): number;
export function lexer_removeFormatClose(a: number, b: number, c: number, d: number): void;
export function lexer_removeStringOpenDelimiterIndex(a: number, b: number, c: number, d: number): void;
export function lexer_hasFormatStringOpen(a: number, b: number, c: number): number;
export function lexer_getStringOpeningDelimiterIndex(a: number, b: number, c: number, d: number): void;
export function lexer_getStringClosingDelimiterIndex(a: number, b: number, c: number, d: number): void;
export function lexer_getFormatedStringOpeningDelimiterIndex(a: number, b: number, c: number, d: number): void;
export function lexer_getFormatedStringClosingDelimiterIndex(a: number, b: number, c: number, d: number): void;
export function lexer_getTokens(a: number, b: number, c: number, d: number): void;
export function lexer_isStringOpenDelimiter(a: number, b: number, c: number): number;
export function lexer_removeStringCloseDelimiterIndex(a: number, b: number, c: number, d: number): void;
export function __wbg_token_free(a: number): void;
export function token_new(a: number, b: number, c: number): number;
export function token_getType(a: number): number;
export function __wbg_theme_free(a: number): void;
export function theme_new(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number, x: number, y: number, z: number, a1: number, b1: number, c1: number, d1: number, e1: number, f1: number, g1: number, h1: number, i1: number, j1: number, k1: number, l1: number): number;
export function theme_getKeywordColor(a: number, b: number): void;
export function theme_getSpecialColor(a: number, b: number): void;
export function theme_getIllegalColor(a: number, b: number): void;
export function theme_getDeclarationColor(a: number, b: number): void;
export function theme_getOperatorColor(a: number, b: number): void;
export function theme_getParenColor(a: number, b: number): void;
export function theme_getConstantColor(a: number, b: number): void;
export function theme_getNumberColor(a: number, b: number): void;
export function theme_getStringColor(a: number, b: number): void;
export function theme_getIdentifierColor(a: number, b: number): void;
export function theme_getAssignColor(a: number, b: number): void;
export function theme_getClassIdentifierColor(a: number, b: number): void;
export function theme_getSeparatorColor(a: number, b: number): void;
export function theme_getMethodDeclarationColor(a: number, b: number): void;
export function theme_getMethodIdentifierColor(a: number, b: number): void;
export function theme_getFormattedStringColor(a: number, b: number): void;
export function theme_getFormatOpenColor(a: number, b: number): void;
export function theme_getFormatCloseColor(a: number, b: number): void;
export function theme_clone(a: number): number;
export function theme_getCommentColor(a: number, b: number): void;
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
export function token_getLiteral(a: number, b: number): void;
export function easeOutBounce(a: number): number;
export function easeOutBack(a: number): number;
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
export function wasmthreedobject_fromVectorObject(a: number): number;
export function wasmthreedobject_getSubobjectsRecursively(a: number, b: number): void;
export function threeDAxes(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number, x: number, y: number, z: number, a1: number, b1: number, c1: number, d1: number, e1: number, f1: number, g1: number): number;
export function coordsToPoint3D(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number): number;
export function pointToCoords3D(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number): number;
export function parametricPlotInAxes3D(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number): number;
export function plotInAxes3D(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number): number;
export function parametricLinePlotInAxes3D(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number): number;
export function getPythonLexer(): number;
export function getGithubDark(): number;
export function textToVector(a: number, b: number, c: number, d: number, e: number, f: number, g: number): number;
export function __wbg_nodescene_free(a: number): void;
export function nodescene_new_js(a: number, b: number, c: number): number;
export function nodescene_getContext(a: number): number;
export function nodescene_initContext(a: number, b: number): void;
export function nodescene_isSVG(a: number): number;
export function nodescene_getFps(a: number): number;
export function nodescene_toggleSaveFrames(a: number): void;
export function nodescene_initFFmpegPartialMovie(a: number, b: number, c: number, d: number, e: number, f: number, g: number): void;
export function nodescene_closeFFmpegPartialMovie(a: number): number;
export function nodescene_setFileNamePrefix(a: number, b: number, c: number): void;
export function nodescene_getHeight(a: number): number;
export function nodescene_getWidth(a: number): number;
export function nodescene_renderFrame(a: number): number;
export function nodescene_clear(a: number): void;
export function nodescene_restore(a: number, b: number): void;
export function nodescene_saveState(a: number, b: number): void;
export function nodescene_setTopLeftCorner(a: number, b: number, c: number): void;
export function nodescene_setBottomRightCorner(a: number, b: number, c: number): void;
export function nodescene_getTopLeftCorner(a: number): number;
export function nodescene_getBottomRightCorner(a: number): number;
export function nodescene_setBackground(a: number, b: number): void;
export function nodescene_add(a: number, b: number): void;
export function nodescene_insert(a: number, b: number, c: number): void;
export function nodescene_remove(a: number, b: number): void;
export function nodescene_saveFramePNG(a: number, b: number, c: number): number;
export function nodescene_saveFrameSVG(a: number, b: number, c: number): number;
export function nodescene_getObjects(a: number): number;
export function nodescene_getObjectsFromIndices(a: number, b: number): number;
export function nodescene_sleep(a: number, b: number): number;
export function nodescene_setObjects(a: number, b: number): void;
export function nodescene_play(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function nodescene_makeFrame(a: number, b: number, c: number, d: number): number;
export function nodescene_wait(a: number, b: number): number;
export function nodescene_setOnRendered(a: number, b: number): void;
export function nodescene_onRendered(a: number): number;
export function makeTimings(a: number, b: number, c: number): void;
export function animationGroup(a: number, b: number, c: number, d: number, e: number): number;
export function create(a: number, b: number): number;
export function drawStrokeThenFill(a: number, b: number, c: number, d: number): number;
export function write(a: number, b: number, c: number, d: number, e: number): number;
export function fadeIn(a: number, b: number, c: number, d: number): number;
export function fadeOut(a: number, b: number, c: number, d: number): number;
export function growArrowWithFinalTip(a: number, b: number): number;
export function growArrowWithInitialTip(a: number, b: number): number;
export function growArrowWithTipsAtBothEnds(a: number, b: number): number;
export function growFromCenter(a: number, b: number): number;
export function morphShape(a: number, b: number, c: number): number;
export function moveCameraNode(a: number, b: number, c: number, d: number): void;
export function rotateAnimation(a: number, b: number, c: number): number;
export function scaleInPlace(a: number, b: number, c: number): number;
export function setFillAnimation(a: number, b: number, c: number): number;
export function setStrokeAnimation(a: number, b: number, c: number): number;
export function shiftAnimation(a: number, b: number, c: number): number;
export function showTemporaily(a: number, b: number): number;
export function spinningGrow(a: number, b: number, c: number): number;
export function mathjax(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function codeObject(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function __wbindgen_malloc(a: number, b: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number, d: number): number;
export const __wbindgen_export_2: WebAssembly.Table;
export function _dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h2ff4523e31cc4f60(a: number, b: number): number;
export function _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h36f54c9e7475dd01(a: number, b: number, c: number): void;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_free(a: number, b: number, c: number): void;
export function __wbindgen_exn_store(a: number): void;
export function wasm_bindgen__convert__closures__invoke2_mut__h271b87efd55193df(a: number, b: number, c: number, d: number): void;
