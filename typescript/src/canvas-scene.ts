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
     * @async
     */
    async render(): Promise<void> {
        await super.render();
        const svgAsBase64 = btoa(new XMLSerializer().serializeToString(this.svg));
        const img = new Image();
        await new Promise(resolve => {
            img.onload = () => {
                this.context.clearRect(0, 0, this.width, this.height);
                this.context.drawImage(img, 0, 0, this.width, this.height);
                resolve(null);
            };
            img.src = `data:image/svg+xml;base64,${svgAsBase64}`;
        });
    }
}