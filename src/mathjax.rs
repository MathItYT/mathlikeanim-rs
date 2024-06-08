// Based on https://github.com/Nigecat/mathjax-rs/blob/master/src/renderer/browser.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{eval, Promise};

use crate::objects::wasm_interface::{svg_to_vector_js, WasmVectorObject};


#[wasm_bindgen]
pub async fn mathjax(expression: String) -> WasmVectorObject {
    let function = eval(
        &r#"function(expression) {
    return new Promise((resolve, reject) => {
        window.MathJax = {
            startup: {
                ready: () => {
                    MathJax.startup.defaultReady();
                    const svg = window.MathJax.tex2svg(expression).children[0].outerHTML;
                    resolve(svg);
                }
            },
        };
        const script = document.createElement('script');
        script.src = 'https://cdnjs.cloudflare.com/ajax/libs/mathjax/3.2.2/es5/tex-svg-full.js';
        script.type = 'text/javascript';
        script.crossOrigin = 'anonymous';
        script.onerror = reject;   
        document.head.appendChild(script);
    });
}"#
    ).unwrap().dyn_into::<js_sys::Function>().unwrap();
    let promise = function.call1(&JsValue::NULL, &JsValue::from(expression)).unwrap().dyn_into::<Promise>().unwrap();
    let result = JsFuture::from(promise).await.unwrap();
    let svg = result.as_string().unwrap();
    svg_to_vector_js(svg)
}