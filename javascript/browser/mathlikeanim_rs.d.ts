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
* @param {string | undefined} [default_font_family]
* @param {number | undefined} [default_font_size]
* @returns {Promise<WasmVectorObject>}
*/
export function svgToVector(svg: string, default_font_family?: string, default_font_size?: number): Promise<WasmVectorObject>;
/**
* @returns {Theme}
*/
export function getGithubDark(): Theme;
/**
* @returns {Lexer}
*/
export function getPythonLexer(): Lexer;
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
* @param {number | undefined} [default_stroke_width]
* @returns {WasmVectorObject}
*/
export function drawStrokeThenFill(vec_obj: WasmVectorObject, t: number, default_stroke_width?: number): WasmVectorObject;
/**
* @param {WasmVectorObject} vec_obj
* @param {number} lag_ratio
* @param {number} t
* @param {number | undefined} [default_stroke_width]
* @returns {WasmVectorObject}
*/
export function write(vec_obj: WasmVectorObject, lag_ratio: number, t: number, default_stroke_width?: number): WasmVectorObject;
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
* @param {SVGScene} scene
* @param {number} t
*/
export function moveCameraSVG(top_left_corner: Array<any>, bottom_right_corner: Array<any>, scene: SVGScene, t: number): void;
/**
* @param {Array<any>} top_left_corner
* @param {Array<any>} bottom_right_corner
* @param {Scene} scene
* @param {number} t
*/
export function moveCamera(top_left_corner: Array<any>, bottom_right_corner: Array<any>, scene: Scene, t: number): void;
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
* @param {string} text
* @param {string} font_family
* @param {number} x
* @param {number} y
* @param {number} font_size
* @returns {Promise<WasmVectorObject>}
*/
export function textToVector(text: string, font_family: string, x: number, y: number, font_size: number): Promise<WasmVectorObject>;
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
* @param {string} expression
* @param {string | undefined} [default_font_family]
* @param {number | undefined} [default_font_size]
* @returns {Promise<WasmVectorObject>}
*/
export function mathjax(expression: string, default_font_family?: string, default_font_size?: number): Promise<WasmVectorObject>;
/**
* @param {string} code
* @param {Lexer} lexer
* @param {Theme} theme
* @param {string} font_family
* @returns {Promise<WasmVectorObject>}
*/
export function codeObject(code: string, lexer: Lexer, theme: Theme, font_family: string): Promise<WasmVectorObject>;
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
*/
export enum TokenType {
  Illegal = 0,
  Declaration = 1,
  Comment = 2,
  MethodDeclaration = 3,
  MethodIdentifier = 4,
  FormattedString = 5,
  FormatOpen = 6,
  FormatClose = 7,
  Newline = 8,
  Identifier = 9,
  ClassIdentifier = 10,
  Separator = 11,
  Number = 12,
  String = 13,
  Assign = 14,
  Operator = 15,
  Whitespace = 16,
  Constant = 17,
  Keyword = 18,
  Special = 19,
  LParen = 20,
  RParen = 21,
}
/**
*/
export class GenericScene {
  free(): void;
/**
* @param {Scene} scene
* @returns {GenericScene}
*/
  static fromScene(scene: Scene): GenericScene;
/**
* @param {SVGScene} scene
* @returns {GenericScene}
*/
  static fromSVGScene(scene: SVGScene): GenericScene;
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
  isNodeScene(): boolean;
/**
* @returns {number}
*/
  getFps(): number;
/**
* @returns {number}
*/
  getHeight(): number;
/**
* @returns {number}
*/
  getWidth(): number;
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
* @param {HTMLDivElement} div_container
*/
  setDivContainer(div_container: HTMLDivElement): void;
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
* @param {number} duration_in_frames
* @param {Function} rate_func
* @returns {Promise<void>}
*/
  play(animation_func: Function, object_indices: Uint32Array, duration_in_frames: number, rate_func: Function): Promise<void>;
/**
* @param {Function} animation_func
* @param {Array<any>} objects
* @param {number} t
* @returns {Promise<void>}
*/
  makeFrame(animation_func: Function, objects: Array<any>, t: number): Promise<void>;
/**
* @param {number} duration_in_frames
* @returns {Promise<void>}
*/
  wait(duration_in_frames: number): Promise<void>;
/**
* @param {Function} callback
*/
  setCallback(callback: Function): void;
/**
* @returns {Promise<void>}
*/
  callCallback(): Promise<void>;
/**
* @param {number} index
* @param {string} _class
*/
  setClass(index: number, _class: string): void;
/**
* @param {number} index
*/
  setStyle(index: number): void;
}
/**
*/
export class Lexer {
  free(): void;
/**
* @param {(string)[]} keywords
* @param {(string)[]} specials
* @param {(string)[]} illegals
* @param {(string)[]} declarations
* @param {(string)[]} method_declaration
* @param {(string)[]} operators
* @param {(string)[]} l_parens
* @param {(string)[]} r_parens
* @param {(string)[]} comment_initial_characters
* @param {(string)[]} constants
* @param {(string)[]} assignments
* @param {(string)[]} separators
* @param {(string)[]} string_open_delimiters
* @param {(string)[]} string_close_delimiters
* @param {(string)[]} formated_string_open_delimiters
* @param {(string)[]} formated_string_close_delimiters
* @param {(string)[]} format_opens
* @param {(string)[]} format_closes
* @param {string} class_identifier_pattern
*/
  constructor(keywords: (string)[], specials: (string)[], illegals: (string)[], declarations: (string)[], method_declaration: (string)[], operators: (string)[], l_parens: (string)[], r_parens: (string)[], comment_initial_characters: (string)[], constants: (string)[], assignments: (string)[], separators: (string)[], string_open_delimiters: (string)[], string_close_delimiters: (string)[], formated_string_open_delimiters: (string)[], formated_string_close_delimiters: (string)[], format_opens: (string)[], format_closes: (string)[], class_identifier_pattern: string);
/**
* @returns {(string)[]}
*/
  getKeywords(): (string)[];
/**
* @returns {(string)[]}
*/
  getSpecials(): (string)[];
/**
* @returns {(string)[]}
*/
  getMethodDeclarations(): (string)[];
/**
* @returns {(string)[]}
*/
  getIllegals(): (string)[];
/**
* @returns {(string)[]}
*/
  getDeclarations(): (string)[];
/**
* @returns {(string)[]}
*/
  getOperators(): (string)[];
/**
* @returns {(string)[]}
*/
  getSeparators(): (string)[];
/**
* @returns {(string)[]}
*/
  getLParens(): (string)[];
/**
* @returns {(string)[]}
*/
  getRParens(): (string)[];
/**
* @returns {(string)[]}
*/
  getFormatOpens(): (string)[];
/**
* @returns {(string)[]}
*/
  getFormatCloses(): (string)[];
/**
* @returns {(string)[]}
*/
  getStringOpenDelimiters(): (string)[];
/**
* @returns {(string)[]}
*/
  getFormatedStringOpenDelimiters(): (string)[];
/**
* @returns {(string)[]}
*/
  getFormatedStringCloseDelimiters(): (string)[];
/**
* @returns {(string)[]}
*/
  getStringCloseDelimiters(): (string)[];
/**
* @returns {(string)[]}
*/
  getCommentCharacters(): (string)[];
/**
* @returns {(string)[]}
*/
  getAssignments(): (string)[];
/**
* @returns {(string)[]}
*/
  getConstants(): (string)[];
/**
* @param {string} token
* @returns {boolean}
*/
  isKeyword(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isSpecial(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isIllegal(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isFormatedStringOpenDelimiter(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isFormatedStringCloseDelimiter(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isMethodDeclaration(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isStringOpenDelimiter(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isStringCloseDelimiter(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isCommentCharacter(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isWhitespace(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isDigit(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isQuote(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  containsQuoteInitial(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isDeclaration(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isSeparator(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isOperator(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isLParen(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isRParen(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isNewline(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isConstant(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  hasFormatedStringOpenInitial(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  hasFormatedStringCloseInitial(token: string): boolean;
/**
* @returns {string}
*/
  getClassIdentifierPattern(): string;
/**
* @param {string} token
* @returns {boolean}
*/
  containsOperator(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  containsAssignment(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  hasFormatOpen(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  hasFormatClose(token: string): boolean;
/**
* @param {string} token
* @returns {string | undefined}
*/
  removeLastOperator(token: string): string | undefined;
/**
* @param {string} token
* @returns {string | undefined}
*/
  removeLastAssignment(token: string): string | undefined;
/**
* @param {string} token
* @returns {string | undefined}
*/
  removeLastSeparator(token: string): string | undefined;
/**
* @param {string} token
* @returns {string | undefined}
*/
  removeLastQuote(token: string): string | undefined;
/**
* @param {string} token
* @returns {boolean}
*/
  hasOperatorInitial(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  hasAssignmentInitial(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  isAssignment(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  hasSeparatorInitial(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  containsQuote(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  containsNonAlphabeticalOperator(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  endsWithStringCloseDelimiter(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  startsWithStringOpenDelimiter(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  hasFormatStringClose(token: string): boolean;
/**
* @returns {Lexer}
*/
  clone(): Lexer;
/**
* @param {string} token
* @returns {string | undefined}
*/
  removeFormatOpen(token: string): string | undefined;
/**
* @param {string} token
* @returns {boolean}
*/
  startsWithCommentCharacter(token: string): boolean;
/**
* @param {string} token
* @returns {boolean}
*/
  hasCommentInitial(token: string): boolean;
/**
* @param {string} token
* @returns {string | undefined}
*/
  removeFormatClose(token: string): string | undefined;
/**
* @param {string} token
* @returns {number | undefined}
*/
  removeStringOpenDelimiterIndex(token: string): number | undefined;
/**
* @param {string} token
* @returns {number | undefined}
*/
  removeStringCloseDelimiterIndex(token: string): number | undefined;
/**
* @param {string} token
* @returns {boolean}
*/
  hasFormatStringOpen(token: string): boolean;
/**
* @param {string} token
* @returns {number | undefined}
*/
  getStringOpeningDelimiterIndex(token: string): number | undefined;
/**
* @param {string} token
* @returns {number | undefined}
*/
  getStringClosingDelimiterIndex(token: string): number | undefined;
/**
* @param {string} token
* @returns {number | undefined}
*/
  getFormatedStringOpeningDelimiterIndex(token: string): number | undefined;
/**
* @param {string} token
* @returns {number | undefined}
*/
  getFormatedStringClosingDelimiterIndex(token: string): number | undefined;
/**
* @param {string} input
* @returns {(Token)[]}
*/
  getTokens(input: string): (Token)[];
}
/**
*/
export class SVGScene {
  free(): void;
/**
* @param {number} width
* @param {number} height
* @param {number} fps
*/
  constructor(width: number, height: number, fps: number);
/**
* @returns {number}
*/
  getFps(): number;
/**
* @returns {number}
*/
  getHeight(): number;
/**
* @returns {number}
*/
  getWidth(): number;
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
* @param {Array<any>} object_indices
* @returns {Map<any, any>}
*/
  getObjectsFromIndices(object_indices: Array<any>): Map<any, any>;
/**
* @param {HTMLDivElement} div_container
*/
  setDivContainer(div_container: HTMLDivElement): void;
/**
* @param {number} duration_in_ms
* @returns {Promise<void>}
*/
  sleep(duration_in_ms: number): Promise<void>;
/**
* @param {Function} animation_func
* @param {Uint32Array} object_indices
* @param {number} duration_in_frames
* @param {Function} rate_func
* @returns {Promise<void>}
*/
  play(animation_func: Function, object_indices: Uint32Array, duration_in_frames: number, rate_func: Function): Promise<void>;
/**
* @param {Function} animation_func
* @param {Array<any>} objects
* @param {number} t
* @returns {Promise<void>}
*/
  makeFrame(animation_func: Function, objects: Array<any>, t: number): Promise<void>;
/**
* @param {Array<any>} objects
*/
  setObjects(objects: Array<any>): void;
/**
* @returns {Array<any>}
*/
  getObjects(): Array<any>;
/**
* @param {number} duration_in_frames
* @returns {Promise<void>}
*/
  wait(duration_in_frames: number): Promise<void>;
/**
* @param {Function} callback
*/
  setCallback(callback: Function): void;
/**
* @returns {Promise<void>}
*/
  callCallback(): Promise<void>;
/**
* @param {number} index
* @param {string} id
*/
  setClass(index: number, id: string): void;
/**
* @param {number} index
*/
  removeClass(index: number): void;
}
/**
*/
export class Scene {
  free(): void;
/**
* @param {number} width
* @param {number} height
* @param {number} fps
*/
  constructor(width: number, height: number, fps: number);
/**
* @returns {number}
*/
  getFps(): number;
/**
* @returns {number}
*/
  getHeight(): number;
/**
* @returns {number}
*/
  getWidth(): number;
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
* @param {CanvasRenderingContext2D} context
*/
  setCanvasContext(context: CanvasRenderingContext2D): void;
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
* @param {number} duration_in_frames
* @param {Function} rate_func
* @returns {Promise<void>}
*/
  play(animation_func: Function, object_indices: Uint32Array, duration_in_frames: number, rate_func: Function): Promise<void>;
/**
* @param {Function} animation_func
* @param {Array<any>} objects
* @param {number} t
* @returns {Promise<void>}
*/
  makeFrame(animation_func: Function, objects: Array<any>, t: number): Promise<void>;
/**
* @param {number} duration_in_frames
* @returns {Promise<void>}
*/
  wait(duration_in_frames: number): Promise<void>;
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
export class Theme {
  free(): void;
/**
* @param {string} keyword_color
* @param {string} special_color
* @param {string} illegal_color
* @param {string} declaration_color
* @param {string} operator_color
* @param {string} paren_color
* @param {string} constant_color
* @param {string} number_color
* @param {string} string_color
* @param {string} identifier_color
* @param {string} assign_color
* @param {string} separator_color
* @param {string} method_identifier_color
* @param {string} method_declaration_color
* @param {string} formatted_string_color
* @param {string} format_open_color
* @param {string} format_close_color
* @param {string} comment_color
* @param {string} class_identifier_color
*/
  constructor(keyword_color: string, special_color: string, illegal_color: string, declaration_color: string, operator_color: string, paren_color: string, constant_color: string, number_color: string, string_color: string, identifier_color: string, assign_color: string, separator_color: string, method_identifier_color: string, method_declaration_color: string, formatted_string_color: string, format_open_color: string, format_close_color: string, comment_color: string, class_identifier_color: string);
/**
* @returns {string}
*/
  getKeywordColor(): string;
/**
* @returns {string}
*/
  getSpecialColor(): string;
/**
* @returns {string}
*/
  getIllegalColor(): string;
/**
* @returns {string}
*/
  getDeclarationColor(): string;
/**
* @returns {string}
*/
  getOperatorColor(): string;
/**
* @returns {string}
*/
  getParenColor(): string;
/**
* @returns {string}
*/
  getConstantColor(): string;
/**
* @returns {string}
*/
  getNumberColor(): string;
/**
* @returns {string}
*/
  getStringColor(): string;
/**
* @returns {string}
*/
  getIdentifierColor(): string;
/**
* @returns {string}
*/
  getAssignColor(): string;
/**
* @returns {string}
*/
  getClassIdentifierColor(): string;
/**
* @returns {string}
*/
  getSeparatorColor(): string;
/**
* @returns {string}
*/
  getMethodDeclarationColor(): string;
/**
* @returns {string}
*/
  getMethodIdentifierColor(): string;
/**
* @returns {string}
*/
  getFormattedStringColor(): string;
/**
* @returns {string}
*/
  getFormatOpenColor(): string;
/**
* @returns {string}
*/
  getFormatCloseColor(): string;
/**
* @returns {Theme}
*/
  clone(): Theme;
/**
* @returns {string}
*/
  getCommentColor(): string;
}
/**
*/
export class Token {
  free(): void;
/**
* @param {TokenType} token_type
* @param {string} literal
*/
  constructor(token_type: TokenType, literal: string);
/**
* @returns {TokenType}
*/
  getType(): TokenType;
/**
* @returns {string}
*/
  getLiteral(): string;
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
* @param {WasmVectorObject} new_subobject
* @returns {WasmVectorObject}
*/
  add(new_subobject: WasmVectorObject): WasmVectorObject;
/**
* @param {number} index
* @returns {WasmVectorObject}
*/
  remove(index: number): WasmVectorObject;
/**
* @param {number} index
* @returns {WasmVectorObject}
*/
  getSubobject(index: number): WasmVectorObject;
/**
* @param {number} start
* @param {number} end
* @returns {(WasmVectorObject)[]}
*/
  sliceSubobjects(start: number, end: number): (WasmVectorObject)[];
/**
* @param {number} index
* @param {WasmVectorObject} new_subobject
* @returns {WasmVectorObject}
*/
  setSubobject(index: number, new_subobject: WasmVectorObject): WasmVectorObject;
/**
* @param {number} start
* @param {number} end
* @param {(WasmVectorObject)[]} new_subobjects
* @returns {WasmVectorObject}
*/
  setSliceSubobjects(start: number, end: number, new_subobjects: (WasmVectorObject)[]): WasmVectorObject;
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

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmgradientimageorcolor_free: (a: number) => void;
  readonly wasmgradientimageorcolor_fromColor: (a: number) => number;
  readonly wasmgradientimageorcolor_fromLinearGradient: (a: number) => number;
  readonly wasmgradientimageorcolor_fromRadialGradient: (a: number) => number;
  readonly wasmgradientimageorcolor_fromImage: (a: number) => number;
  readonly wasmgradientimageorcolor_isColor: (a: number) => number;
  readonly wasmgradientimageorcolor_isLinearGradient: (a: number) => number;
  readonly wasmgradientimageorcolor_isRadialGradient: (a: number) => number;
  readonly wasmgradientimageorcolor_isImage: (a: number) => number;
  readonly wasmgradientimageorcolor_getColor: (a: number) => number;
  readonly wasmgradientimageorcolor_getLinearGradient: (a: number) => number;
  readonly wasmgradientimageorcolor_getRadialGradient: (a: number) => number;
  readonly wasmgradientimageorcolor_getImage: (a: number) => number;
  readonly wasmgradientimageorcolor_clone: (a: number) => number;
  readonly __wbg_wasmcolor_free: (a: number) => void;
  readonly wasmcolor_new: (a: number, b: number, c: number, d: number) => number;
  readonly wasmcolor_getR: (a: number) => number;
  readonly wasmcolor_getG: (a: number) => number;
  readonly wasmcolor_getB: (a: number) => number;
  readonly wasmcolor_getA: (a: number) => number;
  readonly __wbg_wasmgradientstop_free: (a: number) => void;
  readonly wasmgradientstop_new: (a: number, b: number) => number;
  readonly wasmgradientstop_getOffset: (a: number) => number;
  readonly wasmgradientstop_getColor: (a: number) => number;
  readonly __wbg_wasmlineargradient_free: (a: number) => void;
  readonly wasmlineargradient_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly wasmlineargradient_getStops: (a: number, b: number) => void;
  readonly __wbg_wasmradialgradient_free: (a: number) => void;
  readonly wasmradialgradient_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly wasmradialgradient_getStops: (a: number, b: number) => void;
  readonly wasmradialgradient_getAlpha: (a: number) => number;
  readonly __wbg_wasmimage_free: (a: number) => void;
  readonly wasmimage_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => number;
  readonly wasmimage_getImageBase64: (a: number, b: number) => void;
  readonly wasmimage_getMimeType: (a: number, b: number) => void;
  readonly __wbg_wasmvectorobject_free: (a: number) => void;
  readonly wasmvectorobject_new: () => number;
  readonly wasmvectorobject_getIndex: (a: number) => number;
  readonly wasmvectorobject_incrementIndex: (a: number, b: number, c: number) => number;
  readonly wasmvectorobject_getPoints: (a: number) => number;
  readonly wasmvectorobject_add: (a: number, b: number) => number;
  readonly wasmvectorobject_remove: (a: number, b: number) => number;
  readonly wasmvectorobject_getSubobject: (a: number, b: number) => number;
  readonly wasmvectorobject_sliceSubobjects: (a: number, b: number, c: number, d: number) => void;
  readonly wasmvectorobject_setSubobject: (a: number, b: number, c: number) => number;
  readonly wasmvectorobject_setSliceSubobjects: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly wasmvectorobject_getFill: (a: number) => number;
  readonly wasmvectorobject_getStroke: (a: number) => number;
  readonly wasmvectorobject_getStrokeWidth: (a: number) => number;
  readonly wasmvectorobject_getLineCap: (a: number, b: number) => void;
  readonly wasmvectorobject_getLineJoin: (a: number, b: number) => void;
  readonly wasmvectorobject_getPartialCopy: (a: number, b: number, c: number, d: number) => number;
  readonly wasmvectorobject_getSubpaths: (a: number) => number;
  readonly wasmvectorobject_getCubicBezierTuples: (a: number) => number;
  readonly wasmvectorobject_getSubobjects: (a: number, b: number) => void;
  readonly wasmvectorobject_scale: (a: number, b: number, c: number) => number;
  readonly wasmvectorobject_stretch: (a: number, b: number, c: number, d: number) => number;
  readonly wasmvectorobject_shift: (a: number, b: number, c: number, d: number) => number;
  readonly wasmvectorobject_mergedPoints: (a: number) => number;
  readonly wasmvectorobject_getBoundingBox: (a: number) => number;
  readonly wasmvectorobject_getCenter: (a: number) => number;
  readonly wasmvectorobject_getCenterOfMass: (a: number) => number;
  readonly wasmvectorobject_getHeight: (a: number) => number;
  readonly wasmvectorobject_getWidth: (a: number) => number;
  readonly wasmvectorobject_setIndex: (a: number, b: number) => number;
  readonly wasmvectorobject_setFill: (a: number, b: number, c: number) => number;
  readonly wasmvectorobject_setFillOpacity: (a: number, b: number, c: number) => number;
  readonly wasmvectorobject_moveTo: (a: number, b: number, c: number, d: number) => number;
  readonly wasmvectorobject_setStroke: (a: number, b: number, c: number) => number;
  readonly wasmvectorobject_setStrokeOpacity: (a: number, b: number, c: number) => number;
  readonly wasmvectorobject_setStrokeWidth: (a: number, b: number, c: number) => number;
  readonly wasmvectorobject_setLineCap: (a: number, b: number, c: number, d: number) => number;
  readonly wasmvectorobject_setLineJoin: (a: number, b: number, c: number, d: number) => number;
  readonly wasmvectorobject_setPoints: (a: number, b: number) => number;
  readonly wasmvectorobject_setSubobjects: (a: number, b: number, c: number) => number;
  readonly wasmvectorobject_rotate: (a: number, b: number, c: number) => number;
  readonly wasmvectorobject_getCriticalPoint: (a: number, b: number, c: number) => number;
  readonly wasmvectorobject_getFillOpacity: (a: number) => number;
  readonly wasmvectorobject_getStrokeOpacity: (a: number) => number;
  readonly wasmvectorobject_nextToOther: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly wasmvectorobject_arrangeSubobjects: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly wasmvectorobject_nextToPoint: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly wasmvectorobject_clone: (a: number) => number;
  readonly addFinalTip: (a: number, b: number, c: number) => number;
  readonly addInitialTip: (a: number, b: number, c: number) => number;
  readonly addBothSidesTips: (a: number, b: number, c: number) => number;
  readonly arc: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number) => number;
  readonly circle: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number) => number;
  readonly ellipticalArc: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number) => number;
  readonly ellipse: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number) => number;
  readonly annularSector: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number) => number;
  readonly line: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => number;
  readonly polygon: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => number;
  readonly regularPolygon: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => number;
  readonly square: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => number;
  readonly rectangle: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => number;
  readonly equilateralTriangle: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => number;
  readonly triangle: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => number;
  readonly rightTriangle: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => number;
  readonly axes: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number, x: number, y: number, z: number, a1: number, b1: number) => number;
  readonly coordsToPoint: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly pointToCoords: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly parametricPlotInAxes: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number) => number;
  readonly plotInAxes: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number) => number;
  readonly contourPlotInAxes: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number) => number;
  readonly areaUnderCurve: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => number;
  readonly riemannRectanglesForPlot: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number) => number;
  readonly secantLineForPlot: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number) => number;
  readonly parametricFunction: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => number;
  readonly contourPlot: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number) => number;
  readonly realFunction: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => number;
  readonly numberLine: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number) => number;
  readonly numberToPoint: (a: number, b: number, c: number, d: number) => number;
  readonly pointToNumber: (a: number, b: number, c: number, d: number) => number;
  readonly getNumbersTex: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => number;
  readonly svgToVector: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly wasmlineargradient_getX1: (a: number) => number;
  readonly wasmlineargradient_getY1: (a: number) => number;
  readonly wasmlineargradient_getX2: (a: number) => number;
  readonly wasmlineargradient_getY2: (a: number) => number;
  readonly wasmlineargradient_getAlpha: (a: number) => number;
  readonly wasmradialgradient_getCx: (a: number) => number;
  readonly wasmradialgradient_getCy: (a: number) => number;
  readonly wasmradialgradient_getR: (a: number) => number;
  readonly wasmradialgradient_getFx: (a: number) => number;
  readonly wasmradialgradient_getFy: (a: number) => number;
  readonly wasmimage_getTop: (a: number) => number;
  readonly wasmimage_getLeft: (a: number) => number;
  readonly wasmimage_getBottom: (a: number) => number;
  readonly wasmimage_getRight: (a: number) => number;
  readonly wasmimage_getAlpha: (a: number) => number;
  readonly __wbg_lexer_free: (a: number) => void;
  readonly lexer_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number, x: number, y: number, z: number, a1: number, b1: number, c1: number, d1: number, e1: number, f1: number, g1: number, h1: number, i1: number, j1: number, k1: number, l1: number) => number;
  readonly lexer_getKeywords: (a: number, b: number) => void;
  readonly lexer_getSpecials: (a: number, b: number) => void;
  readonly lexer_getMethodDeclarations: (a: number, b: number) => void;
  readonly lexer_getIllegals: (a: number, b: number) => void;
  readonly lexer_getDeclarations: (a: number, b: number) => void;
  readonly lexer_getOperators: (a: number, b: number) => void;
  readonly lexer_getSeparators: (a: number, b: number) => void;
  readonly lexer_getLParens: (a: number, b: number) => void;
  readonly lexer_getRParens: (a: number, b: number) => void;
  readonly lexer_getFormatOpens: (a: number, b: number) => void;
  readonly lexer_getFormatCloses: (a: number, b: number) => void;
  readonly lexer_getStringOpenDelimiters: (a: number, b: number) => void;
  readonly lexer_getFormatedStringOpenDelimiters: (a: number, b: number) => void;
  readonly lexer_getFormatedStringCloseDelimiters: (a: number, b: number) => void;
  readonly lexer_getStringCloseDelimiters: (a: number, b: number) => void;
  readonly lexer_getCommentCharacters: (a: number, b: number) => void;
  readonly lexer_getAssignments: (a: number, b: number) => void;
  readonly lexer_getConstants: (a: number, b: number) => void;
  readonly lexer_isKeyword: (a: number, b: number, c: number) => number;
  readonly lexer_isSpecial: (a: number, b: number, c: number) => number;
  readonly lexer_isIllegal: (a: number, b: number, c: number) => number;
  readonly lexer_isFormatedStringOpenDelimiter: (a: number, b: number, c: number) => number;
  readonly lexer_isFormatedStringCloseDelimiter: (a: number, b: number, c: number) => number;
  readonly lexer_isMethodDeclaration: (a: number, b: number, c: number) => number;
  readonly lexer_isStringCloseDelimiter: (a: number, b: number, c: number) => number;
  readonly lexer_isCommentCharacter: (a: number, b: number, c: number) => number;
  readonly lexer_isWhitespace: (a: number, b: number, c: number) => number;
  readonly lexer_isDigit: (a: number, b: number, c: number) => number;
  readonly lexer_isQuote: (a: number, b: number, c: number) => number;
  readonly lexer_containsQuoteInitial: (a: number, b: number, c: number) => number;
  readonly lexer_isDeclaration: (a: number, b: number, c: number) => number;
  readonly lexer_isSeparator: (a: number, b: number, c: number) => number;
  readonly lexer_isOperator: (a: number, b: number, c: number) => number;
  readonly lexer_isLParen: (a: number, b: number, c: number) => number;
  readonly lexer_isRParen: (a: number, b: number, c: number) => number;
  readonly lexer_isNewline: (a: number, b: number, c: number) => number;
  readonly lexer_isConstant: (a: number, b: number, c: number) => number;
  readonly lexer_hasFormatedStringOpenInitial: (a: number, b: number, c: number) => number;
  readonly lexer_hasFormatedStringCloseInitial: (a: number, b: number, c: number) => number;
  readonly lexer_getClassIdentifierPattern: (a: number, b: number) => void;
  readonly lexer_containsOperator: (a: number, b: number, c: number) => number;
  readonly lexer_containsAssignment: (a: number, b: number, c: number) => number;
  readonly lexer_hasFormatOpen: (a: number, b: number, c: number) => number;
  readonly lexer_hasFormatClose: (a: number, b: number, c: number) => number;
  readonly lexer_removeLastOperator: (a: number, b: number, c: number, d: number) => void;
  readonly lexer_removeLastAssignment: (a: number, b: number, c: number, d: number) => void;
  readonly lexer_removeLastSeparator: (a: number, b: number, c: number, d: number) => void;
  readonly lexer_removeLastQuote: (a: number, b: number, c: number, d: number) => void;
  readonly lexer_hasOperatorInitial: (a: number, b: number, c: number) => number;
  readonly lexer_hasAssignmentInitial: (a: number, b: number, c: number) => number;
  readonly lexer_isAssignment: (a: number, b: number, c: number) => number;
  readonly lexer_hasSeparatorInitial: (a: number, b: number, c: number) => number;
  readonly lexer_containsQuote: (a: number, b: number, c: number) => number;
  readonly lexer_containsNonAlphabeticalOperator: (a: number, b: number, c: number) => number;
  readonly lexer_endsWithStringCloseDelimiter: (a: number, b: number, c: number) => number;
  readonly lexer_startsWithStringOpenDelimiter: (a: number, b: number, c: number) => number;
  readonly lexer_hasFormatStringClose: (a: number, b: number, c: number) => number;
  readonly lexer_clone: (a: number) => number;
  readonly lexer_removeFormatOpen: (a: number, b: number, c: number, d: number) => void;
  readonly lexer_startsWithCommentCharacter: (a: number, b: number, c: number) => number;
  readonly lexer_hasCommentInitial: (a: number, b: number, c: number) => number;
  readonly lexer_removeFormatClose: (a: number, b: number, c: number, d: number) => void;
  readonly lexer_removeStringOpenDelimiterIndex: (a: number, b: number, c: number, d: number) => void;
  readonly lexer_hasFormatStringOpen: (a: number, b: number, c: number) => number;
  readonly lexer_getStringOpeningDelimiterIndex: (a: number, b: number, c: number, d: number) => void;
  readonly lexer_getStringClosingDelimiterIndex: (a: number, b: number, c: number, d: number) => void;
  readonly lexer_getFormatedStringOpeningDelimiterIndex: (a: number, b: number, c: number, d: number) => void;
  readonly lexer_getFormatedStringClosingDelimiterIndex: (a: number, b: number, c: number, d: number) => void;
  readonly lexer_getTokens: (a: number, b: number, c: number, d: number) => void;
  readonly lexer_isStringOpenDelimiter: (a: number, b: number, c: number) => number;
  readonly lexer_removeStringCloseDelimiterIndex: (a: number, b: number, c: number, d: number) => void;
  readonly __wbg_scene_free: (a: number) => void;
  readonly scene_new_js: (a: number, b: number, c: number) => number;
  readonly scene_getFps: (a: number) => number;
  readonly scene_getHeight: (a: number) => number;
  readonly scene_getWidth: (a: number) => number;
  readonly scene_renderFrame: (a: number) => void;
  readonly scene_clear: (a: number) => void;
  readonly scene_restore: (a: number, b: number) => void;
  readonly scene_saveState: (a: number, b: number) => void;
  readonly scene_setTopLeftCorner: (a: number, b: number, c: number) => void;
  readonly scene_setBottomRightCorner: (a: number, b: number, c: number) => void;
  readonly scene_getTopLeftCorner: (a: number) => number;
  readonly scene_getBottomRightCorner: (a: number) => number;
  readonly scene_setBackground: (a: number, b: number) => void;
  readonly scene_add: (a: number, b: number) => void;
  readonly scene_insert: (a: number, b: number, c: number) => void;
  readonly scene_remove: (a: number, b: number) => void;
  readonly scene_getObjects: (a: number) => number;
  readonly scene_getObjectsFromIndices: (a: number, b: number) => number;
  readonly scene_setCanvasContext: (a: number, b: number) => void;
  readonly scene_sleep: (a: number, b: number) => number;
  readonly scene_setObjects: (a: number, b: number) => void;
  readonly scene_play: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly scene_makeFrame: (a: number, b: number, c: number, d: number) => number;
  readonly scene_wait: (a: number, b: number) => number;
  readonly scene_setCallback: (a: number, b: number) => void;
  readonly scene_callCallback: (a: number) => number;
  readonly __wbg_theme_free: (a: number) => void;
  readonly theme_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number, x: number, y: number, z: number, a1: number, b1: number, c1: number, d1: number, e1: number, f1: number, g1: number, h1: number, i1: number, j1: number, k1: number, l1: number) => number;
  readonly theme_getKeywordColor: (a: number, b: number) => void;
  readonly theme_getSpecialColor: (a: number, b: number) => void;
  readonly theme_getIllegalColor: (a: number, b: number) => void;
  readonly theme_getDeclarationColor: (a: number, b: number) => void;
  readonly theme_getOperatorColor: (a: number, b: number) => void;
  readonly theme_getParenColor: (a: number, b: number) => void;
  readonly theme_getConstantColor: (a: number, b: number) => void;
  readonly theme_getNumberColor: (a: number, b: number) => void;
  readonly theme_getStringColor: (a: number, b: number) => void;
  readonly theme_getIdentifierColor: (a: number, b: number) => void;
  readonly theme_getAssignColor: (a: number, b: number) => void;
  readonly theme_getClassIdentifierColor: (a: number, b: number) => void;
  readonly theme_getSeparatorColor: (a: number, b: number) => void;
  readonly theme_getMethodDeclarationColor: (a: number, b: number) => void;
  readonly theme_getMethodIdentifierColor: (a: number, b: number) => void;
  readonly theme_getFormattedStringColor: (a: number, b: number) => void;
  readonly theme_getFormatOpenColor: (a: number, b: number) => void;
  readonly theme_getFormatCloseColor: (a: number, b: number) => void;
  readonly theme_clone: (a: number) => number;
  readonly theme_getCommentColor: (a: number, b: number) => void;
  readonly getGithubDark: () => number;
  readonly getPythonLexer: () => number;
  readonly makeTimings: (a: number, b: number, c: number) => void;
  readonly animationGroup: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly create: (a: number, b: number) => number;
  readonly drawStrokeThenFill: (a: number, b: number, c: number, d: number) => number;
  readonly write: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly fadeIn: (a: number, b: number, c: number, d: number) => number;
  readonly fadeOut: (a: number, b: number, c: number, d: number) => number;
  readonly growArrowWithFinalTip: (a: number, b: number) => number;
  readonly growArrowWithInitialTip: (a: number, b: number) => number;
  readonly growArrowWithTipsAtBothEnds: (a: number, b: number) => number;
  readonly growFromCenter: (a: number, b: number) => number;
  readonly morphShape: (a: number, b: number, c: number) => number;
  readonly moveCameraSVG: (a: number, b: number, c: number, d: number) => void;
  readonly moveCamera: (a: number, b: number, c: number, d: number) => void;
  readonly rotateAnimation: (a: number, b: number, c: number) => number;
  readonly scaleInPlace: (a: number, b: number, c: number) => number;
  readonly setFillAnimation: (a: number, b: number, c: number) => number;
  readonly setStrokeAnimation: (a: number, b: number, c: number) => number;
  readonly shiftAnimation: (a: number, b: number, c: number) => number;
  readonly showTemporaily: (a: number, b: number) => number;
  readonly spinningGrow: (a: number, b: number, c: number) => number;
  readonly textToVector: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly __wbg_svgscene_free: (a: number) => void;
  readonly svgscene_new_js: (a: number, b: number, c: number) => number;
  readonly svgscene_getFps: (a: number) => number;
  readonly svgscene_getHeight: (a: number) => number;
  readonly svgscene_getWidth: (a: number) => number;
  readonly svgscene_renderFrame: (a: number) => void;
  readonly svgscene_clear: (a: number) => void;
  readonly svgscene_restore: (a: number, b: number) => void;
  readonly svgscene_saveState: (a: number, b: number) => void;
  readonly svgscene_setTopLeftCorner: (a: number, b: number, c: number) => void;
  readonly svgscene_setBottomRightCorner: (a: number, b: number, c: number) => void;
  readonly svgscene_getTopLeftCorner: (a: number) => number;
  readonly svgscene_getBottomRightCorner: (a: number) => number;
  readonly svgscene_setBackground: (a: number, b: number) => void;
  readonly svgscene_add: (a: number, b: number) => void;
  readonly svgscene_insert: (a: number, b: number, c: number) => void;
  readonly svgscene_remove: (a: number, b: number) => void;
  readonly svgscene_getObjectsFromIndices: (a: number, b: number) => number;
  readonly svgscene_setDivContainer: (a: number, b: number) => void;
  readonly svgscene_sleep: (a: number, b: number) => number;
  readonly svgscene_play: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly svgscene_makeFrame: (a: number, b: number, c: number, d: number) => number;
  readonly svgscene_setObjects: (a: number, b: number) => void;
  readonly svgscene_getObjects: (a: number) => number;
  readonly svgscene_wait: (a: number, b: number) => number;
  readonly svgscene_setCallback: (a: number, b: number) => void;
  readonly svgscene_callCallback: (a: number) => number;
  readonly svgscene_setClass: (a: number, b: number, c: number, d: number) => void;
  readonly svgscene_removeClass: (a: number, b: number) => void;
  readonly rotMatrix: (a: number, b: number) => number;
  readonly matrixProduct: (a: number, b: number) => number;
  readonly rotMatrixEuler: (a: number, b: number, c: number) => number;
  readonly transposeMatrix: (a: number) => number;
  readonly applyMatrix: (a: number, b: number) => number;
  readonly shiftPoints3D: (a: number, b: number) => number;
  readonly ensureValidThreeDColor: (a: number) => number;
  readonly __wbg_wasmlightsource_free: (a: number) => void;
  readonly wasmlightsource_new: (a: number) => number;
  readonly wasmlightsource_getPosition: (a: number) => number;
  readonly wasmlightsource_clone: (a: number) => number;
  readonly __wbg_wasmcamera_free: (a: number) => void;
  readonly wasmcamera_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly wasmcamera_getPosition: (a: number) => number;
  readonly wasmcamera_getRotation: (a: number) => number;
  readonly wasmcamera_getFocalDistance: (a: number) => number;
  readonly wasmcamera_getZoom: (a: number) => number;
  readonly wasmcamera_getWidth: (a: number) => number;
  readonly wasmcamera_getHeight: (a: number) => number;
  readonly wasmcamera_clone: (a: number) => number;
  readonly getShadedRgb: (a: number, b: number, c: number, d: number) => number;
  readonly getStartCorner: (a: number) => number;
  readonly getEndCorner: (a: number) => number;
  readonly crossProduct: (a: number, b: number) => number;
  readonly getUnitNormal: (a: number, b: number) => number;
  readonly getStartAnchors: (a: number) => number;
  readonly getEndAnchors: (a: number) => number;
  readonly getAnchors: (a: number) => number;
  readonly getCornerUnitNormal: (a: number, b: number) => number;
  readonly getStartCornerUnitNormal: (a: number) => number;
  readonly getEndCornerUnitNormal: (a: number) => number;
  readonly getShadedColor: (a: number, b: number, c: number, d: number) => number;
  readonly projectPoints: (a: number, b: number) => number;
  readonly lineAsCubicBezier3D: (a: number, b: number) => number;
  readonly __wbg_wasmthreedobject_free: (a: number) => void;
  readonly wasmthreedobject_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly wasmthreedobject_getPoints: (a: number) => number;
  readonly wasmthreedobject_getSubobjects: (a: number, b: number) => void;
  readonly wasmthreedobject_getFill: (a: number) => number;
  readonly wasmthreedobject_getStroke: (a: number) => number;
  readonly wasmthreedobject_getStrokeWidth: (a: number) => number;
  readonly wasmthreedobject_setPoints: (a: number, b: number) => number;
  readonly wasmthreedobject_setSubobjects: (a: number, b: number, c: number) => number;
  readonly wasmthreedobject_setFill: (a: number, b: number) => number;
  readonly wasmthreedobject_setStroke: (a: number, b: number) => number;
  readonly wasmthreedobject_setStrokeWidth: (a: number, b: number) => number;
  readonly wasmthreedobject_scale: (a: number, b: number, c: number) => number;
  readonly wasmthreedobject_stretch: (a: number, b: number, c: number) => number;
  readonly wasmthreedobject_shift: (a: number, b: number, c: number) => number;
  readonly wasmthreedobject_rotateX: (a: number, b: number, c: number) => number;
  readonly wasmthreedobject_rotateY: (a: number, b: number, c: number) => number;
  readonly wasmthreedobject_rotateZ: (a: number, b: number, c: number) => number;
  readonly wasmthreedobject_projectAndShade: (a: number, b: number, c: number) => number;
  readonly wasmthreedobject_fromUvFunction: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => number;
  readonly wasmthreedobject_getBoundingBox: (a: number) => number;
  readonly wasmthreedobject_getCenter: (a: number) => number;
  readonly wasmthreedobject_moveTo: (a: number, b: number, c: number) => number;
  readonly wasmthreedobject_clone: (a: number) => number;
  readonly mathjax: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly __wbg_token_free: (a: number) => void;
  readonly token_new: (a: number, b: number, c: number) => number;
  readonly token_getType: (a: number) => number;
  readonly token_getLiteral: (a: number, b: number) => void;
  readonly codeObject: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly radian: (a: number, b: number, c: number, d: number) => number;
  readonly ellipticalArcPath: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly getBbox: (a: number) => number;
  readonly center: (a: number, b: number) => number;
  readonly factorial: (a: number) => number;
  readonly hexToColor: (a: number, b: number, c: number) => number;
  readonly bezier: (a: number, b: number) => number;
  readonly bezierNumber: (a: number, b: number) => number;
  readonly permutation: (a: number, b: number) => number;
  readonly choose: (a: number, b: number) => number;
  readonly distanceSquared: (a: number, b: number, c: number, d: number) => number;
  readonly interpolate: (a: number, b: number, c: number) => number;
  readonly interpolateTuple: (a: number, b: number, c: number) => number;
  readonly interpolateTuple3D: (a: number, b: number, c: number) => number;
  readonly interpolateColor: (a: number, b: number, c: number) => number;
  readonly pointsFromAnchorsAndHandles: (a: number, b: number, c: number, d: number) => number;
  readonly startNewPath: (a: number, b: number) => number;
  readonly hasNewPathBegun: (a: number) => number;
  readonly getNthSubpath: (a: number, b: number) => number;
  readonly insertNCurvesToPointList: (a: number, b: number) => number;
  readonly nullPointAlign: (a: number, b: number) => number;
  readonly alignPoints: (a: number, b: number, c: number) => number;
  readonly addNMoreSubobjects: (a: number, b: number, c: number) => number;
  readonly alignSubobjects: (a: number, b: number, c: number, d: number) => void;
  readonly alignData: (a: number, b: number, c: number, d: number) => number;
  readonly integerInterpolate: (a: number, b: number, c: number) => number;
  readonly lineAsCubicBezier: (a: number, b: number, c: number, d: number) => number;
  readonly quadraticBezierAsCubicBezier: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly considerPointsEquals: (a: number, b: number, c: number, d: number) => number;
  readonly sigmoid: (a: number) => number;
  readonly linear: (a: number) => number;
  readonly smooth: (a: number, b: number) => number;
  readonly smoothstep: (a: number) => number;
  readonly smootherstep: (a: number) => number;
  readonly smoothererstep: (a: number) => number;
  readonly rushInto: (a: number, b: number) => number;
  readonly rushFrom: (a: number, b: number) => number;
  readonly doubleSmooth: (a: number) => number;
  readonly thereAndBack: (a: number, b: number) => number;
  readonly thereAndBackWithPause: (a: number, b: number) => number;
  readonly runningStart: (a: number, b: number) => number;
  readonly notQuiteThere: (a: number, b: number, c: number) => number;
  readonly wiggle: (a: number, b: number) => number;
  readonly squishRateFunc: (a: number, b: number, c: number, d: number) => number;
  readonly lingering: (a: number) => number;
  readonly exponentialDecay: (a: number, b: number) => number;
  readonly easeInSine: (a: number) => number;
  readonly easeOutSine: (a: number) => number;
  readonly easeInOutSine: (a: number) => number;
  readonly easeInQuad: (a: number) => number;
  readonly easeOutQuad: (a: number) => number;
  readonly easeInOutQuad: (a: number) => number;
  readonly easeInCubic: (a: number) => number;
  readonly easeOutCubic: (a: number) => number;
  readonly easeInOutCubic: (a: number) => number;
  readonly easeInQuart: (a: number) => number;
  readonly easeOutQuart: (a: number) => number;
  readonly easeInOutQuart: (a: number) => number;
  readonly easeInQuint: (a: number) => number;
  readonly easeOutQuint: (a: number) => number;
  readonly easeInOutQuint: (a: number) => number;
  readonly easeInExpo: (a: number) => number;
  readonly easeOutExpo: (a: number) => number;
  readonly easeInOutExpo: (a: number) => number;
  readonly easeInCirc: (a: number) => number;
  readonly easeOutCirc: (a: number) => number;
  readonly easeInOutCirc: (a: number) => number;
  readonly easeInBack: (a: number) => number;
  readonly easeInOutBack: (a: number) => number;
  readonly easeInElastic: (a: number) => number;
  readonly easeOutElastic: (a: number) => number;
  readonly easeInOutElastic: (a: number) => number;
  readonly easeInBounce: (a: number) => number;
  readonly easeInOutBounce: (a: number) => number;
  readonly __wbg_genericscene_free: (a: number) => void;
  readonly genericscene_fromScene: (a: number) => number;
  readonly genericscene_fromSVGScene: (a: number) => number;
  readonly genericscene_isScene: (a: number) => number;
  readonly genericscene_isSVGScene: (a: number) => number;
  readonly genericscene_isNodeScene: (a: number) => number;
  readonly genericscene_getFps: (a: number) => number;
  readonly genericscene_getHeight: (a: number) => number;
  readonly genericscene_getWidth: (a: number) => number;
  readonly genericscene_renderFrame: (a: number) => void;
  readonly genericscene_clear: (a: number) => void;
  readonly genericscene_restore: (a: number, b: number) => void;
  readonly genericscene_saveState: (a: number, b: number) => void;
  readonly genericscene_setTopLeftCorner: (a: number, b: number, c: number) => void;
  readonly genericscene_setBottomRightCorner: (a: number, b: number, c: number) => void;
  readonly genericscene_getTopLeftCorner: (a: number) => number;
  readonly genericscene_getBottomRightCorner: (a: number) => number;
  readonly genericscene_setBackground: (a: number, b: number) => void;
  readonly genericscene_add: (a: number, b: number) => void;
  readonly genericscene_insert: (a: number, b: number, c: number) => void;
  readonly genericscene_remove: (a: number, b: number) => void;
  readonly genericscene_getObjects: (a: number) => number;
  readonly genericscene_getObjectsFromIndices: (a: number, b: number) => number;
  readonly genericscene_setCanvasContext: (a: number, b: number) => void;
  readonly genericscene_setDivContainer: (a: number, b: number) => void;
  readonly genericscene_sleep: (a: number, b: number) => number;
  readonly genericscene_setObjects: (a: number, b: number) => void;
  readonly genericscene_play: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly genericscene_makeFrame: (a: number, b: number, c: number, d: number) => number;
  readonly genericscene_wait: (a: number, b: number) => number;
  readonly genericscene_setCallback: (a: number, b: number) => void;
  readonly genericscene_callCallback: (a: number) => number;
  readonly genericscene_setClass: (a: number, b: number, c: number, d: number) => void;
  readonly genericscene_setStyle: (a: number, b: number) => void;
  readonly slowInto: (a: number) => number;
  readonly easeOutBounce: (a: number) => number;
  readonly easeOutBack: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h737a3a4fd05449e6: (a: number, b: number) => number;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h36f54c9e7475dd01: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h271b87efd55193df: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
