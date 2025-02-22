# MathLikeAnim-rs

![MathLikeAnim-rs](https://raw.githubusercontent.com/MathItYT/mathlikeanim-rs/refs/heads/master/banner.png "MathLikeAnim-rs")

This is an animation library written in Rust, widely inspired by [Manim](https://manim.community/) and, unlike Manim, it allows interactivity thanks to WebAssembly, JavaScript and the web.


## Features
- [x] Interactivity.
- [x] Basic shapes.
- [ ] Function plotting.
- [x] Animations in HTML Canvas and SVG.
- [x] Text rendering.
- [x] Math formulas rendering.
- [ ] 3D rendering.
- [x] Browser support.
- [ ] Python support.


## How to install it?
If you want to start creating mathematical and interactive animations, you can install it by running:

```bash
npm i mathlikeanim-rs
```


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

    const scene = new CanvasScene(1920, 1080); // or SVGScene(1920, 1080)
    document.body.appendChild(scene.canvas); // or scene.svg

    initWasm().then((wasm) => {
        ...
    });
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
In progress...