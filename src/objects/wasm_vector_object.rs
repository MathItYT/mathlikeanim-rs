pub use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct WasmVectorObject {
    points: js_sys::Array,
    subobjects: js_sys::Array,
    index: usize,
    stroke_color: js_sys::Array,
    fill_color: js_sys::Array,
    stroke_width: f64,
    line_cap: String,
    line_join: String
}


#[wasm_bindgen]
impl WasmVectorObject {
    #[wasm_bindgen(constructor)]
    pub fn new(
        points: js_sys::Array,
        subobjects: js_sys::Array,
        index: usize,
        stroke_color: js_sys::Array,
        fill_color: js_sys::Array,
        stroke_width: f64,
        line_cap: String,
        line_join: String
    ) -> WasmVectorObject {
        WasmVectorObject {
            points,
            subobjects,
            index,
            stroke_color,
            fill_color,
            stroke_width,
            line_cap,
            line_join
        }
    }

    #[wasm_bindgen(getter)]
    pub fn get_points(&self) -> js_sys::Array {
        return self.points.clone();
    }

    #[wasm_bindgen(getter)]
    pub fn get_subobjects(&self) -> js_sys::Array {
        return self.subobjects.clone();
    }

    #[wasm_bindgen(getter)]
    pub fn get_index(&self) -> usize {
        return self.index;
    }

    #[wasm_bindgen(getter)]
    pub fn get_stroke_color(&self) -> js_sys::Array {
        return self.stroke_color.clone();
    }

    #[wasm_bindgen(getter)]
    pub fn get_fill_color(&self) -> js_sys::Array {
        return self.fill_color.clone();
    }

    #[wasm_bindgen(getter)]
    pub fn get_stroke_width(&self) -> f64 {
        return self.stroke_width;
    }

    #[wasm_bindgen(getter)]
    pub fn get_line_cap(&self) -> String {
        return self.line_cap.clone();
    }

    #[wasm_bindgen(getter)]
    pub fn get_line_join(&self) -> String {
        return self.line_join.clone();
    }

    #[wasm_bindgen(setter)]
    pub fn set_points(&mut self, points: js_sys::Array) {
        self.points = points;
    }

    #[wasm_bindgen(setter)]
    pub fn set_subobjects(&mut self, subobjects: js_sys::Array) {
        self.subobjects = subobjects;
    }

    #[wasm_bindgen(setter)]
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    #[wasm_bindgen(setter)]
    pub fn set_stroke_color(&mut self, stroke_color: js_sys::Array) {
        self.stroke_color = stroke_color;
    }

    #[wasm_bindgen(setter)]
    pub fn set_fill_color(&mut self, fill_color: js_sys::Array) {
        self.fill_color = fill_color;
    }

    #[wasm_bindgen(setter)]
    pub fn set_stroke_width(&mut self, stroke_width: f64) {
        self.stroke_width = stroke_width;
    }

    #[wasm_bindgen(setter)]
    pub fn set_line_cap(&mut self, line_cap: String) {
        self.line_cap = line_cap;
    }

    #[wasm_bindgen(setter)]
    pub fn set_line_join(&mut self, line_join: String) {
        self.line_join = line_join;
    }

    #[wasm_bindgen]
    pub fn clone(&self) -> WasmVectorObject {
        return WasmVectorObject {
            points: self.points.clone(),
            subobjects: self.subobjects.clone(),
            index: self.index,
            stroke_color: self.stroke_color.clone(),
            fill_color: self.fill_color.clone(),
            stroke_width: self.stroke_width,
            line_cap: self.line_cap.clone(),
            line_join: self.line_join.clone()
        };
    }
}