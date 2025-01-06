# MathLikeAnim-rs

![MathLikeAnim-rs](https://raw.githubusercontent.com/MathItYT/mathlikeanim-rs/refs/heads/master/banner.png "MathLikeAnim-rs")

This is an animation library written in Rust, widely inspired by [Manim](https://manim.community/) and, unlike Manim, it allows interactivity thanks to WebAssembly, JavaScript and the web.


## Features
- [x] Interactivity.
- [x] Basic shapes.
- [x] Function plotting.
- [x] Animations in HTML Canvas and SVG.
- [x] Text rendering.
- [x] LaTeX rendering.
- [x] 3D rendering.
- [x] Browser support.
- [x] Python support.


## How to install it?
If you want to start creating mathematical and interactive animations, you can install it by running:

```bash
npm i mathlikeanim-rs
```


## How to use it in the browser?
You can embed the library in your HTML file by adding a script tag, and don't forget to include your HTML Canvas for raster animations or DIV container for SVG animations!

```html
<script type="module">
    import initWasm from './node_modules/mathlikeanim-rs/index.js';

    initWasm().then((wasm) => {
        ...
    });
</script>
```


## How to use it in Python?
You can use the Python package to create animations in the same way you would do with the Rust library. You can install it by running:

```bash
pip install mathlikeanim-rs
```


## Development
If you want to contribute to this project, you can clone a fork of this repository and work with the Rust codebase.

By the moment we don't have any tests, so your contributions are welcome!


## Documentation
You can find the documentation of this project in [https://mathlikeanim-rs.vercel.app/](https://mathlikeanim-rs.vercel.app/).