use std::rc::Rc;

use typst::{foundations::Bytes, layout::Abs, text::Font};
use typst_as_lib::TypstTemplate;
use typst_svg::svg_merged;
use wasm_bindgen::prelude::*;

use crate::{objects::vector_object::VectorObjectBuilder, utils::font_face::FontFace};

/// A @type {Typst} is a typesetting object that can be used to render math text with Typst.
#[wasm_bindgen]
pub struct Typst {
    /// The source of the @type {Typst} object.
    source: Rc<String>,
}

#[wasm_bindgen]
impl Typst {
    /// Creates a new @type {Typst} object from a source string.
    #[wasm_bindgen(constructor, return_description = "A new typesetting object.")]
    pub fn new(
        #[wasm_bindgen(param_description = "The source of the typesetting object.")]
        source: String,
    ) -> Typst {
        Typst { source: Rc::new(source) }
    }
    /// Returns the source of the @type {Typst} object.
    #[wasm_bindgen(getter, return_description = "The source of the typesetting object.")]
    pub fn source(&self) -> String {
        self.source.to_string()
    }
    /// Renders the @type {Typst} object to an SVG string.
    #[wasm_bindgen(return_description = "The SVG string.")]
    pub fn render_to_svg(
        &self,
        #[wasm_bindgen(param_description = "The font faces to use when rendering the typesetting object.")]
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
            .map_err(|_| JsError::new("Failed to render typesetting object."))?;
        let padding = Abs::zero();
        let svg = svg_merged(&doc, padding);
        Ok(svg)
    }
    /// Renders the @type {Typst} object to a @type {VectorObjectBuilder}.
    #[wasm_bindgen(return_description = "A @type {VectorObjectBuilder} representing the typesetting object.")]
    pub fn render_to_vector_object_builder(
        &self,
        #[wasm_bindgen(param_description = "The @type {FontFace}s to use when rendering the typesetting object, if any.")]
        font_faces: Option<Vec<FontFace>>
    ) -> Result<VectorObjectBuilder, JsError> {
        let svg = self.render_to_svg(font_faces.clone())?;
        let builder = VectorObjectBuilder::from_svg(svg, font_faces);
        Ok(builder)
    }
}
