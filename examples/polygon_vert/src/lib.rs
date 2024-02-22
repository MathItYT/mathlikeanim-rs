use std::f64::consts::PI;

use mathlikeanim_rs::{animations::{grow_arrow::grow_arrow_with_final_tip, move_camera_svg::move_camera_svg}, objects::{geometry::{add_tip::add_final_tip, line::line, poly::regular_polygon}, svg_to_vector::svg_to_vector, vector_object::{VectorFeatures, VectorObject}}, svg_scene::SVGScene, utils::{hex_to_color, integer_interpolate, interpolate, smooth}};
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;


static mut SCENE: Lazy<SVGScene> = Lazy::new(|| SVGScene::new(1920, 1080, 60, "svg-container"));
static mut NEW_TOP_LEFT_CORNER: (f64, f64) = (0.0, 0.0);
static mut NEW_BOTTOM_RIGHT_CORNER: (f64, f64) = (0.0, 0.0);


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


pub fn draw_stroke_then_fill_anim(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let mut new_vec_obj = vec_obj.clone();
    let (index, subalpha) = integer_interpolate(0.0, 2.0, t);
    if index == 0 {
        let stroke_width = if vec_obj.stroke_width == 0.0 {
            4.0 / 1920.0
        } else {
            vec_obj.stroke_width
        };
        new_vec_obj = vec_obj
            .set_stroke_width(stroke_width, false)
            .get_partial_copy(0.0, subalpha, false)
            .set_fill_opacity(0.0, false)
            .set_subobjects(vec_obj.subobjects.iter().map(|subobj| {
                draw_stroke_then_fill_anim(subobj.clone(), t)
            }).collect());
        return new_vec_obj;
    } else if index == 1 {
        let vec_obj = vec_obj.clone();
        let stroke_width = if vec_obj.stroke_width == 0.0 {
            interpolate(4.0 / 1920.0, 0.0, subalpha)
        } else {
            vec_obj.stroke_width
        };
        new_vec_obj = vec_obj
            .set_stroke_width(stroke_width, false)
            .set_fill_opacity(interpolate(0.0, vec_obj.fill_color.3, subalpha), false)
            .set_subobjects(vec_obj.subobjects.iter().map(|subobj| {
                draw_stroke_then_fill_anim(subobj.clone(), t)
            }).collect());
        return new_vec_obj;
    }
    return new_vec_obj;
}


pub fn move_camera_anim(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let top_left_corner = unsafe { NEW_TOP_LEFT_CORNER };
    let bottom_right_corner = unsafe { NEW_BOTTOM_RIGHT_CORNER };
    let sn = unsafe { &mut SCENE };
    move_camera_svg(top_left_corner, bottom_right_corner, sn, t);
    let scale = (sn.get_bottom_right_corner().0 - sn.get_top_left_corner().0) / 1920.0;
    return vec_obj.set_stroke_width(8.0 * scale, true);
}


#[wasm_bindgen(start)]
pub async fn start() {
    let sn = unsafe { &mut SCENE };
    sn.set_background_color((0.0, 0.0, 0.0, 0.0));
    slide0().await;
}


#[wasm_bindgen(js_name = previousSlide)]
pub async fn previous_slide(n: usize) {
    let sn = unsafe { &mut SCENE };
    sn.restore(n - 1);
    sn.update();
}


#[wasm_bindgen(js_name = nextSlide)]
pub async fn next_slide(n: usize) {
    if n == 0 {
        slide1().await;
    } else if n == 1 {
        slide2().await;
    }
}


async fn slide0() {
    let sn = unsafe { &mut SCENE };
    let almost_circ = regular_polygon(
        (960.0, 540.0),
        500.0 * (PI / 100.0).sin(),
        100,
        Some(hex_to_color("#FC6255", 1.0)),
        Some(hex_to_color("#FC6255", 0.7)),
        Some(8.0),
        Some("butt"),
        Some("miter"),
        Some(0),
        None
    );
    sn.add(almost_circ.clone());
    let center = (960.0, 540.0);
    let height = 500.0;
    let top_point = (center.0, center.1 - height / 2.0);
    let new_top_left_corner = (top_point.0 - 1.0, top_point.1 - 9.0 / 16.0);
    let new_bottom_right_corner = (top_point.0 + 1.0, top_point.1 + 9.0 / 16.0);
    unsafe {
        NEW_TOP_LEFT_CORNER = new_top_left_corner;
        NEW_BOTTOM_RIGHT_CORNER = new_bottom_right_corner;
    }
    sn.play(
        vec![move_camera_anim],
        vec![0],
        180,
        |t| smooth(t, 10.0)
    ).await;
    sn.update();
    sn.save_state(0);
}


async fn slide1() {
    let sn = unsafe { &mut SCENE };
    let top_point = (960.0, 540.0 - 250.0);
    let mut arrow = line(
        (top_point.0, top_point.1 - 4.0 / 16.0),
        (top_point.0, top_point.1 - 1.0 / 16.0),
        Some((1.0, 1.0, 1.0, 1.0)),
        Some(16.0 / 1920.0),
        Some("butt"),
        Some("miter"),
        Some(1),
        None
    );
    arrow = add_final_tip(
        arrow,
        0.75 / 16.0,
        (1.0, 1.0, 1.0, 1.0)
    );
    sn.add(arrow);
    sn.play(
        vec![grow_arrow_with_final_tip],
        vec![1],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.update();
    sn.save_state(1);
}


async fn slide2() {
    let sn = unsafe { &mut SCENE };
    let arrow = sn.get_objects_from_indices(vec![1])[&1].clone();
    let mut vec_obj = tex_to_vector("Vértice".to_string()).await;
    vec_obj = vec_obj.scale(1.5 / (16.0 * vec_obj.get_height()), true);
    vec_obj = vec_obj.next_to_other(&arrow, (0.0, -1.0), 0.5 / 16.0, (0.0, 0.0), true);
    vec_obj.index = 2;
    sn.add(vec_obj);
    sn.play(
        vec![draw_stroke_then_fill_anim],
        vec![2],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.update();
    sn.save_state(2);
}
