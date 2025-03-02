## About Text and Formulas

<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.21/dist/katex.min.css" integrity="sha384-zh0CIslj+VczCZtlzBcjt5ppRcsAmDnRem7ESsYwWwg3m/OaJ2l4x7YBZl9Kxxib" crossorigin="anonymous">

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

// Replace 'worker.js' with the path to '/node_modules/@mathlikeanim-rs/renderer/dist/offscreen-canvas-worker.js'
const scene = new CanvasScene(1920, 1080, 'worker.js');
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

    const fontFace = new Uint8Array(await fetch('./latinmodern-math.otf').then(res => res.arrayBuffer()))

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

    const scene = new CanvasScene(1920, 1080, './node_modules/@mathlikeanim-rs/renderer/dist/offscreen-canvas-worker.js');
    const button = document.getElementById('run-button');
    const svgContent = await fetch('./assets/play.svg').then(res => res.text());
    button.innerHTML = svgContent;
    button.addEventListener('click', run);
    const canvas = document.getElementById('canvas');
    scene.canvas = canvas;
    const offscreen = canvas.transferControlToOffscreen();
    const worker = new Worker('./node_modules/@mathlikeanim-rs/renderer/dist/offscreen-canvas-worker.js');
    worker.postMessage(offscreen, [offscreen]);
    scene.worker = worker;
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

    const scene2 = new CanvasScene(1920, 1080, './node_modules/@mathlikeanim-rs/renderer/dist/offscreen-canvas-worker.js');
    const button2 = document.getElementById('run-button-2');
    const canvas2 = document.getElementById('canvas-2');
    scene2.canvas = canvas2;
    const offscreen2 = canvas2.transferControlToOffscreen();
    const worker2 = new Worker('./node_modules/@mathlikeanim-rs/renderer/dist/offscreen-canvas-worker.js');
    worker2.postMessage(offscreen2, [offscreen2]);
    scene2.worker = worker2;
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