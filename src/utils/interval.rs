use wasm_bindgen::prelude::*;

/// A ClosedInterval represents a closed interval [start, end].
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct ClosedInterval {
    /// The start of the interval.
    start: f32,
    /// The end of the interval.
    end: f32,
}

#[wasm_bindgen]
impl ClosedInterval {
    /// Creates a new Interval from a start and end value.
    #[wasm_bindgen(constructor, return_description = "A new interval.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The start of the interval.")]
        start: f32,
        #[wasm_bindgen(param_description = "The end of the interval.")]
        end: f32
    ) -> ClosedInterval {
        ClosedInterval { start, end }
    }

    /// Returns the start of the interval.
    #[wasm_bindgen(getter, return_description = "The start of the interval.")]
    pub fn start(&self) -> f32 {
        self.start
    }

    /// Returns the end of the interval.
    #[wasm_bindgen(getter, return_description = "The end of the interval.")]
    pub fn end(&self) -> f32 {
        self.end
    }

    /// Returns the length of the interval.
    #[wasm_bindgen(getter, return_description = "The length of the interval.")]
    pub fn length(&self) -> f32 {
        self.end - self.start
    }

    /// Returns the midpoint of the interval.
    #[wasm_bindgen(return_description = "The midpoint of the interval.")]
    pub fn midpoint(&self) -> f32 {
        (self.start + self.end) * 0.5
    }

    /// Samples the interval at a given number of points.
    #[wasm_bindgen(return_description = "The sampled points of the interval.")]
    pub fn sample(
        &self,
        #[wasm_bindgen(param_description = "The number of points to sample.")]
        num_points: usize
    ) -> Vec<f32> {
        let step = self.length() / (num_points as f32 - 1.0);
        (0..num_points)
            .map(|i| self.start + step * (i as f32))
            .collect()
    }

    /// Checks if a number is contained within the interval.
    #[wasm_bindgen(return_description = "Whether the number is contained within the interval.")]
    pub fn contains(
        &self,
        #[wasm_bindgen(param_description = "The number to check if it is contained.")]
        number: f32
    ) -> bool {
        number >= self.start && number <= self.end
    }
}