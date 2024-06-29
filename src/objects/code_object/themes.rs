use super::theme::Theme;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = getGithubDark)]
pub fn get_github_dark() -> Theme {
    Theme::new(
        "#fa7970".to_string(),
        "#cea5fb".to_string(),
        "#ff0000".to_string(),
        "#fa7970".to_string(),
        "#fa7970".to_string(),
        "#ecf2f8".to_string(),
        "#fa7970".to_string(),
        "#77bdfb".to_string(),
        "#a2d2fb".to_string(),
        "#ecf2f8".to_string(),
        "#fa7970".to_string(),
        "#ecf2f8".to_string(),
        "#cea5fb".to_string(),
        "#fa7970".to_string(),
        "#a2d2fb".to_string(),
        "#ecf2f8".to_string(),
        "#ecf2f8".to_string(),
        "#89929b".to_string(),
        "#faa356".to_string()
    )
}