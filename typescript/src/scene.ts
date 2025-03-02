import { VectorObject } from "@mathlikeanim-rs/mathlikeanim-rs";

export type Animation = (oldObject: VectorObject, t: number) => VectorObject;
export type Easing = (t: number) => number;

/**
 * A scene that can animate VectorObjects.
 */
export default abstract class Scene {
    /**
     * The width of the scene.
     * @type {number}
     */
    readonly width: number;
    /**
     * The height of the scene.
     * @type {number}
     */
    readonly height: number;
    /**
     * The objects in the scene.
     * @type {VectorObject[]}
     */
    objects: VectorObject[];
    /**
     * Objects when previous animation was entirely finished.
     * @type {VectorObject[]}
     * @private
     */
    private oldObjects: VectorObject[];
    /**
     * Whether the scene is stopped.
     * @type {boolean}
     * @private
     */
    private stopped: boolean = false;

    /**
     * Creates a new Scene.
     * @param {number} width - The width of the scene.
     * @param {number} height - The height of the scene.
     */
    constructor(width: number, height: number) {
        this.width = width;
        this.height = height;
        this.objects = [];
        this.oldObjects = [];
    }

    /**
     * Plays an animation.
     * @param {Map<string | number, Animation>} animations - The animations to play.
     * @param {number} duration - The duration of the animation in milliseconds.
     * @param {Easing} easing - The easing function to use.
     * @returns {Promise<void>} - A promise that resolves when the animation has finished.
     * @async
     */
    async play(
        animations: Map<string | number, Animation>,
        duration: number,
        easing: Easing,
    ): Promise<void> {
        if (this.stopped) {
            return;
        }
        this.oldObjects = this.objects.map(object => object.clone());
        const start = performance.now();
        const animate = async () => {
            if (this.stopped) {
                return;
            }
            const t = (performance.now() - start) / duration;
            if (t < 1) {
                const progress = easing(t);
                this.animate(animations, progress);
                this.render();
                await new Promise(resolve => requestAnimationFrame(() => animate().then(resolve)));
            } else {
                const progress = easing(1);
                this.animate(animations, progress);
                this.render();
            }
        };
        await animate();
    };

    /**
     * Stops the scene.
     * @returns {void}
     */
    stop(): void {
        this.stopped = true;
    }

    /**
     * Resumes the scene.
     * @returns {void}
     */
    resume(): void {
        this.stopped = false;
    }

    /**
     * Animates the objects. Internal use only.
     * @param {Map<string | number, Animation>} animations - The animations to play.
     * @param {number} t - The progress of the animation.
     * @returns {void}
     * @private
     */
    private animate(animations: Map<string | number, (oldObject: VectorObject, t: number) => VectorObject>, t: number): void {
        for (const key of animations.keys()) {
            if (typeof key === "string") {
                const result = this.findOldObjectWithName(key);
                if (result === null) {
                    continue;
                }
                this.objects[result.index] = animations.get(key)!(result.object, t);
            }
            if (typeof key === "number") {
                this.objects[key] = animations.get(key)!(this.oldObjects[key], t);
            }
        }
    }

    /**
     * Finds an old object by name. Internal use only.
     * @param {string} name - The name of the object.
     * @returns {{ object: VectorObject, index: number } | null} - The old object and its index or null if not found.
     * @private
     */
    private findOldObjectWithName(name: string): { object: VectorObject, index: number } | null {
        for (const [index, object] of this.oldObjects.entries()) {
            if (object.name === name) {
                return { object, index };
            }
        }
        return null;
    }

    /**
     * Renders the scene. Must be implemented by subclasses.
     * @returns {Promise<void>} - A promise that resolves when the scene has been rendered.
     * @abstract
     * @async
     */
    abstract render(): Promise<void>;
}