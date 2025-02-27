use std::rc::Rc;

use typst::{foundations::Bytes, layout::Abs, text::Font};
use typst_as_lib::TypstTemplate;
use typst_svg::svg_merged;
use wasm_bindgen::prelude::*;

use crate::{objects::vector_object::VectorObjectBuilder, utils::font_face::FontFace};

/// A Typst is a typesetting object that can be used to render math text with Typst.
#[wasm_bindgen]
pub struct Typst {
    /// The source of the Typst object.
    source: Rc<String>,
}

#[wasm_bindgen]
impl Typst {
    /// Creates a new Typst object from a source string.
    #[wasm_bindgen(constructor, return_description = "A new Typst object.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The source of the Typst object.")]
        source: String,
    ) -> Typst {
        Typst { source: Rc::new(source) }
    }
    /// Returns the source of the Typst object.
    #[wasm_bindgen(getter, return_description = "The source of the Typst object.")]
    pub fn source(&self) -> String {
        self.source.to_string()
    }
    /// Renders the Typst object to an SVG string.
    #[wasm_bindgen(return_description = "The SVG string.")]
    pub fn to_svg(
        &self,
        #[wasm_bindgen(param_description = "The font faces to use when rendering the Typst object.")]
        font_faces: Option<Vec<FontFace>>
    ) -> Result<String, JsError> {
        let fonts = font_faces.unwrap_or(Vec::new())
            .iter()
            .map(|font_face| font_face.data())
            .map(|font_data| Font::new(Bytes::new(font_data), 0))
            .collect::<Vec<Option<Font>>>();
        if fonts.iter().any(|font| font.is_none()) {
            return Err(JsError::new("Failed to load font."));
        }
        let fonts = fonts.into_iter().map(|font| font.unwrap()).collect::<Vec<Font>>();
        let template = TypstTemplate::new(self.source()).add_fonts(fonts);
        let doc = template
            .compile()
            .output
            .map_err(|_| JsError::new("Failed to render Typst object."))?;
        let padding = Abs::zero();
        let svg = svg_merged(&doc, padding);
        Ok(svg)
    }
    /// Renders the Typst object to a VectorObjectBuilder.
    #[wasm_bindgen(return_description = "A VectorObjectBuilder representing the typesetting object.")]
    pub fn vector_object_builder(
        &self,
        #[wasm_bindgen(param_description = "The FontFaces to use when rendering the Typst object, if any.")]
        font_faces: Option<Vec<FontFace>>,
    ) -> Result<VectorObjectBuilder, JsError> {
        let svg = self.to_svg(font_faces.clone())?;
        let builder = VectorObjectBuilder::from_svg(svg, font_faces, None);
        Ok(builder)
    }
}
