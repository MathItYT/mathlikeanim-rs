use std::f64::consts::PI;

use mathlikeanim_rs::{animations::create::create, colors::{GradientImageOrColor, GradientStop, LinearGradient}, objects::{geometry::arc::circle, svg_to_vector::svg_to_vector, vector_object::{VectorFeatures, VectorObject}}, scene::Scene, scene_api::SceneAPI, utils::{hex_to_color, smooth}};
use wasm_bindgen::prelude::*;
use once_cell::sync::Lazy;


static mut SCENE: Lazy<Scene> = Lazy::new(|| {
    Scene::new(
        1920,
        1080,
        144
    )
});


#[wasm_bindgen(module = "/text2path.js")]
extern "C" {
    async fn text2path(text: &str) -> JsValue;
}


async fn text_to_vector(text: &str) -> VectorFeatures {
    let svg = text2path(text).await;
    let vector = svg_to_vector(svg.as_string().unwrap().as_str())
        .set_fill(GradientImageOrColor::Color(hex_to_color("#FCFCFC", 1.0)), true)
        .set_stroke_width(0.0, true)
        .scale(2.0, true)
        .move_to((960.0, 540.0), true)
        .set_index(1);
    return vector;
}


fn draw_text(vector_list: Vec<VectorFeatures>, t: f64) -> VectorFeatures {
    let vector = vector_list[(t * 100.0).round() as usize].clone();
    return vector;
}


#[wasm_bindgen(start)]
pub async fn start() {
    let scene = unsafe { &mut SCENE };
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    scene.context = Some(context);
    let grey = hex_to_color("#121212", 1.0);
    scene.set_background(GradientImageOrColor::Color(grey));
}


#[wasm_bindgen]
pub async fn draw() {
    let scene = unsafe { &mut SCENE };
    let vector = text_to_vector("0%").await;
    let mut circ = circle(
        (960.0, 540.0),
        250.0,
        None,
        None,
        Some((0.0, 0.0, 0.0, 0.0)),
        Some(30.0),
        Some("round"),
        Some("round"),
        None,
    );
    circ = circ.rotate(-PI / 2.0, true).move_to((960.0, 540.0), true);
    let (x1, y1) = (960.0 + 250.0 * (3.0 * PI / 4.0).cos(), 540.0 - 250.0 * (3.0 * PI / 4.0).sin());
    let (x2, y2) = (960.0 + 250.0 * (7.0 * PI / 4.0).cos(), 540.0 - 250.0 * (7.0 * PI / 4.0).sin());
    let gradient = GradientImageOrColor::LinearGradient(LinearGradient {
        x1,
        y1,
        x2,
        y2,
        stops: vec![
            GradientStop {
                offset: 0.0,
                color: hex_to_color("#FC6255", 1.0)
            },
            GradientStop {
                offset: 1.0,
                color: hex_to_color("#FFFF00", 1.0)
            },
        ],
        alpha: 1.0
    });
    circ = circ.set_stroke(gradient, true);
    scene.add(circ);
    scene.add(vector);
    let mut vector_list = Vec::new();
    for i in 0..101 {
        vector_list.push(text_to_vector(&format!("{}%", i)).await);
    }
    scene.play(
        |vec_objs, t| {
            let mut vec_objs = vec_objs;
            vec_objs[0] = create(vec_objs[0].clone(), t);
            vec_objs[1] = draw_text(vector_list.clone(), t);
            return vec_objs;
        },
        vec![0, 1],
        144,
        |t| smooth(t, 10.0)
    ).await;
}
