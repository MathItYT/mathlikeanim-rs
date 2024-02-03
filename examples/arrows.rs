use std::f64::consts::PI;

use mathlikeanim_rs::{animations::grow_arrow::{grow_arrow_with_final_tip, grow_arrow_with_initial_tip, grow_arrow_with_tips_at_both_ends}, objects::{geometry::{add_tip::{add_both_sides_tips, add_final_tip, add_initial_tip}, arc::{arc, circle}, line::line}, vector_object::{VectorFeatures, VectorObject}}, scene::Scene, utils::smooth};

fn main() {
    let mut scene = Scene::new(1920, 1080, 60, "arrows.mp4".to_string());
    let mut arrows = VectorFeatures {
        index: 0,
        subobjects: vec![],
        stroke_width: 0.0,
        stroke_color: (0.0, 0.0, 0.0, 0.0),
        fill_color: (0.0, 0.0, 0.0, 0.0),
        line_cap: "butt",
        line_join: "miter",
        points: vec![]
    };
    let line1 = line(
        (1920.0 / 2.0, 1080.0 / 2.0),
        (1920.0 / 2.0 + 100.0, 1080.0 / 2.0),
        Some((1.0, 1.0, 1.0, 1.0)),
        Some(8.0),
        None,
        None,
        None,
    );
    let arrow1 = add_initial_tip(
        line1.clone(),
        20.0,
        (1.0, 1.0, 1.0, 1.0)
    );
    arrows.subobjects.push(arrow1);
    let arc1 = arc(
        (1920.0 / 2.0, 1080.0 / 2.0),
        100.0,
        0.0,
        PI / 3.0,
        None,
        Some((1.0, 1.0, 1.0, 1.0)),
        Some((0.0, 0.0, 0.0, 0.0)),
        Some(8.0),
        None,
        None,
        None,
    );
    let arrow2 = add_both_sides_tips(
        arc1.clone(),
        20.0,
        (1.0, 1.0, 1.0, 1.0)
    );
    arrows.subobjects.push(arrow2);
    let circ1 = circle(
        (1920.0 / 2.0, 1080.0 / 2.0),
        100.0,
        None,
        Some((1.0, 1.0, 1.0, 1.0)),
        Some((0.0, 0.0, 0.0, 0.0)),
        Some(8.0),
        None,
        None,
        None
    );
    let arrow3 = add_final_tip(
        circ1.clone(),
        20.0,
        (1.0, 1.0, 1.0, 1.0)
    );
    arrows.subobjects.push(arrow3);
    arrows = arrows.arrange_subobjects((1.0, 0.0), 100.0, (0.0, 0.0), true)
        .move_to((1920.0 / 2.0, 1080.0 / 2.0), true);
    let arrows_subobjects = arrows.subobjects.clone();
    for subobj in arrows_subobjects {
        scene.add(subobj);
    }
    let arrows_indices = scene.objects.iter().map(|arrow| arrow.index).collect::<Vec<usize>>();
    scene.play(
        vec![
            grow_arrow_with_initial_tip,
            grow_arrow_with_tips_at_both_ends,
            grow_arrow_with_final_tip
        ],
        arrows_indices,
        60,
        |t| smooth(t, 10.0)
    );
    scene.wait(60);
    scene.finish();
}
