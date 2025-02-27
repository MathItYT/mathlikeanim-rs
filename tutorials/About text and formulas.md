## About Text and Formulas

<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.21/dist/katex.min.css" integrity="sha384-zh0CIslj+VczCZtlzBcjt5ppRcsAmDnRem7ESsYwWwg3m/OaJ2l4x7YBZl9Kxxib" crossorigin="anonymous">

<!-- Welcome to the **Get Started** tutorial! Here you will learn how to create interactive mathematical animations for the web using the **MathLikeAnim-rs** library. If you haven't installed the library yet, we recommend you to go to the [Installation Guide](./tutorial-Install%20the%20library.html) tutorial to learn how to install the library on your JavaScript project.

In the last guide, we coded the basic structure of a MathLikeAnim-rs project:

```javascript
import initWasm from '@mathlikeanim-rs/mathlikeanim-rs';
import { CanvasScene, SVGScene } from '@mathlikeanim-rs/renderer';

const scene = new CanvasScene(1920, 1080); // or SVGScene(1920, 1080)
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
const scene = new CanvasScene(1920, 1080); // or SVGScene(1920, 1080)
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
            // Set the stroke width to 0.05
            .set_stroke_width(0.05)
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
            .set_stroke_width(0.05)
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

    const scene = new CanvasScene(1920, 1080);
    const button = document.getElementById('run-button');
    const svgContent = await fetch('./assets/play.svg').then(res => res.text());
    button.innerHTML = svgContent;
    button.addEventListener('click', run);
    const canvas = document.getElementById('canvas');
    scene.canvas = canvas;
    scene.context = canvas.getContext('2d');
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

In the next tutorial, we will learn how to create more complex animations with multiple objects and interactions. Stay tuned! -->

You learned the basics of **MathLikeAnim-rs** in the previous tutorials ([Installation Guide](./tutorial-Install%20the%20library.html) and [Get Started](./tutorial-Get%20started.html)). Now, let's dive deeper into the library and learn how to work with text and formulas in your animations.

In this tutorial, we will cover the following topics:

1. **Handling Text**: How to add text to your animations in the right way.
2. **Working with Formulas**: How to render mathematical formulas in your animations.

Let's begin!

### Handling Text
Text may be an essential part of your animations, especially when explaining concepts or showing results. In **MathLikeAnim-rs**, you can add text to your animations using `VectorObjectBuilder.from_svg` method. This method allows you to create the shapes you see in SVG files, including text elements.

There's no built-in text object in **MathLikeAnim-rs**, but you can write the following function:

```javascript
/**
 * Create a text object with the specified text, position, font size, font family, and fill style.
 * @param {string} text The text to display.
 * @param {number} x The left x coordinate of the text.
 * @param {number} y The top y coordinate of the text.
 * @param {number} font_size The font size of the text.
 * @param {string} font_family The font family of the text.
 * @param {Style} fill_style The fill style of the text.
 * @returns {VectorObjectBuilder} The text object.
 */
function text(text, x, y, font_size, font_family, fill_style) {
    return VectorObjectBuilder.from_svg(`<svg xmlns="http://www.w3.org/2000/svg">
        <text x="${x}" y="${y}" font-size="${font_size}" font-family="${font_family}">${text}</text>
    </svg>`)
        .set_fill(fill_style);
}
```

This function creates an SVG text element with the specified text, position, font size, font family, and fill style. But wait, there's a catch! MathLikeAnim-rs doesn't know your font family from a simple string. You need to serve the font file (TTF or OTF) to the library and use the `FontFace` class to load the font data. To import `FontFace`, you can use the following import statement:

```javascript
import { FontFace } from '@mathlikeanim-rs/mathlikeanim-rs';
```

Then you can use JavaScript's `fetch` API to load the font file and create a `FontFace` instance:

```javascript
// Replace '/path/to/font.ttf' with the path to your font file
const font_data = new Uint8Array(await fetch('/path/to/font.ttf').then(res => res.arrayBuffer()));
const font = new FontFace(font_data);
```

Now, let's modify our `text` function to use the `FontFace` instance:

```javascript
/**
 * Create a text object with the specified text, position, font size, font family, and fill style.
 * @param {string} text The text to display.
 * @param {number} x The left x coordinate of the text.
 * @param {number} y The top y coordinate of the text.
 * @param {number} font_size The font size of the text.
 * @param {string} font_family The font family of the text.
 * @param {Style} fill_style The fill style of the text.
 * @param {FontFace} font_data The font data of the text.
 * @returns {VectorObjectBuilder} The text object.
 */
function text(text, x, y, font_size, font_family, fill_style, font_data) {
    return VectorObjectBuilder.from_svg(`<svg xmlns="http://www.w3.org/2000/svg">
        <text x="${x}" y="${y}" font-size="${font_size}" font-family="${font_family}">${text}</text>
    </svg>`, [font_data])
        .set_fill(fill_style)
}
```

With this function, you can create text objects in your animations. Let's see an example:

```javascript
// Replace '/path/to/font.ttf' with the path to your font file
const font = new FontFace(new Uint8Array(await fetch('/path/to/font.ttf').then(res => res.arrayBuffer())));
// Replace 'My font family' with the font family of your font
const text_obj = text('Hello, MathLikeAnim-rs!', 100, 100, 100, 'My font family', Style.from_color(new Color(0, 0, 0, 1)), font);
scene.objects.push(text_obj.build());
```

In this example, we created a text object with the text "Hello, MathLikeAnim-rs!" at position (100, 100), font size 100, font family Arial, and fill color black. We added the text object to the scene.

Now there's a full example of how to use text in your animations.

```javascript
import initWasm, { VectorObjectBuilder, Style, Color, FontFace, Rectangle, BoundingBox } from '@mathlikeanim-rs/mathlikeanim-rs';
import { CanvasScene } from '@mathlikeanim-rs/renderer';

/**
 * Create a text object with the specified text, position, font size, font family, and fill style.
 * @param {string} text The text to display.
 * @param {number} x The left x coordinate of the text.
 * @param {number} y The top y coordinate of the text.
 * @param {number} font_size The font size of the text.
 * @param {string} font_family The font family of the text.
 * @param {Style} fill_style The fill style of the text.
 * @param {FontFace} font_data The font data of the text.
 * @returns {VectorObjectBuilder} The text object.
 */
function text(text, x, y, font_size, font_family, fill_style, font_data) {
    return VectorObjectBuilder.from_svg(`<svg xmlns="http://www.w3.org/2000/svg">
        <text x="${x}" y="${y}" font-size="${font_size}" font-family="${font_family}">${text}</text>
    </svg>`, [font_data])
        .set_fill(fill_style);
}

const run = async () => {
    // You must serve the font file in your server
    const background = new Rectangle(new BoundingBox(0, 0, 1920, 1080))
        .vector_object_builder
        .set_fill(Style.from_color(new Color(0, 0, 0, 1)))
        .build();
    const font = new FontFace(new Uint8Array(await fetch('/latinmodern-math.otf').then(res => res.arrayBuffer())));
    const text_obj = text('Hello, MathLikeAnim-rs!', 100, 100, 100, 'Latin Modern Math', Style.from_color(new Color(0, 0, 0, 1)), font);
    scene.objects.push(background);
    scene.objects.push(text_obj.set_name('Text').build());
    const animations = new Map();
    animations.set('Text', (old, t) => {
        if (t == 1) {
            return old;
        }
        return new VectorObjectBuilder(old)
            .scale(1.5 - t * 0.5, 1.5 - t * 0.5)
            .set_name('Text')
            .build();
    });
    await scene.play(animations, 2000, t => t);
    // Free the objects and clear the objects array to avoid memory leaks
    scene.objects.map(obj => obj.free());
    scene.objects = [];
};

const scene = new CanvasScene(1920, 1080);
document.body.appendChild(scene.canvas);

initWasm().then(run);
```

<script type="importmap">
    {
        "imports": {
            "@mathlikeanim-rs/renderer": "./node_modules/@mathlikeanim-rs/renderer/dist/index.js",
            "./node_modules/@mathlikeanim-rs/renderer/dist/svg-scene": "./node_modules/@mathlikeanim-rs/renderer/dist/svg-scene.js",
            "./node_modules/@mathlikeanim-rs/renderer/dist/canvas-scene": "./node_modules/@mathlikeanim-rs/renderer/dist/canvas-scene.js",
            "./node_modules/@mathlikeanim-rs/renderer/dist/scene": "./node_modules/@mathlikeanim-rs/renderer/dist/scene.js",
            "@mathlikeanim-rs/mathlikeanim-rs": "./node_modules/@mathlikeanim-rs/mathlikeanim-rs/index.js",
            "@mathlikeanim-rs/mathlikeanim-rs/": "./node_modules/@mathlikeanim-rs/mathlikeanim-rs/",
            "katex": "https://cdn.jsdelivr.net/npm/katex@0.16.21/dist/katex.mjs"
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

Now you know how to handle text in your animations. Next, let's learn how to work with formulas.

### Working with Formulas
Mathematical formulas are a common element in educational and scientific animations. In **MathLikeAnim-rs**, you can render mathematical formulas using the `Typst` class. This class allows you to create text objects with [Typst syntax](https://typst.app/docs/), 

To use `Typst`, you need to import it:

```javascript
import { Typst } from '@mathlikeanim-rs/mathlikeanim-rs';
```

Then you can create a formula object using the `Typst` class (you can learn more about the syntax in the [Typst documentation](https://typst.app/docs/)):

```javascript
const formula = new Typst(`#set page(fill: none)
#show math.equation: set text(font: "Latin Modern Math")

$ sum_{i=1}^n i = (n(n + 1)) / 2 $`);
```

In this example, we created a formula object with <span class="katex">\displaystyle\sum_{i=1}^n i = \frac{n(n + 1)}{2}</span> using the Typst syntax. Remember that you **always** must serve the font file in your server and load it using `FontFace` and declare it in the `Typst` object.

To get a renderable object from the formula object, you can use the `vector_object_builder` method, where you pass the font data:

```javascript
const font = new FontFace(new Uint8Array(await fetch('/latinmodern-math.otf').then(res => res.arrayBuffer())));
const formula_obj = formula.vector_object_builder([font]);
scene.objects.push(formula_obj.build());
```

<div class="pre-div">
    <div class="pre-top-bar-container">
        <div class="code-lang-name-container">
            <div class="code-lang-name">RESULT</div>
        </div>
    </div>
    <div style="margin-top: 2rem;">
        <div style="display: flex; justify-content: center;">
            <canvas id="canvas-2" width="1920" height="1080"></canvas>
        </div>
        <div style="display: flex; justify-content: center;">
            <button class="icon-button" id="run-button-2"></button>
        </div>
    </div>
</div>

<script type="module">
    import initWasm, { VectorObjectBuilder, Style, Color, Rectangle, BoundingBox, Typst, FontFace, Point2D } from '@mathlikeanim-rs/mathlikeanim-rs';
    import { CanvasScene } from '@mathlikeanim-rs/renderer';
    import katex from "katex";

    /**
     * Create a text object with the specified text, position, font size, font family, and fill style.
     * @param {string} text The text to display.
     * @param {number} x The left x coordinate of the text.
     * @param {number} y The top y coordinate of the text.
     * @param {number} font_size The font size of the text.
     * @param {string} font_family The font family of the text.
     * @param {Style} fill_style The fill style of the text.
     * @param {FontFace} font_data The font data of the text.
     * @returns {VectorObjectBuilder} The text object.
     */
    function text(text, x, y, font_size, font_family, fill_style, font_data) {
        return VectorObjectBuilder.from_svg(`<svg xmlns="http://www.w3.org/2000/svg">
            <text x="${x}" y="${y}" font-size="${font_size}" font-family="${font_family}">${text}</text>
        </svg>`, [font_data])
            .set_fill(fill_style);
    }

    const fontFace = new Uint8Array(await fetch('/latinmodern-math.otf').then(res => res.arrayBuffer()))

    const run = async () => {
        button.disabled = true;
        const background = new Rectangle(new BoundingBox(0, 0, 1920, 1080))
            .vector_object_builder
            .set_fill(Style.from_color(new Color(0, 0, 0, 1)))
            .build();
        const font = new FontFace(fontFace);
        const text_obj = text('Hello, MathLikeAnim-rs!', 100, 100, 100, 'Latin Modern Math', Style.from_color(new Color(255, 255, 255, 1)), font);
        scene.objects.push(background);
        scene.objects.push(text_obj.set_name('Text').build());
        const animations = new Map();
        animations.set('Text', (old, t) => {
            if (t == 1) {
                return old;
            }
            return new VectorObjectBuilder(old)
                .scale(1.5 - t * 0.5, 1.5 - t * 0.5)
                .set_name('Text')
                .build();
        });
        await scene.play(animations, 2000, t => t);
        scene.objects.map(obj => obj.free());
        scene.objects = [];
        button.disabled = false;
    };

    const scene = new CanvasScene(1920, 1080);
    const button = document.getElementById('run-button');
    const svgContent = await fetch('./assets/play.svg').then(res => res.text());
    button.innerHTML = svgContent;
    button.addEventListener('click', run);
    const canvas = document.getElementById('canvas');
    scene.canvas = canvas;
    scene.context = canvas.getContext('2d');
    canvas.style.width = '80%';
    canvas.style.height = 'auto';

    const run2 = async () => {
        const background = new Rectangle(new BoundingBox(0, 0, 1920, 1080))
            .vector_object_builder
            .set_fill(Style.from_color(new Color(127, 127, 127, 1)))
            .build();
        const formula = new Typst(`#set page(fill: none)
#show math.equation: set text(font: "Latin Modern Math")

$ sum_(i=1)^n i = (n(n + 1)) / 2 $`);
        const font = new FontFace(fontFace);
        const formula_obj = formula.vector_object_builder([font]);
        scene2.objects.push(background);
        scene2.objects.push(formula_obj.scale_to_width(960).move_to(new Point2D(960, 540)).build());
        scene2.render();
        scene2.objects.map(obj => obj.free());
        scene2.objects = [];
    };

    const scene2 = new CanvasScene(1920, 1080);
    const button2 = document.getElementById('run-button-2');
    const canvas2 = document.getElementById('canvas-2');
    scene2.canvas = canvas2;
    scene2.context = canvas2.getContext('2d');
    canvas2.style.width = '80%';
    canvas2.style.height = 'auto';
    button2.innerHTML = svgContent;
    button2.addEventListener('click', run2);

    initWasm().then(() => {
        run();
        run2();
    });
    const katexEls = document.querySelectorAll('.katex');
    katexEls.forEach((element) => {
        element.outerHTML = katex.renderToString(element.innerHTML, { throwOnError: false });
    });
</script>

In the next tutorial, we will learn how to create more complex animations with multiple objects and interactions. Stay tuned! 👀