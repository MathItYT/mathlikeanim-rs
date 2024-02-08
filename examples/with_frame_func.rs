use std::fs::read_to_string;
use mathlikeanim_rs::{
    objects::latex_to_vector::latex_to_vector, renderer::{render_all_vectors, render_video}, objects::svg_to_vector::svg_to_vector, utils::interpolate, objects::vector_object::VectorObject
};


fn make_frame(frame: u64, width: u64, height: u64, _: u64) -> Option<Vec<u8>> {
    let file_name = "temp.tex";
    let svg = read_to_string(file_name[0..file_name.len() - 4].to_string() + ".svg");
    let vec_obj = if svg.is_ok() {
        svg_to_vector(svg.unwrap().as_str())
    } else {
        latex_to_vector(
            r#"$$\int_{-\infty}^{\infty} e^{-x^2} dx$$"#,
            None,
            file_name
        )
    };
    let vec_obj = vec_obj
        .scale(20.0, true)
        .set_stroke_color((1.0, 1.0, 1.0, 1.0), true)
        .set_stroke_width(interpolate(2.0, 0.0, (frame as f64 - 60.0).max(0.0) / 60.0), true)
        .move_to((width as f64 / 2.0, height as f64 / 2.0), true)
        .get_partial_copy(0.0, interpolate(0.0, 1.0, (frame as f64).min(60.0) / 60.0), true)
        .set_fill_opacity(interpolate(0.0, 1.0, (frame as f64 - 60.0).max(0.0) / 60.0), true);
    let surface_data = render_all_vectors(&vec![vec_obj], width, height, None, (0.0, 0.0, 0.0, 0.0));
    return surface_data;
}


fn main() {
    let width = 1920;
    let height = 1080;
    let fps = 60;
    let total_frames = 120;
    let output_file = "reto.mp4";
    render_video(&mut make_frame, width, height, fps, total_frames, output_file)
}