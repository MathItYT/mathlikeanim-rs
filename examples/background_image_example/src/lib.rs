use mathlikeanim_rs::{animations::grow_from_center::grow_from_center, objects::geometry::arc::circle, scene::Scene, utils::{log, sleep, smooth}};
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;


static mut SCENE: Lazy<Scene> = Lazy::new(|| Scene::new(1920, 1080, 60, ""));


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
    let image = document.get_element_by_id("image").unwrap().dyn_into::<web_sys::HtmlImageElement>().unwrap();
    log("Loaded image");
    let circ = circle(
        (960.0, 540.0),
        520.0,
        Some(10),
        Some((1.0, 1.0, 1.0, 1.0)),
        Some((1.0, 1.0, 1.0, 1.0)),
        Some(4.0),
        Some("butt"),
        Some("miter"),
        Some(0),
        Some(image)
    );
    sn.add(circ);
    sn.play(
        vec![grow_from_center],
        vec![0],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.wait(60).await;
}
