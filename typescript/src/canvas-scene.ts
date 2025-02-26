import { TransformationMatrix, VectorObject } from "@mathlikeanim-rs/mathlikeanim-rs";
import SVGScene from "./svg-scene";

/**
 * A scene that renders to a HTMLCanvasElement.
 */
export default class CanvasScene extends SVGScene {
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
        this.context.clearRect(0, 0, this.width, this.height);
        await super.render().then(async () => {;
            const svgAsBase64 = btoa(new XMLSerializer().serializeToString(this.svg));
            const img = new Image();
            const promise = new Promise(resolve => {
                img.onload = () => {
                    this.context.drawImage(img, 0, 0);
                    resolve(null);
                };
            });
            img.src = `data:image/svg+xml;base64,${svgAsBase64}`;
            await promise;
        });
    }
}