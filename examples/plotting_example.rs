use std::f64::consts::PI;

use mathlikeanim_rs::{animations::create::create, objects::{plotting::functions::{function, parametric_function}, vector_object::{VectorFeatures, VectorObject}}, scene::Scene, utils::linear};

#[async_std::main]
async fn main() {
    let mut scene = Scene::new(1920, 1080, 60, "plotting_example.mp4");
    let f = |x: f64| x.sin();
    let g = |x: f64| x.cos();
    let h = |t: f64| (t.cos(), t.sin());
    let mut f_obj = function(
        f,
        -10.0,
        10.0,
        0.001,
        None,
        None,
        None,
        None,
        None
    );
    f_obj = f_obj.scale(300.0 / f_obj.get_width(), true);
    let mut g_obj = function(
        g,
        -10.0,
        10.0,
        0.001,
        None,
        None,
        None,
        None,
        None
    );
    g_obj = g_obj.scale(300.0 / g_obj.get_width(), true);
    let mut h_obj = parametric_function(
        h,
        0.0,
        2.0 * PI,
        0.001,
        None,
        None,
        None,
        None,
        None
    );
    h_obj = h_obj.scale(300.0 / h_obj.get_width(), true);
    let arranged = VectorFeatures {
        points: vec![],
        fill_color: (0.0, 0.0, 0.0, 0.0),
        stroke_color: (0.0, 0.0, 0.0, 0.0),
        stroke_width: 0.0,
        line_cap: "butt",
        line_join: "miter",
        index: 0,
        subobjects: vec![f_obj, g_obj, h_obj]
    }
        .arrange_subobjects((1.0, 0.0), 100.0, (0.0, 0.0), true)
        .move_to((1920.0 / 2.0, 1080.0 / 2.0), true);
    let new_func_objs = arranged.subobjects;
    for obj in new_func_objs {
        scene.add(obj);
    }
    let indices = scene.objects.iter().map(|obj| obj.index).collect::<Vec<usize>>();
    scene.play(
        vec![create, create, create],
        indices,
        60,
        linear
    ).await;
    scene.wait(60).await;
    scene.finish();
}