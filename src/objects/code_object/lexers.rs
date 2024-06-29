use wasm_bindgen::prelude::*;
use super::lexer::Lexer;

#[wasm_bindgen(js_name = getPythonLexer)]
pub fn get_python_lexer() -> Lexer {
    return Lexer::new(
        vec![
            "as".to_string(),
            "assert".to_string(),
            "break".to_string(),
            "continue".to_string(),
            "del".to_string(),
            "elif".to_string(),
            "else".to_string(),
            "except".to_string(),
            "finally".to_string(),
            "for".to_string(),
            "from".to_string(),
            "global".to_string(),
            "if".to_string(),
            "import".to_string(),
            "lambda".to_string(),
            "nonlocal".to_string(),
            "pass".to_string(),
            "raise".to_string(),
            "return".to_string(),
            "try".to_string(),
            "while".to_string(),
            "with".to_string(),
            "yield".to_string(),
        ],
        vec![
            "print".to_string(),
            "input".to_string(),
            "int".to_string(),
            "float".to_string(),
            "str".to_string(),
            "bool".to_string(),
            "list".to_string(),
            "tuple".to_string(),
            "dict".to_string(),
            "set".to_string(),
            "frozenset".to_string(),
            "complex".to_string(),
            "range".to_string(),
            "enumerate".to_string(),
            "zip".to_string(),
            "reversed".to_string(),
            "sorted".to_string(),
            "sum".to_string(),
            "min".to_string(),
            "max".to_string(),
            "abs".to_string(),
            "round".to_string(),
            "len".to_string(),
            "type".to_string(),
            "isinstance".to_string(),
            "issubclass".to_string(),
            "callable".to_string(),
            "hasattr".to_string(),
            "getattr".to_string(),
            "setattr".to_string(),
            "delattr".to_string(),
            "vars".to_string(),
            "dir".to_string(),
            "locals".to_string(),
            "globals".to_string(),
            "id".to_string(),
            "hash".to_string(),
            "next".to_string(),
            "iter".to_string(),
            "open".to_string(),
            "super".to_string(),
            "exec".to_string(),
            "eval".to_string(),
            "compile".to_string(),
            "execfile".to_string(),
            "exit".to_string(),
            "quit".to_string(),
            "help".to_string(),
            "dir".to_string(),
            "vars".to_string(),
            "locals".to_string(),
            "globals".to_string(),
            "filter".to_string(),
            "map".to_string(),
            "reduce".to_string(),
            "any".to_string(),
            "all".to_string(),
            "ascii".to_string(),
            "bin".to_string(),
        ],
        vec![],
        vec![],
        vec!["def".to_string(), "class".to_string()],
        vec![
            "+".to_string(),
            "-".to_string(),
            "*".to_string(),
            "/".to_string(),
            "//".to_string(),
            "**".to_string(),
            "%".to_string(),
            "^".to_string(),
            "&".to_string(),
            "|".to_string(),
            "==".to_string(),
            "!=".to_string(),
            "<".to_string(),
            "<=".to_string(),
            ">".to_string(),
            ">=".to_string(),
            "and".to_string(),
            "or".to_string(),
            "in".to_string(),
            "is".to_string(),
            "not".to_string(),
            "in".to_string()
        ],
        vec!["(".to_string(), "[".to_string(), "{".to_string()],
        vec![")".to_string(), "]".to_string(), "}".to_string()],
        vec!["#".to_string()],
        vec!["True".to_string(), "False".to_string(), "None".to_string()],
        vec![
            "=".to_string(),
            "+=".to_string(),
            "-=".to_string(),
            "*=".to_string(),
            "/=".to_string(),
            "%=".to_string(),
            "//=".to_string(),
            "**=".to_string(),
            "&=".to_string(),
            "|=".to_string(),
            "^=".to_string(),
            ":=".to_string(),
        ],
        vec![
            ":".to_string(),
            ",".to_string(),
            ".".to_string(),
            ";".to_string(), // This is not correct, but we need to handle wrong semicolons
            "...".to_string(),
        ],
        vec![
            "\"".to_string(),
            "'".to_string(),
            "\"\"\"".to_string(),
            "'''".to_string(),
        ],
        vec![
            "\"".to_string(),
            "'".to_string(),
            "\"\"\"".to_string(),
            "'''".to_string(),
        ],
        vec![
            "f\"".to_string(),
            "f'".to_string(),
            "f\"\"\"".to_string(),
            "f'''".to_string()
        ],
        vec![
            "\"".to_string(),
            "'".to_string(),
            "\"\"\"".to_string(),
            "'''".to_string()
        ],
        vec!["{".to_string()],
        vec!["}".to_string()],
        r"^[A-Z][a-zA-Z0-9_]*$".to_string()
    )
}
