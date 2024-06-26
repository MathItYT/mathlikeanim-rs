/* tslint:disable */
/* eslint-disable */
/**
* @param {WasmVectorObject} shape
* @param {number} tip_side_length
* @param {WasmColor} tip_color
* @returns {WasmVectorObject}
*/
export function addFinalTip(shape: WasmVectorObject, tip_side_length: number, tip_color: WasmColor): WasmVectorObject;
/**
* @param {WasmVectorObject} shape
* @param {number} tip_side_length
* @param {WasmColor} tip_color
* @returns {WasmVectorObject}
*/
export function addInitialTip(shape: WasmVectorObject, tip_side_length: number, tip_color: WasmColor): WasmVectorObject;
/**
* @param {WasmVectorObject} shape
* @param {number} tip_side_length
* @param {WasmColor} tip_color
* @returns {WasmVectorObject}
*/
export function addBothSidesTips(shape: WasmVectorObject, tip_side_length: number, tip_color: WasmColor): WasmVectorObject;
/**
* @param {Array<any>} center
* @param {number} radius
* @param {number} start_angle
* @param {number} end_angle
* @param {number | undefined} [num_points]
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function arc(center: Array<any>, radius: number, start_angle: number, end_angle: number, num_points?: number, stroke_color?: WasmColor, fill_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Array<any>} center
* @param {number} radius
* @param {number | undefined} [num_points]
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function circle(center: Array<any>, radius: number, num_points?: number, stroke_color?: WasmColor, fill_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Array<any>} center
* @param {number} x_radius
* @param {number} y_radius
* @param {number} start_angle
* @param {number} end_angle
* @param {number | undefined} [num_points]
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function ellipticalArc(center: Array<any>, x_radius: number, y_radius: number, start_angle: number, end_angle: number, num_points?: number, stroke_color?: WasmColor, fill_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Array<any>} center
* @param {number} x_radius
* @param {number} y_radius
* @param {number | undefined} [num_points]
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function ellipse(center: Array<any>, x_radius: number, y_radius: number, num_points?: number, stroke_color?: WasmColor, fill_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Array<any>} center
* @param {number} inner_radius
* @param {number} outer_radius
* @param {number} start_angle
* @param {number} end_angle
* @param {number | undefined} [num_points]
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function annularSector(center: Array<any>, inner_radius: number, outer_radius: number, start_angle: number, end_angle: number, num_points?: number, stroke_color?: WasmColor, fill_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Array<any>} start_point
* @param {Array<any>} end_point
* @param {WasmColor | undefined} [stroke_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function line(start_point: Array<any>, end_point: Array<any>, stroke_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Array<any>} points
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function polygon(points: Array<any>, stroke_color?: WasmColor, fill_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Array<any>} center
* @param {number} side_length
* @param {number} num_sides
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function regularPolygon(center: Array<any>, side_length: number, num_sides: number, stroke_color?: WasmColor, fill_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Array<any>} center
* @param {number} side_length
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function square(center: Array<any>, side_length: number, stroke_color?: WasmColor, fill_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Array<any>} center
* @param {number} width
* @param {number} height
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function rectangle(center: Array<any>, width: number, height: number, stroke_color?: WasmColor, fill_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Array<any>} center
* @param {number} side_length
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function equilateralTriangle(center: Array<any>, side_length: number, stroke_color?: WasmColor, fill_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Array<any>} point1
* @param {Array<any>} point2
* @param {Array<any>} point3
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function triangle(point1: Array<any>, point2: Array<any>, point3: Array<any>, stroke_color?: WasmColor, fill_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Array<any>} point1
* @param {Array<any>} point2
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function rightTriangle(point1: Array<any>, point2: Array<any>, stroke_color?: WasmColor, fill_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {number} x_min
* @param {number} x_max
* @param {number} x_step
* @param {number} y_min
* @param {number} y_max
* @param {number} y_step
* @param {Array<any>} center
* @param {number | undefined} [x_length]
* @param {number | undefined} [y_length]
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @param {boolean | undefined} [add_x_ticks]
* @param {boolean | undefined} [add_y_ticks]
* @param {number | undefined} [x_tick_size]
* @param {number | undefined} [y_tick_size]
* @param {boolean | undefined} [add_x_tip]
* @param {boolean | undefined} [add_y_tip]
* @returns {WasmVectorObject}
*/
export function axes(x_min: number, x_max: number, x_step: number, y_min: number, y_max: number, y_step: number, center: Array<any>, x_length?: number, y_length?: number, color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number, add_x_ticks?: boolean, add_y_ticks?: boolean, x_tick_size?: number, y_tick_size?: number, add_x_tip?: boolean, add_y_tip?: boolean): WasmVectorObject;
/**
* @param {WasmVectorObject} axes
* @param {number} x
* @param {number} y
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @returns {Array<any>}
*/
export function coordsToPoint(axes: WasmVectorObject, x: number, y: number, x_min: number, x_max: number, y_min: number, y_max: number): Array<any>;
/**
* @param {WasmVectorObject} axes
* @param {Array<any>} point
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @returns {Array<any>}
*/
export function pointToCoords(axes: WasmVectorObject, point: Array<any>, x_min: number, x_max: number, y_min: number, y_max: number): Array<any>;
/**
* @param {Function} f
* @param {number} t_min
* @param {number} t_max
* @param {number} t_step
* @param {WasmVectorObject} axes
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function parametricPlotInAxes(f: Function, t_min: number, t_max: number, t_step: number, axes: WasmVectorObject, x_min: number, x_max: number, y_min: number, y_max: number, color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Function} f
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {number} x1
* @param {number} x2
* @param {number} x_step
* @param {WasmVectorObject} axes
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function plotInAxes(f: Function, x_min: number, x_max: number, y_min: number, y_max: number, x1: number, x2: number, x_step: number, axes: WasmVectorObject, color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Function} f
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {number} x_1
* @param {number} x_2
* @param {number} x_step
* @param {number} y_1
* @param {number} y_2
* @param {number} y_step
* @param {WasmVectorObject} axes
* @param {WasmColor | undefined} color
* @param {number | undefined} stroke_width
* @param {string | undefined} line_cap
* @param {string | undefined} line_join
* @param {number | undefined} index
* @param {Float64Array} intervals
* @returns {WasmVectorObject}
*/
export function contourPlotInAxes(f: Function, x_min: number, x_max: number, y_min: number, y_max: number, x_1: number, x_2: number, x_step: number, y_1: number, y_2: number, y_step: number, axes: WasmVectorObject, color: WasmColor | undefined, stroke_width: number | undefined, line_cap: string | undefined, line_join: string | undefined, index: number | undefined, intervals: Float64Array): WasmVectorObject;
/**
* @param {WasmVectorObject} axes
* @param {WasmVectorObject} plot
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {number} x1
* @param {number} x2
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function areaUnderCurve(axes: WasmVectorObject, plot: WasmVectorObject, x_min: number, x_max: number, y_min: number, y_max: number, x1: number, x2: number, color?: WasmColor, index?: number): WasmVectorObject;
/**
* @param {Function} f
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {number} direction
* @param {number} x_1
* @param {number} x_2
* @param {number} n_rects
* @param {WasmVectorObject} axes
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function riemannRectanglesForPlot(f: Function, x_min: number, x_max: number, y_min: number, y_max: number, direction: number, x_1: number, x_2: number, n_rects: number, axes: WasmVectorObject, stroke_color?: WasmColor, fill_color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Function} f
* @param {number} x_1
* @param {number} x_2
* @param {number} length
* @param {WasmVectorObject} axes
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function secantLineForPlot(f: Function, x_1: number, x_2: number, length: number, axes: WasmVectorObject, x_min: number, x_max: number, y_min: number, y_max: number, color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Function} f
* @param {number} t_min
* @param {number} t_max
* @param {number} t_step
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function parametricFunction(f: Function, t_min: number, t_max: number, t_step: number, color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {Function} f
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {number} x_step
* @param {number} y_step
* @param {WasmColor | undefined} color
* @param {number | undefined} stroke_width
* @param {string | undefined} line_cap
* @param {string | undefined} line_join
* @param {number | undefined} index
* @param {Float64Array} intervals
* @returns {WasmVectorObject}
*/
export function contourPlot(f: Function, x_min: number, x_max: number, y_min: number, y_max: number, x_step: number, y_step: number, color: WasmColor | undefined, stroke_width: number | undefined, line_cap: string | undefined, line_join: string | undefined, index: number | undefined, intervals: Float64Array): WasmVectorObject;
/**
* @param {Function} f
* @param {number} x_min
* @param {number} x_max
* @param {number} x_step
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function realFunction(f: Function, x_min: number, x_max: number, x_step: number, color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number): WasmVectorObject;
/**
* @param {number} x_min
* @param {number} x_max
* @param {number} x_step
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @param {Array<any> | undefined} [center]
* @param {number | undefined} [length]
* @param {boolean | undefined} [add_tip]
* @param {boolean | undefined} [add_ticks]
* @param {number | undefined} [tick_size]
* @param {number | undefined} [angle]
* @returns {WasmVectorObject}
*/
export function numberLine(x_min: number, x_max: number, x_step: number, color?: WasmColor, stroke_width?: number, line_cap?: string, line_join?: string, index?: number, center?: Array<any>, length?: number, add_tip?: boolean, add_ticks?: boolean, tick_size?: number, angle?: number): WasmVectorObject;
/**
* @param {WasmVectorObject} number_line
* @param {number} number
* @param {number} x_min
* @param {number} x_max
* @returns {Array<any>}
*/
export function numberToPoint(number_line: WasmVectorObject, number: number, x_min: number, x_max: number): Array<any>;
/**
* @param {WasmVectorObject} number_line
* @param {Array<any>} point
* @param {number} x_min
* @param {number} x_max
* @returns {number}
*/
export function pointToNumber(number_line: WasmVectorObject, point: Array<any>, x_min: number, x_max: number): number;
/**
* @param {WasmVectorObject} number_line
* @param {Array<any>} numbers
* @param {Function} number_to_vector
* @param {number} x_min
* @param {number} x_max
* @param {number} height
* @param {Array<any> | undefined} [direction]
* @param {number | undefined} [buff]
* @param {number | undefined} [index]
* @returns {Promise<WasmVectorObject>}
*/
export function getNumbersTex(number_line: WasmVectorObject, numbers: Array<any>, number_to_vector: Function, x_min: number, x_max: number, height: number, direction?: Array<any>, buff?: number, index?: number): Promise<WasmVectorObject>;
/**
* @param {string} svg
* @returns {WasmVectorObject}
*/
export function svgToVector(svg: string): WasmVectorObject;
/**
* @param {number} angle
* @param {number} axis
* @returns {Array<any>}
*/
export function rotMatrix(angle: number, axis: number): Array<any>;
/**
* @param {Array<any>} a
* @param {Array<any>} b
* @returns {Array<any>}
*/
export function matrixProduct(a: Array<any>, b: Array<any>): Array<any>;
/**
* @param {number} phi
* @param {number} theta
* @param {number} gamma
* @returns {Array<any>}
*/
export function rotMatrixEuler(phi: number, theta: number, gamma: number): Array<any>;
/**
* @param {Array<any>} a
* @returns {Array<any>}
*/
export function transposeMatrix(a: Array<any>): Array<any>;
/**
* @param {Array<any>} matrix
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function applyMatrix(matrix: Array<any>, points: Array<any>): Array<any>;
/**
* @param {Array<any>} points
* @param {Array<any>} shift
* @returns {Array<any>}
*/
export function shiftPoints3D(points: Array<any>, shift: Array<any>): Array<any>;
/**
* @param {WasmGradientImageOrColor} color
* @returns {WasmGradientImageOrColor}
*/
export function ensureValidThreeDColor(color: WasmGradientImageOrColor): WasmGradientImageOrColor;
/**
* @param {WasmColor} color
* @param {Array<any>} point
* @param {Array<any>} unit_normal
* @param {WasmLightSource} light_source
* @returns {WasmColor}
*/
export function getShadedRgb(color: WasmColor, point: Array<any>, unit_normal: Array<any>, light_source: WasmLightSource): WasmColor;
/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getStartCorner(points: Array<any>): Array<any>;
/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getEndCorner(points: Array<any>): Array<any>;
/**
* @param {Array<any>} a
* @param {Array<any>} b
* @returns {Array<any>}
*/
export function crossProduct(a: Array<any>, b: Array<any>): Array<any>;
/**
* @param {Array<any>} v1
* @param {Array<any>} v2
* @returns {Array<any>}
*/
export function getUnitNormal(v1: Array<any>, v2: Array<any>): Array<any>;
/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getStartAnchors(points: Array<any>): Array<any>;
/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getEndAnchors(points: Array<any>): Array<any>;
/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getAnchors(points: Array<any>): Array<any>;
/**
* @param {Array<any>} points
* @param {number} index
* @returns {Array<any>}
*/
export function getCornerUnitNormal(points: Array<any>, index: number): Array<any>;
/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getStartCornerUnitNormal(points: Array<any>): Array<any>;
/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getEndCornerUnitNormal(points: Array<any>): Array<any>;
/**
* @param {WasmGradientImageOrColor} color
* @param {Array<any>} points
* @param {WasmLightSource} light_source
* @param {WasmCamera} camera
* @returns {WasmGradientImageOrColor}
*/
export function getShadedColor(color: WasmGradientImageOrColor, points: Array<any>, light_source: WasmLightSource, camera: WasmCamera): WasmGradientImageOrColor;
/**
* @param {Array<any>} points
* @param {WasmCamera} camera
* @returns {Array<any>}
*/
export function projectPoints(points: Array<any>, camera: WasmCamera): Array<any>;
/**
* @param {Array<any>} point1
* @param {Array<any>} point2
* @returns {Array<any>}
*/
export function lineAsCubicBezier3D(point1: Array<any>, point2: Array<any>): Array<any>;
/**
* @param {number} num_anim_funcs
* @param {number} lag_ratio
* @returns {Float64Array}
*/
export function makeTimings(num_anim_funcs: number, lag_ratio: number): Float64Array;
/**
* @param {WasmVectorObject} vec_obj
* @param {(Function)[]} anim_funcs
* @param {number} lag_ratio
* @param {number} t
* @returns {WasmVectorObject}
*/
export function animationGroup(vec_obj: WasmVectorObject, anim_funcs: (Function)[], lag_ratio: number, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function create(vec_obj: WasmVectorObject, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function drawStrokeThenFill(vec_obj: WasmVectorObject, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} lag_ratio
* @param {number} t
* @returns {WasmVectorObject}
*/
export function write(vec_obj: WasmVectorObject, lag_ratio: number, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} scale_factor
* @param {Array<any>} shift
* @param {number} t
* @returns {WasmVectorObject}
*/
export function fadeIn(vec_obj: WasmVectorObject, scale_factor: number, shift: Array<any>, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} scale_factor
* @param {Array<any>} shift
* @param {number} t
* @returns {WasmVectorObject}
*/
export function fadeOut(vec_obj: WasmVectorObject, scale_factor: number, shift: Array<any>, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function growArrowWithFinalTip(vec_obj: WasmVectorObject, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function growArrowWithInitialTip(vec_obj: WasmVectorObject, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function growArrowWithTipsAtBothEnds(vec_obj: WasmVectorObject, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function growFromCenter(vec_obj: WasmVectorObject, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} original
* @param {WasmVectorObject} target
* @param {number} t
* @returns {WasmVectorObject}
*/
export function morphShape(original: WasmVectorObject, target: WasmVectorObject, t: number): WasmVectorObject;
/**
* @param {Array<any>} top_left_corner
* @param {Array<any>} bottom_right_corner
* @param {VideoScene} scene
* @param {number} t
*/
export function moveCameraVideo(top_left_corner: Array<any>, bottom_right_corner: Array<any>, scene: VideoScene, t: number): void;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} angle
* @param {number} t
* @returns {WasmVectorObject}
*/
export function rotateAnimation(vec_obj: WasmVectorObject, angle: number, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} scale_factor
* @param {number} t
* @returns {WasmVectorObject}
*/
export function scaleInPlace(vec_obj: WasmVectorObject, scale_factor: number, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {WasmColor} target_fill
* @param {number} t
* @returns {WasmVectorObject}
*/
export function setFillAnimation(vec_obj: WasmVectorObject, target_fill: WasmColor, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {WasmColor} target_stroke
* @param {number} t
* @returns {WasmVectorObject}
*/
export function setStrokeAnimation(vec_obj: WasmVectorObject, target_stroke: WasmColor, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {Array<any>} shift
* @param {number} t
* @returns {WasmVectorObject}
*/
export function shiftAnimation(vec_obj: WasmVectorObject, shift: Array<any>, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function showTemporaily(vec_obj: WasmVectorObject, t: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} angle
* @param {number} t
* @returns {WasmVectorObject}
*/
export function spinningGrow(vec_obj: WasmVectorObject, angle: number, t: number): WasmVectorObject;
/**
* @param {number} ux
* @param {number} uy
* @param {number} vx
* @param {number} vy
* @returns {number}
*/
export function radian(ux: number, uy: number, vx: number, vy: number): number;
/**
* @param {Array<any>} last_move
* @param {number} rx
* @param {number} ry
* @param {number} rotation
* @param {boolean} large_arc
* @param {boolean} sweep
* @param {number} x
* @param {number} y
* @returns {Array<any>}
*/
export function ellipticalArcPath(last_move: Array<any>, rx: number, ry: number, rotation: number, large_arc: boolean, sweep: boolean, x: number, y: number): Array<any>;
/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getBbox(points: Array<any>): Array<any>;
/**
* @param {Array<any>} points
* @param {Array<any>} center_if_no_points
* @returns {Array<any>}
*/
export function center(points: Array<any>, center_if_no_points: Array<any>): Array<any>;
/**
* @param {bigint} n
* @returns {bigint}
*/
export function factorial(n: bigint): bigint;
/**
* @param {string} hex
* @param {number} a
* @returns {WasmColor}
*/
export function hexToColor(hex: string, a: number): WasmColor;
/**
* @param {Array<any>} points
* @param {number} t
* @returns {Array<any>}
*/
export function bezier(points: Array<any>, t: number): Array<any>;
/**
* @param {Array<any>} numbers
* @param {number} t
* @returns {number}
*/
export function bezierNumber(numbers: Array<any>, t: number): number;
/**
* @param {bigint} n
* @param {bigint} r
* @returns {bigint}
*/
export function permutation(n: bigint, r: bigint): bigint;
/**
* @param {bigint} n
* @param {bigint} r
* @returns {bigint}
*/
export function choose(n: bigint, r: bigint): bigint;
/**
* @param {number} x1
* @param {number} y1
* @param {number} x2
* @param {number} y2
* @returns {number}
*/
export function distanceSquared(x1: number, y1: number, x2: number, y2: number): number;
/**
* @param {number} x
* @param {number} y
* @param {number} t
* @returns {number}
*/
export function interpolate(x: number, y: number, t: number): number;
/**
* @param {Array<any>} x
* @param {Array<any>} y
* @param {number} t
* @returns {Array<any>}
*/
export function interpolateTuple(x: Array<any>, y: Array<any>, t: number): Array<any>;
/**
* @param {Array<any>} x
* @param {Array<any>} y
* @param {number} t
* @returns {Array<any>}
*/
export function interpolateTuple3D(x: Array<any>, y: Array<any>, t: number): Array<any>;
/**
* @param {WasmColor} x
* @param {WasmColor} y
* @param {number} t
* @returns {WasmColor}
*/
export function interpolateColor(x: WasmColor, y: WasmColor, t: number): WasmColor;
/**
* @param {Array<any>} anchors1
* @param {Array<any>} handles1
* @param {Array<any>} handles2
* @param {Array<any>} anchors2
* @returns {Array<any>}
*/
export function pointsFromAnchorsAndHandles(anchors1: Array<any>, handles1: Array<any>, handles2: Array<any>, anchors2: Array<any>): Array<any>;
/**
* @param {Array<any>} points
* @param {Array<any>} point
* @returns {Array<any>}
*/
export function startNewPath(points: Array<any>, point: Array<any>): Array<any>;
/**
* @param {Array<any>} points
* @returns {boolean}
*/
export function hasNewPathBegun(points: Array<any>): boolean;
/**
* @param {Array<any>} points
* @param {number} n
* @returns {Array<any>}
*/
export function getNthSubpath(points: Array<any>, n: number): Array<any>;
/**
* @param {number} n
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function insertNCurvesToPointList(n: number, points: Array<any>): Array<any>;
/**
* @param {WasmVectorObject} vec_obj1
* @param {WasmVectorObject} vec_obj2
* @returns {Array<any>}
*/
export function nullPointAlign(vec_obj1: WasmVectorObject, vec_obj2: WasmVectorObject): Array<any>;
/**
* @param {Array<any>} points1
* @param {Array<any>} points2
* @param {Array<any>} center_if_no_points
* @returns {Array<any>}
*/
export function alignPoints(points1: Array<any>, points2: Array<any>, center_if_no_points: Array<any>): Array<any>;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} n
* @param {Array<any>} center_if_no_points
* @returns {WasmVectorObject}
*/
export function addNMoreSubobjects(vec_obj: WasmVectorObject, n: number, center_if_no_points: Array<any>): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj1
* @param {WasmVectorObject} vec_obj2
* @param {Array<any>} center_if_no_points
* @returns {(WasmVectorObject)[]}
*/
export function alignSubobjects(vec_obj1: WasmVectorObject, vec_obj2: WasmVectorObject, center_if_no_points: Array<any>): (WasmVectorObject)[];
/**
* @param {WasmVectorObject} vec_obj1
* @param {WasmVectorObject} vec_obj2
* @param {boolean} skip_point_align
* @param {Array<any>} center_if_no_points
* @returns {Array<any>}
*/
export function alignData(vec_obj1: WasmVectorObject, vec_obj2: WasmVectorObject, skip_point_align: boolean, center_if_no_points: Array<any>): Array<any>;
/**
* @param {number} x
* @param {number} y
* @param {number} t
* @returns {Array<any>}
*/
export function integerInterpolate(x: number, y: number, t: number): Array<any>;
/**
* @param {number} x1
* @param {number} y1
* @param {number} x2
* @param {number} y2
* @returns {Array<any>}
*/
export function lineAsCubicBezier(x1: number, y1: number, x2: number, y2: number): Array<any>;
/**
* @param {number} x1
* @param {number} y1
* @param {number} x2
* @param {number} y2
* @param {number} x3
* @param {number} y3
* @returns {Array<any>}
*/
export function quadraticBezierAsCubicBezier(x1: number, y1: number, x2: number, y2: number, x3: number, y3: number): Array<any>;
/**
* @param {number} x1
* @param {number} y1
* @param {number} x2
* @param {number} y2
* @returns {boolean}
*/
export function considerPointsEquals(x1: number, y1: number, x2: number, y2: number): boolean;
/**
* @param {number} t
* @returns {number}
*/
export function sigmoid(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function linear(t: number): number;
/**
* @param {number} t
* @param {number} inflection
* @returns {number}
*/
export function smooth(t: number, inflection: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function smoothstep(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function smootherstep(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function smoothererstep(t: number): number;
/**
* @param {number} t
* @param {number} inflection
* @returns {number}
*/
export function rushInto(t: number, inflection: number): number;
/**
* @param {number} t
* @param {number} inflection
* @returns {number}
*/
export function rushFrom(t: number, inflection: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function slowInto(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function doubleSmooth(t: number): number;
/**
* @param {number} t
* @param {number} inflection
* @returns {number}
*/
export function thereAndBack(t: number, inflection: number): number;
/**
* @param {number} t
* @param {number} pause_ratio
* @returns {number}
*/
export function thereAndBackWithPause(t: number, pause_ratio: number): number;
/**
* @param {number} t
* @param {number} pull_factor
* @returns {number}
*/
export function runningStart(t: number, pull_factor: number): number;
/**
* @param {Function} func
* @param {number} t
* @param {number} proportion
* @returns {number}
*/
export function notQuiteThere(func: Function, t: number, proportion: number): number;
/**
* @param {number} t
* @param {number} wiggles
* @returns {number}
*/
export function wiggle(t: number, wiggles: number): number;
/**
* @param {Function} func
* @param {number} t
* @param {number} a
* @param {number} b
* @returns {number}
*/
export function squishRateFunc(func: Function, t: number, a: number, b: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function lingering(t: number): number;
/**
* @param {number} t
* @param {number} half_life
* @returns {number}
*/
export function exponentialDecay(t: number, half_life: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInSine(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeOutSine(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInOutSine(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInQuad(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeOutQuad(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInOutQuad(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInCubic(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeOutCubic(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInOutCubic(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInQuart(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeOutQuart(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInOutQuart(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInQuint(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeOutQuint(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInOutQuint(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInExpo(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeOutExpo(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInOutExpo(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInCirc(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeOutCirc(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInOutCirc(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInBack(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeOutBack(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInOutBack(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInElastic(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeOutElastic(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInOutElastic(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeOutBounce(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInBounce(t: number): number;
/**
* @param {number} t
* @returns {number}
*/
export function easeInOutBounce(t: number): number;
/**
* @param {string} expression
* @returns {Promise<WasmVectorObject>}
*/
export function mathjax(expression: string): Promise<WasmVectorObject>;
/**
* @param {string} text
* @param {string} font_family
* @returns {Promise<WasmVectorObject>}
*/
export function textToVector(text: string, font_family: string): Promise<WasmVectorObject>;
/**
*/
export class GenericScene {
  free(): void;
/**
* @param {VideoScene} scene
* @returns {GenericScene}
*/
  static fromVideoScene(scene: VideoScene): GenericScene;
/**
* @returns {boolean}
*/
  isScene(): boolean;
/**
* @returns {boolean}
*/
  isSVGScene(): boolean;
/**
* @returns {boolean}
*/
  isVideoScene(): boolean;
/**
* @returns {bigint}
*/
  getFps(): bigint;
/**
* @returns {bigint}
*/
  getHeight(): bigint;
/**
* @returns {bigint}
*/
  getWidth(): bigint;
/**
*/
  renderFrame(): void;
/**
*/
  clear(): void;
/**
* @param {number} n
*/
  restore(n: number): void;
/**
* @param {number} n
*/
  saveState(n: number): void;
/**
* @param {number} x
* @param {number} y
*/
  setTopLeftCorner(x: number, y: number): void;
/**
* @param {number} x
* @param {number} y
*/
  setBottomRightCorner(x: number, y: number): void;
/**
* @returns {Array<any>}
*/
  getTopLeftCorner(): Array<any>;
/**
* @returns {Array<any>}
*/
  getBottomRightCorner(): Array<any>;
/**
* @param {WasmGradientImageOrColor} color
*/
  setBackground(color: WasmGradientImageOrColor): void;
/**
* @param {WasmVectorObject} object
*/
  add(object: WasmVectorObject): void;
/**
* @param {number} index
* @param {WasmVectorObject} object
*/
  insert(index: number, object: WasmVectorObject): void;
/**
* @param {number} index
*/
  remove(index: number): void;
/**
* @returns {Array<any>}
*/
  getObjects(): Array<any>;
/**
* @param {Array<any>} object_indices
* @returns {Map<any, any>}
*/
  getObjectsFromIndices(object_indices: Array<any>): Map<any, any>;
/**
* @param {any} context
*/
  setCanvasContext(context: any): void;
/**
* @param {string} path
* @param {string | undefined} [codec]
* @param {string | undefined} [pix_fmt]
* @param {string | undefined} [qp]
*/
  initFFmpeg(path: string, codec?: string, pix_fmt?: string, qp?: string): void;
/**
*/
  closeFFmpeg(): void;
/**
* @param {number} duration_in_ms
* @returns {Promise<void>}
*/
  sleep(duration_in_ms: number): Promise<void>;
/**
* @param {Array<any>} objects
*/
  setObjects(objects: Array<any>): void;
/**
* @param {Function} animation_func
* @param {Uint32Array} object_indices
* @param {bigint} duration_in_frames
* @param {Function} rate_func
* @returns {Promise<void>}
*/
  play(animation_func: Function, object_indices: Uint32Array, duration_in_frames: bigint, rate_func: Function): Promise<void>;
/**
* @param {Function} animation_func
* @param {Array<any>} objects
* @param {number} t
* @returns {Promise<void>}
*/
  makeFrame(animation_func: Function, objects: Array<any>, t: number): Promise<void>;
/**
* @param {bigint} duration_in_frames
* @returns {Promise<void>}
*/
  wait(duration_in_frames: bigint): Promise<void>;
/**
* @param {Function} callback
*/
  setCallback(callback: Function): void;
/**
* @returns {Promise<void>}
*/
  callCallback(): Promise<void>;
}
/**
*/
export class VideoScene {
  free(): void;
/**
* @param {bigint} width
* @param {bigint} height
* @param {bigint} fps
*/
  constructor(width: bigint, height: bigint, fps: bigint);
/**
* @returns {bigint}
*/
  getFps(): bigint;
/**
* @returns {bigint}
*/
  getHeight(): bigint;
/**
* @returns {bigint}
*/
  getWidth(): bigint;
/**
*/
  renderFrame(): void;
/**
*/
  clear(): void;
/**
* @param {number} n
*/
  restore(n: number): void;
/**
* @param {number} n
*/
  saveState(n: number): void;
/**
* @param {number} x
* @param {number} y
*/
  setTopLeftCorner(x: number, y: number): void;
/**
* @param {number} x
* @param {number} y
*/
  setBottomRightCorner(x: number, y: number): void;
/**
* @returns {Array<any>}
*/
  getTopLeftCorner(): Array<any>;
/**
* @returns {Array<any>}
*/
  getBottomRightCorner(): Array<any>;
/**
* @param {WasmGradientImageOrColor} background
*/
  setBackground(background: WasmGradientImageOrColor): void;
/**
* @param {WasmVectorObject} vec_obj
*/
  add(vec_obj: WasmVectorObject): void;
/**
* @param {number} index
* @param {WasmVectorObject} vec_obj
*/
  insert(index: number, vec_obj: WasmVectorObject): void;
/**
* @param {number} index
*/
  remove(index: number): void;
/**
* @returns {Array<any>}
*/
  getObjects(): Array<any>;
/**
* @param {Array<any>} object_indices
* @returns {Map<any, any>}
*/
  getObjectsFromIndices(object_indices: Array<any>): Map<any, any>;
/**
* @param {any} context
*/
  setCanvasContext(context: any): void;
/**
* @param {number} duration_in_ms
* @returns {Promise<void>}
*/
  sleep(duration_in_ms: number): Promise<void>;
/**
* @param {Array<any>} objects
*/
  setObjects(objects: Array<any>): void;
/**
* @param {Function} animation_func
* @param {Uint32Array} object_indices
* @param {bigint} duration_in_frames
* @param {Function} rate_func
* @returns {Promise<void>}
*/
  play(animation_func: Function, object_indices: Uint32Array, duration_in_frames: bigint, rate_func: Function): Promise<void>;
/**
* @param {Function} animation_func
* @param {Array<any>} objects
* @param {number} t
* @returns {Promise<void>}
*/
  makeFrame(animation_func: Function, objects: Array<any>, t: number): Promise<void>;
/**
* @param {bigint} duration_in_frames
* @returns {Promise<void>}
*/
  wait(duration_in_frames: bigint): Promise<void>;
/**
* @param {Function} callback
*/
  setCallback(callback: Function): void;
/**
* @returns {Promise<void>}
*/
  callCallback(): Promise<void>;
/**
* @param {string} path
* @param {string | undefined} [codec]
* @param {string | undefined} [pix_fmt]
* @param {string | undefined} [qp]
*/
  initFFmpeg(path: string, codec?: string, pix_fmt?: string, qp?: string): void;
/**
*/
  closeFFmpeg(): void;
}
/**
*/
export class WasmCamera {
  free(): void;
/**
* @param {Array<any>} position
* @param {Array<any>} rotation
* @param {number} focal_distance
* @param {number} zoom
* @param {number} width
* @param {number} height
*/
  constructor(position: Array<any>, rotation: Array<any>, focal_distance: number, zoom: number, width: number, height: number);
/**
* @returns {Array<any>}
*/
  getPosition(): Array<any>;
/**
* @returns {Array<any>}
*/
  getRotation(): Array<any>;
/**
* @returns {number}
*/
  getFocalDistance(): number;
/**
* @returns {number}
*/
  getZoom(): number;
/**
* @returns {number}
*/
  getWidth(): number;
/**
* @returns {number}
*/
  getHeight(): number;
/**
* @returns {WasmCamera}
*/
  clone(): WasmCamera;
}
/**
*/
export class WasmColor {
  free(): void;
/**
* @param {number} r
* @param {number} g
* @param {number} b
* @param {number} a
*/
  constructor(r: number, g: number, b: number, a: number);
/**
* @returns {number}
*/
  getR(): number;
/**
* @returns {number}
*/
  getG(): number;
/**
* @returns {number}
*/
  getB(): number;
/**
* @returns {number}
*/
  getA(): number;
}
/**
*/
export class WasmGradientImageOrColor {
  free(): void;
/**
* @param {WasmColor} color
* @returns {WasmGradientImageOrColor}
*/
  static fromColor(color: WasmColor): WasmGradientImageOrColor;
/**
* @param {WasmLinearGradient} linear_gradient
* @returns {WasmGradientImageOrColor}
*/
  static fromLinearGradient(linear_gradient: WasmLinearGradient): WasmGradientImageOrColor;
/**
* @param {WasmRadialGradient} radial_gradient
* @returns {WasmGradientImageOrColor}
*/
  static fromRadialGradient(radial_gradient: WasmRadialGradient): WasmGradientImageOrColor;
/**
* @param {WasmImage} image
* @returns {WasmGradientImageOrColor}
*/
  static fromImage(image: WasmImage): WasmGradientImageOrColor;
/**
* @returns {boolean}
*/
  isColor(): boolean;
/**
* @returns {boolean}
*/
  isLinearGradient(): boolean;
/**
* @returns {boolean}
*/
  isRadialGradient(): boolean;
/**
* @returns {boolean}
*/
  isImage(): boolean;
/**
* @returns {WasmColor | undefined}
*/
  getColor(): WasmColor | undefined;
/**
* @returns {WasmLinearGradient | undefined}
*/
  getLinearGradient(): WasmLinearGradient | undefined;
/**
* @returns {WasmRadialGradient | undefined}
*/
  getRadialGradient(): WasmRadialGradient | undefined;
/**
* @returns {WasmImage | undefined}
*/
  getImage(): WasmImage | undefined;
/**
* @returns {WasmGradientImageOrColor}
*/
  clone(): WasmGradientImageOrColor;
}
/**
*/
export class WasmGradientStop {
  free(): void;
/**
* @param {number} offset
* @param {WasmColor} color
*/
  constructor(offset: number, color: WasmColor);
/**
* @returns {number}
*/
  getOffset(): number;
/**
* @returns {WasmColor}
*/
  getColor(): WasmColor;
}
/**
*/
export class WasmImage {
  free(): void;
/**
* @param {string} image_base64
* @param {string} mime_type
* @param {number} top
* @param {number} left
* @param {number} bottom
* @param {number} right
* @param {number} alpha
*/
  constructor(image_base64: string, mime_type: string, top: number, left: number, bottom: number, right: number, alpha: number);
/**
* @returns {string}
*/
  getImageBase64(): string;
/**
* @returns {string}
*/
  getMimeType(): string;
/**
* @returns {number}
*/
  getTop(): number;
/**
* @returns {number}
*/
  getLeft(): number;
/**
* @returns {number}
*/
  getBottom(): number;
/**
* @returns {number}
*/
  getRight(): number;
/**
* @returns {number}
*/
  getAlpha(): number;
}
/**
*/
export class WasmLightSource {
  free(): void;
/**
* @param {Array<any>} position
*/
  constructor(position: Array<any>);
/**
* @returns {Array<any>}
*/
  getPosition(): Array<any>;
/**
* @returns {WasmLightSource}
*/
  clone(): WasmLightSource;
}
/**
*/
export class WasmLinearGradient {
  free(): void;
/**
* @param {number} x1
* @param {number} y1
* @param {number} x2
* @param {number} y2
* @param {(WasmGradientStop)[]} stops
* @param {number} alpha
*/
  constructor(x1: number, y1: number, x2: number, y2: number, stops: (WasmGradientStop)[], alpha: number);
/**
* @returns {number}
*/
  getX1(): number;
/**
* @returns {number}
*/
  getY1(): number;
/**
* @returns {number}
*/
  getX2(): number;
/**
* @returns {number}
*/
  getY2(): number;
/**
* @returns {(WasmGradientStop)[]}
*/
  getStops(): (WasmGradientStop)[];
/**
* @returns {number}
*/
  getAlpha(): number;
}
/**
*/
export class WasmRadialGradient {
  free(): void;
/**
* @param {number} cx
* @param {number} cy
* @param {number} r
* @param {number} fx
* @param {number} fy
* @param {(WasmGradientStop)[]} stops
* @param {number} alpha
*/
  constructor(cx: number, cy: number, r: number, fx: number, fy: number, stops: (WasmGradientStop)[], alpha: number);
/**
* @returns {number}
*/
  getCx(): number;
/**
* @returns {number}
*/
  getCy(): number;
/**
* @returns {number}
*/
  getR(): number;
/**
* @returns {number}
*/
  getFx(): number;
/**
* @returns {number}
*/
  getFy(): number;
/**
* @returns {(WasmGradientStop)[]}
*/
  getStops(): (WasmGradientStop)[];
/**
* @returns {number}
*/
  getAlpha(): number;
}
/**
*/
export class WasmThreeDObject {
  free(): void;
/**
* @param {Array<any>} points
* @param {(WasmThreeDObject)[]} subobjects
* @param {WasmGradientImageOrColor} fill
* @param {WasmGradientImageOrColor} stroke
* @param {number} stroke_width
*/
  constructor(points: Array<any>, subobjects: (WasmThreeDObject)[], fill: WasmGradientImageOrColor, stroke: WasmGradientImageOrColor, stroke_width: number);
/**
* @returns {Array<any>}
*/
  getPoints(): Array<any>;
/**
* @returns {(WasmThreeDObject)[]}
*/
  getSubobjects(): (WasmThreeDObject)[];
/**
* @returns {WasmGradientImageOrColor}
*/
  getFill(): WasmGradientImageOrColor;
/**
* @returns {WasmGradientImageOrColor}
*/
  getStroke(): WasmGradientImageOrColor;
/**
* @returns {number}
*/
  getStrokeWidth(): number;
/**
* @param {Array<any>} points
* @returns {WasmThreeDObject}
*/
  setPoints(points: Array<any>): WasmThreeDObject;
/**
* @param {(WasmThreeDObject)[]} subobjects
* @returns {WasmThreeDObject}
*/
  setSubobjects(subobjects: (WasmThreeDObject)[]): WasmThreeDObject;
/**
* @param {WasmGradientImageOrColor} fill
* @returns {WasmThreeDObject}
*/
  setFill(fill: WasmGradientImageOrColor): WasmThreeDObject;
/**
* @param {WasmGradientImageOrColor} stroke
* @returns {WasmThreeDObject}
*/
  setStroke(stroke: WasmGradientImageOrColor): WasmThreeDObject;
/**
* @param {number} stroke_width
* @returns {WasmThreeDObject}
*/
  setStrokeWidth(stroke_width: number): WasmThreeDObject;
/**
* @param {number} factor
* @param {boolean} recursive
* @returns {WasmThreeDObject}
*/
  scale(factor: number, recursive: boolean): WasmThreeDObject;
/**
* @param {Array<any>} factor
* @param {boolean} recursive
* @returns {WasmThreeDObject}
*/
  stretch(factor: Array<any>, recursive: boolean): WasmThreeDObject;
/**
* @param {Array<any>} shift
* @param {boolean} recursive
* @returns {WasmThreeDObject}
*/
  shift(shift: Array<any>, recursive: boolean): WasmThreeDObject;
/**
* @param {number} angle
* @param {boolean} recursive
* @returns {WasmThreeDObject}
*/
  rotateX(angle: number, recursive: boolean): WasmThreeDObject;
/**
* @param {number} angle
* @param {boolean} recursive
* @returns {WasmThreeDObject}
*/
  rotateY(angle: number, recursive: boolean): WasmThreeDObject;
/**
* @param {number} angle
* @param {boolean} recursive
* @returns {WasmThreeDObject}
*/
  rotateZ(angle: number, recursive: boolean): WasmThreeDObject;
/**
* @param {WasmCamera} camera
* @param {WasmLightSource} light_source
* @returns {WasmVectorObject}
*/
  projectAndShade(camera: WasmCamera, light_source: WasmLightSource): WasmVectorObject;
/**
* @param {Function} uv_function
* @param {Array<any>} u_range
* @param {Array<any>} v_range
* @param {number} u_segments
* @param {number} v_segments
* @param {(WasmColor)[]} fills
* @param {(WasmColor)[]} strokes
* @param {number} stroke_width
* @returns {WasmThreeDObject}
*/
  static fromUvFunction(uv_function: Function, u_range: Array<any>, v_range: Array<any>, u_segments: number, v_segments: number, fills: (WasmColor)[], strokes: (WasmColor)[], stroke_width: number): WasmThreeDObject;
/**
* @returns {Array<any>}
*/
  getBoundingBox(): Array<any>;
/**
* @returns {Array<any>}
*/
  getCenter(): Array<any>;
/**
* @param {Array<any>} point
* @param {boolean} recursive
* @returns {WasmThreeDObject}
*/
  moveTo(point: Array<any>, recursive: boolean): WasmThreeDObject;
/**
* @returns {WasmThreeDObject}
*/
  clone(): WasmThreeDObject;
}
/**
*/
export class WasmVectorObject {
  free(): void;
/**
*/
  constructor();
/**
* @returns {number}
*/
  getIndex(): number;
/**
* @param {number} increment
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  incrementIndex(increment: number, recursive: boolean): WasmVectorObject;
/**
* @returns {Array<any>}
*/
  getPoints(): Array<any>;
/**
* @returns {WasmGradientImageOrColor}
*/
  getFill(): WasmGradientImageOrColor;
/**
* @returns {WasmGradientImageOrColor}
*/
  getStroke(): WasmGradientImageOrColor;
/**
* @returns {number}
*/
  getStrokeWidth(): number;
/**
* @returns {string}
*/
  getLineCap(): string;
/**
* @returns {string}
*/
  getLineJoin(): string;
/**
* @param {number} start
* @param {number} end
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  getPartialCopy(start: number, end: number, recursive: boolean): WasmVectorObject;
/**
* @returns {Array<any>}
*/
  getSubpaths(): Array<any>;
/**
* @returns {Array<any>}
*/
  getCubicBezierTuples(): Array<any>;
/**
* @returns {(WasmVectorObject)[]}
*/
  getSubobjects(): (WasmVectorObject)[];
/**
* @param {number} factor
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  scale(factor: number, recursive: boolean): WasmVectorObject;
/**
* @param {number} x_factor
* @param {number} y_factor
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  stretch(x_factor: number, y_factor: number, recursive: boolean): WasmVectorObject;
/**
* @param {number} x_shift
* @param {number} y_shift
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  shift(x_shift: number, y_shift: number, recursive: boolean): WasmVectorObject;
/**
* @returns {Array<any>}
*/
  mergedPoints(): Array<any>;
/**
* @returns {Array<any>}
*/
  getBoundingBox(): Array<any>;
/**
* @returns {Array<any>}
*/
  getCenter(): Array<any>;
/**
* @returns {Array<any>}
*/
  getCenterOfMass(): Array<any>;
/**
* @returns {number}
*/
  getHeight(): number;
/**
* @returns {number}
*/
  getWidth(): number;
/**
* @param {number} index
* @returns {WasmVectorObject}
*/
  setIndex(index: number): WasmVectorObject;
/**
* @param {WasmGradientImageOrColor} fill
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  setFill(fill: WasmGradientImageOrColor, recursive: boolean): WasmVectorObject;
/**
* @param {number} opacity
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  setFillOpacity(opacity: number, recursive: boolean): WasmVectorObject;
/**
* @param {number} x
* @param {number} y
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  moveTo(x: number, y: number, recursive: boolean): WasmVectorObject;
/**
* @param {WasmGradientImageOrColor} stroke
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  setStroke(stroke: WasmGradientImageOrColor, recursive: boolean): WasmVectorObject;
/**
* @param {number} opacity
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  setStrokeOpacity(opacity: number, recursive: boolean): WasmVectorObject;
/**
* @param {number} width
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  setStrokeWidth(width: number, recursive: boolean): WasmVectorObject;
/**
* @param {string} line_cap
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  setLineCap(line_cap: string, recursive: boolean): WasmVectorObject;
/**
* @param {string} line_join
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  setLineJoin(line_join: string, recursive: boolean): WasmVectorObject;
/**
* @param {Array<any>} points
* @returns {WasmVectorObject}
*/
  setPoints(points: Array<any>): WasmVectorObject;
/**
* @param {(WasmVectorObject)[]} subobjects
* @returns {WasmVectorObject}
*/
  setSubobjects(subobjects: (WasmVectorObject)[]): WasmVectorObject;
/**
* @param {number} angle
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  rotate(angle: number, recursive: boolean): WasmVectorObject;
/**
* @param {number} key_x
* @param {number} key_y
* @returns {Array<any>}
*/
  getCriticalPoint(key_x: number, key_y: number): Array<any>;
/**
* @returns {number}
*/
  getFillOpacity(): number;
/**
* @returns {number}
*/
  getStrokeOpacity(): number;
/**
* @param {WasmVectorObject} other
* @param {Array<any>} direction
* @param {number} buff
* @param {Array<any>} aligned_edge
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  nextToOther(other: WasmVectorObject, direction: Array<any>, buff: number, aligned_edge: Array<any>, recursive: boolean): WasmVectorObject;
/**
* @param {Array<any>} direction
* @param {number} buff
* @param {Array<any>} aligned_edge
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  arrangeSubobjects(direction: Array<any>, buff: number, aligned_edge: Array<any>, recursive: boolean): WasmVectorObject;
/**
* @param {Array<any>} point
* @param {Array<any>} direction
* @param {number} buff
* @param {Array<any>} aligned_edge
* @param {boolean} recursive
* @returns {WasmVectorObject}
*/
  nextToPoint(point: Array<any>, direction: Array<any>, buff: number, aligned_edge: Array<any>, recursive: boolean): WasmVectorObject;
/**
* @returns {WasmVectorObject}
*/
  clone(): WasmVectorObject;
}
