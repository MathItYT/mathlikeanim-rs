## Get started with MathLikeAnim-rs!

Welcome to the **Get Started** tutorial! Here you will learn how to create interactive mathematical animations for the web using the **MathLikeAnim-rs** library. If you haven't installed the library yet, we recommend you to go to the [Installation Guide](./tutorial-Install%20the%20library.html) tutorial to learn how to install the library on your JavaScript project.

In the last guide, we coded the basic structure of a MathLikeAnim-rs project:

```javascript
import initWasm from '@mathlikeanim-rs/mathlikeanim-rs';
import { CanvasScene, SVGScene } from '@mathlikeanim-rs/renderer';

// Replace 'worker.js' with the server's path to `/node_modules/@mathlikeanim-rs/renderer/dist/offscreen-canvas-worker.js`
const scene = new CanvasScene(1920, 1080, 'worker.js'); // or SVGScene(1920, 1080)
document.body.appendChild(scene.canvas); // or scene.svg

initWasm().then((wasm) => {
    ...
});
```

Let's analyze this code:

1. We imported the `initWasm` function from the `@mathlikeanim-rs/mathlikeanim-rs` package. This function initializes the Rust WASM core of the library. We also imported the `CanvasScene` and `SVGScene` classes from the `@mathlikeanim-rs/renderer` package. These classes are used to create an HTML Canvas or SVG scene to render the animations.
```javascript
import initWasm from '@mathlikeanim-rs/mathlikeanim-rs';
import { CanvasScene, SVGScene } from '@mathlikeanim-rs/renderer';
```

2. We created a new scene with the `CanvasScene` or `SVGScene` class. The constructor of these classes receives the width and height of the scene. We appended the scene's canvas or SVG element to the document body.
```javascript
// Replace 'worker.js' with the server's path to `/node_modules/@mathlikeanim-rs/renderer/dist/offscreen-canvas-worker.js`
const scene = new CanvasScene(1920, 1080, 'worker.js'); // or SVGScene(1920, 1080)
document.body.appendChild(scene.canvas); // or scene.svg
```

3. We initialized the WASM core using the `initWasm` function. This function returns a promise that resolves to the WASM core instance. You can start using the library inside the promise's callback.
```javascript
initWasm().then((wasm) => {
    ...
});
```

Now that we have the basic structure of our project, let's create our first animation!

Replace the third step with the following code:

```javascript
const run = async () => {
    // Make a circle object
    const obj = new Circle(
        // Center will be the center of the screen
        // Resolution is 1920x1080
        // We calculate bottom right corner by adding 1920 to the left-most x and 1080 to the top-most y
        // Then we do (topLeft + bottomRight) / 2 to get the center
        // ((0, 0) + (1920, 1080)) / 2 = (960, 540)
        new Point2D(960, 540),
        // Radius of 500
        500,
    )
            // Render the object to a VectorObjectBuilder with many operations
            .vector_object_builder()
            // Set the fill color to red with 50% opacity
            .set_fill(Style.from_color(new Color(255, 0, 0, 0.5)))
            // Set the stroke color to red with 100% opacity
            .set_stroke(Style.from_color(new Color(255, 0, 0, 1)))
            // Set the stroke width to 10
            .set_stroke_width(10)
            // Set the name of the object to 'Circle'
            .set_name('Circle')
            // Build the object, this will run Rust WASM code in the background
            // and return a VectorObject instance, which is the finished object,
            // ready to add to the scene.
            .build();
    // Add a black background
    scene.objects.push(new Rectangle(new BoundingBox(
        // Top left corner
        0, 0,
        // Width and height
        1920, 1080
    )).vector_object_builder
        // Set the fill color to black
        .set_fill(Style.from_color(new Color(0, 0, 0, 1)))
        // Build the object, this will run Rust WASM code in the background
        // and return a VectorObject instance, which is the finished object,
        // ready to add to the scene.
        .build());
    // Add the VectorObject associated with the Circle to the scene
    scene.objects.push(obj);
    // Create a map of animations
    const animations = new Map();
    // Add an animation to the map. This animation will be for the object we named 'Circle'.
    animations.set('Circle', (old, t) => {
        if (t == 1) {
            return old;
        }
        // IntegerLerp is like lerp, but it has an index (floor of the lerp) and a remainder (fractional part of the lerp)
        // With lerp being from 0 to 1, index is 0 and remainder goes from 0 to 1 until the lerp is 1.
        // With lerp being from 1 to 2, index is 1 and remainder goes from 0 to 1 until the lerp is 2.
        // This is useful for animations that have multiple steps. In this case, we have 2 steps: 0 to 1 and 1 to 2.
        const drawPathT = new IntegerLerp(0, 2, t);
        // Return a new VectorObjectBuilder initially with the same object as the old one
        return new VectorObjectBuilder(old)
            // If we're in the first step, we want to draw the stroke of the circle.
            // This mathematically means that if we're at a time alpha (proportion from 0 to 1), we want to draw the stroke from the very start, proportion 0, to the current alpha.
            // If we're in the second step, the stroke is already drawn, so we want to draw the fill of the circle.
            // For that, we take the full path (from 0 to 1).
            .become_partial(0, drawPathT.index === 0 ? drawPathT.remainder : 1)
            // We want to make the fill fully transparent in the first step, because we're only drawing the stroke,
            // and fade in the second step. In the first step, we set fade factor to 1 (fully transparent), and in the second step, we set it to 1 - remainder (fully opaque). That goes from 1 to 0, so it fades in (0 means fully opaque).
            .fade_fill(drawPathT.index === 0 ? 1 : 1 - drawPathT.remainder)
            // Build the object, this will run Rust WASM code in the background
            .build();
    });
    // Await the play function, which will play the animations in the scene for 2000 milliseconds (2 seconds).
    await scene.play(animations, 2000, t => t);
    scene.objects.map(obj => obj.free());
    scene.objects = [];
};

init().then(run);
```

<script type="importmap">
    {
        "imports": {
            "@mathlikeanim-rs/renderer": "./node_modules/@mathlikeanim-rs/renderer/dist/index.js",
            "./node_modules/@mathlikeanim-rs/renderer/dist/canvas-scene": "./node_modules/@mathlikeanim-rs/renderer/dist/canvas-scene.js",
            "./node_modules/@mathlikeanim-rs/renderer/dist/scene": "./node_modules/@mathlikeanim-rs/renderer/dist/scene.js",
            "./node_modules/@mathlikeanim-rs/renderer/dist/svg-scene": "./node_modules/@mathlikeanim-rs/renderer/dist/svg-scene.js",
            "@mathlikeanim-rs/mathlikeanim-rs": "./node_modules/@mathlikeanim-rs/mathlikeanim-rs/index.js",
            "@mathlikeanim-rs/mathlikeanim-rs/": "./node_modules/@mathlikeanim-rs/mathlikeanim-rs/"
        }
    }
</script>

<div class="pre-div">
    <div class="pre-top-bar-container">
        <div class="code-lang-name-container">
            <div class="code-lang-name">RESULT</div>
        </div>
    </div>
    <div style="margin-top: 2rem;">
        <div style="display: flex; justify-content: center;">
            <canvas id="canvas" width="1920" height="1080"></canvas>
        </div>
        <div style="display: flex; justify-content: center;">
            <button class="icon-button" id="run-button"></button>
        </div>
    </div>
</div>

<script type="module">
    import initWasm from '@mathlikeanim-rs/mathlikeanim-rs';
    import { CanvasScene, SVGScene } from '@mathlikeanim-rs/renderer';
    import { Circle, Point2D, Style, Color, Rectangle, BoundingBox, VectorObjectBuilder, IntegerLerp } from '@mathlikeanim-rs/mathlikeanim-rs';

    const run = async () => {
        button.disabled = true;
        const obj = new Circle(new Point2D(960, 540), 500)
            .vector_object_builder()
            .set_fill(Style.from_color(new Color(255, 0, 0, 0.5)))
            .set_stroke(Style.from_color(new Color(255, 0, 0, 1)))
            .set_stroke_width(10)
            .set_name('Circle')
            .build();
        scene.objects.push(new Rectangle(new BoundingBox(0, 0, 1920, 1080))
            .vector_object_builder
            .set_fill(Style.from_color(new Color(0, 0, 0, 1)))
            .build());
        scene.objects.push(obj);
        const animations = new Map();
        animations.set('Circle', (old, t) => {
            if (t == 1) {
                return old;
            }
            const drawPathT = new IntegerLerp(0, 2, t);
            return new VectorObjectBuilder(old)
                .become_partial(0, drawPathT.index === 0 ? drawPathT.remainder : 1)
                .fade_fill(drawPathT.index === 0 ? 1 : 1 - drawPathT.remainder)
                .build();
        });
        await scene.play(animations, 2000, t => t);
        scene.objects.map(obj => obj.free());
        scene.objects = [];
        button.disabled = false;
    };

    const scene = new CanvasScene(1920, 1080, './node_modules/@mathlikeanim-rs/renderer/dist/offscreen-canvas-worker.js');
    const button = document.getElementById('run-button');
    const svgContent = await fetch('./assets/play.svg').then(res => res.text());
    button.innerHTML = svgContent;
    button.addEventListener('click', run);
    const canvas = document.getElementById('canvas');
    const offscreen = canvas.transferControlToOffscreen();
    scene.canvas = canvas;
    const worker = new Worker('./node_modules/@mathlikeanim-rs/renderer/dist/offscreen-canvas-worker.js');
    worker.postMessage(offscreen, [offscreen]);
    scene.worker = worker;
    canvas.style.width = '80%';
    canvas.style.height = 'auto';
    initWasm().then(() => run());
</script>

In this code, we created a circle object with a radius of 500 pixels and a center at the center of the screen. Then, to get a renderable object, we used the `vector_object_builder` getter to get a `VectorObjectBuilder` instance. This class allows us to apply operations to the object, like setting the fill and stroke colors, the stroke width, and the name of the object. After applying the operations, we called the `build` method to get a `VectorObject` instance, which is the finished object ready to add to the scene.

We also added a black background to the scene using a rectangle object. We set the fill color to black and added it to the scene.

Next, we created a map of animations. This map will contain the animations for the objects in the scene. We added an animation for the circle object. This animation will change the fill color of the circle from fully transparent to fully opaque in 2 seconds.

Finally, we called the `play` method on the scene object to play the animations. This method receives the map of animations, the duration of the animations in milliseconds, and a timing function. The timing function is used to control the speed of the animations. In this case, we used a linear timing function that goes from 0 to 1.

After playing the animations, we freed the objects in the scene and cleared the objects array. This is **so important** because the objects are stored in the WASM memory, and if we don't free them, we will have a memory leak.

Now you can click the play button to see the animation in action!

Congratulations! You have created your first animation with the MathLikeAnim-rs project! 🎉

In the next tutorial, we will learn how to work with text and formulas in the animations, just [here](./tutorial-About%20text%20and%20formulas.html). See you there! 👋
