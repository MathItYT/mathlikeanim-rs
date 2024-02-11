use mathlikeanim_rs::{animations::{grow_from_center::grow_from_center, shift_animation::shift_animation, shift_image_position::shift_image_position}, objects::{geometry::arc::circle, vector_object::VectorFeatures}, scene::Scene, utils::{log, smooth}};
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;


static mut SCENE: Lazy<Scene> = Lazy::new(|| Scene::new(1920, 1080, 60, ""));


pub fn shift_anim(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    return shift_animation((300.0, 0.0))(vec_obj, t);
}


pub fn shift_img_pos(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    return shift_image_position((300.0, 0.0))(vec_obj, t);
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
    sn.play(
        vec![shift_anim, shift_img_pos],
        vec![0, 0],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.wait(60).await;
}
