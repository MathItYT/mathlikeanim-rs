use js_sys::{eval, Function, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::objects::{svg_to_vector::svg_to_vector, wasm_interface::WasmVectorObject};


#[cfg(feature = "browser")]
#[wasm_bindgen(js_name = textToVector)]
pub async fn text_to_vector_browser(text: String, font_family: String) -> WasmVectorObject {
    let function = eval(
        &r#"(text, fontFamily) => {
    return new Promise((resolve, reject) => {
        if (!document.getElementById('opentype-script')) {
            const script = document.createElement('script');
            script.setAttribute('src', 'https://cdn.jsdelivr.net/npm/opentype.js@1.3.4/dist/opentype.min.js');
            script.setAttribute('type', 'text/javascript');
            script.setAttribute('async', true);
            script.setAttribute('crossorigin', 'anonymous');
            script.setAttribute('id', 'opentype-script');
            script.onerror = () => {
                reject('Failed to load opentype.js');
            };
            script.onload = () => {
                opentype.load(fontFamily).then((font) => {
                    const paths = font.getPaths(text, 0, 0, 144);
                    const svgs = paths.map((path) => path.toSVG());
                    const svg = svgs.join('\n');
                    resolve(svg);
                });
            };
            document.head.appendChild(script);
        } else {
            opentype.load(fontFamily).then((font) => {
                const paths = font.getPaths(text, 0, 0, 144);
                const svgs = paths.map((path) => path.toSVG());
                const svg = svgs.join('\n');
                resolve(svg);
            });
        }
    });
}"#,
    ).unwrap().dyn_into::<Function>().unwrap();
    let promise = function.call2(
        &JsValue::NULL,
        &JsValue::from(text),
        &JsValue::from(font_family),
    ).unwrap().dyn_into::<Promise>().unwrap();
    let result = JsFuture::from(promise).await.unwrap();
    return WasmVectorObject {
        native_vec_features: svg_to_vector(&result.as_string().unwrap())
    };
}


#[cfg(feature = "node")]
#[wasm_bindgen(js_name = textToVector)]
pub async fn text_to_vector_node(text: String, font_family: String) -> WasmVectorObject {
    let function = eval(
        &r#"(text, fontFamily) => {
    return new Promise((resolve, reject) => {
        const opentype = require('opentype.js');
        opentype.load(fontFamily).then((font) => {
            const paths = font.getPaths(text, 0, 0, 144);
            const svgs = paths.map((path) => path.toSVG());
            const svg = svgs.join('\n');
            resolve(svg);
        });
    });
}"#,
    ).unwrap().dyn_into::<Function>().unwrap();
    let promise = function.call2(
        &JsValue::NULL,
        &JsValue::from(text),
        &JsValue::from(font_family),
    ).unwrap().dyn_into::<Promise>().unwrap();
    let result = JsFuture::from(promise).await.unwrap();
    return WasmVectorObject {
        native_vec_features: svg_to_vector(&result.as_string().unwrap())
    };
}
