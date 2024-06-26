// Based on https://github.com/Nigecat/mathjax-rs/blob/master/src/renderer/browser.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{eval, Promise};

use crate::objects::wasm_interface::{svg_to_vector_js, WasmVectorObject};


#[cfg(feature = "browser")]
#[wasm_bindgen(js_name = mathjax)]
pub async fn mathjax_web(expression: String, default_font_family: Option<String>, default_font_size: Option<f64>) -> WasmVectorObject {
    let function = eval(
        &r#"(expression) => {
    return new Promise((resolve, reject) => {
        if (document.getElementById('mathjax-script')) {
            const svg = window.MathJax.tex2svg(expression).children[0].outerHTML;
            resolve(svg);
            return;
        }
        window.MathJax = {
            startup: {
                ready: () => {
                    MathJax.startup.defaultReady();
                    const svg = window.MathJax.tex2svg(expression).children[0].outerHTML;
                    resolve(svg);
                }
            }
        };
        const script = document.createElement('script');
        script.setAttribute('src', 'https://cdnjs.cloudflare.com/ajax/libs/mathjax/3.2.2/es5/tex-svg-full.js');
        script.setAttribute('crossorigin', 'anonymous');
        script.setAttribute('type', 'text/javascript');
        script.setAttribute('async', true);
        script.setAttribute('id', 'mathjax-script');
        script.addEventListener('error', reject);
        document.head.appendChild(script);
    });
}"#,
    ).unwrap().dyn_into::<js_sys::Function>().unwrap();
    let promise = function.call1(
        &JsValue::NULL,
        &JsValue::from(expression)
    ).unwrap().dyn_into::<Promise>().unwrap();
    let result = JsFuture::from(promise).await.unwrap();
    let svg = result.as_string().unwrap();
    svg_to_vector_js(svg, default_font_family, default_font_size).await
}


#[cfg(feature = "node")]
#[wasm_bindgen(js_name = mathjax)]
pub async fn mathjax_node(expression: String, default_font_family: Option<String>, default_font_size: Option<f64>) -> WasmVectorObject {
    let func = eval(
        &r#"if (typeof MathJax === 'undefined') {
  MathJax = {
    loader: {
      paths: {mathjax: 'mathjax-full/es5'},
      require: require,
      load: ['adaptors/liteDOM']
    },
    tex: {
      packages: ['base', 'autoload', 'require', 'ams', 'newcommand'],
    },
    svg: {
      fontCache: 'local'
    },
    startup: {
      typeset: false
    }
  };

  require('mathjax-full/es5/tex-svg.js');
}

const texConfig = {
  display: true,
  em: 32,
  ex: 16,
  containerWidth: 80 * 16,
};

async (tex) => {
  if (!MathJax.startup.adaptor) {
    await MathJax.startup.promise;
  }
  const svg = await MathJax.tex2svgPromise(tex, texConfig).then(node =>
    MathJax.startup.adaptor.outerHTML(node.children[0])
  );
  return svg;
}"#,
    ).unwrap().dyn_into::<js_sys::Function>().unwrap();
    let promise = func.call1(
        &JsValue::NULL,
        &JsValue::from(expression)
    ).unwrap().dyn_into::<Promise>().unwrap();
    let result = JsFuture::from(promise).await.unwrap();
    let svg = result.as_string().unwrap();
    svg_to_vector_js(svg, default_font_family, default_font_size).await
}