use mathlikeanim_rs::{animations::morph::morph, objects::{latex_to_vector::latex_to_vector, vector_object::VectorObject}, scene::Scene, utils::smooth};

fn main() {
    let mut scene = Scene::new(1920, 1080, 60, Some("morph_example.mp4".to_string()));
    let vec_obj1 = latex_to_vector(
        r#"$$\int_{-\infty}^{\infty} e^{-x^2} dx$$"#,
        None,
        "temp.tex"
    )
        .scale(20.0, true)
        .set_stroke_color((1.0, 1.0, 1.0, 1.0), true)
        .move_to((1920.0 / 2.0, 1080.0 / 2.0), true);
    let vec_obj2 = latex_to_vector(
        "$$x^2 + y^2 = 1$$",
        None,
        "temp2.tex"
    )
        .scale(20.0, true)
        .set_stroke_color((1.0, 1.0, 1.0, 1.0), true)
        .move_to((1920.0 / 2.0, 1080.0 / 2.0), true);
    scene.add(vec_obj1.clone());
    scene.play(
        vec![morph(vec_obj2.clone())],
        vec![vec_obj1.clone().index],
        60,
        |t| smooth(t, 10.0)
    );
    scene.remove(vec_obj1.index);
    scene.add(vec_obj2.clone());
    scene.wait(60);
    scene.finish();
}