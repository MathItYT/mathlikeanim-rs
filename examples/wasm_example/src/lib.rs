use mathlikeanim_rs::{animations::draw_stroke_then_fill::write, objects::{geometry::{arc::circle, poly::square}, svg_to_vector::svg_to_vector}, utils::{linear, log}};
pub use mathlikeanim_rs::{animations::morph::morph, objects::{latex_to_vector::latex_to_vector, vector_object::VectorObject}, scene::Scene, utils::smooth};
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;


static mut SCENE: Lazy<Scene> = Lazy::new(|| Scene::new(1920, 1080, 60, ""));


#[wasm_bindgen(module = "tex2svg")]
extern "C" {
    #[wasm_bindgen(js_name = tex2svg)]
    pub async fn tex2svg(latex: JsValue) -> JsValue;
}


pub async fn tex_to_svg(latex: &str) -> String {
    let tex = JsValue::from_str(latex);
    let svg = tex2svg(tex).await;
    return svg.as_string().unwrap();
}


#[wasm_bindgen(start)]
pub async fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    log("Loaded document");
    let canvas = document.get_element_by_id("canvas").unwrap();
    log("Loaded canvas");
    let context = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    log("Got context");
    let sn = unsafe { &mut SCENE };
    sn.init_context(context);
    log("Initialized context");
    let circ = circle(
        (1920.0 / 2.0, 1080.0 / 2.0),
        200.0,
        None,
        Some((1.0, 0.0, 0.0, 1.0)),
        Some((1.0, 0.0, 0.0, 0.5)),
        None,
        None,
        None,
        None
    );
    sn.add(circ.clone());
    sn.wait(60).await;
    let sq = square(
        (1920.0 / 2.0, 1080.0 / 2.0),
        200.0,
        Some((0.0, 0.0, 1.0, 1.0)),
        Some((0.0, 0.0, 1.0, 0.5)),
        None,
        None,
        None,
        None
    );
    sn.play(
        vec![morph(sq.clone())],
        vec![circ.clone().index],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.remove(circ.index);
    sn.add(sq.clone());
    let mut txt = svg_to_vector(tex_to_svg(r"$$\int_0^\infty e^{-x^2} dx$$").await.as_str());
    txt = txt.set_stroke_color((1.0, 1.0, 1.0, 1.0), true);
    txt = txt.scale(200.0 / txt.get_height(), true);
    txt = txt.move_to((1920.0 / 2.0, 1080.0 / 2.0), true).shift((0.0, 400.0), true);
    txt.subobjects.sort_by(|a, b| a.get_center().0.partial_cmp(&b.get_center().0).unwrap());
    let number_of_subobjects = txt.subobjects.len();
    for subobj in &txt.subobjects {
        sn.add(subobj.clone());
    }
    let indices = sn.objects[sn.objects.len() - number_of_subobjects..].iter().map(|obj| obj.index).collect::<Vec<usize>>();
    sn.play(
        write(number_of_subobjects, 0.4),
        indices.clone(),
        60,
        linear
    ).await;
    sn.wait(60).await;
}


#[wasm_bindgen(js_name = randomDot)]
pub async fn random_dot() {
    let x = js_sys::Math::random() * 1920.0;
    let y = js_sys::Math::random() * 1080.0;
    let r = js_sys::Math::random();
    let g = js_sys::Math::random();
    let b = js_sys::Math::random();
    let a = js_sys::Math::random();
    let dot = circle(
        (x, y),
        20.0,
        None,
        Some((r, g, b, a)),
        Some((r, g, b, a)),
        None,
        None,
        None,
        None
    );
    let sn = unsafe { &mut SCENE };
    sn.add(dot);
    sn.wait(1).await; // To make sure the dot is drawn
}
