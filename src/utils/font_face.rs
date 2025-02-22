use std::rc::Rc;

use wasm_bindgen::prelude::*;

/// A @type {FontFace} is a font face that can be used to render text.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct FontFace {
    /// The data of the font face.
    data: Rc<Vec<u8>>,
}

#[wasm_bindgen]
impl FontFace {
    /// Creates a new @type {FontFace} with the given data.
    #[wasm_bindgen(constructor, return_description = "A font face with the given name, weight, style, and data.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The data of the font face.")]
        data: Vec<u8>,
    ) -> FontFace {
        FontFace { data: Rc::new(data) }
    }

    /// Returns the data of the font face.
    #[wasm_bindgen(getter, return_description = "The data of the font face.")]
    pub fn data(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}