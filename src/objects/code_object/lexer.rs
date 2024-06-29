use crate::utils::log;

use super::{token::Token, token_type::TokenType};
use regex::Regex;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[derive(Clone)]
pub struct Lexer {
    keywords: Vec<String>,
    specials: Vec<String>,
    illegals: Vec<String>,
    declarations: Vec<String>,
    method_declaration: Vec<String>,
    operators: Vec<String>,
    l_parens: Vec<String>,
    r_parens: Vec<String>,
    comment_initial_characters: Vec<String>,
    constants: Vec<String>,
    assignments: Vec<String>,
    separators: Vec<String>,
    string_open_delimiters: Vec<String>,
    string_close_delimiters: Vec<String>,
    formated_string_open_delimiters: Vec<String>,
    formated_string_close_delimiters: Vec<String>,
    format_opens: Vec<String>,
    format_closes: Vec<String>,
    class_identifier_pattern: String,
}


#[wasm_bindgen]
impl Lexer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        keywords: Vec<String>,
        specials: Vec<String>,
        illegals: Vec<String>,
        declarations: Vec<String>,
        method_declaration: Vec<String>,
        operators: Vec<String>,
        l_parens: Vec<String>,
        r_parens: Vec<String>,
        comment_initial_characters: Vec<String>,
        constants: Vec<String>,
        assignments: Vec<String>,
        separators: Vec<String>,
        string_open_delimiters: Vec<String>,
        string_close_delimiters: Vec<String>,
        formated_string_open_delimiters: Vec<String>,
        formated_string_close_delimiters: Vec<String>,
        format_opens: Vec<String>,
        format_closes: Vec<String>,
        class_identifier_pattern: String,
    ) -> Lexer {
        Lexer {
            keywords,
            specials,
            illegals,
            declarations,
            method_declaration,
            operators,
            l_parens,
            r_parens,
            comment_initial_characters,
            constants,
            assignments,
            separators,
            string_open_delimiters,
            string_close_delimiters,
            formated_string_open_delimiters,
            formated_string_close_delimiters,
            format_opens,
            format_closes,
            class_identifier_pattern,
        }
    }
    #[wasm_bindgen(js_name = getKeywords)]
    pub fn get_keywords(&self) -> Vec<String> {
        self.keywords.clone()
    }
    #[wasm_bindgen(js_name = getSpecials)]
    pub fn get_specials(&self) -> Vec<String> {
        self.specials.clone()
    }
    #[wasm_bindgen(js_name = getMethodDeclarations)]
    pub fn get_method_declarations(&self) -> Vec<String> {
        self.method_declaration.clone()
    }
    #[wasm_bindgen(js_name = getIllegals)]
    pub fn get_illegals(&self) -> Vec<String> {
        self.illegals.clone()
    }
    #[wasm_bindgen(js_name = getDeclarations)]
    pub fn get_declarations(&self) -> Vec<String> {
        self.declarations.clone()
    }
    #[wasm_bindgen(js_name = getOperators)]
    pub fn get_operators(&self) -> Vec<String> {
        self.operators.clone()
    }
    #[wasm_bindgen(js_name = getSeparators)]
    pub fn get_separators(&self) -> Vec<String> {
        self.separators.clone()
    }
    #[wasm_bindgen(js_name = getLParens)]
    pub fn get_l_parens(&self) -> Vec<String> {
        self.l_parens.clone()
    }
    #[wasm_bindgen(js_name = getRParens)]
    pub fn get_r_parens(&self) -> Vec<String> {
        self.r_parens.clone()
    }
    #[wasm_bindgen(js_name = getFormatOpens)]
    pub fn get_format_opens(&self) -> Vec<String> {
        self.format_opens.clone()
    }
    #[wasm_bindgen(js_name = getFormatCloses)]
    pub fn get_format_closes(&self) -> Vec<String> {
        self.format_closes.clone()
    }
    #[wasm_bindgen(js_name = getStringOpenDelimiters)]
    pub fn get_string_open_delimiters(&self) -> Vec<String> {
        self.string_open_delimiters.clone()
    }
    #[wasm_bindgen(js_name = getFormatedStringOpenDelimiters)]
    pub fn get_formated_string_open_delimiters(&self) -> Vec<String> {
        self.formated_string_open_delimiters.clone()
    }
    #[wasm_bindgen(js_name = getFormatedStringCloseDelimiters)]
    pub fn get_formated_string_close_delimiters(&self) -> Vec<String> {
        self.formated_string_close_delimiters.clone()
    }
    #[wasm_bindgen(js_name = getStringCloseDelimiters)]
    pub fn get_string_close_delimiters(&self) -> Vec<String> {
        self.string_close_delimiters.clone()
    }
    #[wasm_bindgen(js_name = getCommentCharacters)]
    pub fn get_comment_characters(&self) -> Vec<String> {
        self.comment_initial_characters.clone()
    }
    #[wasm_bindgen(js_name = getAssignments)]
    pub fn get_assignments(&self) -> Vec<String> {
        self.assignments.clone()
    }
    #[wasm_bindgen(js_name = getConstants)]
    pub fn get_constants(&self) -> Vec<String> {
        self.constants.clone()
    }
    #[wasm_bindgen(js_name = isKeyword)]
    pub fn is_keyword(&self, token: &str) -> bool {
        self.keywords.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isSpecial)]
    pub fn is_special(&self, token: &str) -> bool {
        self.specials.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isIllegal)]
    pub fn is_illegal(&self, token: &str) -> bool {
        self.illegals.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isFormatedStringOpenDelimiter)]
    pub fn is_formated_string_open_delimiter(&self, token: &str) -> bool {
        self.formated_string_open_delimiters.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isFormatedStringCloseDelimiter)]
    pub fn is_formated_string_close_delimiter(&self, token: &str) -> bool {
        self.formated_string_close_delimiters.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isMethodDeclaration)]
    pub fn is_method_declaration(&self, token: &str) -> bool {
        self.method_declaration.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isStringOpenDelimiter)]
    pub fn is_string_open_delimiter(&self, token: &str) -> bool {
        self.string_open_delimiters.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isStringCloseDelimiter)]
    pub fn is_string_close_delimiter(&self, token: &str) -> bool {
        self.string_close_delimiters.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isCommentCharacter)]
    pub fn is_comment_character(&self, token: &str) -> bool {
        self.comment_initial_characters.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isWhitespace)]
    pub fn is_whitespace(&self, token: &str) -> bool {
        token == " " || token == "\t"
    }
    #[wasm_bindgen(js_name = isDigit)]
    pub fn is_digit(&self, token: &str) -> bool {
        token.chars().all(char::is_numeric)
    }
    #[wasm_bindgen(js_name = isQuote)]
    pub fn is_quote(&self, token: &str) -> bool {
        self.string_open_delimiters.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = containsQuoteInitial)]
    pub fn contains_quote_initial(&self, token: &str) -> bool {
        self.string_open_delimiters.iter().any(|quote| token.starts_with(quote.chars().next().unwrap()))
    }
    #[wasm_bindgen(js_name = isDeclaration)]
    pub fn is_declaration(&self, token: &str) -> bool {
        self.declarations.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isSeparator)]
    pub fn is_separator(&self, token: &str) -> bool {
        self.separators.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isOperator)]
    pub fn is_operator(&self, token: &str) -> bool {
        self.operators.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isLParen)]
    pub fn is_l_paren(&self, token: &str) -> bool {
        self.l_parens.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isRParen)]
    pub fn is_r_paren(&self, token: &str) -> bool {
        self.r_parens.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = isNewline)]
    pub fn is_newline(&self, token: &str) -> bool {
        token == "\n" || token == "\r"
    }
    #[wasm_bindgen(js_name = isConstant)]
    pub fn is_constant(&self, token: &str) -> bool {
        self.constants.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = hasFormatedStringOpenInitial)]
    pub fn has_formated_string_open_initial(&self, token: &str) -> bool {
        self.formated_string_open_delimiters.iter().any(|quote| token.starts_with(quote.chars().next().unwrap()))
    }
    #[wasm_bindgen(js_name = hasFormatedStringCloseInitial)]
    pub fn has_formated_string_close_initial(&self, token: &str) -> bool {
        self.formated_string_close_delimiters.iter().any(|quote| token.starts_with(quote.chars().next().unwrap()))
    }
    #[wasm_bindgen(js_name = getClassIdentifierPattern)]
    pub fn get_class_identifier_pattern(&self) -> String {
        self.class_identifier_pattern.clone()
    }
    #[wasm_bindgen(js_name = containsOperator)]
    pub fn contains_operator(&self, token: &str) -> bool {
        self.operators.iter().any(|operator| token.contains(operator))
    }
    #[wasm_bindgen(js_name = containsAssignment)]
    pub fn contains_assignment(&self, token: &str) -> bool {
        self.assignments.iter().any(|assignment| token.contains(assignment))
    }
    #[wasm_bindgen(js_name = hasFormatOpen)]
    pub fn has_format_open(&self, token: &str) -> bool {
        self.format_opens.iter().any(|quote| token.contains(quote))
    }
    #[wasm_bindgen(js_name = hasFormatClose)]
    pub fn has_format_close(&self, token: &str) -> bool {
        self.format_closes.iter().any(|quote| token.contains(quote))
    }
    #[wasm_bindgen(js_name = removeLastOperator)]
    pub fn remove_last_operator(&self, token: &str) -> Option<String> {
        let mut token = token.to_string();
        for operator in self.operators.iter() {
            if token.ends_with(operator) {
                token.pop();
                return Some(operator.to_string());
            }
        }
        None
    }
    #[wasm_bindgen(js_name = removeLastAssignment)] 
    pub fn remove_last_assignment(&self, token: &str) -> Option<String> {
        let mut token = token.to_string();
        for assignment in self.assignments.iter() {
            if token.ends_with(assignment) {
                token.pop();
                return Some(assignment.to_string());
            }
        }
        None
    }
    #[wasm_bindgen(js_name = removeLastSeparator)]
    pub fn remove_last_separator(&self, token: &str) -> Option<String> {
        let mut token = token.to_string();
        for separator in self.separators.iter() {
            if token.ends_with(separator) {
                token.pop();
                return Some(separator.to_string());
            }
        }
        None
    }
    #[wasm_bindgen(js_name = removeLastQuote)]
    pub fn remove_last_quote(&self, token: &str) -> Option<String> {
        let mut token = token.to_string();
        for quote in self.string_open_delimiters.iter() {
            if token.ends_with(quote) {
                token.pop();
                return Some(quote.to_string());
            }
        }
        None
    }
    #[wasm_bindgen(js_name = hasOperatorInitial)]
    pub fn has_operator_initial(&self, token: &str) -> bool {
        self.operators.iter().any(|operator| token.starts_with(operator.chars().next().unwrap()))
    }
    #[wasm_bindgen(js_name = hasAssignmentInitial)]
    pub fn has_assignment_initial(&self, token: &str) -> bool {
        self.assignments.iter().any(|assignment| token.starts_with(assignment.chars().next().unwrap()))
    }
    #[wasm_bindgen(js_name = isAssignment)]
    pub fn is_assignment(&self, token: &str) -> bool {
        self.assignments.contains(&token.to_string())
    }
    #[wasm_bindgen(js_name = hasSeparatorInitial)]
    pub fn has_separator_initial(&self, token: &str) -> bool {
        self.separators.iter().any(|separator| token.starts_with(separator.chars().next().unwrap()))
    }
    #[wasm_bindgen(js_name = containsQuote)]
    pub fn contains_quote(&self, token: &str) -> bool {
        self.string_open_delimiters.iter().any(|quote| token.contains(quote))
    }
    #[wasm_bindgen(js_name = containsNonAlphabeticalOperator)]
    pub fn contains_non_alphabetical_operator(&self, token: &str) -> bool {
        self.operators.iter().any(|operator| token.contains(operator) && !operator.chars().all(char::is_alphabetic))
    }
    #[wasm_bindgen(js_name = endsWithStringCloseDelimiter)]
    pub fn ends_with_string_close_delimiter(&self, token: &str) -> bool {
        self.string_close_delimiters.iter().any(|delimiter| token.ends_with(delimiter))
    }
    #[wasm_bindgen(js_name = startsWithStringOpenDelimiter)]
    pub fn starts_with_string_open_delimiter(&self, token: &str) -> bool {
        self.string_open_delimiters.iter().any(|delimiter| token.starts_with(delimiter))
    }
    #[wasm_bindgen(js_name = hasFormatStringClose)]
    pub fn has_formated_string_close(&self, token: &str) -> bool {
        self.formated_string_close_delimiters.iter().any(|delimiter| token.ends_with(delimiter))
    }
    #[wasm_bindgen(js_name = clone)]
    pub fn clone_js(&self) -> Lexer {
        self.clone()
    }
    #[wasm_bindgen(js_name = removeFormatOpen)]
    pub fn remove_format_open(&self, token: &str) -> Option<String> {
        let token = token.to_string();
        for format_open in self.format_opens.iter() {
            if token.ends_with(format_open) {
                return Some(format_open.to_string());
            }
        }
        None
    }
    #[wasm_bindgen(js_name = startsWithCommentCharacter)]
    pub fn starts_with_comment_character(&self, token: &str) -> bool {
        self.comment_initial_characters.iter().any(|comment| token.starts_with(comment))
    }
    #[wasm_bindgen(js_name = hasCommentInitial)]
    pub fn has_comment_initial(&self, token: &str) -> bool {
        self.comment_initial_characters.iter().any(|comment| token.starts_with(comment.chars().next().unwrap()))
    }
    #[wasm_bindgen(js_name = removeFormatClose)]
    pub fn remove_format_close(&self, token: &str) -> Option<String> {
        let token = token.to_string();
        for format_close in self.format_closes.iter() {
            if token.ends_with(format_close) {
                return Some(format_close.to_string());
            }
        }
        None
    }
    #[wasm_bindgen(js_name = hasFormatStringOpen)]
    pub fn has_formated_string_open(&self, token: &str) -> bool {
        self.formated_string_open_delimiters.iter().any(|delimiter| token.starts_with(delimiter))
    }
    #[wasm_bindgen(js_name = getTokens)]
    pub fn get_tokens(&self, input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut current = 0;
        let input = input.chars().collect::<Vec<char>>();
        while current < input.len() {
            let c = input[current];
            if self.is_whitespace(&c.to_string()) {
                tokens.push(Token::new(TokenType::Whitespace, c.to_string()));
                current += 1;
                log(&format!("Whitespace: {}", c.to_string()));
                continue;
            }
            if self.is_illegal(&c.to_string()) {
                tokens.push(Token::new(TokenType::Illegal, c.to_string()));
                current += 1;
                log(&format!("Illegal: {}", c.to_string()));
                continue;
            }
            if self.contains_quote_initial(&c.to_string()) {
                let initial_current = current;
                let mut literal = c.to_string();
                current += 1;
                while current < input.len() {
                    let c = input[current];
                    literal.push(c);
                    current += 1;
                    if self.ends_with_string_close_delimiter(&literal) {
                        log(&format!("String: {}", literal));
                        tokens.push(Token::new(TokenType::String, literal.clone()));
                        break;
                    }
                }
                if !self.ends_with_string_close_delimiter(&literal) || !self.starts_with_string_open_delimiter(&literal) {
                    current = initial_current;
                } else {
                    continue;
                }
            }
            if self.has_formated_string_open_initial(&c.to_string()) {
                log(&format!("Formated String Open: {}", c.to_string()));
                let initial_current = current;
                let mut finished = false;
                let mut has_format_open = false;
                let mut literal = c.to_string();
                current += 1;
                while current < input.len() {
                    let c = input[current];
                    literal.push(c);
                    current += 1;
                    if !self.has_formated_string_open(&literal) && (c.is_alphabetic() || c.is_numeric() || c == '_' || c.is_whitespace() || self.is_l_paren(&c.to_string()) || self.is_r_paren(&c.to_string()) || self.is_newline(&c.to_string())) {
                        current = initial_current;
                        break;
                    }
                    if self.has_formated_string_close(&literal) && (c.is_alphabetic() || c.is_numeric() || c == '_' || c.is_whitespace() || self.is_l_paren(&c.to_string()) || self.is_r_paren(&c.to_string()) || self.is_newline(&c.to_string())) {
                        log(&format!("Formated String Finish: {}", literal));
                        finished = true;
                        break;
                    }
                    if self.has_format_open(&literal) {
                        log(&format!("Formated String: {}", literal));
                        let format_open = self.remove_format_open(&literal).unwrap();
                        literal = literal[..literal.len() - format_open.len()].to_string();
                        tokens.push(Token::new(TokenType::FormattedString, literal.clone()));
                        tokens.push(Token::new(TokenType::FormatOpen, format_open.clone()));
                        has_format_open = true;
                        break;
                    }
                }
                if finished && self.has_formated_string_open(&literal) && self.has_formated_string_close(&literal) {
                    log(&format!("Formated String: {}", literal));
                    tokens.push(Token::new(TokenType::FormattedString, literal));
                    current += 1;
                    continue;
                }
                if has_format_open {
                    let mut finished = false;
                    while !finished {
                        let mut literal = String::new();
                        while current < input.len() {
                            log(&format!("Current: {}", current));
                            let c = input[current];
                            literal.push(c);
                            current += 1;
                            if self.has_format_close(&literal) {
                                log(&format!("Formated String 2: {}", literal));
                                let format_close = self.remove_format_close(&literal).unwrap();
                                literal = literal[..literal.len() - format_close.len()].to_string();
                                tokens.extend(self.get_tokens(&literal));
                                tokens.push(Token::new(TokenType::FormatClose, format_close.clone()));
                                break;
                            }
                        }
                        literal = String::new();
                        while current < input.len() {
                            let c = input[current];
                            log(&format!("Current: {}", c));
                            literal.push(c);
                            if self.has_format_open(&literal) {
                                log(&format!("Formated String: {}", literal));
                                let format_open = self.remove_format_open(&literal).unwrap();
                                literal = literal[..literal.len() - format_open.len()].to_string();
                                tokens.push(Token::new(TokenType::FormattedString, literal.clone()));
                                tokens.push(Token::new(TokenType::FormatOpen, format_open.clone()));
                                break;
                            }
                            if self.has_formated_string_close(&literal) && ((current + 1 < input.len() && (self.is_whitespace(&input[current + 1].to_string()) || self.is_l_paren(&input[current + 1].to_string()) || self.is_r_paren(&input[current + 1].to_string()) || self.is_separator(&input[current + 1].to_string()) || self.is_newline(&input[current + 1].to_string()))) || (current + 1 == input.len())) {
                                log(&format!("Formated String Finished: {}", literal));
                                tokens.push(Token::new(TokenType::FormattedString, literal));
                                current += 1;
                                finished = true;
                                break;
                            }
                            current += 1;
                        }
                    }
                    continue;
                }
            }
            if self.is_digit(&c.to_string()) {
                let mut literal = String::new();
                while current < input.len() {
                    let c = input[current];
                    if !self.is_digit(&c.to_string()) && c != '.' {
                        break;
                    }
                    literal.push(c);
                    current += 1;
                }
                log(&format!("Number: {}", literal));
                tokens.push(Token::new(TokenType::Number, literal));
                continue;
            }
            if self.is_newline(&c.to_string()) {
                tokens.push(Token::new(TokenType::Newline, c.to_string()));
                current += 1;
                log(&format!("Newline: {}", c.to_string()));
                continue;
            }
            if self.is_l_paren(&c.to_string()) {
                tokens.push(Token::new(TokenType::LParen, c.to_string()));
                current += 1;
                log(&format!("LParen: {}", c.to_string()));
                continue;
            }
            if self.is_r_paren(&c.to_string()) {
                tokens.push(Token::new(TokenType::RParen, c.to_string()));
                current += 1;
                log(&format!("RParen: {}", c.to_string()));
                continue;
            }
            if self.has_operator_initial(&c.to_string()) {
                let mut literal = String::new();
                let initial_current = current;
                while current < input.len() {
                    let c = input[current];
                    if self.is_illegal(&c.to_string())
                    || self.is_whitespace(&c.to_string())
                    || self.is_newline(&c.to_string())
                    || self.is_l_paren(&c.to_string())
                    || self.is_r_paren(&c.to_string())
                    || self.is_separator(&c.to_string())
                    || self.contains_assignment(&literal)
                    || c.is_alphabetic()
                    || c.is_numeric()
                    {
                        if self.contains_assignment(&literal) {
                            let assignment = self.remove_last_assignment(&literal).unwrap();
                            current -= assignment.len();
                        } else if self.contains_quote(&literal) {
                            let quote = self.remove_last_quote(&literal).unwrap();
                            current -= quote.len();
                        }
                        break;
                    }
                    literal.push(c);
                    current += 1;
                }
                if self.is_operator(&literal) {
                    log(&format!("Operator: {}", literal));
                    tokens.push(Token::new(TokenType::Operator, literal));
                    continue;
                } else {
                    current = initial_current;
                }
            }
            if self.has_assignment_initial(&c.to_string())  {
                let mut literal = String::new();
                let initial_current = current;
                while current < input.len() {
                    let c = input[current];
                    if self.is_illegal(&c.to_string())
                    || self.is_whitespace(&c.to_string())
                    || self.is_newline(&c.to_string())
                    || self.is_l_paren(&c.to_string())
                    || self.is_r_paren(&c.to_string())
                    || self.contains_quote(&literal)
                    || c.is_alphabetic()
                    || c.is_numeric()
                    {
                        if self.contains_quote(&literal) {
                            let quote = self.remove_last_quote(&literal).unwrap();
                            current -= quote.len();
                        }
                        break;
                    }
                    literal.push(c);
                    current += 1;
                }
                if self.is_assignment(&literal) {
                    log(&format!("Assign: {}", literal));
                    tokens.push(Token::new(TokenType::Assign, literal));
                    continue;
                } else {
                    current = initial_current;
                }
            }
            if self.has_comment_initial(&c.to_string()) {
                let mut literal = String::new();
                let initial_current = current;
                while current < input.len() {
                    let c = input[current];
                    if self.is_newline(&c.to_string()) {
                        break;
                    }
                    literal.push(c);
                    current += 1;
                }
                if !self.starts_with_comment_character(&literal) {
                    current = initial_current;
                }
                else {
                    tokens.push(Token::new(TokenType::Comment, literal));
                    continue;
                }
            }
            if self.has_separator_initial(&c.to_string()) {
                let mut literal = String::new();
                let initial_current = current;
                while current < input.len() {
                    let c = input[current];
                    if self.is_illegal(&c.to_string())
                        || self.is_whitespace(&c.to_string())
                        || self.is_newline(&c.to_string())
                        || self.is_l_paren(&c.to_string())
                        || self.is_r_paren(&c.to_string())
                        || self.contains_operator(&literal)
                        || self.contains_assignment(&literal)
                        || self.contains_quote(&literal)
                        || c.is_alphabetic()
                        || c.is_numeric()
                    {
                        if self.contains_operator(&literal) {
                            let operator = self.remove_last_operator(&literal).unwrap();
                            current -= operator.len();
                        } else if self.contains_assignment(&literal) {
                            let assignment = self.remove_last_assignment(&literal).unwrap();
                            current -= assignment.len();
                        } else if self.contains_quote(&literal) {
                            let quote = self.remove_last_quote(&literal).unwrap();
                            current -= quote.len();
                        }
                        break;
                    }
                    literal.push(c);
                    current += 1;
                }
                if self.is_separator(&literal) {
                    log(&format!("Separator: {}", literal));
                    tokens.push(Token::new(TokenType::Separator, literal));
                    continue;
                } else {
                    current = initial_current;
                }
            }
            let mut literal = String::new();
            while current < input.len() {
                let c = input[current];
                if self.is_illegal(&c.to_string())
                    || self.is_whitespace(&c.to_string())
                    || self.is_newline(&c.to_string())
                    || self.is_l_paren(&c.to_string())
                    || self.is_r_paren(&c.to_string())
                    || self.is_separator(&c.to_string())
                    || self.contains_non_alphabetical_operator(&literal)
                    || self.contains_assignment(&literal)
                    || self.contains_quote(&literal)
                {
                    if self.contains_operator(&literal) {
                        self.remove_last_operator(&literal);
                    } else if self.contains_assignment(&literal) {
                        self.remove_last_assignment(&literal);
                    } else if self.contains_quote(&literal) {
                        self.remove_last_quote(&literal);
                    }
                    break;
                }
                literal.push(c);
                current += 1;
            }
            if self.is_keyword(&literal) {
                log(&format!("Keyword: {}", literal));
                tokens.push(Token::new(TokenType::Keyword, literal));
                continue;
            }
            if self.is_special(&literal) {
                log(&format!("Special: {}", literal));
                tokens.push(Token::new(TokenType::Special, literal));
                continue;
            }
            if self.is_declaration(&literal) {
                log(&format!("Declaration: {}", literal));
                tokens.push(Token::new(TokenType::Declaration, literal));
                continue;
            }
            if self.is_constant(&literal) {
                log(&format!("Constant: {}", literal));
                tokens.push(Token::new(TokenType::Constant, literal));
                continue;
            }
            if self.is_method_declaration(&literal) && current < input.len() && self.is_whitespace(&input[current].to_string()) {
                log(&format!("Method Declaration: {}", literal));
                tokens.push(Token::new(TokenType::MethodDeclaration, literal));
                let mut literal = String::new();
                let space = input[current];
                tokens.push(Token::new(TokenType::Whitespace, space.to_string()));
                current += 1;
                while current < input.len() {
                    let c = input[current];
                    if self.is_illegal(&c.to_string())
                        || self.is_whitespace(&c.to_string())
                        || self.is_newline(&c.to_string())
                        || self.is_l_paren(&c.to_string())
                        || self.is_r_paren(&c.to_string())
                        || self.is_separator(&c.to_string())
                        || self.contains_non_alphabetical_operator(&literal)
                        || self.contains_assignment(&literal)
                        || self.contains_quote(&literal)
                    {
                        if self.contains_operator(&literal) {
                            self.remove_last_operator(&literal);
                        } else if self.contains_assignment(&literal) {
                            self.remove_last_assignment(&literal);
                        } else if self.contains_quote(&literal) {
                            self.remove_last_quote(&literal);
                        }
                        break;
                    }
                    literal.push(c);
                    current += 1;
                }
                if Regex::new(&self.class_identifier_pattern).unwrap().is_match(&literal) {
                    log(&format!("Class Identifier: {}", literal));
                    tokens.push(Token::new(TokenType::ClassIdentifier, literal));
                    continue;
                } else {
                    log(&format!("Identifier: {}", literal));
                    tokens.push(Token::new(TokenType::MethodIdentifier, literal));
                    continue;
                }
            }
            if Regex::new(&self.class_identifier_pattern).unwrap().is_match(&literal) {
                log(&format!("Class Identifier: {}", literal));
                tokens.push(Token::new(TokenType::ClassIdentifier, literal));
                continue;
            }
            log(&format!("Identifier: {}", literal));
            tokens.push(Token::new(TokenType::Identifier, literal));
        }
        tokens
    }
}