use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub enum TokenType {
    Illegal,
    Declaration,
    Comment,
    MethodDeclaration,
    MethodIdentifier,
    FormattedString,
    FormatOpen,
    FormatClose,
    Newline,
    Identifier,
    ClassIdentifier,
    Separator,
    Number,
    String,
    Assign,
    Operator,
    Whitespace,
    Constant,
    Keyword,
    Special,
    LParen,
    RParen
}