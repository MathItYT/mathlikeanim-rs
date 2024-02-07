use mathlikeanim_rs::{objects::{geometry::{arc::circle, line::line}, vector_object::{VectorFeatures, VectorObject}}, scene::Scene};


fn bounce_ball(ground: VectorFeatures) -> impl Fn(VectorFeatures, f64) -> VectorFeatures {
    let ground_limit = ground.get_center().1 - ground.stroke_width / 2.0;
    let duration_in_frames = 300;
    let anim_func = move |vec_obj: VectorFeatures, t: f64| -> VectorFeatures {
        let time_in_frames = (t * duration_in_frames as f64) as i32;
        let mut result = vec_obj.clone();
        let gravity = 900.0;
        let mut initial_velocity = 0.0;
        for _ in 0..time_in_frames.clone() {
            initial_velocity += gravity / 60.0;
            result = result.shift((0.0, initial_velocity / 60.0), true);
            let bottom = result.get_critical_point((0.0, 1.0));
            if bottom.1 >= ground_limit {
                result = result.shift((0.0, ground_limit - bottom.1), true);
                initial_velocity *= -0.8;
            }
        }
        return result;
    };
    return anim_func;
}

#[async_std::main]
async fn main() {
    let mut scene = Scene::new(1920, 1080, 60, "physics_example.mp4");
    let ball = circle(
        (1920.0 / 2.0, 100.0),
        10.0,
        None,
        Some((0.0, 0.0, 0.0, 0.0)),
        Some((1.0, 0.0, 0.0, 1.0)),
        Some(0.0),
        None,
        None,
        None
    );
    let ground = line(
        (100.0, 0.0),
        (1820.0, 0.0),
        Some((1.0, 1.0, 1.0, 1.0)),
        Some(8.0),
        None,
        None,
        None
    ).move_to((960.0, 1000.0), true);
    scene.add(ball.clone());
    scene.add(ground.clone());
    scene.play(
        vec![bounce_ball(ground.clone())],
        vec![ball.index],
        300,
        |t| t
    ).await;
    scene.finish();
}