use std::rc::Rc;

use wasm_bindgen::prelude::*;

/// A FontFace represents a font that can be used for rendering text.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct FontFace {
    /// The data of the font face.
    data: Rc<Vec<u8>>,
}

#[wasm_bindgen]
impl FontFace {
    /// Creates a new FontFace from font data.
    #[wasm_bindgen(constructor, return_description = "A new font face.")]
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