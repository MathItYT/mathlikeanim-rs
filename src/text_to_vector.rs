use js_sys::{eval, Function, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::objects::wasm_interface::WasmVectorObject;


#[cfg(feature = "browser")]
#[wasm_bindgen(js_name = textToVector)]
pub async fn text_to_vector_browser(text: String, font_family: String, x: f64, y: f64, font_size: f64) -> WasmVectorObject {
    use js_sys::Map;

    use crate::objects::svg_to_vector::svg_to_vector_pin;

    let function = eval(
        &r#"(options) => {
    const text = options.get('text');
    const fontFamily = options.get('fontFamily');
    const x = options.get('x');
    const y = options.get('y');
    const fontSize = options.get('fontSize');
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
                    const paths = font.getPaths(text, x, y, fontSize);
                    const svgs = paths.map((path) => path.toSVG());
                    const svg = svgs.join('\n');
                    resolve(svg);
                });
            };
            document.head.appendChild(script);
        } else {
            opentype.load(fontFamily).then((font) => {
                const paths = font.getPaths(text, x, y, fontSize);
                const svgs = paths.map((path) => path.toSVG());
                const svg = svgs.join('\n');
                resolve(svg);
            });
        }
    });
}"#,
    ).unwrap().dyn_into::<Function>().unwrap();
    let options = Map::new();
    options.set(&"text".into(), &JsValue::from(text));
    options.set(&"fontFamily".into(), &JsValue::from(font_family.clone()));
    options.set(&"x".into(), &JsValue::from(x));
    options.set(&"y".into(), &JsValue::from(y));
    options.set(&"fontSize".into(), &JsValue::from(font_size));
    let promise = function.call1(
        &JsValue::NULL,
        &JsValue::from(options)
    ).unwrap().dyn_into::<Promise>().unwrap();
    let result = JsFuture::from(promise).await.unwrap();
    return WasmVectorObject {
        native_vec_features: svg_to_vector_pin(&result.as_string().unwrap(), Some(font_family), Some(font_size)).await
    };
}


#[cfg(feature = "node")]
#[wasm_bindgen(js_name = textToVector)]
pub async fn text_to_vector_node(text: String, font_family: String, x: f64, y: f64, font_size: f64) -> WasmVectorObject {
    use js_sys::Map;

    use crate::objects::svg_to_vector::svg_to_vector_pin;

    let function = eval(
        &r#"(options) => {
    const text = options.get('text');
    const fontFamily = options.get('fontFamily');
    const x = options.get('x');
    const y = options.get('y');
    const fontSize = options.get('fontSize');
    return new Promise((resolve, reject) => {
        const opentype = require('opentype.js');
        opentype.load(fontFamily).then((font) => {
            const paths = font.getPaths(text, x, y, fontSize);
            const svgs = paths.map((path) => path.toSVG());
            const svg = svgs.join('\n');
            resolve(svg);
        });
    });
}"#,
    ).unwrap().dyn_into::<Function>().unwrap();
    let options = Map::new();
    options.set(&"text".into(), &JsValue::from(text));
    options.set(&"fontFamily".into(), &JsValue::from(font_family.clone()));
    options.set(&"x".into(), &JsValue::from(x));
    options.set(&"y".into(), &JsValue::from(y));
    options.set(&"fontSize".into(), &JsValue::from(font_size));
    let promise = function.call1(
        &JsValue::NULL,
        &JsValue::from(options)
    ).unwrap().dyn_into::<Promise>().unwrap();
    let result = JsFuture::from(promise).await.unwrap();
    return WasmVectorObject {
        native_vec_features: svg_to_vector_pin(&result.as_string().unwrap(), Some(font_family), Some(font_size)).await
    };
}
