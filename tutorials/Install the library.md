## Installation guide
Here is a step-by-step guide to install MathLikeAnim-rs on your JavaScript project.
### Step 1: Install the package
First, you'll need to run the following command to install the package:
```bash
npm i @mathlikeanim-rs/renderer
```
The above will install the Rust WASM core `@mathlikeanim-rs/mathlikeanim-rs` and the renderer `@mathlikeanim-rs/renderer`.
### Step 2: Import the package
After installing the package, you can import the package in your JavaScript code:
```javascript
import initWasm from '@mathlikeanim-rs/mathlikeanim-rs';
import { CanvasScene, SVGScene } from '@mathlikeanim-rs/renderer';
```

### Step 3: Create a scene
Now you can create a scene with the renderer you want:
```javascript
const scene = new CanvasScene(1920, 1080); // or SVGScene(1920, 1080)
document.body.appendChild(scene.canvas); // or scene.svg
```

### Step 4: Initialize the WASM core
Finally, you can initialize the WASM core and start using the library:
```javascript
initWasm().then((wasm) => {
    ...
});
```
That's it! You have successfully installed MathLikeAnim-rs on your JavaScript project. To use it, go to [Get Started](./tutorial-Get%20started.html) tutorial to learn how to create interactive mathematical animations for the web.

## How to use it in Python?
Python support is available, but for older versions of the library. We are working on updating the Python package to the latest version of the project. If you still want to use the Python package, you can install it by running:
```bash
pip install mathlikeanim-rs  # It's an old version of the project
```