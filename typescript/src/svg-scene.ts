import { Style, VectorObject } from "@mathlikeanim-rs/mathlikeanim-rs";
import Scene from "./scene";

/**
 * A scene that renders to a @type {SVGSVGElement}.
 */
export default class SVGScene extends Scene {
    svg: SVGSVGElement;
    
    /**
     * Creates a new @type {SVGScene}.
     * @param {number} width - The width of the SVG.
     * @param {number} height - The height of the SVG.
     */
    constructor(width: number, height: number) {
        super(width, height);
        this.svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
        this.svg.setAttribute("width", width.toString());
        this.svg.setAttribute("height", height.toString());
        this.svg.setAttribute("viewBox", `0 0 ${width} ${height}`);
    }

    /**
     * Renders the scene to the SVG. Preferably don't await to keep the animation smooth.
     * @returns {Promise<void>} - A promise that resolves when the scene has been rendered.
     */
    async render(): Promise<void> {
        this.svg.innerHTML = "";
        const defs = document.createElementNS("http://www.w3.org/2000/svg", "defs");
        this.svg.appendChild(defs);
        for (const [index, object] of this.objects.entries()) {
            this.renderObject(object, [index], defs, this.svg);
        }
    }

    /**
     * Renders a @type {VectorObject} to the SVG. Internal use only.
     * @param {VectorObject} object - The object to render.
     * @param {number[]} index - The index of the object.
     * @param {SVGDefsElement} defs - The defs element.
     * @param {SVGElement} parent - The parent element.
     * @returns {void}
     * @private
     */
    private renderObject(object: VectorObject, index: number[], defs: SVGDefsElement, parent: SVGElement): void {
        const g = document.createElementNS("http://www.w3.org/2000/svg", "g");
        g.setAttribute("id", `group-${index.join("-")}`);
        parent.appendChild(g);
        if (object.num_curves === 0) {
            const subG = document.createElementNS("http://www.w3.org/2000/svg", "g");
            g.appendChild(subG);
            for (const [child_index, child] of object.children.entries()) {
                this.renderObject(child, [...index, child_index], defs, subG);
            }
            return;
        }
        const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
        path.setAttribute("id", `path-${index.join("-")}`);
        path.setAttribute("d", this.renderPathData(object));
        path.setAttribute("fill", this.renderFill(object, defs, index));
        path.setAttribute("fill-rule", object.fill_rule);
        path.setAttribute("stroke", this.renderStroke(object, defs, index));
        path.setAttribute("stroke-width", object.stroke_width.toString());
        path.setAttribute("stroke-linecap", object.stroke_line_cap);
        path.setAttribute("stroke-linejoin", object.stroke_line_join);
        path.setAttribute("stroke-miterlimit", object.stroke_miter_limit.toString());
        path.setAttribute("stroke-dasharray", object.stroke_dash_array.join(" "));
        path.setAttribute("stroke-dashoffset", object.stroke_dash_offset.toString());
        path.setAttribute("transform", `matrix(${object.transform.a} ${object.transform.b} ${object.transform.c} ${object.transform.d} ${object.transform.e} ${object.transform.f})`);
        g.appendChild(path);
        if (object.num_children > 0) {
            const subG = document.createElementNS("http://www.w3.org/2000/svg", "g");
            g.appendChild(subG);
            for (const [child_index, child] of object.children.entries()) {
                this.renderObject(child, [...index, child_index], defs, subG);
            }
        }
    }

    /**
     * Renders the path data of a @type {VectorObject}. Internal use only.
     * @param {VectorObject} object - The object to render.
     * @returns {string} - The path data.
     * @private
     */
    private renderPathData(object: VectorObject): string {
        let path_data = "";
        object.subpaths.forEach(subpath => {
            const start = subpath.get(0);
            const end = subpath.get(subpath.len - 1);
            path_data += ` M ${start.x} ${start.y}`;
            subpath.cubic_bezier_tuples.forEach(tuple => {
                path_data += ` C ${tuple.first_control.x} ${tuple.first_control.y} ${tuple.second_control.x} ${tuple.second_control.y} ${tuple.end_anchor.x} ${tuple.end_anchor.y}`;
            });
            if (start.equals(end)) {
                path_data += " Z";
            }
        });
        return path_data.trim();
    }

    /**
     * Renders the fill of a @type {VectorObject}. Internal use only.
     * @param {VectorObject} object - The object to render the fill of.
     * @param {SVGDefsElement} defs - The defs element.
     * @param {number[]} index - The index of the object.
     * @returns {string} - The fill.
     * @private
     */
    private renderStyle(style: Style, defs: SVGDefsElement, index: number[], prefix: string): string {
        if (style.color) {
            return `rgba(${style.color.red}, ${style.color.green}, ${style.color.blue}, ${style.color.alpha})`;
        }
        if (style.linear_gradient) {
            const linear_gradient = style.linear_gradient;
            const linearGradient = document.createElementNS("http://www.w3.org/2000/svg", "linearGradient");
            linearGradient.setAttribute("id", `${prefix}-linear-gradient-${index.join("-")}`);
            linearGradient.setAttribute("x1", linear_gradient.p1.x.toString());
            linearGradient.setAttribute("y1", linear_gradient.p1.y.toString());
            linearGradient.setAttribute("x2", linear_gradient.p2.x.toString());
            linearGradient.setAttribute("y2", linear_gradient.p2.y.toString());
            linear_gradient.color_stops.forEach((stop, index) => {
                const stopElement = document.createElementNS("http://www.w3.org/2000/svg", "stop");
                stopElement.setAttribute("offset", stop.position.toString());
                stopElement.setAttribute("stop-color", `rgba(${stop.color.red}, ${stop.color.green}, ${stop.color.blue}, ${stop.color.alpha})`);
                linearGradient.appendChild(stopElement);
            });
            defs.appendChild(linearGradient);
            return `url(#${prefix}-linear-gradient-${index.join("-")})`;
        }
        if (style.radial_gradient) {
            const radial_gradient = style.radial_gradient;
            const radialGradient = document.createElementNS("http://www.w3.org/2000/svg", "radialGradient");
            radialGradient.setAttribute("id", `${prefix}-radial-gradient-${index.join("-")}`);
            radialGradient.setAttribute("cx", radial_gradient.c.x.toString());
            radialGradient.setAttribute("cy", radial_gradient.c.y.toString());
            radialGradient.setAttribute("r", radial_gradient.r.toString());
            radialGradient.setAttribute("fx", radial_gradient.f.x.toString());
            radialGradient.setAttribute("fy", radial_gradient.f.y.toString());
            radial_gradient.color_stops.forEach((stop, index) => {
                const stopElement = document.createElementNS("http://www.w3.org/2000/svg", "stop");
                stopElement.setAttribute("offset", stop.position.toString());
                stopElement.setAttribute("stop-color", `rgba(${stop.color.red}, ${stop.color.green}, ${stop.color.blue}, ${stop.color.alpha})`);
                radialGradient.appendChild(stopElement);
            });
            defs.appendChild(radialGradient);
            return `url(#${prefix}-radial-gradient-${index.join("-")})`;
        }
        if (style.image) {
            const pattern = document.createElementNS("http://www.w3.org/2000/svg", "pattern");
            pattern.setAttribute("id", `${prefix}-pattern-${index.join("-")}`);
            pattern.setAttribute("x", style.image.x.toString());
            pattern.setAttribute("y", style.image.y.toString());
            pattern.setAttribute("width", style.image.width.toString());
            pattern.setAttribute("height", style.image.height.toString());
            pattern.setAttribute("patternUnits", "userSpaceOnUse");
            const data = style.image.data;
            const base64 = btoa(String.fromCharCode(...data));
            const image = document.createElementNS("http://www.w3.org/2000/svg", "image");
            image.setAttribute("x", "0");
            image.setAttribute("y", "0");
            image.setAttribute("width", style.image.width.toString());
            image.setAttribute("height", style.image.height.toString());
            image.setAttribute("href", `data:image/png;base64,${base64}`);
            pattern.appendChild(image);
            defs.appendChild(pattern);
            return `url(#${prefix}-pattern-${index.join("-")})`;
        }
        return "none";
    }

    /**
     * Renders the stroke of a @type {VectorObject}. Internal use only.
     * @param {VectorObject} object - The object to render the stroke of.
     * @param {SVGDefsElement} defs - The defs element.
     * @param {number[]} index - The index of the object.
     * @returns {string} - The stroke.
     * @private
     */
    private renderFill(object: VectorObject, defs: SVGDefsElement, index: number[]): string {
        return this.renderStyle(object.fill, defs, index, "fill");
    }

    /**
     * Renders the stroke of a @type {VectorObject}. Internal use only.
     * @param {VectorObject} object - The object to render the stroke of.
     * @param {SVGDefsElement} defs - The defs element.
     * @param {number[]} index - The index of the object.
     * @returns {string} - The stroke.
     * @private
     */
    private renderStroke(object: VectorObject, defs: SVGDefsElement, index: number[]): string {
        return this.renderStyle(object.stroke, defs, index, "stroke");
    }
}