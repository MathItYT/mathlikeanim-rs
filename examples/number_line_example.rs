use mathlikeanim_rs::{animations::draw_stroke_then_fill::draw_stroke_then_fill, objects::{latex_to_vector::latex_to_vector, plotting::number_line::{get_numbers_tex, number_line}, vector_object::VectorObject}, scene::Scene, utils::smooth};

#[async_std::main]
async fn main() {
    let mut scene = Scene::new(1920, 1080, 60, "number_line_example.mp4");
    let number_line = number_line(
        -10.0,
        10.0,
        1.0,
        None,
        None,
        None,
        None,
        None,
        Some((960.0, 540.0)),
        Some(1800.0),
        Some(true),
        Some(true),
        None
    );
    let tex_to_vector_func = |tex: String| {
        latex_to_vector(
            &tex,
            None,
            "temp.tex"
        )
            .set_fill_color((1.0, 1.0, 1.0, 1.0), true)
            .set_stroke_color((1.0, 1.0, 1.0, 1.0), true)
    };
    let tex_numbers = get_numbers_tex(
        number_line.clone(),
        -10.0,
        10.0,
        1.0,
        &tex_to_vector_func,
        &|x: f64| format!("$${}$$", x),
        30.0,
        None,
        None
    );
    scene.add(number_line.clone());
    scene.add(tex_numbers);
    scene.play(
        vec![draw_stroke_then_fill, draw_stroke_then_fill],
        vec![0, 1],
        60,
        |t| smooth(t, 10.0)
    ).await;
    scene.wait(60).await;
    scene.finish();
}