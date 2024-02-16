use std::{f64::consts::PI, vec};

use mathlikeanim_rs::{animations::{draw_stroke_then_fill::{draw_stroke_then_fill, write}, fade::{fade_in, fade_out}, morph::morph, scale_in_place::scale_in_place, set_fill_animation::set_fill_animation, show_temporarily::show_temporarily, spinning_grow::spinning_grow}, objects::{geometry::poly::{rectangle, square}, svg_to_vector::svg_to_vector, vector_object::{VectorFeatures, VectorObject}}, scene::Scene, utils::{hex_to_color, interpolate, interpolate_color, linear, smooth}};
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;


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


pub fn square_rotate_scale(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let new_vec_obj = vec_obj.rotate(PI / 4.0 * t, true)
        .scale(interpolate(1.0, 0.5, t), true)
        .move_to((960.0, 540.0), true);
    return new_vec_obj;
}


pub fn morph_letter_a(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let sn = unsafe { &mut SCENE };
    let mut target = sn.get_objects_from_indices(vec![4])[&4].clone();
    target = target.set_fill_opacity(1.0, true);
    return morph(target)(vec_obj, t);
}


pub fn morph_equals_sign(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let sn = unsafe { &mut SCENE };
    let mut target = sn.get_objects_from_indices(vec![8])[&8].clone();
    target = target.set_fill_opacity(1.0, true);
    return morph(target)(vec_obj, t);
}


pub fn morph_l(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let sn = unsafe { &mut SCENE };
    let mut target = sn.get_objects_from_indices(vec![9])[&9].clone();
    target = target.set_fill_opacity(1.0, true);
    return morph(target)(vec_obj, t);
}


pub fn morph_2(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let sn = unsafe { &mut SCENE };
    let mut target = sn.get_objects_from_indices(vec![10])[&10].clone();
    target = target.set_fill_opacity(1.0, true);
    return morph(target)(vec_obj, t);
}


pub async fn update_side_area_tex() {
    let sn = unsafe { &mut SCENE };
    let square = sn.get_objects_from_indices(vec![0])[&0].clone();
    let l = (square.get_height() / 175.0 * 2.0 * 100.0).round() / 100.0;
    let a = (l * l * 100.0).round() / 100.0;
    let mut side_area_tex = tex_to_vector(format!("\\begin{{align*}} &l = {} \\\\ &A = {} \\end{{align*}}", l, a)).await;
    side_area_tex = side_area_tex.scale(150.0 / side_area_tex.get_height(), true);
    side_area_tex = side_area_tex.next_to_other(&square, (1.0, 0.0), 50.0, (0.0, 0.0), true);
    side_area_tex.index = 1;
    sn.add(side_area_tex);
}


pub fn update_side_area_tex_sync() {
    async_std::task::block_on(update_side_area_tex());
}


pub fn temp_animation(vec_obj: VectorFeatures, _: f64) -> VectorFeatures {
    update_side_area_tex_sync();
    return vec_obj;
}


pub fn scale_double(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    return scale_in_place(2.0)(vec_obj, t);
}


pub fn turn_first_subobject_fill_blue(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    let mut new_vec_obj = vec_obj;
    new_vec_obj.subobjects[0] = new_vec_obj.subobjects[0].set_fill_color(interpolate_color((1.0, 1.0, 1.0, 1.0), hex_to_color("#236B8E", 1.0), t), true);
    return new_vec_obj;
}


pub fn fade_in_fn(vec_obj: VectorFeatures, t: f64) -> VectorFeatures {
    return fade_in(1.0, (0.0, 0.0))(vec_obj, t);
}


#[wasm_bindgen(start)]
pub async fn start() {
    let sn = unsafe { &mut SCENE };
    sn.set_background_color((34.0 / 255.0, 34.0 / 255.0, 34.0 / 255.0, 1.0));
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let context = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    sn.init_context(context);
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
    } else if n == 2 {
        slide3().await;
    } else if n == 3 {
        slide4().await;
    } else if n == 4 {
        slide5().await;
    } else if n == 5 {
        slide6().await;
    } else if n == 6 {
        slide7().await;
    } else if n == 7 {
        slide8().await;
    } else if n == 8 {
        slide9().await;
    } else if n == 9 {
        slide10().await;
    } else if n == 10 {
        slide11().await;
    } else if n == 11 {
        slide12().await;
    } else if n == 12 {
        slide13().await;
    }
}


#[wasm_bindgen]
pub async fn slide0() {
    let sn = unsafe { &mut SCENE };
    let mut mathlike_logo = mathlike_to_vector().await;
    mathlike_logo = mathlike_logo.scale(350.0 / mathlike_logo.get_width(), true);
    mathlike_logo.subobjects[0] = mathlike_logo.subobjects[0].set_stroke_width(4.0, true);
    mathlike_logo = mathlike_logo.shift((960.0 - mathlike_logo.get_center().0, 540.0 - mathlike_logo.get_center().1), true);
    sn.add(mathlike_logo);
    sn.play(
        vec![spinning_grow(PI / 2.0)],
        vec![0],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.update();
    sn.save_state(0);
}


#[wasm_bindgen]
pub async fn slide1() {
    let sn = unsafe { &mut SCENE };
    sn.clear();
    let mut title = tex_to_vector("¿Qué son las funciones?".to_string()).await;
    title = title.scale(1500.0 / title.get_width(), true);
    title = title.shift((960.0 - title.get_center().0, 540.0 - title.get_center().1), true);
    sn.add(title);
    sn.play(
        vec![draw_stroke_then_fill],
        vec![0],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.update();
    sn.save_state(1);
}


#[wasm_bindgen]
pub async fn slide2() {
    let sn = unsafe { &mut SCENE };
    sn.play(
        vec![fade_out(10.0, (0.0, 0.0))],
        vec![0],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.remove(0);
    let sq = square(
        (960.0, 540.0),
        350.0,
        Some(hex_to_color("#58C4DD", 1.0)),
        Some(hex_to_color("#58C4DD", 0.7)),
        Some(8.0),
        None,
        None,
        None,
        None
    );
    sn.add(sq);
    sn.play(
        vec![fade_in(0.0, (0.0, 0.0))],
        vec![0],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.update();
    sn.save_state(2);
}


#[wasm_bindgen]
pub async fn slide3() {
    let sn = unsafe { &mut SCENE };
    sn.play(
        vec![square_rotate_scale],
        vec![0],
        60,
        |t| smooth(t, 10.0)
    ).await;
    update_side_area_tex().await;
    sn.play(
        vec![fade_in(1.5, (0.0, 0.0))],
        vec![1],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.update();
    sn.save_state(3);
}


#[wasm_bindgen]
pub async fn slide4() {
    let sn = unsafe { &mut SCENE };
    let temp_obj = VectorFeatures {
        points: vec![],
        subobjects: vec![],
        stroke_color: (0.0, 0.0, 0.0, 0.0),
        fill_color: (0.0, 0.0, 0.0, 0.0),
        stroke_width: 0.0,
        index: 2,
        background_image: None,
        image_position: (0.0, 0.0),
        line_cap: "butt",
        line_join: "miter"
    };
    sn.add(temp_obj);
    sn.play(
        vec![scale_double, temp_animation],
        vec![0, 2],
        60,
        |t| smooth(t, 10.0)
    ).await;
    update_side_area_tex().await;
    sn.update();
    sn.save_state(4);
}


#[wasm_bindgen]
pub async fn slide5() {
    let sn = unsafe { &mut SCENE };
    let mut area_formula = tex_to_vector("$$A = l\\times l$$".to_string()).await;
    area_formula = area_formula.scale(60.0 / area_formula.get_height(), true);
    area_formula = area_formula.next_to_point(
        (960.0, 1080.0),
        (0.0, -1.0),
        100.0,
        (0.0, 0.0),
        true
    );
    area_formula.index = 3;
    for (i, subobj) in area_formula.subobjects.iter_mut().enumerate() {
        subobj.index = i + 3;
        sn.add(subobj.clone());
    }
    let indices = (3..3 + area_formula.subobjects.len()).collect::<Vec<usize>>();
    sn.play(
        write(area_formula.subobjects.len(), 0.4),
        indices.clone(),
        60,
        linear
    ).await;
    for index in indices {
        sn.remove(index);
    }
    sn.add(area_formula);
    sn.update();
    sn.save_state(5);
}


#[wasm_bindgen]
pub async fn slide6() {
    let sn = unsafe { &mut SCENE };
    let mut area_formula = tex_to_vector("$$A = l^2$$".to_string()).await;
    area_formula = area_formula.scale(60.0 / area_formula.get_height(), true);
    area_formula = area_formula.next_to_point(
        (960.0, 1080.0),
        (0.0, -1.0),
        100.0,
        (0.0, 0.0),
        true
    );
    area_formula.index = 3;
    sn.play(
        vec![morph(area_formula)],
        vec![3],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.update();
    sn.save_state(6);
}


#[wasm_bindgen]
pub async fn slide7() {
    let sn = unsafe { &mut SCENE };
    let area_formula = sn.get_objects_from_indices(vec![3])[&3].clone();
    let rect = rectangle(
        area_formula.get_center(),
        area_formula.get_width() + 15.0,
        area_formula.get_height() + 15.0,
        Some(hex_to_color("#FFFF00", 1.0)),
        Some((0.0, 0.0, 0.0, 0.0)),
        Some(6.0),
        None,
        None,
        Some(4),
        None
    );
    sn.add(rect);
    sn.play(
        vec![show_temporarily],
        vec![4],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.remove(4);
    sn.update();
    sn.save_state(7);
}


#[wasm_bindgen]
pub async fn slide8() {
    let sn = unsafe { &mut SCENE };
    sn.play(
        vec![fade_out(1.0, (0.0, 0.0)), fade_out(1.0, (0.0, 0.0)), fade_out(1.0, (0.0, 0.0)), fade_out(1.0, (0.0, 0.0))],
        vec![0, 1, 2, 3],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.clear();
    let mut expr = tex_to_vector("$$A = l^2$$".to_string()).await;
    expr = expr.scale(300.0 / expr.get_height(), true);
    expr = expr.move_to((960.0, 540.0), true);
    expr.subobjects.sort_by(|a, b| a.get_center().0.partial_cmp(&b.get_center().0).unwrap());
    sn.add(expr);
    sn.play(
        vec![fade_in(1.5, (0.0, 0.0))],
        vec![0],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.update();
    sn.save_state(8);
}


#[wasm_bindgen]
pub async fn slide9() {
    let sn = unsafe { &mut SCENE };
    let mut expr = sn.get_objects_from_indices(vec![0])[&0].clone();
    sn.remove(0);
    for (i, subobj) in expr.subobjects.iter_mut().enumerate() {
        subobj.index = i;
        sn.add(subobj.clone());
    }
    sn.play(
        vec![set_fill_animation(hex_to_color("#FFFF00", 1.0))],
        vec![2],
        60,
        |t| smooth(t, 10.0),
    ).await;
    sn.update();
    sn.save_state(9);
}


#[wasm_bindgen]
pub async fn slide10() {
    let sn = unsafe { &mut SCENE };
    let mut new_expr = tex_to_vector("$$A(l) = l^2$$".to_string()).await;
    new_expr = new_expr.scale(300.0 / new_expr.get_height(), true);
    new_expr = new_expr.move_to((960.0, 540.0), true);
    new_expr.subobjects.sort_by(|a, b| a.get_center().0.partial_cmp(&b.get_center().0).unwrap());
    new_expr.subobjects[2] = new_expr.subobjects[2].set_fill_color(hex_to_color("#FFFF00", 1.0), true);
    new_expr.subobjects[5] = new_expr.subobjects[5].set_fill_color(hex_to_color("#FFFF00", 1.0), true);
    for (i, subobj) in new_expr.subobjects.iter_mut().enumerate() {
        subobj.index = i + 4;
        *subobj = subobj.set_fill_opacity(0.0, true);
        sn.add(subobj.clone());
    };
    sn.play(
        vec![morph_letter_a, morph_equals_sign, morph_l, morph_2],
        vec![0, 1, 2, 3],
        60,
        |t| smooth(t, 10.0)
    ).await;
    for i in 0..4 {
        sn.remove(i);
    }
    for i in vec![4, 5, 6, 7, 8, 9, 10] {
        let obj = sn.get_objects_from_indices(vec![i])[&i].clone();
        sn.add(obj.set_fill_opacity(1.0, true));
    }
    sn.play(
        vec![draw_stroke_then_fill, draw_stroke_then_fill, draw_stroke_then_fill],
        vec![5, 6, 7],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.clear();
    for (i, subobj) in new_expr.subobjects.iter_mut().enumerate() {
        subobj.index = i;
    }
    new_expr = new_expr.set_fill_opacity(1.0, true);
    sn.add(new_expr);
    sn.update();
    sn.save_state(10);
}


#[wasm_bindgen]
pub async fn slide11() {
    let sn = unsafe { &mut SCENE };
    let mut l_belongs_r_plus = tex_to_vector("$$l \\in \\mathbb{R}^+$$".to_string()).await;
    l_belongs_r_plus = l_belongs_r_plus.scale(100.0 / l_belongs_r_plus.get_height(), true);
    l_belongs_r_plus = l_belongs_r_plus.next_to_point(
        (960.0, 1080.0),
        (0.0, -1.0),
        100.0,
        (0.0, 0.0),
        true
    );
    l_belongs_r_plus.index = 1;
    l_belongs_r_plus.subobjects.sort_by(|a, b| a.get_center().0.partial_cmp(&b.get_center().0).unwrap());
    l_belongs_r_plus.subobjects[0] = l_belongs_r_plus.subobjects[0].set_fill_color(hex_to_color("#FFFF00", 1.0), true);
    sn.add(l_belongs_r_plus);
    sn.play(
        vec![fade_in(1.5, (0.0, 0.0))],
        vec![1],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.update();
    sn.save_state(11);
}


#[wasm_bindgen]
pub async fn slide12() {
    let sn = unsafe { &mut SCENE };
    let mut a_belongs_r_plus = tex_to_vector("$$A(l) \\in \\mathbb{R}^+$$".to_string()).await;
    a_belongs_r_plus = a_belongs_r_plus.scale(100.0 / a_belongs_r_plus.get_height(), true);
    a_belongs_r_plus = a_belongs_r_plus.next_to_point(
        (960.0, 1080.0),
        (0.0, -1.0),
        100.0,
        (0.0, 0.0),
        true
    );
    a_belongs_r_plus.index = 2;
    a_belongs_r_plus.subobjects.sort_by(|a, b| a.get_center().0.partial_cmp(&b.get_center().0).unwrap());
    a_belongs_r_plus.subobjects[0] = a_belongs_r_plus.subobjects[0].set_fill_color(hex_to_color("#236B8E", 1.0), true);
    a_belongs_r_plus.subobjects[2] = a_belongs_r_plus.subobjects[2].set_fill_color(hex_to_color("#FFFF00", 1.0), true);
    a_belongs_r_plus = a_belongs_r_plus.set_fill_opacity(0.0, true);
    sn.add(a_belongs_r_plus.clone());
    let mut l_belongs_r_plus = sn.get_objects_from_indices(vec![1])[&1].clone();
    l_belongs_r_plus = l_belongs_r_plus.next_to_other(&a_belongs_r_plus, (0.0, -1.0), 10.0, (0.0, 0.0), true);
    l_belongs_r_plus.index = 1;
    sn.play(
        vec![morph(l_belongs_r_plus)],
        vec![1],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.update();
    sn.save_state(12);
}


#[wasm_bindgen]
pub async fn slide13() {
    let sn = unsafe { &mut SCENE };
    let mut a_belongs_r_plus = sn.get_objects_from_indices(vec![2])[&2].clone();
    a_belongs_r_plus = a_belongs_r_plus.set_fill_opacity(1.0, true);
    sn.add(a_belongs_r_plus);
    sn.play(
        vec![turn_first_subobject_fill_blue, fade_in_fn],
        vec![0, 2],
        60,
        |t| smooth(t, 10.0)
    ).await;
    sn.update();
    sn.save_state(13);
}
