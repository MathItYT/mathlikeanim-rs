# MathLikeAnim-rs

![MathLikeAnim-rs](https://raw.githubusercontent.com/MathItYT/mathlikeanim-rs/refs/heads/master/banner.png "MathLikeAnim-rs")

This is an animation library written in Rust, widely inspired by [Manim](https://manim.community/) and, unlike Manim, it allows interactivity thanks to WebAssembly, JavaScript and the web.


## Features
- 🟢 Interactivity.
- 🟢 Basic shapes.
- 🟡 Function plotting.
- 🟢 Animations in HTML Canvas and SVG.
- 🟢 Text rendering.
- 🟢 Math formulas rendering.
- 🟡 3D rendering.
- 🟢 Browser support.
- 🟡 Python support.

### Meaning
- 🟢: Done.
- 🟡: Meant to be supported, but not available yet.
- 🔴: Not supported.


## How to use it in the browser?
You must install the `@mathlikeanim-rs/mathlikeanim-rs` (the Rust core) package and the `@mathlikeanim-rs/renderer`, both from NPM. Then, write an importmap and a module script tag to load the library.

```html
<script type="importmap">
    {
        "imports": {
            "@mathlikeanim-rs/renderer": "/node_modules/@mathlikeanim-rs/renderer/dist/index.js",
            "/node_modules/@mathlikeanim-rs/renderer/dist/canvas-scene": "/node_modules/@mathlikeanim-rs/renderer/dist/canvas-scene.js",
            "/node_modules/@mathlikeanim-rs/renderer/dist/scene": "/node_modules/@mathlikeanim-rs/renderer/dist/scene.js",
            "/node_modules/@mathlikeanim-rs/renderer/dist/svg-scene": "/node_modules/@mathlikeanim-rs/renderer/dist/svg-scene.js",
            "@mathlikeanim-rs/mathlikeanim-rs": "/node_modules/@mathlikeanim-rs/mathlikeanim-rs/index.js",
            "@mathlikeanim-rs/mathlikeanim-rs/": "/node_modules/@mathlikeanim-rs/mathlikeanim-rs/"
        }
    }
</script>
<script type="module">
    import initWasm from '@mathlikeanim-rs/mathlikeanim-rs';
    import { CanvasScene, SVGScene } from '@mathlikeanim-rs/renderer';

    const scene = new CanvasScene(
        // width
        1920,
        // height
        1080,
        // worker file (usually '/node_modules/@mathlikeanim-rs/renderer/dist/offscreen-canvas-worker.js')
        'worker.js'
    ); // or SVGScene(1920, 1080)
    document.body.appendChild(scene.canvas); // or scene.svg

    initWasm().then((wasm) => {
        ...
    });
</script>
```


## How to use it in Python?
Python support is available, but for older versions of the library. We are working on updating the Python package to the latest version of the project. If you still want to use the Python package, you can install it by running:

```bash
pip install mathlikeanim-rs  # It's an old version of the project
```


## Development
If you want to contribute to this project, you can clone a fork of this repository and work with the Rust codebase.

By the moment we don't have any tests, so your contributions are welcome!


## Documentation
Available [here](https://mathityt.github.io/mathlikeanim-rs/)!