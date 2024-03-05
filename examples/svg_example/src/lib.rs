use once_cell::sync::Lazy;

use mathlikeanim_rs::{animations::{create::create, draw_stroke_then_fill::draw_stroke_then_fill}, colors::{GradientImageOrColor, GradientStop, LinearGradient}, objects::{plotting::axes::{axes, plot_in_axes, riemann_rectangles_for_plot}, vector_object::VectorObject}, svg_scene::SVGScene, utils::{hex_to_color, smooth}};

use wasm_bindgen::prelude::*;


static mut SVG_SCENE: Lazy<SVGScene> = Lazy::new(|| {
    SVGScene::new(
        1920,
        1080,
        144
    )
});


#[wasm_bindgen(start)]
pub async fn start() {
    let scene = unsafe { &mut SVG_SCENE };
    let document = web_sys::window().unwrap().document().unwrap();
    let div_container = document.get_element_by_id("svg-container")
        .unwrap()
        .dyn_into::<web_sys::HtmlDivElement>()
        .unwrap();
    scene.init_div_container(div_container);
    let grey = hex_to_color("#121212", 1.0);
    scene.set_background(GradientImageOrColor::Color(grey));
}


#[wasm_bindgen]
pub async fn draw() {
    let scene = unsafe { &mut SVG_SCENE };
    let axes = axes(
        0.0,
        10.0,
        1.0,
        0.0,
        10.0,
        1.0,
        (960.0, 540.0),
        Some(600.0),
        Some(600.0),
        Some((1.0, 1.0, 1.0, 1.0)),
        None,
        None,
        None,
        Some(0),
        Some(true),
        Some(true),
        Some(20.0),
        Some(20.0),
        Some(true),
        Some(true)
    );
    scene.add(axes.clone());
    scene.play(
        |vec_objs, t| {
            let mut vec_objs = vec_objs;
            vec_objs[0] = draw_stroke_then_fill(vec_objs[0].clone(), t);
            return vec_objs;
        },
        vec![0],
        144,
        |t| smooth(t, 10.0)
    ).await;
    let mut graph = plot_in_axes(
        |x| x.powi(2) / 10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        0.01,
        &axes,
        None,
        None,
        None,
        None,
        Some(1)
    );
    let x1 = graph.get_center().0 - graph.get_width() / 2.0;
    let y1 = graph.get_center().1 + graph.get_height() / 2.0;
    let x2 = graph.get_center().0 + graph.get_width() / 2.0;
    let y2 = graph.get_center().1 - graph.get_height() / 2.0;
    graph = graph.set_stroke(
        GradientImageOrColor::LinearGradient(
            LinearGradient {
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
            }
        ),
        true
    );
    scene.add(graph);
    scene.play(
        |vec_objs, t| {
            let mut vec_objs = vec_objs;
            vec_objs[0] = create(vec_objs[0].clone(), t);
            return vec_objs;
        },
        vec![1, 0],
        144,
        |t| smooth(t, 10.0)
    ).await;
    let mut riemann_rects = riemann_rectangles_for_plot(
        |x| x.powi(2) / 10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        1.0,
        0.0,
        10.0,
        10,
        &axes,
        Some((0.0, 0.0, 0.0, 1.0)),
        None,
        Some(2.0),
        None,
        None,
        Some(2)
    );
    riemann_rects = riemann_rects.set_fill(
        GradientImageOrColor::LinearGradient(
            LinearGradient {
                x1,
                y1,
                x2,
                y2,
                stops: vec![
                    GradientStop {
                        offset: 0.0,
                        color: hex_to_color("#FC6255", 0.4)
                    },
                    GradientStop {
                        offset: 1.0,
                        color: hex_to_color("#FFFF00", 0.4)
                    },
                ],
                alpha: 1.0
            }
        ),
        true
    );
    scene.add(riemann_rects);
    scene.play(
        |vec_objs, t| {
            let mut vec_objs = vec_objs;
            vec_objs[0] = draw_stroke_then_fill(vec_objs[0].clone(), t);
            return vec_objs;
        },
        vec![2, 1, 0],
        144,
        |t| smooth(t, 10.0)
    ).await;
}


#[wasm_bindgen(js_name = changeNRects)]
pub fn change_n_rects(n_rects: usize) {
    let scene = unsafe { &mut SVG_SCENE };
    let mut riemann_rects = riemann_rectangles_for_plot(
        |x| x.powi(2) / 10.0,
        0.0,
        10.0,
        0.0,
        10.0,
        1.0,
        0.0,
        10.0,
        n_rects,
        &scene.objects[scene.objects.len()-1].clone(),
        Some((0.0, 0.0, 0.0, 1.0)),
        None,
        Some(2.0),
        None,
        None,
        Some(2)
    );
    let x1 = riemann_rects.get_center().0 - riemann_rects.get_width() / 2.0;
    let y1 = riemann_rects.get_center().1 + riemann_rects.get_height() / 2.0;
    let x2 = riemann_rects.get_center().0 + riemann_rects.get_width() / 2.0;
    let y2 = riemann_rects.get_center().1 - riemann_rects.get_height() / 2.0;
    riemann_rects = riemann_rects.set_fill(
        GradientImageOrColor::LinearGradient(
            LinearGradient {
                x1,
                y1,
                x2,
                y2,
                stops: vec![
                    GradientStop {
                        offset: 0.0,
                        color: hex_to_color("#FC6255", 0.4)
                    },
                    GradientStop {
                        offset: 1.0,
                        color: hex_to_color("#FFFF00", 0.4)
                    },
                ],
                alpha: 1.0
            }
        ),
        true
    );
    scene.add(riemann_rects);
    let mut objects_to_add_again = scene.objects.clone();
    objects_to_add_again.sort_by(|a, b| (-(a.index as i32)).cmp(&(-(b.index as i32))));
    for object in objects_to_add_again {
        scene.add(object);
    }
    scene.update();
}
