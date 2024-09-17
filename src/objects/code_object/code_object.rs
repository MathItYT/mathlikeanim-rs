use crate::{objects::wasm_interface::{WasmGradientImageOrColor, WasmVectorObject}, wasm_interface::hex_to_color_js};
use crate::objects::text_to_vector::text_to_vector;
use wasm_bindgen::prelude::*;

use super::{lexer::Lexer, theme::Theme, token_type::TokenType};

#[wasm_bindgen(js_name = codeObject)]
pub async fn code_object(
    code: String,
    lexer: &Lexer,
    theme: &Theme,
    font_family: String,
) -> WasmVectorObject {
    let mut subobjects = Vec::new();
    let code_lines = code.lines().collect::<Vec<&str>>();
    for (line_number, code_line) in code_lines.iter().enumerate() {
        let mut line = Vec::new();
        let mut code_line = code_line.to_string();
        if code_line.is_empty() {
            continue;
        }
        if code_line.ends_with("\r") {
            code_line.pop();
        }
        let line_object = text_to_vector(code_line.clone(), font_family.clone(), 0.0, 0.0, 144.0).await;
        let tokens = lexer.get_tokens(&code_line);
        let mut current_char = 0;
        for token in tokens.iter() {
            let literal = token.get_literal();
            let token_type = token.get_type();
            match token_type {
                TokenType::Whitespace => {
                    current_char += literal.len();
                    continue;
                }
                TokenType::Comment => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_comment_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::FormattedString => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_formatted_string_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::FormatOpen => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_format_open_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::FormatClose => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_format_close_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::Separator => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_separator_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::MethodDeclaration => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_method_declaration_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::MethodIdentifier => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_method_identifier_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::Illegal => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_illegal_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::String => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_string_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::Number => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_number_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::Newline => {
                    current_char += literal.len();
                    continue;
                }
                TokenType::Assign => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_assign_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::Identifier => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_identifier_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::Constant => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_constant_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::Keyword => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_keyword_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::Special => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_special_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::Declaration => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_declaration_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::Operator => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_operator_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::LParen => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_paren_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::RParen => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_paren_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
                TokenType::ClassIdentifier => {
                    let slice_subobjects = line_object.slice_subobjects(current_char, current_char + literal.len());
                    for subobject in slice_subobjects {
                        let fill = WasmGradientImageOrColor::from_color(hex_to_color_js(theme.get_class_identifier_color(), 1.0));
                        line.push(subobject.set_fill(fill, false));
                    }
                    current_char += literal.len();
                    continue;
                }
            }
        }
        let mut obj = WasmVectorObject::new().set_subobjects(line);
        if line_number > 0 {
            obj = obj.shift(0.0, 1.2 * 144.0 * line_number as f64, true)
        }
        subobjects.push(obj);
    }
    let text = WasmVectorObject::new().set_subobjects(subobjects);
    return text;
}
