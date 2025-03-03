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
     * The Worker used to render the scene offscreen.
     * @type {Worker}
     * @private
     */
    worker: Worker;

    /**
     * Creates a new CanvasScene.
     * @param {number} width - The width of the canvas.
     * @param {number} height - The height of the canvas.
     * @param {string} [workerFileName] - The endpoint of the worker script.
     */
    constructor(width: number, height: number, workerFileName?: string) {
        super(width, height);
        this.canvas = document.createElement("canvas");
        this.canvas.width = width;
        this.canvas.height = height;
        const offscreen = this.canvas.transferControlToOffscreen();
        this.worker = new Worker(workerFileName || '/node_modules/@mathlikeanim-rs/renderer/dist/offscreen-canvas-worker.js');
        this.worker.postMessage(offscreen, [offscreen]);
    }

    /**
     * Renders the scene to the canvas. Preferably don't await to keep the animation smooth.
     * @returns {Promise<void>} - A promise that resolves when the scene has been rendered.
     * @async
     */
    async render(): Promise<void> {
        await super.render();
        const svgAsBase64 = btoa(new XMLSerializer().serializeToString(this.svg));
        const dataUrl = `data:image/svg+xml;base64,${svgAsBase64}`;
        const img = new Image();
        img.src = dataUrl;
        await img.decode();
        const bmp = await createImageBitmap(img);
        this.worker.postMessage(bmp, [bmp]);
    }
}