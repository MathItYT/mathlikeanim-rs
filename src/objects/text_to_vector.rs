use js_sys::{eval, Function, Map, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use crate::objects::svg_to_vector::svg_to_vector_pin;

use crate::objects::wasm_interface::WasmVectorObject;


#[wasm_bindgen(js_name = textToVector)]
pub async fn text_to_vector(text: String, font_array_buffers: Map, font_family: Option<String>, font_weight: Option<String>, font_style: Option<String>, x: Option<f64>, y: Option<f64>, font_size: Option<f64>) -> WasmVectorObject {
    let function = eval(
        &r#"(options) => {
    const text = options.get('text');
    let fontFamily = options.get('fontFamily') || 'Times New Roman';
    if (fontFamily === 'serif') {
        fontFamily = 'Times New Roman';
    }
    if (fontFamily === 'sans-serif') {
        fontFamily = 'Arial';
    }
    if (fontFamily === 'monospace') {
        fontFamily = 'Courier New';
    }
    const x = options.get('x') || 0;
    const y = options.get('y') || 0;
    let fontWeight = options.get('fontWeight') || '400';
    if (fontWeight === 'normal') {
        fontWeight = '400';
    }
    if (fontWeight === 'bold') {
        fontWeight = '700';
    }
    const fontStyle = options.get('fontStyle') || 'normal';
    const fontSize = options.get('fontSize') || 16;
    const fontUrlsMap = options.get('fontUrlsMap');
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
            script.onload = async () => {
                const fontBuffer = fontUrlsMap.get(`font-family:'${fontFamily}';font-weight:${fontWeight};font-style:${fontStyle}`);
                if (!fontBuffer) {
                    reject('Font not found in fontUrlsMap');
                    return;
                }
                const font = opentype.parse(fontBuffer);
                const path = font.getPath(text, x, y, fontSize);
                resolve(path.toSVG());
            };
            document.head.appendChild(script);
        } else {
            const fontBuffer = fontUrlsMap.get(`font-family:'${fontFamily}';font-weight:${fontWeight};font-style:${fontStyle}`);
            if (!fontBuffer) {
                reject('Font not found in fontUrlsMap');
                return;
            }
            const font = opentype.parse(fontBuffer);
            const path = font.getPath(text, x, y, fontSize);
            resolve(path.toSVG());
        }
    });
}"#,
    ).unwrap().dyn_into::<Function>().unwrap();
    let options = Map::new();
    options.set(&"text".into(), &JsValue::from(text));
    options.set(&"fontFamily".into(), &JsValue::from(font_family));
    options.set(&"fontStyle".into(), &JsValue::from(font_style.clone()));
    options.set(&"fontWeight".into(), &JsValue::from(font_weight.clone()));
    options.set(&"x".into(), &JsValue::from(x));
    options.set(&"y".into(), &JsValue::from(y));
    options.set(&"fontSize".into(), &JsValue::from(font_size));
    options.set(&"fontUrlsMap".into(), &JsValue::from(font_array_buffers.clone()));
    let promise = function.call1(
        &JsValue::NULL,
        &JsValue::from(options)
    ).unwrap().dyn_into::<Promise>().unwrap();
    let result = JsFuture::from(promise).await.unwrap();
    return WasmVectorObject {
        native_vec_features: svg_to_vector_pin(&result.as_string().unwrap(), Some(font_array_buffers)).await
    };
}
