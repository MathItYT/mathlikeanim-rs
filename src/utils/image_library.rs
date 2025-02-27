use std::{collections::HashMap, sync::Arc};

use wasm_bindgen::prelude::*;

/// A ImageData represents the data of an image.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct ImageData {
    /// The data of the image.
    data: Arc<Vec<u8>>,
}

#[wasm_bindgen]
impl ImageData {
    /// Creates a new ImageData from image data.
    #[wasm_bindgen(constructor, return_description = "A new image data.")]
    pub fn new(data: Vec<u8>) -> ImageData {
        ImageData { data: Arc::new(data) }
    }

    /// Returns the data of the image.
    #[wasm_bindgen(getter, return_description = "The data of the image.")]
    pub fn data(&self) -> Vec<u8> {
        self.data.to_vec()
    }

    /// Clones the image data.
    #[wasm_bindgen(js_name = clone)]
    pub fn copy(&self) -> ImageData {
        self.clone()
    }
}

/// A ImageLibrary represents a mapping of image names to image data.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct ImageLibrary {
    /// The mapping of image names to image data.
    data: HashMap<String, ImageData>,
}

#[wasm_bindgen]
impl ImageLibrary {
    /// Creates a new ImageLibrary from image data.
    #[wasm_bindgen(constructor, return_description = "A new image library.")]
    pub fn new() -> ImageLibrary {
        ImageLibrary { data: HashMap::new() }
    }

    /// Returns the image names in the library.
    #[wasm_bindgen(getter, return_description = "The names of the images in the library.")]
    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    /// Returns the image data
    #[wasm_bindgen(getter, return_description = "The data of the image.")]
    pub fn values(&self) -> Vec<ImageData> {
        self.data.values().cloned().collect()
    }

    /// Gets the image data for a given image name.
    #[wasm_bindgen(return_description = "The data of the image.")]
    pub fn get(&self, name: &str) -> Option<ImageData> {
        self.data.get(name).cloned()
    }

    /// Sets the image data for a given image name.
    #[wasm_bindgen]
    pub fn set(&mut self, name: String, data: ImageData) {
        self.data.insert(name, data);
    }

    /// Removes the image data for a given image name.
    #[wasm_bindgen]
    pub fn remove(&mut self, name: &str) {
        self.data.remove(name);
    }

    /// Clones the image library.
    #[wasm_bindgen(js_name = clone)]
    pub fn copy(&self) -> ImageLibrary {
        self.clone()
    }
}
