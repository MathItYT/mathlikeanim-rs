use wasm_bindgen::prelude::*;
use once_cell::sync::Lazy;
use mathlikeanim_rs::{objects::{geometry::{arc::circle, poly::{rectangle, square}}, plotting::axes::{axes, plot_in_axes}, svg_to_vector::svg_to_vector, vector_object::{VectorFeatures, VectorObject}}, scene::Scene, utils::hex_to_color};


static mut SCENE: Lazy<Scene> = Lazy::new(|| Scene::new(1920, 1080, 60, ""));


#[wasm_bindgen(module = "tex2svg")]
extern "C" {
    #[wasm_bindgen(js_name = tex2svg)]
    pub async fn tex2svg(latex: JsValue) -> JsValue;
}


#[wasm_bindgen(module = "mathlike2svg")]
extern "C" {
    #[wasm_bindgen(js_name = mathlike2svg)]
    pub async fn mathlike2svg() -> JsValue;
}


pub async fn tex_to_vector(latex: String) -> VectorFeatures {
    let tex = JsValue::from_str(latex.as_str());
    let svg = tex2svg(tex).await;
    return svg_to_vector(svg.as_string().unwrap().as_str())
        .set_stroke_color((1.0, 1.0, 1.0, 1.0), true);
}


pub async fn mathlike_to_vector() -> VectorFeatures {
    let svg = mathlike2svg().await;
    return svg_to_vector(svg.as_string().unwrap().as_str());
}


#[wasm_bindgen(start)]
pub async fn single_image_example() {
    let sn = unsafe { &mut SCENE };
    let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    sn.init_context(context);
    sn.set_background_color((34.0 / 255.0, 34.0 / 255.0, 34.0 / 255.0, 1.0));
    let mut mathlike_logo = mathlike_to_vector().await;
    mathlike_logo = mathlike_logo.scale(300.0 / mathlike_logo.get_width(), true);
    mathlike_logo.subobjects[0] = mathlike_logo.subobjects[0].set_stroke_width(4.0, true);
    mathlike_logo = mathlike_logo.next_to_point((1920.0, 1080.0), (-1.0, -1.0), 30.0, (0.0, 0.0), true);
    sn.add(mathlike_logo);
    let mut title = tex_to_vector("MathLikeAnim-rs".to_string()).await;
    title = title.scale(150.0 / title.get_height(), true);
    title = title.next_to_point((960.0, 0.0), (0.0, 1.0), 30.0, (0.0, 0.0), true);
    title.index = 1;
    sn.add(title.clone());
    let mut subtitle = tex_to_vector("A Rust alternative to Manim".to_string()).await;
    subtitle = subtitle.scale(75.0 / subtitle.get_height(), true);
    subtitle = subtitle.next_to_other(&title, (0.0, 1.0), 30.0, (0.0, 0.0), true);
    subtitle.index = 2;
    sn.add(subtitle);
    let img = web_sys::window().unwrap().document().unwrap().get_element_by_id("rust-logo").unwrap().dyn_into::<web_sys::HtmlImageElement>().unwrap();
    let mut rust_logo = rectangle(
        (0.0, 0.0),
        300.0,
        300.0,
        Some((0.0, 0.0, 0.0, 1.0)),
        Some((0.0, 0.0, 0.0, 1.0)),
        Some(0.0),
        Some("butt"),
        Some("miter"),
        Some(3),
        Some(img)
    );
    rust_logo = rust_logo.next_to_point((0.0, 1080.0), (1.0, -1.0), 30.0, (0.0, 0.0), true);
    let center = rust_logo.get_center();
    let width = rust_logo.get_width();
    let height = rust_logo.get_height();
    let top_left_corner = (center.0 - width / 2.0, center.1 - height / 2.0);
    rust_logo = rust_logo.set_image_position(top_left_corner, true);
    sn.add(rust_logo);
    let axes = axes(
        0.0,
        10.0,
        1.0,
        0.0,
        10.0,
        1.0,
        (960.0, 540.0),
        Some(350.0),
        Some(350.0),
        Some((1.0, 1.0, 1.0, 1.0)),
        Some(4.0),
        Some("butt"),
        Some("miter"),
        Some(4),
        Some(true),
        Some(true),
        Some(20.0),
        Some(20.0),
        Some(true),
        Some(true),
        None
    );
    sn.add(axes.clone());
    let plot = plot_in_axes(
        |x| x.powi(2) / 10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        0.001,
        &axes,
        Some(hex_to_color("#FC6255", 1.0)),
        Some(4.0),
        Some("butt"),
        Some("miter"),
        Some(5),
        None
    );
    sn.add(plot);
    let mut sq = square(
        (0.0, 0.0),
        (2.0 * 175.0 * 175.0 as f64).sqrt(),
        Some(hex_to_color("#58C4DD", 1.0)),
        Some(hex_to_color("#58C4DD", 0.7)),
        Some(4.0),
        Some("butt"),
        Some("miter"),
        Some(6),
        None
    );
    sq = sq.next_to_other(&axes, (-1.0, 0.0), 150.0, (0.0, 0.0), true);
    sn.add(sq);
    let mut circ = circle(
        (0.0, 0.0),
        175.0,
        None,
        Some(hex_to_color("#5CD0B3", 1.0)),
        Some(hex_to_color("#5CD0B3", 0.7)),
        Some(4.0),
        Some("butt"),
        Some("miter"),
        Some(7),
        None
    );
    circ = circ.next_to_other(&axes, (1.0, 0.0), 150.0, (0.0, 0.0), true);
    sn.add(circ);
    sn.update();
}