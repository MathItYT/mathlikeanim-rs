use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Theme {
    #[wasm_bindgen(skip)]
    pub keyword_color: String,
    #[wasm_bindgen(skip)]
    pub special_color: String,
    #[wasm_bindgen(skip)]
    pub illegal_color: String,
    #[wasm_bindgen(skip)]
    pub declaration_color: String,
    #[wasm_bindgen(skip)]
    pub operator_color: String,
    #[wasm_bindgen(skip)]
    pub paren_color: String,
    #[wasm_bindgen(skip)]
    pub constant_color: String,
    #[wasm_bindgen(skip)]
    pub number_color: String,
    #[wasm_bindgen(skip)]
    pub string_color: String,
    #[wasm_bindgen(skip)]
    pub identifier_color: String,
    #[wasm_bindgen(skip)]
    pub assign_color: String,
    #[wasm_bindgen(skip)]
    pub separator_color: String,
    #[wasm_bindgen(skip)]
    pub method_identifier_color: String,
    #[wasm_bindgen(skip)]
    pub method_declaration_color: String,
    #[wasm_bindgen(skip)]
    pub formatted_string_color: String,
    #[wasm_bindgen(skip)]
    pub format_open_color: String,
    #[wasm_bindgen(skip)]
    pub format_close_color: String,
    #[wasm_bindgen(skip)]
    pub comment_color: String,
    #[wasm_bindgen(skip)]
    pub class_identifier_color: String,
}

#[wasm_bindgen]
impl Theme {
    #[wasm_bindgen(constructor)]
    pub fn new(
        keyword_color: String,
        special_color: String,
        illegal_color: String,
        declaration_color: String,
        operator_color: String,
        paren_color: String,
        constant_color: String,
        number_color: String,
        string_color: String,
        identifier_color: String,
        assign_color: String,
        separator_color: String,
        method_identifier_color: String,
        method_declaration_color: String,
        formatted_string_color: String,
        format_open_color: String,
        format_close_color: String,
        comment_color: String,
        class_identifier_color: String,
    ) -> Theme {
        Theme {
            keyword_color,
            special_color,
            illegal_color,
            declaration_color,
            operator_color,
            paren_color,
            constant_color,
            number_color,
            string_color,
            identifier_color,
            assign_color,
            separator_color,
            method_identifier_color,
            method_declaration_color,
            formatted_string_color,
            format_open_color,
            format_close_color,
            comment_color,
            class_identifier_color,
        }
    }
    #[wasm_bindgen(js_name = getKeywordColor)]
    pub fn get_keyword_color(&self) -> String {
        self.keyword_color.clone()
    }
    #[wasm_bindgen(js_name = getSpecialColor)]
    pub fn get_special_color(&self) -> String {
        self.special_color.clone()
    }
    #[wasm_bindgen(js_name = getIllegalColor)]
    pub fn get_illegal_color(&self) -> String {
        self.illegal_color.clone()
    }
    #[wasm_bindgen(js_name = getDeclarationColor)]
    pub fn get_declaration_color(&self) -> String {
        self.declaration_color.clone()
    }
    #[wasm_bindgen(js_name = getOperatorColor)]
    pub fn get_operator_color(&self) -> String {
        self.operator_color.clone()
    }
    #[wasm_bindgen(js_name = getParenColor)]
    pub fn get_paren_color(&self) -> String {
        self.paren_color.clone()
    }
    #[wasm_bindgen(js_name = getConstantColor)]
    pub fn get_constant_color(&self) -> String {
        self.constant_color.clone()
    }
    #[wasm_bindgen(js_name = getNumberColor)]
    pub fn get_number_color(&self) -> String {
        self.number_color.clone()
    }
    #[wasm_bindgen(js_name = getStringColor)]
    pub fn get_string_color(&self) -> String {
        self.string_color.clone()
    }
    #[wasm_bindgen(js_name = getIdentifierColor)]
    pub fn get_identifier_color(&self) -> String {
        self.identifier_color.clone()
    }
    #[wasm_bindgen(js_name = getAssignColor)]
    pub fn get_assign_color(&self) -> String {
        self.assign_color.clone()
    }
    #[wasm_bindgen(js_name = getClassIdentifierColor)]
    pub fn get_class_identifier_color(&self) -> String {
        self.class_identifier_color.clone()
    }
    #[wasm_bindgen(js_name = getSeparatorColor)]
    pub fn get_separator_color(&self) -> String {
        self.separator_color.clone()
    }
    #[wasm_bindgen(js_name = getMethodDeclarationColor)]
    pub fn get_method_declaration_color(&self) -> String {
        self.method_declaration_color.clone()
    }
    #[wasm_bindgen(js_name = getMethodIdentifierColor)]
    pub fn get_method_identifier_color(&self) -> String {
        self.method_identifier_color.clone()
    }
    #[wasm_bindgen(js_name = getFormattedStringColor)]
    pub fn get_formatted_string_color(&self) -> String {
        self.formatted_string_color.clone()
    }
    #[wasm_bindgen(js_name = getFormatOpenColor)]
    pub fn get_format_open_color(&self) -> String {
        self.format_open_color.clone()
    }
    #[wasm_bindgen(js_name = getFormatCloseColor)]
    pub fn get_format_close_color(&self) -> String {
        self.format_close_color.clone()
    }
    #[wasm_bindgen(js_name = clone)]
    pub fn clone_js(&self) -> Theme {
        self.clone()
    }
    #[wasm_bindgen(js_name = getCommentColor)]
    pub fn get_comment_color(&self) -> String {
        self.comment_color.clone()
    }
}
