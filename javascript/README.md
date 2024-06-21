# MathLikeAnim-rs

![MathLikeAnim-rs](https://github.com/MathItYT/mathlikeanim-rs/blob/master/banner.png "MathLikeAnim-rs")

This is an animation library written in Rust, widely inspired by [Manim](https://manim.community/) and, unlike Manim, it allows interactivity thanks to WebAssembly, JavaScript and the web.


## Features
- [x] Interactivity.
- [x] Basic shapes.
- [x] Function plotting.
- [x] Animations in HTML Canvas and SVG.
- [x] Text rendering.
- [ ] 3D rendering (coming soon).


## How to use it?
You can embed the library in your HTML file by adding a script tag, and don't forget to include your HTML Canvas for raster animations or DIV container for SVG animations! You can either use the CDN or install it via npm.

```html
<script type="module">
import mathlikeanimRs from 'https://cdn.jsdelivr.net/npm/mathlikeanim-rs@0.7.2/+esm'
</script>
```


## How to install it?
If you want to start creating mathematical and interactive animations, you can install it by running:

```bash
npm i mathlikeanim-rs
```


## Development
If you want to contribute to this project, you can clone a fork of this repository and work with the Rust codebase, and then build the WebAssembly module by running in the main directory:

```bash
wasm-pack build --target web --out-dir javascript
```

By the moment we don't have any tests, so your contributions are welcome!