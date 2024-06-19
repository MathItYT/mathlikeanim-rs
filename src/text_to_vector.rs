use js_sys::{eval, Function, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::objects::{svg_to_vector::svg_to_vector, wasm_interface::WasmVectorObject};


#[wasm_bindgen(js_name = textToVector)]
pub async fn text_to_vector(text: String, font_family: String) -> WasmVectorObject {
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
                    const path = font.getPath(text, 0, 0, 144);
                    const svg = path.toSVG();
                    resolve(svg);
                });
            };
            document.head.appendChild(script);
        } else {
            opentype.load(fontFamily).then((font) => {
                const path = font.getPath(text, 0, 0, 144);
                const svg = path.toSVG();
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