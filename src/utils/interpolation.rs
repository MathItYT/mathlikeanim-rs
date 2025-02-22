use wasm_bindgen::prelude::*;

/// Linearly interpolates between two values given a progress value.
#[wasm_bindgen(return_description = "The interpolated value.")]
pub fn lerp(
    #[wasm_bindgen(param_description = "The start value.")]
    a: f32,
    #[wasm_bindgen(param_description = "The end value.")]
    b: f32,
    #[wasm_bindgen(param_description = "The progress value.")]
    t: f32
) -> f32 {
    a + (b - a) * t
}

/// Returns the progress value between two numbers given an interpolated value.
#[wasm_bindgen(return_description = "The progress value.")]
pub fn inverse_lerp(
    #[wasm_bindgen(param_description = "The start value.")]
    a: f32,
    #[wasm_bindgen(param_description = "The end value.")]
    b: f32,
    #[wasm_bindgen(param_description = "The value to find the progress of.")]
    value: f32) -> f32 {
    if a == b {
        return 0.0;
    }
    (value - a) / (b - a)
}

/// A variant of lerp that returns the integer index and remainder. Useful for discrete interpolation.
#[wasm_bindgen]
pub struct IntegerLerp {
    /// The integer index.
    index: i32,
    /// The remainder.
    remainder: f32,
}

#[wasm_bindgen]
impl IntegerLerp {
    /// Interpolates between two values and returns the integer index and remainder.
    #[wasm_bindgen(constructor, return_description = "A class containing the integer index and remainder.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The start value.")]
        a: f32,
        #[wasm_bindgen(param_description = "The end value.")]
        b: f32,
        #[wasm_bindgen(param_description = "The progress value.")]
        t: f32
    ) -> IntegerLerp {
        if t >= 1.0 {
            return IntegerLerp { index: (b - 1.0).floor() as i32, remainder: 1.0 };
        }
        if t <= 0.0 {
            return IntegerLerp { index: a.floor() as i32, remainder: 0.0 };
        }
        let value = lerp(a, b, t);
        let index = value.floor() as i32;
        let remainder = value - index as f32;
        IntegerLerp { index, remainder }
    }

    /// Gets the integer index.
    #[wasm_bindgen(getter, return_description = "The integer index.")]
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Gets the remainder.
    #[wasm_bindgen(getter, return_description = "The remainder.")]
    pub fn remainder(&self) -> f32 {
        self.remainder
    }
}