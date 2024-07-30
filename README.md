# MathLikeAnim-rs

![MathLikeAnim-rs](https://github.com/MathItYT/mathlikeanim-rs/blob/master/banner.png "MathLikeAnim-rs")

This is an animation library written in Rust, widely inspired by [Manim](https://manim.community/) and, unlike Manim, it allows interactivity thanks to WebAssembly, JavaScript and the web.


## Features
- [x] Interactivity.
- [x] Basic shapes.
- [x] Function plotting.
- [x] Animations in HTML Canvas, SVG and videos for Node.js.
- [x] Text rendering.
- [x] LaTeX rendering.
- [x] 3D rendering.
- [x] Browser and Node.js support.
- [ ] Python support (coming soon).


## How to use it in the browser?
You can embed the library in your HTML file by adding a script tag, and don't forget to include your HTML Canvas for raster animations or DIV container for SVG animations!

```html
<script type="module">
    import mathlikeanimRs from './node_modules/mathlikeanim-rs/browser/mathlikeanim_rs.js';
</script>
```

## How to use it in Node.js?
You can use the library in Node.js by importing the module and using it as a CommonJS module.

**Note:** Interactivity is not supported in Node.js.

```javascript
const mathlikeanimRs = require('mathlikeanim-rs')
```


## How to install it?
If you want to start creating mathematical and interactive animations, you can install it by running:

```bash
npm i mathlikeanim-rs
```


## Development
If you want to contribute to this project, you can clone a fork of this repository and work with the Rust codebase, and then build the WebAssembly module by running in the main directory:

```bash
make
```

By the moment we don't have any tests, so your contributions are welcome!


## Documentation
You can find the documentation of this project in [https://mathlikeanim-rs.vercel.app/](https://mathlikeanim-rs.vercel.app/).