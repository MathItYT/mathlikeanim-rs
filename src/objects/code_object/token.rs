use super::token_type::TokenType;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    literal: String
}

#[wasm_bindgen]
impl Token {
    #[wasm_bindgen(constructor)]
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal
        }
    }
    #[wasm_bindgen(js_name = getType)]
    pub fn get_type(&self) -> TokenType {
        self.token_type.clone()
    }
    #[wasm_bindgen(js_name = getLiteral)]
    pub fn get_literal(&self) -> String {
        self.literal.clone()
    }
}