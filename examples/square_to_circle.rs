use mathlikeanim_rs::{animations::morph::morph, objects::geometry::{arc::circle, poly::square}, scene::Scene, utils::smooth};

fn main() {
    let mut scene = Scene::new(1920, 1080, 60, "square_to_circle.mp4".to_string());
    let circ = circle(
        (1920.0 / 2.0, 1080.0 / 2.0),
        200.0,
        None,
        Some((1.0, 0.0, 0.0, 1.0)),
        Some((1.0, 0.0, 0.0, 0.5)),
        None,
        None,
        None,
        None
    );
    scene.add(circ.clone());
    scene.wait(60);
    let sq = square(
        (1920.0 / 2.0, 1080.0 / 2.0),
        200.0,
        Some((0.0, 0.0, 1.0, 1.0)),
        Some((0.0, 0.0, 1.0, 0.5)),
        None,
        None,
        None,
        None
    );
    scene.play(
        vec![morph(sq.clone())],
        vec![circ.clone().index],
        60,
        |t| smooth(t, 10.0)
    );
    scene.remove(circ.index);
    scene.add(sq.clone());
    scene.wait(60);
    scene.finish();
}
