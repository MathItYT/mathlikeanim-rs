import { VectorObject } from "@mathlikeanim-rs/mathlikeanim-rs";
import Scene from "./scene";

/**
 * A scene that renders to a HTMLCanvasElement.
 */
export default class CanvasScene extends Scene {
    /**
     * The canvas element. Must be appended to the DOM to be visible.
     * @type {HTMLCanvasElement}
     */
    canvas: HTMLCanvasElement;
    /**
     * The rendering context.
     * @type {CanvasRenderingContext2D}
     */
    context: CanvasRenderingContext2D;

    /**
     * Creates a new CanvasScene.
     * @param {number} width - The width of the canvas.
     * @param {number} height - The height of the canvas.
     */
    constructor(width: number, height: number) {
        super(width, height);
        this.canvas = document.createElement("canvas");
        this.canvas.width = width;
        this.canvas.height = height;
        this.context = this.canvas.getContext("2d")!;
    }

    /**
     * Renders the scene to the canvas. Preferably don't await to keep the animation smooth.
     * @returns {Promise<void>} - A promise that resolves when the scene has been rendered.
     */
    async render(): Promise<void> {
        this.context.resetTransform();
        this.context.clearRect(0, 0, this.width, this.height);
        for (const object of this.objects) {
            this.renderObject(object);
        }
    }

    /**
     * Renders a VectorObject to the canvas. Internal use only.
     * @param {VectorObject} object - The object to render.
     * @returns {void}
     * @private
     */
    private renderObject(object: VectorObject): void {
        if (object.num_curves === 0) {
            for (const child of object.children) {
                this.renderObject(child);
            }
            return;
        }
        const a = object.transform.a;
        const b = object.transform.b;
        const c = object.transform.c;
        const d = object.transform.d;
        const e = object.transform.e;
        const f = object.transform.f;
        this.context.setTransform(a, b, c, d, e, f);
        this.context.beginPath();
        object.subpaths.forEach(subpath => {
            const start = subpath.get(0);
            const end = subpath.get(subpath.len - 1);
            this.context.moveTo(start.x, start.y);
            subpath.cubic_bezier_tuples.forEach(tuple => {
                this.context.bezierCurveTo(
                    tuple.first_control.x, tuple.first_control.y,
                    tuple.second_control.x, tuple.second_control.y,
                    tuple.end_anchor.x, tuple.end_anchor.y
                );
            });
            if (start.equals(end)) {
                this.context.closePath();
            }
        });
        this.renderFill(object);
        this.renderStroke(object);
        for (const child of object.children) {
            this.renderObject(child);
        }
    }

    /**
     * Renders the fill of a VectorObject. Internal use only.
     * @param {VectorObject} object - The object to render the fill of.
     * @returns {void}
     * @private
     */
    private renderFill(object: VectorObject): void {
        const fill = object.fill;
        const fillRule = object.fill_rule === "nonzero" ? "nonzero" : "evenodd";
        if (fill.color) {
            this.context.fillStyle = `rgba(${fill.color.red}, ${fill.color.green}, ${fill.color.blue}, ${fill.color.alpha})`;
            this.context.fill(fillRule);
            return;
        }
        if (fill.linear_gradient) {
            const gradient = this.context.createLinearGradient(
                fill.linear_gradient.p1.x, fill.linear_gradient.p1.y,
                fill.linear_gradient.p2.x, fill.linear_gradient.p2.y
            );
            fill.linear_gradient.color_stops.forEach(stop => {
                gradient.addColorStop(stop.position, `rgba(${stop.color.red}, ${stop.color.green}, ${stop.color.blue}, ${stop.color.alpha})`);
            });
            this.context.fillStyle = gradient;
            this.context.fill(fillRule);
            return;
        }
        if (fill.radial_gradient) {
            // f, c and r are the focal point, center and radius of the circle
            const gradient = this.context.createRadialGradient(
                fill.radial_gradient.f.x, fill.radial_gradient.f.y, 0,
                fill.radial_gradient.c.x, fill.radial_gradient.c.y, fill.radial_gradient.r
            );
            fill.radial_gradient.color_stops.forEach(stop => {
                gradient.addColorStop(stop.position, `rgba(${stop.color.red}, ${stop.color.green}, ${stop.color.blue}, ${stop.color.alpha})`);
            });
            this.context.fillStyle = gradient;
            this.context.fill(fillRule);
            return;
        }
        if (fill.image) {
            const data = fill.image.data;
            const imageData = new ImageData(new Uint8ClampedArray(data), fill.image.width, fill.image.height);
            const ctx2 = document.createElement("canvas").getContext("2d")!;
            ctx2.putImageData(imageData, fill.image.x, fill.image.y, 0, 0, fill.image.width, fill.image.height);
            const pattern = this.context.createPattern(ctx2.canvas, "repeat")!;
            this.context.fillStyle = pattern;
            this.context.fill(fillRule);
            return;
        }
    }

    /**
     * Renders the stroke of a VectorObject. Internal use only.
     * @param {VectorObject} object - The object to render the stroke of.
     * @returns {void}
     * @private
     */
    private renderStroke(object: VectorObject): void {
        if (object.stroke_width === 0) {
            return;
        }
        const stroke = object.stroke;
        const strokeWidth = object.stroke_width;
        const lineCap = object.stroke_line_cap === "butt" ? "butt" : object.stroke_line_cap === "round" ? "round" : "square";
        const lineJoin = object.stroke_line_join === "miter" ? "miter" : object.stroke_line_join === "round" ? "round" : "bevel";
        const miterLimit = object.stroke_miter_limit;
        const dashArray = object.stroke_dash_array;
        const dashOffset = object.stroke_dash_offset;
        if (stroke.color) {
            this.context.strokeStyle = `rgba(${stroke.color.red}, ${stroke.color.green}, ${stroke.color.blue}, ${stroke.color.alpha})`;
            this.context.lineWidth = strokeWidth;
            this.context.lineCap = lineCap;
            this.context.lineJoin = lineJoin;
            this.context.miterLimit = miterLimit;
            this.context.setLineDash(new Array<number>(...dashArray));
            this.context.lineDashOffset = dashOffset;
            this.context.stroke();
            return;
        }
        if (stroke.linear_gradient) {
            const gradient = this.context.createLinearGradient(
                stroke.linear_gradient.p1.x, stroke.linear_gradient.p1.y,
                stroke.linear_gradient.p2.x, stroke.linear_gradient.p2.y
            );
            stroke.linear_gradient.color_stops.forEach(stop => {
                gradient.addColorStop(stop.position, `rgba(${stop.color.red}, ${stop.color.green}, ${stop.color.blue}, ${stop.color.alpha})`);
            });
            this.context.strokeStyle = gradient;
            this.context.lineWidth = strokeWidth;
            this.context.lineCap = lineCap;
            this.context.lineJoin = lineJoin;
            this.context.miterLimit = miterLimit;
            this.context.setLineDash(new Array<number>(...dashArray));
            this.context.lineDashOffset = dashOffset;
            this.context.stroke();
            return;
        }
        if (stroke.radial_gradient) {
            // f, c and r are the focal point, center and radius of the circle
            const gradient = this.context.createRadialGradient(
                stroke.radial_gradient.f.x, stroke.radial_gradient.f.y, 0,
                stroke.radial_gradient.c.x, stroke.radial_gradient.c.y, stroke.radial_gradient.r
            );
            stroke.radial_gradient.color_stops.forEach(stop => {
                gradient.addColorStop(stop.position, `rgba(${stop.color.red}, ${stop.color.green}, ${stop.color.blue}, ${stop.color.alpha})`);
            });
            this.context.strokeStyle = gradient;
            this.context.lineWidth = strokeWidth;
            this.context.lineCap = lineCap;
            this.context.lineJoin = lineJoin;
            this.context.miterLimit = miterLimit;
            this.context.setLineDash(new Array<number>(...dashArray));
            this.context.lineDashOffset = dashOffset;
            this.context.stroke();
            return;
        }
        if (stroke.image) {
            const data = stroke.image.data;
            const imageData = new ImageData(new Uint8ClampedArray(data), stroke.image.width, stroke.image.height);
            const ctx2 = document.createElement("canvas").getContext("2d")!;
            ctx2.putImageData(imageData, stroke.image.x, stroke.image.y, 0, 0, stroke.image.width, stroke.image.height);
            const pattern = this.context.createPattern(ctx2.canvas, "repeat")!;
            this.context.strokeStyle = pattern;
            this.context.lineWidth = strokeWidth;
            this.context.lineCap = lineCap;
            this.context.lineJoin = lineJoin;
            this.context.miterLimit = miterLimit;
            this.context.setLineDash(new Array<number>(...dashArray));
            this.context.lineDashOffset = dashOffset;
            this.context.stroke();
            return;
        }
    }
}