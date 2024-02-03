use mathlikeanim_rs::animations::draw_stroke_then_fill::write;
use mathlikeanim_rs::animations::fade::fade_out;
use mathlikeanim_rs::scene::Scene;
use mathlikeanim_rs::objects::latex_to_vector::latex_to_vector;
use mathlikeanim_rs::objects::vector_object::{VectorFeatures, VectorObject};
use mathlikeanim_rs::utils::{linear, smooth};

fn main() {
    let width = 1920;
    let height = 1080;
    let fps = 60;
    let mut scene = Scene::new(width, height, fps, "reto.mp4".to_string());
    let mut vec_obj = latex_to_vector(
        r#"$$\int_{-\infty}^{\infty} e^{-x^2} dx$$"#,
        None,
        "temp.tex"
    )
        .scale(20.0, true)
        .set_stroke_color((1.0, 1.0, 1.0, 1.0), true)
        .move_to((width as f64 / 2.0, height as f64 / 2.0), true);
    vec_obj.subobjects.sort_by(|a, b| a.get_center().0.partial_cmp(&b.get_center().0).unwrap());
    for subobj in &vec_obj.subobjects {
        scene.add(subobj.clone());
    }
    let indices = vec_obj.subobjects.iter().map(|obj| obj.index).collect::<Vec<usize>>();
    scene.play(
        write(vec_obj.subobjects.len(), 0.4),
        indices.clone(),
        60,
        linear
    );
    scene.wait(60);
    let new_vec_obj = VectorFeatures {
        index: vec_obj.index,
        subobjects: scene.objects.clone(),
        stroke_width: 0.0,
        stroke_color: (1.0, 1.0, 1.0, 1.0),
        fill_color: (1.0, 1.0, 1.0, 1.0),
        line_cap: "butt",
        line_join: "miter",
        points: vec![]
    };
    for subobj in &vec_obj.subobjects {
        scene.remove(subobj.index);
    }
    scene.add(new_vec_obj.clone());
    scene.play(
        vec![fade_out(50.0, (0.0, 0.0))],
        vec![new_vec_obj.index],
        60,
        |t| smooth(t, 10.0)
    );
    scene.wait(60);
    scene.finish();
}